//! # Reinhardt Core
//!
//! Core components for the Reinhardt framework, providing fundamental types,
//! exception handling, signals, macros, security, and validation utilities.
//!
//! ## Planned Features
//!
//! The following enhancements are planned for future releases:
//!
//! ### Additional Middleware Types
//!
//! Planned middleware additions:
//! - **RateLimitMiddleware**: Per-route or per-user rate limiting
//! - **CacheMiddleware**: Response caching with various backends
//! - **ETa gMiddleware**: Automatic ETag generation and validation
//! - **SessionMiddleware**: Enhanced session management
//! - **MetricsMiddleware**: Request/response metrics collection
//! - **CircuitBreakerMiddleware**: Fault tolerance and resilience
//!
//! ### Enhanced Security Features
//!
//! Planned security enhancements:
//! - **Content Security Policy (CSP)**: Automatic CSP header management
//! - **HSTS**: HTTP Strict Transport Security support
//! - **XSS Protection**: Enhanced cross-site scripting prevention
//! - **CSRF Token Rotation**: Automatic token refresh
//! - **IP Filtering**: Whitelist/blacklist IP address ranges
//! - **Security Headers**: Automatic secure header injection
//!
//! ### Additional Validator Types
//!
//! Planned validator additions:
//! - **IPAddressValidator**: Validate IPv4/IPv6 addresses
//! - **PhoneNumberValidator**: International phone number validation
//! - **CreditCardValidator**: Credit card number validation
//! - **IBANValidator**: International bank account number validation
//! - **ColorValidator**: Hex, RGB, HSL color validation
//! - **FileTypeValidator**: MIME type and extension validation
//! - **CustomRegexValidator**: User-defined regex patterns
//!
//! ### Additional Backend Implementations
//!
//! Planned backend support:
//! - **Cache Backends**: Redis, Memcached, DynamoDB
//! - **Session Backends**: Database, Redis, JWT
//! - **Email Backends**: SMTP, SendGrid, AWS SES, Mailgun
//! - **Storage Backends**: S3, Azure Blob, GCS
//! - **Queue Backends**: Redis, RabbitMQ, AWS SQS
//!
//! For detailed implementation plans and design discussions, see the individual
//! crate documentation in `reinhardt-middleware`, `reinhardt-security`,
//! `reinhardt-validators`, and `reinhardt-backends`.

#[cfg(feature = "types")]
pub use reinhardt_types as types;

#[cfg(feature = "exception")]
pub use reinhardt_exception as exception;

#[cfg(feature = "signals")]
pub use reinhardt_signals as signals;

#[cfg(feature = "macros")]
pub use reinhardt_macros as macros;

#[cfg(feature = "security")]
pub use reinhardt_security as security;

#[cfg(feature = "validators")]
pub use reinhardt_validators as validators;
