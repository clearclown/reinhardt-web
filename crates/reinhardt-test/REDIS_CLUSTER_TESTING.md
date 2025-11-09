# Redis Cluster Testing Guide

## Overview

Redis Cluster tests use an **RAII-based automatic cleanup pattern** with **file-based inter-process locking** to ensure reliable execution on both `cargo test` and `cargo nextest`.

## Architecture

### RAII Pattern with RedisClusterGuard

The `RedisClusterGuard` type encapsulates Redis Cluster container lifecycle management:

```rust
pub struct RedisClusterGuard {
    _container: ContainerAsync<GenericImage>,  // Dropped automatically
    urls: Vec<String>,
}

impl Drop for RedisClusterGuard {
    fn drop(&mut self) {
        // Automatic cleanup via TestContainers
    }
}
```

**Benefits:**
- ✅ No manual cleanup required
- ✅ Guaranteed cleanup even on panic
- ✅ Clean separation of concerns

### File-Based Inter-Process Locking

The `redis_cluster()` fixture uses file-based locking for inter-process synchronization:

```rust
#[fixture]
pub async fn redis_cluster() -> RedisClusterGuard {
    // Acquire exclusive file lock (blocks until available)
    let _lock = FileLockGuard::acquire().expect("Failed to acquire lock");

    // Cleanup + container startup
    cleanup_stale_redis_cluster_containers().await;
    // ...
}
```

**Why File Lock?**
- `cargo-nextest` runs each test in a separate **process**
- `std::sync::Mutex` only works within a single process
- File locks work across process boundaries via OS kernel

## Usage

### Basic Test Pattern

```rust
use reinhardt_test::containers::RedisClusterGuard;

#[rstest]
#[serial(redis_cluster)]  // Optional but recommended
#[tokio::test]
async fn test_redis_cluster_operations(
    #[future] redis_cluster: RedisClusterGuard,
) {
    let cluster = redis_cluster.await;

    // Use cluster.urls() to get connection URLs
    let cache = RedisClusterCache::new(cluster.urls()).await.unwrap();

    // Test operations...
}
```

### Running Tests

**With cargo-nextest (Recommended)**:
```bash
# All tests run serially via file lock - no special flags needed!
cargo nextest run -p reinhardt-cache --all-features redis_cluster
```

**With cargo test**:
```bash
# Also works, respects #[serial] attribute
cargo test -p reinhardt-cache --all-features redis_cluster -- --test-threads=1
```

### Test Execution Behavior

| Command | Synchronization | Result |
|---------|----------------|--------|
| `cargo nextest run` | File lock | ✅ All pass (sequential via lock) |
| `cargo test --test-threads=1` | Single thread | ✅ All pass (sequential) |
| `cargo test` | `#[serial]` | ✅ All pass (sequential) |

## Problem Background

### Why Fixed Port Mapping?

Redis Cluster requires **fixed port mapping** (host port = container port):

```rust
.with_mapped_port(17000, ContainerPort::Tcp(17000))
.with_mapped_port(17001, ContainerPort::Tcp(17001))
// ... 17002-17005
```

**Reason**: Redis Cluster nodes advertise ports via `CLUSTER SLOTS` command. Dynamic port mapping (e.g., 7000→38483) breaks client connections.

### Why Ports 17000-17005?

- Avoids macOS ControlCenter conflict on port 7000
- Standard Redis ports (7000-7005) may be in use
- 17000-17005 less likely to conflict with other services

### Port Conflict Resolution

The fixture implements a 3-layer protection strategy:

1. **Cleanup**: Remove stale containers before starting
2. **Port polling**: Wait up to 10 seconds for ports to be released
3. **File lock**: Serialize fixture execution across processes

## Implementation Details

### Files Modified

1. **`crates/reinhardt-test/src/containers.rs`**
   - Added `RedisClusterGuard` with RAII cleanup (lines 525-605)

2. **`crates/reinhardt-test/src/fixtures.rs`**
   - Added `FileLockGuard` for inter-process sync (lines 498-551)
   - Implemented `ensure_ports_available()` polling (lines 553-620)
   - Implemented `redis_cluster()` fixture (lines 723-829)

3. **`crates/reinhardt-utils/crates/cache/src/redis_cluster.rs`**
   - Updated 7 tests to use new fixture pattern (lines 351-535)

### Port Availability Checking

Uses `lsof` command for reliable port checking:

```rust
async fn ensure_ports_available(ports: &[u16], max_attempts: u32, retry_interval_ms: u64) {
    for attempt in 1..=max_attempts {
        // Check if any process is listening on ports
        let output = Command::new("lsof")
            .args(&["-i", &format!("TCP:{}", port), "-s", "TCP:LISTEN"])
            .output();

        if output.stdout.is_empty() {
            // Port available
        } else {
            // Retry after interval
        }
    }
}
```

**Why lsof instead of TcpListener?**
- `TcpListener::bind()` causes race conditions (bind→close→race)
- `lsof` checks actual listening processes (more reliable)

## Test Results

### Cargo Nextest (Parallel Processes)

```
$ cargo nextest run -p reinhardt-cache --all-features redis_cluster

     Summary [ 102.936s] 7 tests run: 7 passed (3 slow), 112 skipped
        PASS [  14.974s] test_redis_cluster_cache_basic_operations
        PASS [  29.544s] test_build_key_with_prefix
        PASS [  44.186s] test_redis_cluster_cache_batch_operations
        PASS [  58.874s] test_build_key_without_prefix
        PASS [  73.450s] test_redis_cluster_cache_creation
        PASS [  88.316s] test_redis_cluster_cache_ttl
        PASS [ 102.935s] test_redis_cluster_cache_atomic_operations
```

**Each test (~15s) = Cleanup + Container start + Health check + Test execution**

### Cargo Test (Single Thread)

```
$ cargo test -p reinhardt-cache --all-features redis_cluster -- --test-threads=1

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 93 filtered out; finished in 107.12s
```

## Troubleshooting

### Issue: "address already in use" error

**Rare, but if it happens**:

1. **Check for zombie processes**:
   ```bash
   lsof -i TCP:17000-17005 -s TCP:LISTEN
   ```

2. **Manual cleanup**:
   ```bash
   podman rm -f $(podman ps -aq --filter "ancestor=neohq/redis-cluster")
   ```

3. **Remove lock file**:
   ```bash
   rm /tmp/reinhardt_redis_cluster.lock
   ```

### Issue: Tests hang

**Timeout is 60 seconds for health check**:

1. Check Podman status:
   ```bash
   podman ps
   podman machine status  # macOS/Windows
   ```

2. Check container logs:
   ```bash
   podman logs <container-id>
   ```

3. Restart Podman:
   ```bash
   podman machine restart
   ```

### Issue: File lock not released

**Automatic cleanup on Drop**, but if lock file persists:

```bash
# Check lock file
ls -l /tmp/reinhardt_redis_cluster.lock

# Remove if stale (no process holding it)
rm /tmp/reinhardt_redis_cluster.lock
```

## CI/CD Integration

### GitHub Actions Example

```yaml
- name: Run Redis Cluster tests
  run: |
    # No special flags needed - file lock handles serialization
    cargo nextest run -p reinhardt-cache --all-features redis_cluster
```

### GitLab CI Example

```yaml
redis_cluster_tests:
  script:
    - cargo nextest run -p reinhardt-cache --all-features redis_cluster
  timeout: 10m  # 7 tests × ~15s each = ~2min, plus overhead
```

## Design Decisions

### Why RAII Pattern?

**Benefits of RAII approach**:
- Guaranteed cleanup even on panic (Drop trait)
- No manual teardown calls needed in test code
- Clear ownership semantics (guard owns the container)
- Compiler-enforced resource management

### Why File Lock vs Mutex?

| Approach | Scope | cargo test | cargo nextest |
|----------|-------|------------|---------------|
| `std::sync::Mutex` | Process | ✅ Works | ❌ Separate processes |
| File lock (`fs2`) | System | ✅ Works | ✅ Works |

**File lock is the only solution that works for cargo-nextest**

### Why Not Just Use cargo test?

**cargo-nextest advantages**:
- Faster parallel test execution (per-test processes)
- Better test isolation (separate processes prevent state leaks)
- Detailed test output and filtering

To support cargo-nextest's separate-process model, file-based locking is necessary.

## Summary

- ✅ **RAII pattern**: Automatic cleanup via `RedisClusterGuard`
- ✅ **File-based locking**: Works with cargo-nextest (separate processes)
- ✅ **Port polling**: Reliable `lsof`-based checking
- ✅ **No manual flags**: `cargo nextest run` works out of the box
- ✅ **7/7 tests pass**: Verified on both cargo test and cargo-nextest

For implementation details, see: `crates/reinhardt-test/src/fixtures.rs` and `crates/reinhardt-test/src/containers.rs`
