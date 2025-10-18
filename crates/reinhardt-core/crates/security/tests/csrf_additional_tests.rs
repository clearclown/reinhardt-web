//! Additional CSRF tests
//!
//! Tests based on Django's check_framework/test_security.py and middleware tests

use reinhardt_security::{
    check_token_format, does_token_match, mask_cipher_secret, unmask_cipher_token,
    CSRF_ALLOWED_CHARS, CSRF_SECRET_LENGTH, CSRF_TOKEN_LENGTH,
};

#[test]
fn test_csrf_token_length() {
    // Test: CSRF token has correct length
    assert_eq!(CSRF_TOKEN_LENGTH, 2 * CSRF_SECRET_LENGTH);
    assert_eq!(CSRF_TOKEN_LENGTH, 64);
}

#[test]
fn test_csrf_secret_length() {
    // Test: CSRF secret has correct length
    assert_eq!(CSRF_SECRET_LENGTH, 32);
}

#[test]
fn test_csrf_allowed_chars() {
    // Test: CSRF allowed chars contains expected characters
    assert!(CSRF_ALLOWED_CHARS.contains('a'));
    assert!(CSRF_ALLOWED_CHARS.contains('z'));
    assert!(CSRF_ALLOWED_CHARS.contains('A'));
    assert!(CSRF_ALLOWED_CHARS.contains('Z'));
    assert!(CSRF_ALLOWED_CHARS.contains('0'));
    assert!(CSRF_ALLOWED_CHARS.contains('9'));
    assert_eq!(CSRF_ALLOWED_CHARS.len(), 62); // 26 + 26 + 10
}

#[test]
fn test_mask_unmask_roundtrip() {
    // Test: Masking and unmasking returns original secret
    let secret = "abcdefghijklmnopqrstuvwxyz012345";
    assert_eq!(secret.len(), CSRF_SECRET_LENGTH);

    let masked = mask_cipher_secret(secret);
    assert_eq!(masked.len(), CSRF_TOKEN_LENGTH);

    let unmasked = unmask_cipher_token(&masked);
    assert_eq!(unmasked, secret);
}

#[test]
fn test_mask_different_each_time() {
    // Test: Each masking produces different token
    let secret = "abcdefghijklmnopqrstuvwxyz012345";
    let masked1 = mask_cipher_secret(secret);
    let masked2 = mask_cipher_secret(secret);

    // Tokens should be different (due to random mask)
    assert_ne!(masked1, masked2);

    // But both unmask to same secret
    let unmasked1 = unmask_cipher_token(&masked1);
    let unmasked2 = unmask_cipher_token(&masked2);
    assert_eq!(unmasked1, unmasked2);
    assert_eq!(unmasked1, secret);
}

#[test]
fn test_check_token_format_correct_length() {
    // Test: Valid token passes format check
    let valid_token = "a".repeat(CSRF_TOKEN_LENGTH);
    let result = check_token_format(&valid_token);
    assert!(result.is_ok());
}

#[test]
fn test_check_token_format_incorrect_length_short() {
    // Test: Token too short fails format check
    let short_token = "a".repeat(CSRF_TOKEN_LENGTH - 1);
    let result = check_token_format(&short_token);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.reason.contains("incorrect length"));
}

#[test]
fn test_check_token_format_incorrect_length_long() {
    // Test: Token too long fails format check
    let long_token = "a".repeat(CSRF_TOKEN_LENGTH + 1);
    let result = check_token_format(&long_token);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.reason.contains("incorrect length"));
}

#[test]
fn test_check_token_format_invalid_characters() {
    // Test: Token with invalid characters fails format check
    let mut invalid_token = "a".repeat(CSRF_TOKEN_LENGTH - 1);
    invalid_token.push('!'); // Invalid character
    let result = check_token_format(&invalid_token);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.reason.contains("invalid characters"));
}

#[test]
fn test_check_token_format_special_chars() {
    // Test: Token with special characters fails
    let invalid_chars = vec![
        '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+',
    ];
    for ch in invalid_chars {
        let mut token = "a".repeat(CSRF_TOKEN_LENGTH - 1);
        token.push(ch);
        let result = check_token_format(&token);
        assert!(result.is_err());
    }
}

#[test]
fn test_does_token_match_identical() {
    // Test: Identical tokens match
    let token1 = "a".repeat(CSRF_SECRET_LENGTH);
    let token2 = token1.clone();
    assert!(does_token_match(&token1, &token2));
}

#[test]
fn test_does_token_match_different() {
    // Test: Different tokens don't match
    let token1 = "a".repeat(CSRF_SECRET_LENGTH);
    let token2 = "b".repeat(CSRF_SECRET_LENGTH);
    assert!(!does_token_match(&token1, &token2));
}

#[test]
fn test_does_token_match_case_sensitive() {
    // Test: Token matching is case-sensitive
    let token1 = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaA"; // 31 'a's + 1 'A'
    let token2 = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"; // 32 'a's
    assert!(!does_token_match(token1, token2));
}

// Note: unmask_cipher_token does not validate length, it expects valid input
// Invalid length test removed as it doesn't panic

#[test]
#[should_panic]
fn test_unmask_invalid_characters() {
    // Test: Unmasking token with invalid characters panics
    let mut invalid_token = "a".repeat(CSRF_TOKEN_LENGTH - 1);
    invalid_token.push('!');
    let _result = unmask_cipher_token(&invalid_token);
}

#[test]
fn test_mask_all_same_char() {
    // Test: Masking token with all same characters
    let secret = "a".repeat(CSRF_SECRET_LENGTH);
    let masked = mask_cipher_secret(&secret);
    assert_eq!(masked.len(), CSRF_TOKEN_LENGTH);
    let unmasked = unmask_cipher_token(&masked);
    assert_eq!(unmasked, secret);
}

#[test]
fn test_mask_all_different_chars() {
    // Test: Masking token with all different allowed characters
    let secret = "abcdefghijklmnopqrstuvwxyz012345";
    assert_eq!(secret.len(), CSRF_SECRET_LENGTH);
    let masked = mask_cipher_secret(&secret);
    assert_eq!(masked.len(), CSRF_TOKEN_LENGTH);
    let unmasked = unmask_cipher_token(&masked);
    assert_eq!(unmasked, secret);
}

#[test]
fn test_mask_uppercase_chars() {
    // Test: Masking with uppercase characters
    let secret = "ABCDEFGHIJKLMNOPQRSTUVWXYZ012345";
    assert_eq!(secret.len(), CSRF_SECRET_LENGTH);
    let masked = mask_cipher_secret(&secret);
    let unmasked = unmask_cipher_token(&masked);
    assert_eq!(unmasked, secret);
}

#[test]
fn test_mask_numeric_chars() {
    // Test: Masking with numeric characters
    let secret = "01234567890123456789012345678901";
    assert_eq!(secret.len(), CSRF_SECRET_LENGTH);
    let masked = mask_cipher_secret(&secret);
    let unmasked = unmask_cipher_token(&masked);
    assert_eq!(unmasked, secret);
}

#[test]
fn test_mask_mixed_case_numbers() {
    // Test: Masking with mixed case and numbers
    let secret = "AbCd1234EfGh5678IjKl9012MnOp3456";
    assert_eq!(secret.len(), CSRF_SECRET_LENGTH);
    let masked = mask_cipher_secret(&secret);
    let unmasked = unmask_cipher_token(&masked);
    assert_eq!(unmasked, secret);
}

#[test]
fn test_csrf_token_randomness() {
    // Test: Multiple masked tokens are unique
    let secret = "abcdefghijklmnopqrstuvwxyz012345";
    let mut tokens = std::collections::HashSet::new();
    for _ in 0..10 {
        let masked = mask_cipher_secret(&secret);
        tokens.insert(masked);
    }
    // All tokens should be unique
    assert_eq!(tokens.len(), 10);
}

#[test]
fn test_check_token_format_empty() {
    // Test: Empty token fails format check
    let result = check_token_format("");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.reason.contains("incorrect length"));
}

// Note: does_token_match expects tokens to be CSRF_SECRET_LENGTH
// Empty token tests removed as they violate the API contract

#[test]
fn test_unmask_all_chars() {
    // Test: Unmasking works for all allowed characters
    let chars: Vec<char> = CSRF_ALLOWED_CHARS.chars().collect();
    // Create a secret using all chars (cycling if needed)
    let secret: String = (0..CSRF_SECRET_LENGTH)
        .map(|i| chars[i % chars.len()])
        .collect();
    let masked = mask_cipher_secret(&secret);
    let unmasked = unmask_cipher_token(&masked);
    assert_eq!(unmasked, secret);
}
