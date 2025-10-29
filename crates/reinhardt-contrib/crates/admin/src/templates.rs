//! Template rendering for admin interface
//!
//! This module provides template rendering using Askama template engine
//! integrated with reinhardt-templates.

use crate::{AdminError, AdminResult};
use askama::Template;
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
    pub is_required: bool,
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

// ============================================================================
// Askama Template Structs
// ============================================================================

/// List view template
#[derive(Template)]
#[template(path = "list.tpl")]
struct ListTemplate<'a> {
    // Base context fields
    site_title: &'a str,
    site_header: &'a str,
    user: Option<&'a UserContext>,
    available_apps: &'a [AppContext],

    // List view specific fields
    model_name: &'a str,
    model_verbose_name: &'a str,
    items: &'a [HashMap<String, serde_json::Value>],
    list_display: &'a [String],
    field_labels: &'a HashMap<String, String>,
    filters: &'a [FilterContext],
    search_query: Option<&'a str>,
    pagination: &'a PaginationContext,
    actions: &'a [ActionContext],
}

/// Form view template
#[derive(Template)]
#[template(path = "form.tpl")]
struct FormTemplate<'a> {
    // Base context fields
    site_title: &'a str,
    site_header: &'a str,
    user: Option<&'a UserContext>,
    available_apps: &'a [AppContext],

    // Form view specific fields
    model_name: &'a str,
    title: &'a str,
    fields: &'a [FormFieldContext],
    inlines: &'a [InlineFormsetContext],
    object_id: Option<&'a str>,
    errors: &'a [String],
}

/// Delete confirmation template
#[derive(Template)]
#[template(path = "delete_confirmation.tpl")]
struct DeleteConfirmationTemplate<'a> {
    // Base context fields
    site_title: &'a str,
    site_header: &'a str,
    user: Option<&'a UserContext>,
    available_apps: &'a [AppContext],

    // Delete confirmation specific fields
    model_name: &'a str,
    object_repr: &'a str,
    related_objects: &'a [RelatedObjectContext],
    total_count: usize,
}

/// Dashboard template
#[derive(Template)]
#[template(path = "dashboard.tpl")]
struct DashboardTemplate<'a> {
    // Base context fields
    site_title: &'a str,
    site_header: &'a str,
    user: Option<&'a UserContext>,
    available_apps: &'a [AppContext],

    // Dashboard specific fields
    widgets: &'a [WidgetContext],
    recent_actions: &'a [RecentActionContext],
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
        let template = ListTemplate {
            // Base context
            site_title: &context.admin.site_title,
            site_header: &context.admin.site_header,
            user: context.admin.user.as_ref(),
            available_apps: &context.admin.available_apps,
            // List view specific
            model_name: &context.model_name,
            model_verbose_name: &context.model_verbose_name,
            items: &context.items,
            list_display: &context.list_display,
            field_labels: &context.field_labels,
            filters: &context.filters,
            search_query: context.search_query.as_deref(),
            pagination: &context.pagination,
            actions: &context.actions,
        };

        template
            .render()
            .map_err(|e| AdminError::TemplateError(e.to_string()))
    }

    /// Render form view
    pub fn render_form(&self, context: &FormViewContext) -> AdminResult<String> {
        let template = FormTemplate {
            // Base context
            site_title: &context.admin.site_title,
            site_header: &context.admin.site_header,
            user: context.admin.user.as_ref(),
            available_apps: &context.admin.available_apps,
            // Form view specific
            model_name: &context.model_name,
            title: &context.title,
            fields: &context.fields,
            inlines: &context.inlines,
            object_id: context.object_id.as_deref(),
            errors: &context.errors,
        };

        template
            .render()
            .map_err(|e| AdminError::TemplateError(e.to_string()))
    }

    /// Render delete confirmation
    pub fn render_delete_confirmation(
        &self,
        context: &DeleteConfirmationContext,
    ) -> AdminResult<String> {
        let template = DeleteConfirmationTemplate {
            // Base context
            site_title: &context.admin.site_title,
            site_header: &context.admin.site_header,
            user: context.admin.user.as_ref(),
            available_apps: &context.admin.available_apps,
            // Delete confirmation specific
            model_name: &context.model_name,
            object_repr: &context.object_repr,
            related_objects: &context.related_objects,
            total_count: context.total_count,
        };

        template
            .render()
            .map_err(|e| AdminError::TemplateError(e.to_string()))
    }

    /// Render dashboard
    pub fn render_dashboard(&self, context: &DashboardContext) -> AdminResult<String> {
        let template = DashboardTemplate {
            // Base context
            site_title: &context.admin.site_title,
            site_header: &context.admin.site_header,
            user: context.admin.user.as_ref(),
            available_apps: &context.admin.available_apps,
            // Dashboard specific
            widgets: &context.widgets,
            recent_actions: &context.recent_actions,
        };

        template
            .render()
            .map_err(|e| AdminError::TemplateError(e.to_string()))
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
