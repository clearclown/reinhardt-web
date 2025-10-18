# reinhardt-middleware

Request/response processing pipeline for Reinhardt framework

## Overview

Middleware system for processing requests and responses. Provides comprehensive built-in middleware for security, performance optimization, authentication, and request handling.

## Implemented Features ✓

### Core Middleware System
- **Middleware Pipeline** - Request/response processing chain with handler composition
- **Custom Middleware Support** - Easy integration of user-defined middleware

### Security Middleware
- **CORS (Cross-Origin Resource Sharing)** - Configurable CORS headers with preflight support
  - Custom origin, method, and header configuration
  - Credentials support
  - Max-age caching
  - Permissive mode for development
- **CSRF Protection** - Cross-Site Request Forgery protection via `reinhardt-security`
  - Token generation and validation
  - Origin and referer checking
  - Secret management and rotation
- **Content Security Policy (CSP)** - XSS protection with customizable directives
  - Custom CSP directives (default-src, script-src, style-src, etc.)
  - Nonce generation for inline scripts/styles
  - Report-Only mode for testing
  - Strict preset configuration
- **X-Frame-Options** - Clickjacking protection
  - DENY mode (no framing)
  - SAMEORIGIN mode (same-origin framing only)
- **Security Headers** - Comprehensive HTTP security headers
  - HSTS (HTTP Strict Transport Security) with preload support
  - SSL/HTTPS redirects
  - X-Content-Type-Options: nosniff
  - Referrer-Policy configuration
  - Cross-Origin-Opener-Policy (COOP)
- **HTTPS Redirect** - Automatic HTTP to HTTPS redirection
  - Configurable exempt paths
  - Custom status codes (301/302)

### Performance Middleware
- **GZip Compression** - Response compression for bandwidth optimization
  - Configurable compression level (0-9)
  - Minimum size threshold
  - Content-type filtering
  - Automatic Accept-Encoding detection
- **Conditional GET** - HTTP caching with ETags and Last-Modified
  - Automatic ETag generation (SHA-256 based)
  - If-None-Match support
  - If-Modified-Since support
  - If-Match and If-Unmodified-Since validation
  - 304 Not Modified responses

### Authentication & Request Processing
- **Authentication** - JWT-based authentication middleware
  - Bearer token extraction
  - Token validation via `reinhardt-auth`
  - User type support
- **Logging** - Request/response logging
  - Timestamp, method, path, status code
  - Request duration tracking

### Dependency Injection Support
- **DI Middleware** - Integration with `reinhardt-di`
  - Middleware factory pattern
  - Injectable middleware components
  - Automatic dependency resolution

### Placeholder Middleware (Configuration Only)
The following middleware have configuration structures but no complete implementation:
- **Broken Link Detection** - Configuration for broken link email notifications
- **Common Middleware** - URL normalization (append_slash, prepend_www)
- **Locale Middleware** - Locale detection configuration
- **Message Framework** - Flash message storage (Session/Cookie)
- **Redirect Fallback** - Fallback URL configuration

## Planned Features

### Django-Inspired Middleware
- **Session Middleware** - Session management and persistence
- **Site Middleware** - Multi-site support
- **Flatpages Middleware** - Static page fallback
- **Cache Middleware** - Response caching layer

### Additional Security
- **Permissions** - Permission-based access control
- **Rate Limiting** - Request throttling and rate limits

### Advanced Features
- **Request ID** - Request tracing and correlation
- **Metrics** - Request/response metrics collection
- **Tracing** - Distributed tracing support

