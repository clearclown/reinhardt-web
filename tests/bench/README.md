# Reinhardt Benchmarks

This directory contains benchmark tests for the Reinhardt framework.

## Usage

Run all benchmarks:

```bash
cargo bench -p reinhardt-benchmarks
```

Run a specific benchmark:

```bash
cargo bench -p reinhardt-benchmarks --bench example_bench
```

## Adding New Benchmarks

1. Create a new file in the `benches/` directory
2. Add a `[[bench]]` entry to `Cargo.toml`
3. Use the `criterion` crate for benchmarking
