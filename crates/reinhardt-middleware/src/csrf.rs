// CSRF (Cross-Site Request Forgery) protection middleware for Reinhardt
//
// This module provides middleware that uses reinhardt-security's CSRF implementation.

// Re-export CSRF functionality from reinhardt-security
pub use reinhardt_security::{
    // Constants
    check_origin,
    check_referer,
    check_token,
    check_token_format,
    does_token_match,
    get_secret,
    get_token,
    is_same_domain,
    mask_cipher_secret,
    rotate_token,
    unmask_cipher_token,
    CsrfMeta,
    InvalidTokenFormat,
    RejectRequest,
    CSRF_ALLOWED_CHARS,
    CSRF_SECRET_LENGTH,
    CSRF_SESSION_KEY,
    CSRF_TOKEN_LENGTH,
    REASON_BAD_ORIGIN,
    REASON_BAD_REFERER,
    REASON_CSRF_TOKEN_MISSING,
    REASON_INCORRECT_LENGTH,
    REASON_INSECURE_REFERER,
    REASON_INVALID_CHARACTERS,
    REASON_MALFORMED_REFERER,
    REASON_NO_CSRF_COOKIE,
    REASON_NO_REFERER,
};
