//! Circular dependency detection integration tests
//!
//! This test suite verifies the automatic circular dependency detection functionality
//! of the DI system.

use super::test_helpers::resolve_injectable;
use reinhardt_di::{DiError, Injectable, InjectionContext, SingletonScope};
use std::sync::Arc;

/// Test fixture: ServiceA (depends on ServiceB)
#[derive(Clone)]
struct ServiceA {
	b: Arc<ServiceB>,
}

/// Test fixture: ServiceB (depends on ServiceC)
#[derive(Clone)]
struct ServiceB {
	c: Arc<ServiceC>,
}

/// Test fixture: ServiceC (depends on ServiceA - circular!)
#[derive(Clone)]
struct ServiceC {
	a: Option<Arc<ServiceA>>,
}

#[async_trait::async_trait]
impl Injectable for ServiceA {
	async fn inject(ctx: &InjectionContext) -> reinhardt_di::DiResult<Self> {
		let b = resolve_injectable::<ServiceB>(ctx).await?;
		Ok(ServiceA { b })
	}
}

#[async_trait::async_trait]
impl Injectable for ServiceB {
	async fn inject(ctx: &InjectionContext) -> reinhardt_di::DiResult<Self> {
		let c = resolve_injectable::<ServiceC>(ctx).await?;
		Ok(ServiceB { c })
	}
}

#[async_trait::async_trait]
impl Injectable for ServiceC {
	async fn inject(ctx: &InjectionContext) -> reinhardt_di::DiResult<Self> {
		// 循環依存を引き起こす
		let a = resolve_injectable::<ServiceA>(ctx).await?;
		Ok(ServiceC { a: Some(a) })
	}
}

/// Direct circular dependency: A → B → A
#[tokio::test]
async fn test_direct_circular_dependency() {
	#[derive(Clone)]
	struct DirectA {
		b: Arc<DirectB>,
	}

	#[derive(Clone)]
	struct DirectB {
		a: Arc<DirectA>,
	}

	#[async_trait::async_trait]
	impl Injectable for DirectA {
		async fn inject(ctx: &InjectionContext) -> reinhardt_di::DiResult<Self> {
			let b = resolve_injectable::<DirectB>(ctx).await?;
			Ok(DirectA { b })
		}
	}

	#[async_trait::async_trait]
	impl Injectable for DirectB {
		async fn inject(ctx: &InjectionContext) -> reinhardt_di::DiResult<Self> {
			let a = resolve_injectable::<DirectA>(ctx).await?;
			Ok(DirectB { a })
		}
	}

	let singleton_scope = Arc::new(SingletonScope::new());
	let ctx = InjectionContext::builder(singleton_scope).build();
	let result = resolve_injectable::<DirectA>(&ctx).await;

	assert!(
		result.is_err(),
		"Direct circular dependency should be detected"
	);

	if let Err(DiError::CircularDependency(msg)) = result {
		assert!(
			msg.contains("DirectA") || msg.contains("DirectB"),
			"Error message should contain circular types: {}",
			msg
		);
	} else {
		panic!("Expected CircularDependency error");
	}
}

/// Indirect circular dependency: A → B → C → A
#[tokio::test]
async fn test_indirect_circular_dependency() {
	let singleton_scope = Arc::new(SingletonScope::new());
	let ctx = InjectionContext::builder(singleton_scope).build();
	let result = resolve_injectable::<ServiceA>(&ctx).await;

	assert!(
		result.is_err(),
		"Indirect circular dependency should be detected"
	);

	if let Err(DiError::CircularDependency(msg)) = result {
		// Verify cycle path contains involved types
		let contains_services =
			msg.contains("ServiceA") || msg.contains("ServiceB") || msg.contains("ServiceC");
		assert!(
			contains_services,
			"Error message should contain circular types: {}",
			msg
		);
	} else {
		panic!("Expected CircularDependency error");
	}
}

/// Self-reference: A → A
#[tokio::test]
async fn test_self_dependency() {
	#[derive(Clone)]
	struct SelfDependent {
		inner: Option<Arc<SelfDependent>>,
	}

	#[async_trait::async_trait]
	impl Injectable for SelfDependent {
		async fn inject(ctx: &InjectionContext) -> reinhardt_di::DiResult<Self> {
			// Attempt self-reference
			let inner = resolve_injectable::<SelfDependent>(ctx).await?;
			Ok(SelfDependent { inner: Some(inner) })
		}
	}

	let singleton_scope = Arc::new(SingletonScope::new());
	let ctx = InjectionContext::builder(singleton_scope).build();
	let result = resolve_injectable::<SelfDependent>(&ctx).await;

	assert!(result.is_err(), "Self-dependency should be detected");
	assert!(
		matches!(result, Err(DiError::CircularDependency(_))),
		"Expected CircularDependency error"
	);
}

/// Complex circular dependency: A → B → C → D → B
#[tokio::test]
async fn test_complex_circular_dependency() {
	#[derive(Clone)]
	struct ComplexA {
		b: Arc<ComplexB>,
	}

	#[derive(Clone)]
	struct ComplexB {
		c: Arc<ComplexC>,
	}

	#[derive(Clone)]
	struct ComplexC {
		d: Arc<ComplexD>,
	}

	#[derive(Clone)]
	struct ComplexD {
		b: Arc<ComplexB>, // Circular: B → C → D → B
	}

	#[async_trait::async_trait]
	impl Injectable for ComplexA {
		async fn inject(ctx: &InjectionContext) -> reinhardt_di::DiResult<Self> {
			let b = resolve_injectable::<ComplexB>(ctx).await?;
			Ok(ComplexA { b })
		}
	}

	#[async_trait::async_trait]
	impl Injectable for ComplexB {
		async fn inject(ctx: &InjectionContext) -> reinhardt_di::DiResult<Self> {
			let c = resolve_injectable::<ComplexC>(ctx).await?;
			Ok(ComplexB { c })
		}
	}

	#[async_trait::async_trait]
	impl Injectable for ComplexC {
		async fn inject(ctx: &InjectionContext) -> reinhardt_di::DiResult<Self> {
			let d = resolve_injectable::<ComplexD>(ctx).await?;
			Ok(ComplexC { d })
		}
	}

	#[async_trait::async_trait]
	impl Injectable for ComplexD {
		async fn inject(ctx: &InjectionContext) -> reinhardt_di::DiResult<Self> {
			let b = resolve_injectable::<ComplexB>(ctx).await?;
			Ok(ComplexD { b })
		}
	}

	let singleton_scope = Arc::new(SingletonScope::new());
	let ctx = InjectionContext::builder(singleton_scope).build();
	let result = resolve_injectable::<ComplexA>(&ctx).await;

	assert!(
		result.is_err(),
		"Complex circular dependency should be detected"
	);
	assert!(
		matches!(result, Err(DiError::CircularDependency(_))),
		"Expected CircularDependency error"
	);
}

/// No circular dependency should succeed
#[tokio::test]
async fn test_no_circular_dependency_succeeds() {
	#[derive(Clone, Default)]
	struct NoCycleA {
		b: Option<Arc<NoCycleB>>,
	}

	#[derive(Clone, Default)]
	struct NoCycleB {
		value: i32,
	}

	#[async_trait::async_trait]
	impl Injectable for NoCycleA {
		async fn inject(ctx: &InjectionContext) -> reinhardt_di::DiResult<Self> {
			let b = resolve_injectable::<NoCycleB>(ctx).await?;
			Ok(NoCycleA { b: Some(b) })
		}
	}

	#[async_trait::async_trait]
	impl Injectable for NoCycleB {
		async fn inject(_ctx: &InjectionContext) -> reinhardt_di::DiResult<Self> {
			Ok(NoCycleB::default())
		}
	}

	let singleton_scope = Arc::new(SingletonScope::new());
	let ctx = InjectionContext::builder(singleton_scope).build();
	let result = resolve_injectable::<NoCycleA>(&ctx).await;

	assert!(result.is_ok(), "Non-circular dependency should succeed");
}

/// Deep dependency chain (without cycle) should not error
#[tokio::test]
async fn test_deep_dependency_chain_without_cycle() {
	#[derive(Clone, Default)]
	struct Level1;

	#[derive(Clone)]
	struct Level2 {
		_dep: Arc<Level1>,
	}

	#[derive(Clone)]
	struct Level3 {
		_dep: Arc<Level2>,
	}

	#[derive(Clone)]
	struct Level4 {
		_dep: Arc<Level3>,
	}

	#[derive(Clone)]
	struct Level5 {
		_dep: Arc<Level4>,
	}

	#[async_trait::async_trait]
	impl Injectable for Level1 {
		async fn inject(_ctx: &InjectionContext) -> reinhardt_di::DiResult<Self> {
			Ok(Level1)
		}
	}

	#[async_trait::async_trait]
	impl Injectable for Level2 {
		async fn inject(ctx: &InjectionContext) -> reinhardt_di::DiResult<Self> {
			let dep = resolve_injectable::<Level1>(ctx).await?;
			Ok(Level2 { _dep: dep })
		}
	}

	#[async_trait::async_trait]
	impl Injectable for Level3 {
		async fn inject(ctx: &InjectionContext) -> reinhardt_di::DiResult<Self> {
			let dep = resolve_injectable::<Level2>(ctx).await?;
			Ok(Level3 { _dep: dep })
		}
	}

	#[async_trait::async_trait]
	impl Injectable for Level4 {
		async fn inject(ctx: &InjectionContext) -> reinhardt_di::DiResult<Self> {
			let dep = resolve_injectable::<Level3>(ctx).await?;
			Ok(Level4 { _dep: dep })
		}
	}

	#[async_trait::async_trait]
	impl Injectable for Level5 {
		async fn inject(ctx: &InjectionContext) -> reinhardt_di::DiResult<Self> {
			let dep = resolve_injectable::<Level4>(ctx).await?;
			Ok(Level5 { _dep: dep })
		}
	}

	let singleton_scope = Arc::new(SingletonScope::new());
	let ctx = InjectionContext::builder(singleton_scope).build();
	let result = resolve_injectable::<Level5>(&ctx).await;

	assert!(
		result.is_ok(),
		"Deep dependency chain (without cycle) should succeed"
	);
}
