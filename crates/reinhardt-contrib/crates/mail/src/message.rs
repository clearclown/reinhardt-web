pub struct Alternative;
pub struct Attachment;

#[derive(Debug, Clone)]
pub struct EmailMessage {
    pub subject: String,
    pub body: String,
    pub from_email: String,
    pub to: Vec<String>,
    pub html_body: Option<String>,
}

impl EmailMessage {
    pub fn new() -> EmailMessageBuilder {
        EmailMessageBuilder::default()
    }

    pub fn subject(&mut self, subject: impl Into<String>) -> &mut Self {
        self.subject = subject.into();
        self
    }

    pub fn body(&mut self, body: impl Into<String>) -> &mut Self {
        self.body = body.into();
        self
    }

    pub fn from_email(&mut self, from: impl Into<String>) -> &mut Self {
        self.from_email = from.into();
        self
    }

    pub fn to(&mut self, to: Vec<String>) -> &mut Self {
        self.to = to;
        self
    }

    pub fn send(&self, backend: &dyn crate::backends::EmailBackend) -> crate::EmailResult<()> {
        backend.send_messages(&[self.clone()])?;
        Ok(())
    }

    pub fn send_with_backend(
        &self,
        backend: &dyn crate::backends::EmailBackend,
    ) -> crate::EmailResult<()> {
        backend.send_messages(&[self.clone()])?;
        Ok(())
    }
}

#[derive(Default)]
pub struct EmailMessageBuilder {
    subject: String,
    body: String,
    from_email: String,
    to: Vec<String>,
    html_body: Option<String>,
}

impl EmailMessageBuilder {
    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = subject.into();
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = body.into();
        self
    }

    pub fn from(mut self, from: impl Into<String>) -> Self {
        self.from_email = from.into();
        self
    }

    pub fn from_email(mut self, from: impl Into<String>) -> Self {
        self.from_email = from.into();
        self
    }

    pub fn to(mut self, to: Vec<String>) -> Self {
        self.to = to;
        self
    }

    pub fn html(mut self, html: impl Into<String>) -> Self {
        self.html_body = Some(html.into());
        self
    }

    pub fn build(self) -> EmailMessage {
        EmailMessage {
            subject: self.subject,
            body: self.body,
            from_email: self.from_email,
            to: self.to,
            html_body: self.html_body,
        }
    }
}
