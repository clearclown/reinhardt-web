//! Error handling integration tests for reinhardt-di
//!
//! Tests covering:
//! - Circular dependency detection
//! - Injectable failure propagation
//! - Async operation timeout handling
//! - Depends lifetime management with Arc

use reinhardt_di::{Depends, DiError, DiResult, Injectable, InjectionContext, SingletonScope};
use std::sync::{Arc, Mutex, Weak};
use std::time::Duration;
use tokio::time::timeout;

// === Circular Dependency Test Structures ===

#[derive(Clone, Debug)]
struct ServiceA {
	_name: String,
	// Holds weak reference to avoid memory leak
	_service_b: Option<Weak<ServiceB>>,
}

#[derive(Clone, Debug)]
struct ServiceB {
	_name: String,
	// Holds weak reference to avoid memory leak
	_service_a: Option<Weak<ServiceA>>,
}

// Track injection calls to detect cycles
static INJECTION_STACK: Mutex<Vec<String>> = Mutex::new(Vec::new());

fn push_injection(name: &str) -> Result<(), DiError> {
	let mut stack = INJECTION_STACK.lock().unwrap();
	if stack.contains(&name.to_string()) {
		return Err(DiError::CircularDependency(format!(
			"Circular dependency: {} -> {}",
			stack.join(" -> "),
			name
		)));
	}
	stack.push(name.to_string());
	Ok(())
}

fn pop_injection() {
	let mut stack = INJECTION_STACK.lock().unwrap();
	stack.pop();
}

fn clear_injection_stack() {
	let mut stack = INJECTION_STACK.lock().unwrap();
	stack.clear();
}

#[async_trait::async_trait]
impl Injectable for ServiceA {
	async fn inject(ctx: &InjectionContext) -> DiResult<Self> {
		push_injection("ServiceA")?;

		// ServiceA depends on ServiceB
		let service_b = ServiceB::inject(ctx).await?;

		pop_injection();

		Ok(ServiceA {
			_name: "ServiceA".to_string(),
			_service_b: Some(Arc::downgrade(&Arc::new(service_b))),
		})
	}
}

#[async_trait::async_trait]
impl Injectable for ServiceB {
	async fn inject(ctx: &InjectionContext) -> DiResult<Self> {
		push_injection("ServiceB")?;

		// ServiceB depends on ServiceA - creates circular dependency
		let service_a = ServiceA::inject(ctx).await?;

		pop_injection();

		Ok(ServiceB {
			_name: "ServiceB".to_string(),
			_service_a: Some(Arc::downgrade(&Arc::new(service_a))),
		})
	}
}

// === Injectable Failure Test Structures ===

#[derive(Clone, Debug)]
struct FailingService {
	_should_fail: bool,
}

#[async_trait::async_trait]
impl Injectable for FailingService {
	async fn inject(_ctx: &InjectionContext) -> DiResult<Self> {
		Err(DiError::ProviderError(
			"Intentional failure for testing".to_string(),
		))
	}
}

#[derive(Clone, Debug)]
struct DependentService {
	_failing_service: Arc<FailingService>,
}

#[async_trait::async_trait]
impl Injectable for DependentService {
	async fn inject(ctx: &InjectionContext) -> DiResult<Self> {
		// This will fail because FailingService::inject() fails
		let failing = FailingService::inject(ctx).await?;
		Ok(DependentService {
			_failing_service: Arc::new(failing),
		})
	}
}

// === Async Timeout Test Structures ===

#[derive(Clone, Debug)]
struct SlowService {
	delay_ms: u64,
}

#[async_trait::async_trait]
impl Injectable for SlowService {
	async fn inject(_ctx: &InjectionContext) -> DiResult<Self> {
		// Simulate slow initialization
		tokio::time::sleep(Duration::from_millis(1000)).await;
		Ok(SlowService { delay_ms: 1000 })
	}
}

// === Lifetime Management Test Structures ===

#[derive(Clone, Debug)]
struct ResourceOwner {
	id: u32,
	data: String,
}

#[async_trait::async_trait]
impl Injectable for ResourceOwner {
	async fn inject(_ctx: &InjectionContext) -> DiResult<Self> {
		Ok(ResourceOwner {
			id: 1,
			data: "owned-data".to_string(),
		})
	}
}

// === Test Cases ===

#[tokio::test]
async fn test_circular_dependency_detection() {
	let singleton = Arc::new(SingletonScope::new());
	let ctx = InjectionContext::builder(singleton).build();

	// Clear stack before test
	clear_injection_stack();

	// Attempt to inject ServiceA, which depends on ServiceB, which depends on ServiceA
	let result = ServiceA::inject(&ctx).await;

	// Verify circular dependency is detected
	assert!(result.is_err());
	match result.unwrap_err() {
		DiError::CircularDependency(msg) => {
			assert!(msg.contains("ServiceA"));
			assert!(msg.contains("ServiceB"));
		}
		other => panic!("Expected CircularDependency error, got: {:?}", other),
	}

	// Clean up
	clear_injection_stack();
}

#[tokio::test]
async fn test_injectable_failure_propagation() {
	let singleton = Arc::new(SingletonScope::new());
	let ctx = InjectionContext::builder(singleton).build();

	// Direct injection of failing service
	let failing_result = FailingService::inject(&ctx).await;
	assert!(failing_result.is_err());
	match failing_result.unwrap_err() {
		DiError::ProviderError(msg) => {
			assert_eq!(msg, "Intentional failure for testing");
		}
		other => panic!("Expected ProviderError, got: {:?}", other),
	}

	// Injection of dependent service should propagate the error
	let dependent_result = DependentService::inject(&ctx).await;
	assert!(dependent_result.is_err());
	match dependent_result.unwrap_err() {
		DiError::ProviderError(msg) => {
			assert_eq!(msg, "Intentional failure for testing");
		}
		other => panic!("Expected ProviderError (propagated), got: {:?}", other),
	}
}

#[tokio::test]
async fn test_injectable_async_timeout() {
	let singleton = Arc::new(SingletonScope::new());
	let ctx = InjectionContext::builder(singleton).build();

	// Attempt injection with timeout shorter than service delay
	let timeout_duration = Duration::from_millis(100);
	let result = timeout(timeout_duration, SlowService::inject(&ctx)).await;

	// Verify timeout occurred
	assert!(result.is_err());
	assert!(result
		.unwrap_err()
		.to_string()
		.contains("deadline has elapsed"));

	// Verify with longer timeout that injection succeeds
	let longer_timeout = Duration::from_millis(2000);
	let result_ok = timeout(longer_timeout, SlowService::inject(&ctx)).await;
	assert!(result_ok.is_ok());
	let service = result_ok.unwrap().unwrap();
	assert_eq!(service.delay_ms, 1000);
}

#[tokio::test]
async fn test_depends_lifetime_management() {
	let singleton = Arc::new(SingletonScope::new());
	let ctx = InjectionContext::builder(singleton).build();

	// Resolve dependency using Depends
	let depends_resource = Depends::<ResourceOwner>::builder()
		.resolve(&ctx)
		.await
		.unwrap();

	// Verify Depends uses Arc internally by cloning
	let depends_clone = depends_resource.clone();

	// Both should access the same data via Deref
	assert_eq!(depends_resource.id, 1);
	assert_eq!(depends_clone.id, 1);
	assert_eq!(depends_resource.data, "owned-data");
	assert_eq!(depends_clone.data, "owned-data");

	// Drop clone - original should still be accessible
	drop(depends_clone);
	assert_eq!(depends_resource.data, "owned-data");

	// Resolve again from context (should return cached instance)
	let depends_resource2 = Depends::<ResourceOwner>::builder()
		.resolve(&ctx)
		.await
		.unwrap();

	// Verify data matches (same instance from cache)
	assert_eq!(depends_resource.id, depends_resource2.id);
	assert_eq!(depends_resource.data, depends_resource2.data);

	// Drop all Depends instances
	drop(depends_resource);
	drop(depends_resource2);

	// Verify context cache still holds the value
	let cached: Option<Arc<ResourceOwner>> = ctx.get_request();
	assert!(cached.is_some());
	let cached_value = cached.unwrap();
	assert_eq!(cached_value.id, 1);
	assert_eq!(cached_value.data, "owned-data");

	// Create a weak reference from cached Arc
	let weak_ref = Arc::downgrade(&cached_value);
	drop(cached_value);

	// Weak reference should still be upgradeable because cache holds strong ref
	let upgraded = weak_ref.upgrade();
	assert!(upgraded.is_some());
	assert_eq!(upgraded.unwrap().data, "owned-data");
}
