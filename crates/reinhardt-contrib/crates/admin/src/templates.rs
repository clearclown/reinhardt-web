//! Template rendering for admin interface
//!
//! This module provides template rendering using Askama template engine
//! integrated with reinhardt-templates.

use crate::{AdminError, AdminResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Admin template context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminContext {
    /// Site title
    pub site_title: String,
    /// Site header
    pub site_header: String,
    /// Current user
    pub user: Option<UserContext>,
    /// Available apps
    pub available_apps: Vec<AppContext>,
    /// Extra context data
    pub extra: HashMap<String, serde_json::Value>,
}

impl AdminContext {
    /// Create a new admin context
    pub fn new(site_title: impl Into<String>) -> Self {
        Self {
            site_title: site_title.into(),
            site_header: "Administration".to_string(),
            user: None,
            available_apps: Vec::new(),
            extra: HashMap::new(),
        }
    }

    /// Set site header
    pub fn with_header(mut self, header: impl Into<String>) -> Self {
        self.site_header = header.into();
        self
    }

    /// Set current user
    pub fn with_user(mut self, user: UserContext) -> Self {
        self.user = Some(user);
        self
    }

    /// Add an app
    pub fn add_app(&mut self, app: AppContext) {
        self.available_apps.push(app);
    }

    /// Add extra context data
    pub fn with_extra(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.extra.insert(key.into(), value);
        self
    }
}

impl Default for AdminContext {
    fn default() -> Self {
        Self::new("Reinhardt Admin")
    }
}

/// User context for templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserContext {
    pub username: String,
    pub email: Option<String>,
    pub is_staff: bool,
    pub is_superuser: bool,
}

/// App context for templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppContext {
    pub name: String,
    pub label: String,
    pub models: Vec<ModelContext>,
}

/// Model context for templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelContext {
    pub name: String,
    pub label: String,
    pub url: String,
    pub add_url: Option<String>,
}

/// List view template context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListViewContext {
    /// Base admin context
    pub admin: AdminContext,
    /// Model name
    pub model_name: String,
    /// Model verbose name
    pub model_verbose_name: String,
    /// List of items
    pub items: Vec<HashMap<String, serde_json::Value>>,
    /// Fields to display
    pub list_display: Vec<String>,
    /// Field labels
    pub field_labels: HashMap<String, String>,
    /// Available filters
    pub filters: Vec<FilterContext>,
    /// Search query
    pub search_query: Option<String>,
    /// Pagination info
    pub pagination: PaginationContext,
    /// Available actions
    pub actions: Vec<ActionContext>,
}

impl ListViewContext {
    /// Create a new list view context
    pub fn new(
        model_name: impl Into<String>,
        items: Vec<HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self {
            admin: AdminContext::default(),
            model_name: model_name.into(),
            model_verbose_name: "Items".to_string(),
            items,
            list_display: Vec::new(),
            field_labels: HashMap::new(),
            filters: Vec::new(),
            search_query: None,
            pagination: PaginationContext::default(),
            actions: Vec::new(),
        }
    }
}

/// Filter context for templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterContext {
    pub title: String,
    pub choices: Vec<FilterChoice>,
}

/// Filter choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterChoice {
    pub label: String,
    pub url: String,
    pub selected: bool,
}

/// Pagination context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationContext {
    pub page: usize,
    pub total_pages: usize,
    pub page_size: usize,
    pub total_count: usize,
    pub has_previous: bool,
    pub has_next: bool,
    pub previous_url: Option<String>,
    pub next_url: Option<String>,
}

impl Default for PaginationContext {
    fn default() -> Self {
        Self {
            page: 1,
            total_pages: 1,
            page_size: 100,
            total_count: 0,
            has_previous: false,
            has_next: false,
            previous_url: None,
            next_url: None,
        }
    }
}

/// Action context for templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionContext {
    pub name: String,
    pub label: String,
    pub description: String,
}

/// Form view template context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormViewContext {
    /// Base admin context
    pub admin: AdminContext,
    /// Model name
    pub model_name: String,
    /// Form title
    pub title: String,
    /// Form fields
    pub fields: Vec<FormFieldContext>,
    /// Inline formsets
    pub inlines: Vec<InlineFormsetContext>,
    /// Object ID (for edit)
    pub object_id: Option<String>,
    /// Form errors
    pub errors: Vec<String>,
}

/// Form field context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormFieldContext {
    pub name: String,
    pub label: String,
    pub widget_html: String,
    pub help_text: Option<String>,
    pub errors: Vec<String>,
    pub is_readonly: bool,
}

/// Inline formset context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineFormsetContext {
    pub model_name: String,
    pub verbose_name: String,
    pub forms: Vec<FormViewContext>,
}

/// Delete confirmation context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteConfirmationContext {
    /// Base admin context
    pub admin: AdminContext,
    /// Model name
    pub model_name: String,
    /// Object representation
    pub object_repr: String,
    /// Related objects that will be deleted
    pub related_objects: Vec<RelatedObjectContext>,
    /// Total count
    pub total_count: usize,
}

/// Related object context for delete confirmation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedObjectContext {
    pub model_name: String,
    pub count: usize,
    pub items: Vec<String>,
}

/// Dashboard context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardContext {
    /// Base admin context
    pub admin: AdminContext,
    /// Widget HTML content
    pub widgets: Vec<WidgetContext>,
    /// Recent actions
    pub recent_actions: Vec<RecentActionContext>,
}

/// Widget context for dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetContext {
    pub title: String,
    pub content_html: String,
    pub css_class: Option<String>,
}

/// Recent action context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentActionContext {
    pub action: String,
    pub model_name: String,
    pub object_repr: String,
    pub user: String,
    pub timestamp: String,
}

/// Template renderer
pub struct AdminTemplateRenderer {
    template_dir: String,
}

impl AdminTemplateRenderer {
    /// Create a new template renderer
    pub fn new(template_dir: impl Into<String>) -> Self {
        Self {
            template_dir: template_dir.into(),
        }
    }

    /// Render list view
    pub fn render_list(&self, context: &ListViewContext) -> AdminResult<String> {
        // TODO: Implement actual template rendering with Askama
        // For now, return basic HTML
        Ok(self.render_list_html(context))
    }

    /// Render form view
    pub fn render_form(&self, context: &FormViewContext) -> AdminResult<String> {
        // TODO: Implement actual template rendering with Askama
        Ok(self.render_form_html(context))
    }

    /// Render delete confirmation
    pub fn render_delete_confirmation(
        &self,
        context: &DeleteConfirmationContext,
    ) -> AdminResult<String> {
        // TODO: Implement actual template rendering with Askama
        Ok(self.render_delete_html(context))
    }

    /// Render dashboard
    pub fn render_dashboard(&self, context: &DashboardContext) -> AdminResult<String> {
        // TODO: Implement actual template rendering with Askama
        Ok(self.render_dashboard_html(context))
    }

    // Temporary HTML rendering methods (will be replaced with Askama templates)

    fn render_list_html(&self, ctx: &ListViewContext) -> String {
        let items_html = ctx
            .items
            .iter()
            .map(|item| {
                let cells = ctx
                    .list_display
                    .iter()
                    .map(|field| {
                        let value = item
                            .get(field)
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        format!("<td>{}</td>", value)
                    })
                    .collect::<Vec<_>>()
                    .join("");
                format!("<tr>{}</tr>", cells)
            })
            .collect::<Vec<_>>()
            .join("");

        let headers = ctx
            .list_display
            .iter()
            .map(|field| {
                let label = ctx
                    .field_labels
                    .get(field)
                    .cloned()
                    .unwrap_or_else(|| field.clone());
                format!("<th>{}</th>", label)
            })
            .collect::<Vec<_>>()
            .join("");

        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>{} - {}</title>
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css" rel="stylesheet">
</head>
<body>
    <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
        <div class="container-fluid">
            <a class="navbar-brand" href="/admin/">{}</a>
        </div>
    </nav>
    <div class="container mt-4">
        <h1>{}</h1>
        <table class="table table-striped">
            <thead>
                <tr>{}</tr>
            </thead>
            <tbody>
                {}
            </tbody>
        </table>
    </div>
</body>
</html>"#,
            ctx.model_verbose_name,
            ctx.admin.site_title,
            ctx.admin.site_header,
            ctx.model_verbose_name,
            headers,
            items_html
        )
    }

    fn render_form_html(&self, ctx: &FormViewContext) -> String {
        let fields_html = ctx
            .fields
            .iter()
            .map(|field| {
                let errors_html = if !field.errors.is_empty() {
                    format!(
                        r#"<div class="invalid-feedback d-block">{}</div>"#,
                        field.errors.join(", ")
                    )
                } else {
                    String::new()
                };

                format!(
                    r#"<div class="mb-3">
                    <label class="form-label">{}</label>
                    {}
                    {}
                    {}
                </div>"#,
                    field.label,
                    field.widget_html,
                    field.help_text.as_ref().map_or(String::new(), |h| format!(
                        r#"<small class="form-text text-muted">{}</small>"#,
                        h
                    )),
                    errors_html
                )
            })
            .collect::<Vec<_>>()
            .join("");

        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>{} - {}</title>
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css" rel="stylesheet">
</head>
<body>
    <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
        <div class="container-fluid">
            <a class="navbar-brand" href="/admin/">{}</a>
        </div>
    </nav>
    <div class="container mt-4">
        <h1>{}</h1>
        <form method="post">
            {}
            <button type="submit" class="btn btn-primary">Save</button>
            <a href="/admin/{}/\" class="btn btn-secondary">Cancel</a>
        </form>
    </div>
</body>
</html>"#,
            ctx.title,
            ctx.admin.site_title,
            ctx.admin.site_header,
            ctx.title,
            fields_html,
            ctx.model_name.to_lowercase()
        )
    }

    fn render_delete_html(&self, ctx: &DeleteConfirmationContext) -> String {
        let related_html = ctx
            .related_objects
            .iter()
            .map(|rel| {
                format!(
                    "<li>{}: {} items</li>",
                    rel.model_name, rel.count
                )
            })
            .collect::<Vec<_>>()
            .join("");

        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>Delete {} - {}</title>
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css" rel="stylesheet">
</head>
<body>
    <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
        <div class="container-fluid">
            <a class="navbar-brand" href="/admin/">{}</a>
        </div>
    </nav>
    <div class="container mt-4">
        <h1>Delete {}</h1>
        <div class="alert alert-danger">
            <p>Are you sure you want to delete "{}"?</p>
            <p>This will also delete:</p>
            <ul>{}</ul>
            <p><strong>Total: {} objects</strong></p>
        </div>
        <form method="post">
            <button type="submit" class="btn btn-danger">Yes, delete</button>
            <a href="/admin/{}/\" class="btn btn-secondary">Cancel</a>
        </form>
    </div>
</body>
</html>"#,
            ctx.model_name,
            ctx.admin.site_title,
            ctx.admin.site_header,
            ctx.model_name,
            ctx.object_repr,
            related_html,
            ctx.total_count,
            ctx.model_name.to_lowercase()
        )
    }

    fn render_dashboard_html(&self, ctx: &DashboardContext) -> String {
        let widgets_html = ctx
            .widgets
            .iter()
            .map(|w| {
                format!(
                    r#"<div class="col-md-4">
                    <div class="card {}">
                        <div class="card-header">{}</div>
                        <div class="card-body">{}</div>
                    </div>
                </div>"#,
                    w.css_class.as_deref().unwrap_or(""),
                    w.title,
                    w.content_html
                )
            })
            .collect::<Vec<_>>()
            .join("");

        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>{}</title>
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css" rel="stylesheet">
</head>
<body>
    <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
        <div class="container-fluid">
            <a class="navbar-brand" href="/admin/">{}</a>
        </div>
    </nav>
    <div class="container mt-4">
        <h1>Dashboard</h1>
        <div class="row g-3">
            {}
        </div>
    </div>
</body>
</html>"#,
            ctx.admin.site_title, ctx.admin.site_header, widgets_html
        )
    }
}

impl Default for AdminTemplateRenderer {
    fn default() -> Self {
        Self::new("templates/admin")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_context_new() {
        let ctx = AdminContext::new("My Admin");
        assert_eq!(ctx.site_title, "My Admin");
        assert_eq!(ctx.site_header, "Administration");
        assert!(ctx.user.is_none());
    }

    #[test]
    fn test_list_view_context() {
        let mut item = HashMap::new();
        item.insert("id".to_string(), serde_json::json!("1"));
        item.insert("name".to_string(), serde_json::json!("Test"));

        let ctx = ListViewContext::new("User", vec![item]);
        assert_eq!(ctx.model_name, "User");
        assert_eq!(ctx.items.len(), 1);
    }

    #[test]
    fn test_pagination_context_default() {
        let pag = PaginationContext::default();
        assert_eq!(pag.page, 1);
        assert_eq!(pag.page_size, 100);
        assert!(!pag.has_previous);
        assert!(!pag.has_next);
    }

    #[test]
    fn test_template_renderer_list() {
        let renderer = AdminTemplateRenderer::default();

        let mut item = HashMap::new();
        item.insert("id".to_string(), serde_json::json!("1"));
        item.insert("username".to_string(), serde_json::json!("alice"));

        let mut ctx = ListViewContext::new("User", vec![item]);
        ctx.list_display = vec!["id".to_string(), "username".to_string()];

        let html = renderer.render_list(&ctx).unwrap();
        assert!(html.contains("User"));
        assert!(html.contains("alice"));
    }
}
