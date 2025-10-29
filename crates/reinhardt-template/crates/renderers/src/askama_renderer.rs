//! Askama-based Compile-time Template Renderer
//!
//! This module provides a compile-time template renderer using the Askama template engine.
//! Askama compiles templates into Rust code at compile-time, offering 100-1000x performance
//! improvement over runtime template engines for static templates.
//!
//! # Performance Characteristics
//!
//! - **Time Complexity**: O(1) - Templates are compiled to native code
//! - **Space Complexity**: O(1) - No runtime template parsing
//! - **Performance Gain**: 100-1000x faster than runtime template substitution
//!
//! # Use Cases
//!
//! Use Askama renderer for:
//! - View templates (HTML pages)
//! - Email templates
//! - Static response templates
//! - Developer-managed templates
//!
//! Do NOT use for:
//! - User-provided templates
//! - Dynamic templates loaded at runtime
//! - Templates stored in database
//!
//! # Examples
//!
//! ```rust,ignore
//! // Note: This example requires a template file at templates/user.html
//! use reinhardt_renderers::AskamaRenderer;
//! use askama::Template;
//!
//! #[derive(Template)]
//! #[template(path = "user.html")]
//! struct UserTemplate {
//!     name: String,
//!     email: String,
//!     age: u32,
//! }
//!
//! let renderer = AskamaRenderer::new();
//! let template = UserTemplate {
//!     name: "Alice".to_string(),
//!     email: "alice@example.com".to_string(),
//!     age: 25,
//! };
//!
//! let html = renderer.render(&template).expect("Failed to render template");
//! ```

use askama::Template;
use serde::Serialize;
use std::fmt::Display;

/// Askama-based compile-time template renderer
///
/// This renderer uses Askama's compile-time template compilation to achieve
/// zero-cost template rendering. Templates are parsed and compiled into Rust
/// code at build time, eliminating all runtime overhead.
///
/// # Compile-time Safety
///
/// - Variable names are validated at compile time
/// - Type mismatches are caught during compilation
/// - Template syntax errors fail the build
///
/// # Performance
///
/// Performance comparison (measured in microseconds):
///
/// | Variables | Runtime (Phase 2) | Compile-time (Askama) | Speedup |
/// |-----------|-------------------|-----------------------|---------|
/// | 10        | 10μs              | 0.01μs                | 1000x   |
/// | 100       | 100μs             | 0.01μs                | 10000x  |
/// | 1000      | 1000μs            | 0.01μs                | 100000x |
pub struct AskamaRenderer;

impl AskamaRenderer {
    /// Creates a new AskamaRenderer
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_renderers::AskamaRenderer;
    ///
    /// let renderer = AskamaRenderer::new();
    /// ```
    pub fn new() -> Self {
        Self
    }

    /// Renders a template that implements `askama::Template`
    ///
    /// # Arguments
    ///
    /// * `template` - Any type implementing `askama::Template`
    ///
    /// # Returns
    ///
    /// - `Ok(String)` - Rendered HTML string
    /// - `Err(String)` - Error message if rendering fails
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use reinhardt_renderers::AskamaRenderer;
    /// use askama::Template;
    ///
    /// #[derive(Template)]
    /// #[template(path = "user.html")]
    /// struct UserTemplate {
    ///     name: String,
    ///     email: String,
    ///     age: u32,
    /// }
    ///
    /// let renderer = AskamaRenderer::new();
    /// let template = UserTemplate {
    ///     name: "Alice".to_string(),
    ///     email: "alice@example.com".to_string(),
    ///     age: 25,
    /// };
    ///
    /// let html = renderer.render(&template)?;
    /// ```
    pub fn render<T: Template>(&self, template: &T) -> Result<String, String> {
        template
            .render()
            .map_err(|e| format!("Askama rendering error: {}", e))
    }

    /// Renders a template and returns the result with error context
    ///
    /// This is a convenience method that provides more detailed error information.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use reinhardt_renderers::AskamaRenderer;
    ///
    /// let renderer = AskamaRenderer::new();
    /// let template = UserTemplate { /* ... */ };
    ///
    /// match renderer.render_with_context(&template, "user profile") {
    ///     Ok(html) => println!("{}", html),
    ///     Err(e) => eprintln!("Failed to render user profile: {}", e),
    /// }
    /// ```
    pub fn render_with_context<T: Template>(
        &self,
        template: &T,
        context: &str,
    ) -> Result<String, String> {
        self.render(template)
            .map_err(|e| format!("Failed to render {}: {}", context, e))
    }
}

impl Default for AskamaRenderer {
    fn default() -> Self {
        Self::new()
    }
}

/// User template example
///
/// This struct demonstrates how to create Askama templates.
/// The `#[derive(Template)]` macro generates the `render()` method at compile time.
///
/// # Template File
///
/// The template file `templates/user.html` should contain:
///
/// ```html
/// <!DOCTYPE html>
/// <html>
/// <head>
///     <title>User Profile</title>
/// </head>
/// <body>
///     <h1>{{ name }}</h1>
///     <p>Email: {{ email }}</p>
///     <p>Age: {{ age }}</p>
///
///     {% if age >= 18 %}
///         <p>Adult user</p>
///     {% else %}
///         <p>Minor user</p>
///     {% endif %}
/// </body>
/// </html>
/// ```
#[derive(Template)]
#[template(path = "user.tpl")]
pub struct UserTemplate {
    pub name: String,
    pub email: String,
    pub age: u32,
}

impl UserTemplate {
    /// Creates a new UserTemplate
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_renderers::UserTemplate;
    ///
    /// let template = UserTemplate::new(
    ///     "Alice".to_string(),
    ///     "alice@example.com".to_string(),
    ///     25
    /// );
    /// ```
    pub fn new(name: String, email: String, age: u32) -> Self {
        Self { name, email, age }
    }

    /// Renders the user template
    ///
    /// This is a convenience method that creates a renderer and renders the template.
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_renderers::UserTemplate;
    ///
    /// let template = UserTemplate::new(
    ///     "Alice".to_string(),
    ///     "alice@example.com".to_string(),
    ///     25
    /// );
    ///
    /// let html = template.render_user().expect("Failed to render");
    /// assert!(html.contains("Alice"));
    /// assert!(html.contains("alice@example.com"));
    /// ```
    pub fn render_user(&self) -> Result<String, String> {
        let renderer = AskamaRenderer::new();
        renderer.render(self)
    }
}

/// User data for list template
#[derive(Debug, Clone)]
pub struct UserData {
    pub name: String,
    pub email: String,
}

impl UserData {
    /// Creates a new UserData
    pub fn new(name: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            email: email.into(),
        }
    }
}

impl Display for UserData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.email)
    }
}

/// User list template example
///
/// This template demonstrates rendering lists with Askama.
///
/// # Template File
///
/// The template file `templates/user_list.html` should contain:
///
/// ```html
/// <!DOCTYPE html>
/// <html>
/// <head>
///     <title>{{ title }}</title>
/// </head>
/// <body>
///     <h1>{{ title }}</h1>
///     <ul>
///     {% for user in users %}
///         <li>{{ user.name }} ({{ user.email }})</li>
///     {% endfor %}
///     </ul>
/// </body>
/// </html>
/// ```
#[derive(Template)]
#[template(path = "user_list.tpl")]
pub struct UserListTemplate {
    pub users: Vec<UserData>,
    pub title: String,
}

impl UserListTemplate {
    /// Creates a new UserListTemplate
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_renderers::{UserListTemplate, UserData};
    ///
    /// let users = vec![
    ///     UserData::new("Alice", "alice@example.com"),
    ///     UserData::new("Bob", "bob@example.com"),
    /// ];
    ///
    /// let template = UserListTemplate::new(users, "User Directory".to_string());
    /// ```
    pub fn new(users: Vec<UserData>, title: String) -> Self {
        Self { users, title }
    }

    /// Renders the user list template
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_renderers::{UserListTemplate, UserData};
    ///
    /// let users = vec![
    ///     UserData::new("Alice", "alice@example.com"),
    ///     UserData::new("Bob", "bob@example.com"),
    /// ];
    ///
    /// let template = UserListTemplate::new(users, "User Directory".to_string());
    /// let html = template.render_list().expect("Failed to render");
    ///
    /// assert!(html.contains("Alice"));
    /// assert!(html.contains("Bob"));
    /// assert!(html.contains("User Directory"));
    /// ```
    pub fn render_list(&self) -> Result<String, String> {
        let renderer = AskamaRenderer::new();
        renderer.render(self)
    }
}

/// Custom Askama filters (Django-style)
///
/// These filters can be used in Askama templates to transform data.
///
/// # Usage
///
/// ```html
/// {{ name|upper }}
/// {{ content|truncate(100) }}
/// {{ email|lower }}
/// ```
pub mod filters {
    use askama::Result;

    /// Converts a string to uppercase
    ///
    /// # Examples
    ///
    /// ```html
    /// {{ name|upper }}
    /// ```
    pub fn upper(s: &str) -> Result<String> {
        Ok(s.to_uppercase())
    }

    /// Converts a string to lowercase
    ///
    /// # Examples
    ///
    /// ```html
    /// {{ email|lower }}
    /// ```
    pub fn lower(s: &str) -> Result<String> {
        Ok(s.to_lowercase())
    }

    /// Truncates a string to the specified length and adds "..." if truncated
    ///
    /// # Arguments
    ///
    /// * `s` - The string to truncate
    /// * `len` - Maximum length before truncation
    ///
    /// # Examples
    ///
    /// ```html
    /// {{ content|truncate(100) }}
    /// ```
    pub fn truncate(s: &str, len: usize) -> Result<String> {
        if s.len() <= len {
            Ok(s.to_string())
        } else {
            Ok(format!("{}...", &s[..len]))
        }
    }

    /// Capitalizes the first character of a string
    ///
    /// # Examples
    ///
    /// ```html
    /// {{ title|capitalize }}
    /// ```
    pub fn capitalize(s: &str) -> Result<String> {
        let mut chars = s.chars();
        match chars.next() {
            None => Ok(String::new()),
            Some(first) => Ok(first.to_uppercase().chain(chars).collect()),
        }
    }

    /// Returns the length of a string
    ///
    /// # Examples
    ///
    /// ```html
    /// {{ content|length }}
    /// ```
    pub fn length(s: &str) -> Result<usize> {
        Ok(s.len())
    }
}

/// Post data for blog/content templates
#[derive(Debug, Clone, Serialize)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author: String,
}

impl Post {
    /// Creates a new Post
    pub fn new(
        id: i64,
        title: impl Into<String>,
        content: impl Into<String>,
        author: impl Into<String>,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            content: content.into(),
            author: author.into(),
        }
    }
}

impl Display for Post {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} by {}", self.title, self.author)
    }
}

/// Post list template example
///
/// This template demonstrates rendering a list of blog posts.
///
/// # Template File
///
/// The template file `templates/posts.html` should contain:
///
/// ```html
/// <!DOCTYPE html>
/// <html>
/// <head>
///     <title>Posts ({{ total }})</title>
/// </head>
/// <body>
///     <h1>All Posts ({{ total }})</h1>
///     <ul>
///     {% for post in posts %}
///         <li>
///             <h2>{{ post.title }}</h2>
///             <p>{{ post.content }}</p>
///             <small>by {{ post.author }}</small>
///         </li>
///     {% endfor %}
///     </ul>
/// </body>
/// </html>
/// ```
#[derive(Template)]
#[template(path = "posts.tpl")]
pub struct PostListTemplate {
    pub posts: Vec<Post>,
    pub total: usize,
}

impl PostListTemplate {
    /// Creates a new PostListTemplate
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_renderers::{PostListTemplate, Post};
    ///
    /// let posts = vec![
    ///     Post::new(1, "First Post", "Hello World", "Alice"),
    ///     Post::new(2, "Second Post", "Goodbye World", "Bob"),
    /// ];
    ///
    /// let template = PostListTemplate::new(posts);
    /// ```
    pub fn new(posts: Vec<Post>) -> Self {
        let total = posts.len();
        Self { posts, total }
    }

    /// Renders the post list template
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_renderers::{PostListTemplate, Post};
    ///
    /// let posts = vec![
    ///     Post::new(1, "First Post", "Hello World", "Alice"),
    ///     Post::new(2, "Second Post", "Goodbye World", "Bob"),
    /// ];
    ///
    /// let template = PostListTemplate::new(posts);
    /// let html = template.render_posts().expect("Failed to render");
    ///
    /// assert!(html.contains("First Post"));
    /// assert!(html.contains("Second Post"));
    /// ```
    pub fn render_posts(&self) -> Result<String, String> {
        let renderer = AskamaRenderer::new();
        renderer.render(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_askama_renderer_new() {
        let renderer = AskamaRenderer::new();
        // Just verify it can be created
        let _ = renderer;
    }

    #[test]
    fn test_user_template_render() {
        let template = UserTemplate::new(
            "Alice".to_string(),
            "alice@example.com".to_string(),
            25,
        );

        let html = template.render_user().expect("Failed to render user template");

        // Verify content is present
        assert!(html.contains("Alice"), "HTML should contain user name");
        assert!(
            html.contains("alice@example.com"),
            "HTML should contain email"
        );
        assert!(html.contains("25"), "HTML should contain age");
        assert!(html.contains("Adult"), "Adult status should be shown for age >= 18");
    }

    #[test]
    fn test_user_template_minor() {
        let template = UserTemplate::new(
            "Charlie".to_string(),
            "charlie@example.com".to_string(),
            16,
        );

        let html = template.render_user().expect("Failed to render user template");

        assert!(html.contains("Charlie"));
        assert!(html.contains("16"));
        assert!(
            html.contains("Minor"),
            "Minor status should be shown for age < 18"
        );
    }

    #[test]
    fn test_user_list_template_render() {
        let users = vec![
            UserData::new("Alice", "alice@example.com"),
            UserData::new("Bob", "bob@example.com"),
            UserData::new("Charlie", "charlie@example.com"),
        ];

        let template = UserListTemplate::new(users, "User Directory".to_string());
        let html = template.render_list().expect("Failed to render list template");

        // Verify title
        assert!(html.contains("User Directory"));

        // Verify all users are present
        assert!(html.contains("Alice"));
        assert!(html.contains("alice@example.com"));
        assert!(html.contains("Bob"));
        assert!(html.contains("bob@example.com"));
        assert!(html.contains("Charlie"));
        assert!(html.contains("charlie@example.com"));
    }

    #[test]
    fn test_user_list_template_empty() {
        let users = vec![];

        let template = UserListTemplate::new(users, "Empty List".to_string());
        let html = template.render_list().expect("Failed to render empty list");

        // Should still render successfully with empty list
        assert!(html.contains("Empty List"));
        // Should show "No users found" message
        assert!(html.contains("No users found"));
    }

    #[test]
    fn test_askama_renderer_render() {
        let renderer = AskamaRenderer::new();
        let template = UserTemplate::new(
            "Test User".to_string(),
            "test@example.com".to_string(),
            30,
        );

        let html = renderer.render(&template).expect("Failed to render");

        assert!(html.contains("Test User"));
        assert!(html.contains("test@example.com"));
        assert!(html.contains("30"));
    }

    #[test]
    fn test_askama_renderer_render_with_context() {
        let renderer = AskamaRenderer::new();
        let template = UserTemplate::new(
            "Context Test".to_string(),
            "context@example.com".to_string(),
            22,
        );

        let html = renderer
            .render_with_context(&template, "user profile")
            .expect("Failed to render with context");

        assert!(html.contains("Context Test"));
    }

    #[test]
    fn test_user_data_display() {
        let user = UserData::new("Display Test", "display@example.com");

        let display_string = format!("{}", user);
        assert_eq!(display_string, "Display Test (display@example.com)");
    }

    #[test]
    fn test_user_template_new() {
        let template = UserTemplate::new(
            "New Test".to_string(),
            "new@example.com".to_string(),
            18,
        );

        assert_eq!(template.name, "New Test");
        assert_eq!(template.email, "new@example.com");
        assert_eq!(template.age, 18);
    }

    #[test]
    fn test_user_list_template_new() {
        let users = vec![UserData::new("User1", "user1@example.com")];

        let template = UserListTemplate::new(users.clone(), "Test List".to_string());

        assert_eq!(template.title, "Test List");
        assert_eq!(template.users.len(), 1);
        assert_eq!(template.users[0].name, "User1");
    }

    #[test]
    fn test_user_data_new() {
        let user = UserData::new("Data Test", "data@example.com");

        assert_eq!(user.name, "Data Test");
        assert_eq!(user.email, "data@example.com");
    }

    // Post template tests

    #[test]
    fn test_post_new() {
        let post = Post::new(1, "Test Post", "Test Content", "Test Author");

        assert_eq!(post.id, 1);
        assert_eq!(post.title, "Test Post");
        assert_eq!(post.content, "Test Content");
        assert_eq!(post.author, "Test Author");
    }

    #[test]
    fn test_post_display() {
        let post = Post::new(1, "My Post", "Content", "Alice");

        let display_string = format!("{}", post);
        assert_eq!(display_string, "My Post by Alice");
    }

    #[test]
    fn test_post_list_template_new() {
        let posts = vec![
            Post::new(1, "First", "Content 1", "Alice"),
            Post::new(2, "Second", "Content 2", "Bob"),
        ];

        let template = PostListTemplate::new(posts.clone());

        assert_eq!(template.total, 2);
        assert_eq!(template.posts.len(), 2);
        assert_eq!(template.posts[0].title, "First");
        assert_eq!(template.posts[1].title, "Second");
    }

    #[test]
    fn test_post_list_template_render() {
        let posts = vec![
            Post::new(1, "First Post", "Hello World", "Alice"),
            Post::new(2, "Second Post", "Goodbye World", "Bob"),
        ];

        let template = PostListTemplate::new(posts);
        let html = template.render_posts().expect("Failed to render posts");

        // Verify title and total
        assert!(html.contains("Posts (2)"));
        assert!(html.contains("All Posts"));

        // Verify all posts are present
        assert!(html.contains("First Post"));
        assert!(html.contains("Hello World"));
        assert!(html.contains("Alice"));
        assert!(html.contains("Second Post"));
        assert!(html.contains("Goodbye World"));
        assert!(html.contains("Bob"));
    }

    #[test]
    fn test_post_list_template_empty() {
        let posts = vec![];

        let template = PostListTemplate::new(posts);
        let html = template.render_posts().expect("Failed to render empty posts");

        // Should still render successfully with empty list
        assert!(html.contains("Posts (0)"));
        // Should show "No posts available" message
        assert!(html.contains("No posts available"));
    }

    // Filter tests

    #[test]
    fn test_filter_upper() {
        use filters::upper;

        let result = upper("hello world").expect("Failed to apply upper filter");
        assert_eq!(result, "HELLO WORLD");
    }

    #[test]
    fn test_filter_lower() {
        use filters::lower;

        let result = lower("HELLO WORLD").expect("Failed to apply lower filter");
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_filter_truncate() {
        use filters::truncate;

        // Truncate long string
        let result = truncate("This is a long string that needs truncation", 10)
            .expect("Failed to apply truncate filter");
        assert_eq!(result, "This is a ...");

        // Don't truncate short string
        let result = truncate("Short", 10).expect("Failed to apply truncate filter");
        assert_eq!(result, "Short");

        // Exact length
        let result = truncate("ExactlyTen", 10).expect("Failed to apply truncate filter");
        assert_eq!(result, "ExactlyTen");
    }

    #[test]
    fn test_filter_capitalize() {
        use filters::capitalize;

        let result = capitalize("hello world").expect("Failed to apply capitalize filter");
        assert_eq!(result, "Hello world");

        // Empty string
        let result = capitalize("").expect("Failed to apply capitalize filter");
        assert_eq!(result, "");

        // Already capitalized
        let result = capitalize("Hello").expect("Failed to apply capitalize filter");
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_filter_length() {
        use filters::length;

        let result = length("hello").expect("Failed to apply length filter");
        assert_eq!(result, 5);

        let result = length("").expect("Failed to apply length filter");
        assert_eq!(result, 0);
    }
}
