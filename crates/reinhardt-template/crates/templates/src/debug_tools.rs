//! Debugging tools for templates
//!
//! Provides utilities for debugging template rendering:
//! - Variable inspection (debug filter)
//! - Template profiling (render time tracking)
//! - Context dumping (view all available variables)

use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, Instant};

/// Template debug filter for printing variables
///
/// # Examples
///
/// ```
/// use reinhardt_templates::debug_filter;
///
/// let value = "test";
/// let result = debug_filter(value);
/// assert!(result.contains("test"));
/// ```
pub fn debug_filter<T: fmt::Debug>(value: &T) -> String {
    format!("{:?}", value)
}

/// Template context for debugging
#[derive(Debug, Clone)]
pub struct TemplateContext {
    /// Available variables and their values
    pub variables: HashMap<String, String>,
}

impl TemplateContext {
    /// Create a new template context
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_templates::TemplateContext;
    ///
    /// let context = TemplateContext::new();
    /// assert!(context.variables.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    /// Add a variable to the context
    pub fn add_variable(&mut self, name: impl Into<String>, value: impl fmt::Display) {
        self.variables.insert(name.into(), value.to_string());
    }

    /// Dump all variables as a formatted string
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_templates::TemplateContext;
    ///
    /// let mut context = TemplateContext::new();
    /// context.add_variable("name", "Alice");
    /// context.add_variable("age", 30);
    ///
    /// let dump = context.dump();
    /// assert!(dump.contains("name = Alice"));
    /// assert!(dump.contains("age = 30"));
    /// ```
    pub fn dump(&self) -> String {
        let mut output = String::from("Template Context:\n");
        for (key, value) in &self.variables {
            output.push_str(&format!("  {} = {}\n", key, value));
        }
        output
    }
}

impl Default for TemplateContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Template profiling information
#[derive(Debug, Clone)]
pub struct TemplateProfile {
    /// Template name
    pub template_name: String,
    /// Render start time
    start_time: Option<Instant>,
    /// Render duration
    pub duration: Option<Duration>,
    /// Number of variables accessed
    pub variable_accesses: usize,
    /// Number of filters applied
    pub filters_applied: usize,
}

impl TemplateProfile {
    /// Create a new template profile
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_templates::TemplateProfile;
    ///
    /// let profile = TemplateProfile::new("user_list.html");
    /// assert_eq!(profile.template_name, "user_list.html");
    /// ```
    pub fn new(template_name: impl Into<String>) -> Self {
        Self {
            template_name: template_name.into(),
            start_time: None,
            duration: None,
            variable_accesses: 0,
            filters_applied: 0,
        }
    }

    /// Start timing the render
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    /// Stop timing the render
    pub fn stop(&mut self) {
        if let Some(start) = self.start_time {
            self.duration = Some(start.elapsed());
        }
    }

    /// Record a variable access
    pub fn record_variable_access(&mut self) {
        self.variable_accesses += 1;
    }

    /// Record a filter application
    pub fn record_filter(&mut self) {
        self.filters_applied += 1;
    }

    /// Get a summary of the profile
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_templates::TemplateProfile;
    ///
    /// let mut profile = TemplateProfile::new("test.html");
    /// profile.start();
    /// profile.record_variable_access();
    /// profile.record_filter();
    /// profile.stop();
    ///
    /// let summary = profile.summary();
    /// assert!(summary.contains("test.html"));
    /// assert!(summary.contains("Variables accessed: 1"));
    /// ```
    pub fn summary(&self) -> String {
        let mut output = format!("Template: {}\n", self.template_name);

        if let Some(duration) = self.duration {
            output.push_str(&format!("Render time: {:?}\n", duration));
        }

        output.push_str(&format!("Variables accessed: {}\n", self.variable_accesses));
        output.push_str(&format!("Filters applied: {}\n", self.filters_applied));

        output
    }
}

/// Debug panel for templates
#[derive(Debug, Clone)]
pub struct DebugPanel {
    /// Whether debug mode is enabled
    pub enabled: bool,
    /// Template profiles
    pub profiles: Vec<TemplateProfile>,
    /// Template contexts
    pub contexts: HashMap<String, TemplateContext>,
}

impl DebugPanel {
    /// Create a new debug panel
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_templates::DebugPanel;
    ///
    /// let panel = DebugPanel::new();
    /// assert!(!panel.enabled);
    /// ```
    pub fn new() -> Self {
        Self {
            enabled: false,
            profiles: Vec::new(),
            contexts: HashMap::new(),
        }
    }

    /// Enable debug mode
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable debug mode
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Add a profile
    pub fn add_profile(&mut self, profile: TemplateProfile) {
        self.profiles.push(profile);
    }

    /// Add a context
    pub fn add_context(&mut self, name: impl Into<String>, context: TemplateContext) {
        self.contexts.insert(name.into(), context);
    }

    /// Get summary of all profiles
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_templates::{DebugPanel, TemplateProfile};
    ///
    /// let mut panel = DebugPanel::new();
    /// panel.enable();
    ///
    /// let mut profile = TemplateProfile::new("test.html");
    /// profile.start();
    /// profile.stop();
    /// panel.add_profile(profile);
    ///
    /// let summary = panel.summary();
    /// assert!(summary.contains("Templates Rendered: 1"));
    /// ```
    pub fn summary(&self) -> String {
        let mut output = String::from("=== Template Debug Panel ===\n\n");

        output.push_str(&format!("Templates Rendered: {}\n\n", self.profiles.len()));

        for profile in &self.profiles {
            output.push_str(&profile.summary());
            output.push('\n');
        }

        output
    }

    /// Get context dump for a template
    pub fn get_context(&self, template: &str) -> Option<String> {
        self.contexts.get(template).map(|ctx| ctx.dump())
    }
}

impl Default for DebugPanel {
    fn default() -> Self {
        Self::new()
    }
}

use std::sync::{Mutex, OnceLock};

/// Global debug panel (for development use)
static DEBUG_PANEL: OnceLock<Mutex<DebugPanel>> = OnceLock::new();

/// Initialize the global debug panel
///
/// # Examples
///
/// ```
/// use reinhardt_templates::init_debug_panel;
///
/// init_debug_panel();
/// ```
pub fn init_debug_panel() {
    DEBUG_PANEL.get_or_init(|| Mutex::new(DebugPanel::new()));
}

/// Get a reference to the global debug panel
pub fn get_debug_panel() -> Option<std::sync::MutexGuard<'static, DebugPanel>> {
    DEBUG_PANEL.get().and_then(|m| m.lock().ok())
}

/// Get a mutable reference to the global debug panel (same as get_debug_panel)
pub fn get_debug_panel_mut() -> Option<std::sync::MutexGuard<'static, DebugPanel>> {
    DEBUG_PANEL.get().and_then(|m| m.lock().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_filter() {
        let value = "test";
        let result = debug_filter(&value);
        assert!(result.contains("test"));
    }

    #[test]
    fn test_template_context_new() {
        let context = TemplateContext::new();
        assert!(context.variables.is_empty());
    }

    #[test]
    fn test_template_context_add_variable() {
        let mut context = TemplateContext::new();
        context.add_variable("name", "Alice");
        context.add_variable("age", 30);

        assert_eq!(context.variables.get("name"), Some(&"Alice".to_string()));
        assert_eq!(context.variables.get("age"), Some(&"30".to_string()));
    }

    #[test]
    fn test_template_context_dump() {
        let mut context = TemplateContext::new();
        context.add_variable("name", "Alice");
        context.add_variable("age", 30);

        let dump = context.dump();
        assert!(dump.contains("Template Context:"));
        assert!(dump.contains("name = Alice"));
        assert!(dump.contains("age = 30"));
    }

    #[test]
    fn test_template_profile_new() {
        let profile = TemplateProfile::new("test.html");
        assert_eq!(profile.template_name, "test.html");
        assert_eq!(profile.variable_accesses, 0);
        assert_eq!(profile.filters_applied, 0);
    }

    #[test]
    fn test_template_profile_timing() {
        let mut profile = TemplateProfile::new("test.html");
        profile.start();
        std::thread::sleep(std::time::Duration::from_millis(10));
        profile.stop();

        assert!(profile.duration.is_some());
        assert!(profile.duration.unwrap().as_millis() >= 10);
    }

    #[test]
    fn test_template_profile_record() {
        let mut profile = TemplateProfile::new("test.html");
        profile.record_variable_access();
        profile.record_variable_access();
        profile.record_filter();

        assert_eq!(profile.variable_accesses, 2);
        assert_eq!(profile.filters_applied, 1);
    }

    #[test]
    fn test_template_profile_summary() {
        let mut profile = TemplateProfile::new("test.html");
        profile.start();
        profile.record_variable_access();
        profile.stop();

        let summary = profile.summary();
        assert!(summary.contains("test.html"));
        assert!(summary.contains("Variables accessed: 1"));
    }

    #[test]
    fn test_debug_panel_new() {
        let panel = DebugPanel::new();
        assert!(!panel.enabled);
        assert!(panel.profiles.is_empty());
        assert!(panel.contexts.is_empty());
    }

    #[test]
    fn test_debug_panel_enable_disable() {
        let mut panel = DebugPanel::new();
        panel.enable();
        assert!(panel.enabled);

        panel.disable();
        assert!(!panel.enabled);
    }

    #[test]
    fn test_debug_panel_add_profile() {
        let mut panel = DebugPanel::new();
        let profile = TemplateProfile::new("test.html");
        panel.add_profile(profile);

        assert_eq!(panel.profiles.len(), 1);
    }

    #[test]
    fn test_debug_panel_add_context() {
        let mut panel = DebugPanel::new();
        let mut context = TemplateContext::new();
        context.add_variable("name", "Alice");
        panel.add_context("test.html", context);

        assert_eq!(panel.contexts.len(), 1);
    }

    #[test]
    fn test_debug_panel_summary() {
        let mut panel = DebugPanel::new();
        let mut profile = TemplateProfile::new("test.html");
        profile.start();
        profile.stop();
        panel.add_profile(profile);

        let summary = panel.summary();
        assert!(summary.contains("Templates Rendered: 1"));
        assert!(summary.contains("test.html"));
    }

    #[test]
    fn test_debug_panel_get_context() {
        let mut panel = DebugPanel::new();
        let mut context = TemplateContext::new();
        context.add_variable("name", "Alice");
        panel.add_context("test.html", context);

        let dump = panel.get_context("test.html");
        assert!(dump.is_some());
        assert!(dump.unwrap().contains("name = Alice"));
    }
}
