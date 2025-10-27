# reinhardt-backends

Shared backend infrastructure for Reinhardt framework

## Overview

This crate provides a unified backend system for storing and retrieving data across different components of the Reinhardt framework, including:

- **Storage Backends**: Throttling, caching, and session storage
- **Email Backends**: Multi-provider email sending system

## Features

### Storage Backend Features

- **Backend Trait**: Generic key-value interface with TTL support
- **MemoryBackend**: High-performance in-memory storage with automatic expiration
- **RedisBackend**: Distributed storage using Redis (feature-gated)

### Email Backend Features

- **EmailBackend Trait**: Unified interface for sending emails
- **MemoryEmailBackend**: In-memory storage for testing
- **SmtpBackend**: Direct SMTP server connection using lettre
- **SendGridBackend**: SendGrid API integration
- **SesBackend**: AWS Simple Email Service
- **MailgunBackend**: Mailgun API integration

### Key Capabilities

- Async-first design using `async-trait`
- Automatic expiration with TTL support (storage)
- Type-safe serialization/deserialization with `serde`
- Thread-safe concurrent access
- HTML and plain text email support
- Email attachments
- CC/BCC recipients
- Bulk email sending

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
# Storage backends only
reinhardt-backends = { workspace = true }

# With Redis support
reinhardt-backends = { workspace = true, features = ["redis-backend"] }

# With all email backends
reinhardt-backends = { workspace = true, features = ["email"] }

# With specific email backends
reinhardt-backends = { workspace = true, features = ["email-smtp"] }
reinhardt-backends = { workspace = true, features = ["email-sendgrid"] }
reinhardt-backends = { workspace = true, features = ["email-ses"] }
reinhardt-backends = { workspace = true, features = ["email-mailgun"] }
```

## Usage Examples

### Memory Backend

```rust
use reinhardt_backends::{Backend, MemoryBackend};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let backend = MemoryBackend::new();

    // Store with TTL
    backend.set("user:123", "active", Some(Duration::from_secs(3600))).await.unwrap();

    // Retrieve
    let value: Option<String> = backend.get("user:123").await.unwrap();
    assert_eq!(value, Some("active".to_string()));

    // Counter operations
    let count = backend.increment("api:calls", Some(Duration::from_secs(60))).await.unwrap();
    println!("API call count: {}", count);
}
```

### Redis Backend

```rust
use reinhardt_backends::{Backend, RedisBackend};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let backend = RedisBackend::new("redis://localhost:6379").await.unwrap();

    // Same API as MemoryBackend
    backend.set("session:abc", vec![1, 2, 3], Some(Duration::from_secs(3600))).await.unwrap();

    let data: Option<Vec<u8>> = backend.get("session:abc").await.unwrap();
    assert_eq!(data, Some(vec![1, 2, 3]));
}
```

### Shared Backend Pattern

```rust
use reinhardt_backends::{Backend, MemoryBackend};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Create a shared backend
    let backend = Arc::new(MemoryBackend::new());

    // Use in throttling
    let throttle_backend = backend.clone();

    // Use in cache
    let cache_backend = backend.clone();

    // Use in session storage
    let session_backend = backend.clone();

    // All components share the same state
}
```

## API Documentation

### Backend Trait

```rust
#[async_trait]
pub trait Backend: Send + Sync {
    async fn set<V>(&self, key: &str, value: V, ttl: Option<Duration>) -> BackendResult<()>;
    async fn get<V>(&self, key: &str) -> BackendResult<Option<V>>;
    async fn delete(&self, key: &str) -> BackendResult<bool>;
    async fn exists(&self, key: &str) -> BackendResult<bool>;
    async fn increment(&self, key: &str, ttl: Option<Duration>) -> BackendResult<i64>;
    async fn clear(&self) -> BackendResult<()>;
}
```

### Memory Backend

- **Thread-safe**: Uses `DashMap` for concurrent access
- **Auto-cleanup**: Expired entries are removed automatically
- **Zero-cost**: No external dependencies when using memory backend

### Redis Backend

- **Distributed**: State shared across multiple servers
- **Persistent**: Data survives application restarts
- **Scalable**: Redis handles millions of operations per second

## Feature Flags

- `memory` (default): Enable in-memory backend
- `redis-backend`: Enable Redis backend

## Testing

```bash
# Run memory backend tests
cargo test --package reinhardt-backends

# Run Redis tests (requires Redis server)
cargo test --package reinhardt-backends --features redis-backend -- --ignored
```

## Performance

### Memory Backend

- **Throughput**: ~1M ops/sec (single-threaded)
- **Latency**: <1μs for get/set operations
- **Memory**: O(n) where n is the number of keys

### Redis Backend

- **Throughput**: ~100K ops/sec (depends on Redis)
- **Latency**: ~1-5ms (network + Redis)
- **Memory**: Managed by Redis

## Integration Examples

### Throttling Integration

```rust
use reinhardt_backends::{Backend, MemoryBackend};
use std::sync::Arc;

pub struct Throttle {
    backend: Arc<dyn Backend>,
    rate: String,
}

impl Throttle {
    pub fn new(backend: Arc<dyn Backend>, rate: &str) -> Self {
        Self {
            backend,
            rate: rate.to_string(),
        }
    }

    pub async fn allow(&self, key: &str) -> bool {
        let count = self.backend.increment(key, Some(std::time::Duration::from_secs(60))).await.unwrap();
        count <= 100 // Allow 100 requests per minute
    }
}
```

## Email Backend Examples

### Memory Email Backend (Testing)

```rust
use reinhardt_backends::email::{Email, EmailBackend, MemoryEmailBackend};

#[tokio::main]
async fn main() {
    let backend = MemoryEmailBackend::new();

    let email = Email::builder()
        .from("sender@example.com")
        .to("recipient@example.com")
        .subject("Test Email")
        .text_body("Hello, World!")
        .build();

    backend.send_email(&email).await.unwrap();

    // Check sent emails (useful for testing)
    let sent = backend.sent_emails();
    assert_eq!(sent.len(), 1);
}
```

### SMTP Backend

```rust
use reinhardt_backends::email::{Email, EmailBackend, SmtpBackend, SmtpConfig, SmtpAuth, SmtpEncryption};

#[tokio::main]
async fn main() {
    let config = SmtpConfig {
        host: "smtp.gmail.com".to_string(),
        port: 587,
        encryption: SmtpEncryption::StartTls,
        auth: Some(SmtpAuth {
            username: "user@gmail.com".to_string(),
            password: "password".to_string(),
        }),
        timeout: std::time::Duration::from_secs(30),
        pool_size: 5,
    };

    let backend = SmtpBackend::new(config).await.unwrap();

    let email = Email::builder()
        .from("sender@example.com")
        .to("recipient@example.com")
        .subject("Hello")
        .both_body("Plain text version", "<h1>HTML version</h1>")
        .build();

    backend.send_email(&email).await.unwrap();
}
```

### SendGrid Backend

```rust
use reinhardt_backends::email::{Email, EmailBackend, SendGridBackend};

#[tokio::main]
async fn main() {
    let backend = SendGridBackend::new("your-api-key".to_string());

    let email = Email::builder()
        .from("sender@example.com")
        .to("recipient@example.com")
        .subject("SendGrid Email")
        .html_body("<h1>Hello from SendGrid!</h1>")
        .build();

    backend.send_email(&email).await.unwrap();
}
```

### AWS SES Backend

```rust
use reinhardt_backends::email::{Email, EmailBackend, SesBackend};
use aws_config::BehaviorVersion;

#[tokio::main]
async fn main() {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region("us-east-1")
        .load()
        .await;

    let backend = SesBackend::new(&config);

    let email = Email::builder()
        .from("sender@example.com")
        .to("recipient@example.com")
        .subject("AWS SES Email")
        .text_body("Hello from AWS SES!")
        .build();

    backend.send_email(&email).await.unwrap();
}
```

### Mailgun Backend

```rust
use reinhardt_backends::email::{Email, EmailBackend, MailgunBackend, MailgunRegion};

#[tokio::main]
async fn main() {
    let backend = MailgunBackend::with_region(
        "your-api-key".to_string(),
        "your-domain.com".to_string(),
        MailgunRegion::US,
    );

    let email = Email::builder()
        .from("sender@your-domain.com")
        .to("recipient@example.com")
        .subject("Mailgun Email")
        .text_body("Hello from Mailgun!")
        .build();

    backend.send_email(&email).await.unwrap();
}
```

### Email with Attachments

```rust
use reinhardt_backends::email::{Email, EmailBackend, Attachment, MemoryEmailBackend};

#[tokio::main]
async fn main() {
    let backend = MemoryEmailBackend::new();

    let attachment = Attachment::new(
        "document.pdf",
        "application/pdf",
        vec![0x25, 0x50, 0x44, 0x46], // PDF content
    );

    let email = Email::builder()
        .from("sender@example.com")
        .to("recipient@example.com")
        .cc("cc@example.com")
        .bcc("bcc@example.com")
        .subject("Email with Attachment")
        .both_body("See attached document", "<p>See attached document</p>")
        .attachment(attachment)
        .build();

    backend.send_email(&email).await.unwrap();
}
```

### Bulk Email Sending

```rust
use reinhardt_backends::email::{Email, EmailBackend, MemoryEmailBackend};

#[tokio::main]
async fn main() {
    let backend = MemoryEmailBackend::new();

    let emails = vec![
        Email::builder()
            .from("sender@example.com")
            .to("user1@example.com")
            .subject("Notification")
            .text_body("Message 1")
            .build(),
        Email::builder()
            .from("sender@example.com")
            .to("user2@example.com")
            .subject("Notification")
            .text_body("Message 2")
            .build(),
    ];

    let results = backend.send_bulk(&emails).await.unwrap();
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(_) => println!("Email {} sent successfully", i),
            Err(e) => println!("Email {} failed: {}", i, e),
        }
    }
}
```

## Configuration Requirements

### SMTP Backend

- SMTP server hostname and port
- Optional authentication credentials
- TLS/STARTTLS configuration

### SendGrid Backend

- SendGrid API key (from SendGrid dashboard)

### AWS SES Backend

- AWS credentials configured (environment variables, IAM role, or config file)
- Verified sender email addresses in AWS SES
- AWS region configuration

### Mailgun Backend

- Mailgun API key (from Mailgun dashboard)
- Verified domain in Mailgun
- Region selection (US or EU)

## Feature Flags

### Storage Backends

- `memory` (default): Enable in-memory backend
- `redis-backend`: Enable Redis backend

### Email Backends

- `email-smtp`: Enable SMTP backend using lettre
- `email-sendgrid`: Enable SendGrid backend
- `email-ses`: Enable AWS SES backend
- `email-mailgun`: Enable Mailgun backend
- `email`: Enable all email backends

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.