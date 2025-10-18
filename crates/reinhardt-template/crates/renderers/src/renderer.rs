//! Core renderer traits and types

use async_trait::async_trait;
use bytes::Bytes;
use reinhardt_exception::{Error, Result};
use serde_json::Value;

pub type RenderResult<T> = Result<T>;

#[derive(Debug, Clone, Default)]
pub struct RendererContext;

#[async_trait]
pub trait Renderer: Send + Sync {
    fn media_types(&self) -> Vec<String>;
    async fn render(&self, data: &Value, context: Option<&RendererContext>) -> RenderResult<Bytes>;
    fn format(&self) -> Option<&str> {
        None
    }
}

/// Registry for managing multiple renderers
#[derive(Default)]
pub struct RendererRegistry {
    renderers: Vec<Box<dyn Renderer>>,
}

impl RendererRegistry {
    pub fn new() -> Self {
        Self {
            renderers: Vec::new(),
        }
    }

    pub fn register<R: Renderer + 'static>(mut self, renderer: R) -> Self {
        self.renderers.push(Box::new(renderer));
        self
    }

    pub fn get_renderer(&self, format: Option<&str>) -> Option<&dyn Renderer> {
        if let Some(fmt) = format {
            self.renderers
                .iter()
                .find(|r| r.format() == Some(fmt))
                .map(|r| r.as_ref())
        } else {
            self.renderers.first().map(|r| r.as_ref())
        }
    }

    pub async fn render(
        &self,
        data: &Value,
        format: Option<&str>,
        context: Option<&RendererContext>,
    ) -> RenderResult<(Bytes, String)> {
        let renderer = self
            .get_renderer(format)
            .ok_or_else(|| Error::Http("No renderer found".to_string()))?;

        let bytes = renderer.render(data, context).await?;
        let content_type = renderer
            .media_types()
            .first()
            .cloned()
            .unwrap_or_else(|| "application/octet-stream".to_string());

        Ok((bytes, content_type))
    }
}
