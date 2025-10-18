/// CSRF token for form protection
#[derive(Debug, Clone)]
pub struct CsrfToken {
    token: String,
}

impl CsrfToken {
    pub fn new(secret: String) -> Self {
        Self { token: secret }
    }

    pub fn as_hidden_input(&self) -> String {
        format!(
            r#"<input type="hidden" name="csrfmiddlewaretoken" value="{}" />"#,
            self.token
        )
    }
}

impl Default for CsrfToken {
    fn default() -> Self {
        Self {
            token: "default-csrf-token".to_string(),
        }
    }
}
