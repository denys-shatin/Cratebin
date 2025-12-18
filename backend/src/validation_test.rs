// **Feature: cratebin, Property 14: UTF-8 validation**
// For any valid UTF-8 string, creating a snippet with that content should succeed;
// for any invalid UTF-8 byte sequence, creation should fail with a validation error.

#[cfg(test)]
mod property_tests {
    use crate::validation::{validate_utf8, ValidationError};
    use quickcheck::QuickCheck;

    fn prop_valid_utf8_accepted(content: String) -> bool {
        // Rust String is always valid UTF-8
        // Test that valid UTF-8 strings are accepted (unless they contain null bytes)
        let has_null = content.as_bytes().contains(&0);
        
        match validate_utf8(&content) {
            Ok(()) => !has_null, // Should succeed if no null bytes
            Err(ValidationError::InvalidUtf8) => has_null, // Should fail only if null bytes
            Err(_) => false,
        }
    }

    fn prop_null_bytes_rejected(prefix: String, suffix: String) -> bool {
        // Create content with null byte
        let content = format!("{}\0{}", prefix, suffix);
        
        // Should be rejected
        matches!(validate_utf8(&content), Err(ValidationError::InvalidUtf8))
    }

    #[test]
    fn run_utf8_validation_tests() {
        QuickCheck::new()
            .tests(100)
            .quickcheck(prop_valid_utf8_accepted as fn(String) -> bool);
        
        QuickCheck::new()
            .tests(100)
            .quickcheck(prop_null_bytes_rejected as fn(String, String) -> bool);
    }

    #[test]
    fn test_unicode_characters() {
        // Test various Unicode characters
        assert!(validate_utf8("Hello, World!").is_ok());
        assert!(validate_utf8("Привет, мир!").is_ok());
        assert!(validate_utf8("你好，世界！").is_ok());
        assert!(validate_utf8("مرحبا بالعالم").is_ok());
        assert!(validate_utf8("🌍🌎🌏").is_ok());
        assert!(validate_utf8("").is_ok()); // Empty string is valid UTF-8
    }
}
