//! Procedural macros for Reinhardt gRPC DI integration

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod crate_paths;
mod grpc_handler;

/// Attribute macro for gRPC handlers with dependency injection support
///
/// This macro enables the use of `#[inject]` parameters in gRPC service methods,
/// allowing automatic dependency resolution from the `InjectionContext`.
///
/// # Usage
///
/// ```rust,ignore
/// use reinhardt_grpc::{GrpcRequestExt, grpc_handler};
/// use reinhardt_di::InjectionContext;
/// use tonic::{Request, Response, Status};
/// use std::sync::Arc;
///
/// pub struct UserServiceImpl {
///     injection_context: Arc<InjectionContext>,
/// }
///
/// #[tonic::async_trait]
/// impl UserService for UserServiceImpl {
///     async fn get_user(&self, mut request: Request<GetUserRequest>)
///         -> Result<Response<User>, Status>
///     {
///         // Set DI context in request extensions
///         request.extensions_mut().insert(self.injection_context.clone());
///
///         // Call handler with DI support
///         self.get_user_impl(request).await
///     }
/// }
///
/// impl UserServiceImpl {
///     #[grpc_handler]
///     async fn get_user_impl(
///         &self,
///         request: Request<GetUserRequest>,
///         #[inject] db: DatabaseConnection,
///     ) -> Result<Response<User>, Status> {
///         let user_id = request.into_inner().id;
///         let user = db.fetch_user(user_id).await?;
///         Ok(Response::new(user))
///     }
/// }
/// ```
///
/// # Parameters
///
/// Regular parameters are passed through as-is. Parameters marked with `#[inject]`
/// are automatically resolved from the DI context.
///
/// ## Cache Control
///
/// By default, dependencies are cached. You can disable caching per parameter:
///
/// ```rust,ignore
/// #[grpc_handler]
/// async fn handler(
///     &self,
///     request: Request<Req>,
///     #[inject] cached_db: DatabaseConnection,          // Cached (default)
///     #[inject(cache = false)] fresh_db: DatabaseConnection,  // Not cached
/// ) -> Result<Response<Resp>, Status> {
///     // ...
/// }
/// ```
///
/// # Requirements
///
/// 1. The function must have a `tonic::Request<T>` parameter
/// 2. The request must have an `InjectionContext` in its extensions
/// 3. All injected types must implement `Injectable`
/// 4. The function must be `async`
///
/// # Error Handling
///
/// If dependency injection fails, the function returns `tonic::Status::internal`
/// with an error message describing the failure.
#[proc_macro_attribute]
pub fn grpc_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let input = parse_macro_input!(item as syn::ItemFn);

	grpc_handler::expand_grpc_handler(input)
		.unwrap_or_else(|err| err.to_compile_error())
		.into()
}
