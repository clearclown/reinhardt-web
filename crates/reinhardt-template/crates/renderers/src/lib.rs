//! # Reinhardt Renderers
//!
//! Response renderers for the Reinhardt framework, inspired by Django REST Framework.
//!
//! ## Renderers
//!
//! - **JSONRenderer**: Render responses as JSON
//! - **BrowsableAPIRenderer**: HTML self-documenting API interface (re-exported from reinhardt-browsable-api)
//! - **XMLRenderer**: Render responses as XML
//! - **YAMLRenderer**: Render responses as YAML
//! - **CSVRenderer**: Render responses as CSV
//! - **OpenAPIRenderer**: Generate OpenAPI 3.0 specifications
//! - **AdminRenderer**: Django-like admin interface renderer
//! - **StaticHTMLRenderer**: Static HTML content renderer
//! - **DocumentationRenderer**: Render API documentation from OpenAPI schemas
//! - **SchemaJSRenderer**: Render OpenAPI schemas as JavaScript
//!
//! ## Renderer Selection
//!
//! The framework provides automatic renderer selection based on:
//!
//! 1. **Format query parameter** (e.g., `?format=json`)
//! 2. **URL format suffix** (e.g., `/api/users.json`)
//! 3. **Accept header** content negotiation with quality values (q-factor)
//! 4. **Default renderer** (first registered)
//!
//! ## Example - Basic Usage
//!
//! ```rust,ignore
//! use reinhardt_renderers::{JSONRenderer, Renderer};
//!
//! let renderer = JSONRenderer::new();
//! let response = renderer.render(&data, None).await?;
//! ```
//!
//! ## Example - Renderer Registry
//!
//! ```rust
//! use reinhardt_renderers::{RendererRegistry, JSONRenderer, XMLRenderer};
//! use reinhardt_renderers::RendererContext;
//! use serde_json::json;
//!
//! # use tokio;
//! # #[tokio::main]
//! # async fn main() {
//! // Create a registry and register renderers
//! let registry = RendererRegistry::new()
//!     .register(JSONRenderer::new())
//!     .register(XMLRenderer::new());
//!
//! let data = json!({"message": "hello"});
//!
//! // Render with automatic selection based on Accept header
//! let context = RendererContext::new()
//!     .with_accept_header("application/json");
//!
//! let (bytes, content_type) = registry.render(&data, None, Some(&context)).await.unwrap();
//! # }
//! ```
//!
//! ## Example - Renderer Selection with Middleware
//!
//! ```rust
//! use reinhardt_renderers::{RendererRegistry, JSONRenderer, XMLRenderer};
//! use reinhardt_renderers::RendererSelector;
//!
//! // Create registry
//! let registry = RendererRegistry::new()
//!     .register(JSONRenderer::new())
//!     .register(XMLRenderer::new());
//!
//! let selector = RendererSelector::new(&registry);
//!
//! // Priority 1: Format parameter takes precedence
//! let renderer = selector.select(
//!     Some("json"),                    // format parameter
//!     Some("/api/users.xml"),          // URL path with suffix
//!     Some("application/xml"),         // Accept header
//! ).unwrap();
//!
//! // Returns JSON renderer because format parameter has highest priority
//! assert_eq!(renderer.format(), Some("json"));
//! ```
//!
//! ## Example - Format Suffix Extraction
//!
//! ```rust
//! use reinhardt_renderers::format_suffix::{extract_format_suffix, get_media_type_for_format};
//!
//! // Extract format suffix from URL path
//! let (clean_path, format) = extract_format_suffix("/api/users.json");
//! assert_eq!(clean_path, "/api/users");
//! assert_eq!(format, Some("json"));
//!
//! // Get media type for format
//! let media_type = get_media_type_for_format("json");
//! assert_eq!(media_type, Some("application/json"));
//! ```
//!
//! ## Planned Features
//!
//! The following advanced features are planned for future releases:
//!
//! ### Renderer Chaining
//!
//! Support for chaining multiple renderers to transform data through multiple stages:
//!
//! ```rust,ignore
//! use reinhardt_renderers::*;
//!
//! let renderer_chain = RendererChain::new()
//!     .pipe(DataTransformRenderer::new())
//!     .pipe(JSONRenderer::new())
//!     .pipe(CompressionRenderer::new("gzip"));
//!
//! // Data flows through: Transform -> JSON -> Compression
//! let result = renderer_chain.render(&data, None).await?;
//! ```
//!
//! **Design Considerations**:
//! - How to handle errors in the middle of the chain?
//! - Should each stage be able to modify the RendererContext?
//! - What's the best way to compose renderers (trait objects vs generics)?
//!
//! ### Response Caching
//!
//! Cache rendered responses to avoid redundant rendering of identical data:
//!
//! ```rust,ignore
//! use reinhardt_renderers::*;
//!
//! let cached_renderer = CachedRenderer::new(
//!     JSONRenderer::new(),
//!     CacheConfig {
//!         ttl: Duration::from_secs(300),
//!         max_size: 1000,
//!         key_strategy: KeyStrategy::Hash,
//!     }
//! );
//!
//! // First call renders and caches
//! let result1 = cached_renderer.render(&data, None).await?;
//!
//! // Second call returns cached result
//! let result2 = cached_renderer.render(&data, None).await?;
//! ```
//!
//! **Design Considerations**:
//! - How to generate cache keys from data and context?
//! - Should caching be renderer-specific or global?
//! - What eviction strategy to use (LRU, TTL, size-based)?
//! - How to handle cache invalidation?
//!
//! ### Streaming Support
//!
//! Stream large responses incrementally instead of buffering entire response:
//!
//! ```rust,ignore
//! use reinhardt_renderers::*;
//! use futures::stream::Stream;
//!
//! let streaming_renderer = StreamingJSONRenderer::new();
//!
//! // Returns Stream<Item = Bytes> instead of Bytes
//! let stream = streaming_renderer.render_stream(&large_dataset, None).await?;
//!
//! // Stream can be consumed incrementally
//! while let Some(chunk) = stream.next().await {
//!     send_to_client(chunk?).await?;
//! }
//! ```
//!
//! **Design Considerations**:
//! - Which renderers should support streaming (JSON arrays, CSV, XML)?
//! - How to handle streaming errors mid-response?
//! - Should streaming be opt-in or automatic based on data size?
//! - What buffer sizes are optimal for different formats?
//!
//! ### Compression Support
//!
//! Automatic response compression with multiple algorithms:
//!
//! ```rust,ignore
//! use reinhardt_renderers::*;
//!
//! let renderer = JSONRenderer::new()
//!     .with_compression(CompressionAlgorithm::Gzip { level: 6 });
//!
//! // Or use content negotiation
//! let compressed_renderer = CompressionRenderer::new(
//!     JSONRenderer::new(),
//!     vec![
//!         CompressionAlgorithm::Brotli { quality: 4 },
//!         CompressionAlgorithm::Gzip { level: 6 },
//!         CompressionAlgorithm::Deflate,
//!     ]
//! );
//!
//! // Automatically selects best compression based on Accept-Encoding header
//! let (bytes, content_type, encoding) = compressed_renderer
//!     .render_compressed(&data, &context)
//!     .await?;
//! ```
//!
//! **Design Considerations**:
//! - Which compression algorithms to support (gzip, brotli, zstd, deflate)?
//! - How to balance compression ratio vs CPU usage?
//! - Should compression be applied before or after caching?
//! - What minimum response size should trigger compression?
//! - How to handle Accept-Encoding negotiation?
//!
//! **Implementation Status**: All features are in planning stage
//!
//! **Required Changes**:
//! 1. Extend Renderer trait with optional streaming and compression methods
//! 2. Implement wrapper renderers (CachedRenderer, CompressedRenderer)
//! 3. Add RendererChain builder with pipe() method
//! 4. Integrate with reinhardt-cache for response caching
//! 5. Add compression crate dependencies (flate2, brotli)
//! 6. Update RendererContext to include Accept-Encoding information

pub mod admin_renderer;
pub mod csv_renderer;
pub mod documentation_renderer;
pub mod format_suffix;
pub mod json;
pub mod middleware;
pub mod openapi;
pub mod renderer;
pub mod schemajs_renderer;
pub mod static_html_renderer;
pub mod template_html_renderer;
pub mod xml;
pub mod yaml_renderer;

#[cfg(test)]
mod tests;

pub use admin_renderer::AdminRenderer;
pub use csv_renderer::CSVRenderer;
pub use documentation_renderer::DocumentationRenderer;
pub use json::JSONRenderer;
pub use middleware::RendererSelector;
pub use openapi::OpenAPIRenderer;
pub use renderer::{RenderResult, Renderer, RendererContext, RendererRegistry};
pub use schemajs_renderer::SchemaJSRenderer;
pub use static_html_renderer::StaticHTMLRenderer;
pub use template_html_renderer::TemplateHTMLRenderer;
pub use xml::XMLRenderer;
pub use yaml_renderer::YAMLRenderer;

// Re-export from specialized crates
pub use reinhardt_browsable_api::BrowsableApiRenderer as BrowsableAPIRenderer;
