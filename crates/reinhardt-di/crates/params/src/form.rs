//! Form data extraction

use async_trait::async_trait;
use reinhardt_apps::Request;
use serde::de::DeserializeOwned;
use std::fmt::{self, Debug};
use std::ops::Deref;

use crate::{extract::FromRequest, ParamContext, ParamError, ParamResult};

/// Extract form data from request body
pub struct Form<T>(pub T);

impl<T> Form<T> {
    /// Unwrap the Form and return the inner value
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_params::Form;
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize, Debug, PartialEq)]
    /// struct LoginForm {
    ///     username: String,
    ///     password: String,
    /// }
    ///
    /// let form = Form(LoginForm {
    ///     username: "alice".to_string(),
    ///     password: "secret123".to_string(),
    /// });
    /// let inner = form.into_inner();
    /// assert_eq!(inner.username, "alice");
    /// assert_eq!(inner.password, "secret123");
    /// ```
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for Form<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Debug> Debug for Form<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[async_trait]
impl<T> FromRequest for Form<T>
where
    T: DeserializeOwned + Send,
{
    async fn from_request(req: &Request, _ctx: &ParamContext) -> ParamResult<Self> {
        // Extract form data from request body
        // Form data is typically sent as application/x-www-form-urlencoded

        // Check Content-Type header
        let content_type = req
            .headers
            .get(http::header::CONTENT_TYPE)
            .and_then(|h| h.to_str().ok())
            .unwrap_or("");

        if !content_type.contains("application/x-www-form-urlencoded")
            && !content_type.contains("multipart/form-data")
        {
            return Err(ParamError::InvalidParameter {
                name: "Content-Type".to_string(),
                message: format!(
                    "Expected application/x-www-form-urlencoded or multipart/form-data, got {}",
                    content_type
                ),
            });
        }

        // Parse the body as form data
        // Note: multipart/form-data requires more complex parsing
        // For now, we handle application/x-www-form-urlencoded
        if content_type.contains("application/x-www-form-urlencoded") {
            let body_bytes = req
                .read_body()
                .map_err(|e| ParamError::BodyError(format!("Failed to read body: {}", e)))?;

            let body_str = std::str::from_utf8(&body_bytes)
                .map_err(|e| ParamError::BodyError(format!("Invalid UTF-8 in body: {}", e)))?;

            serde_urlencoded::from_str(body_str)
                .map(Form)
                .map_err(|e| e.into())
        } else {
            // multipart/form-data not yet implemented
            Err(ParamError::BodyError(
                "multipart/form-data parsing not yet implemented".to_string(),
            ))
        }
    }
}
