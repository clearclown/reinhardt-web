use reinhardt_di::{DiResult, Injectable, InjectionContext, SingletonScope};
use std::sync::Arc;

// This should fail: Injectable requires Clone
struct ServiceWithoutClone {
	value: String,
}

#[async_trait::async_trait]
impl Injectable for ServiceWithoutClone {
	async fn inject(_ctx: &InjectionContext) -> DiResult<Self> {
		Ok(Self {
			value: "test".to_string(),
		})
	}
}

#[tokio::main]
async fn main() {
	let singleton = Arc::new(SingletonScope::new());
	let ctx = InjectionContext::builder(singleton).build();
	// This should fail because ServiceWithoutClone doesn't implement Clone
	let _service = ServiceWithoutClone::inject(&ctx).await.unwrap();
}
