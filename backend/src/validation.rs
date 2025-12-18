use crate::ttl::parse_ttl;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    ContentTooLarge { size: usize, max: usize },
    InvalidUtf8,
    InvalidTtl(String),
    EmptyContent,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::ContentTooLarge { size, max } => {
                write!(f, "Content size {} exceeds maximum {}", size, max)
            }
            ValidationError::InvalidUtf8 => write!(f, "Content is not valid UTF-8"),
            ValidationError::InvalidTtl(ttl) => write!(f, "Invalid TTL format: {}", ttl),
            ValidationError::EmptyContent => write!(f, "Content cannot be empty"),
        }
    }
}

impl std::error::Error for ValidationError {}

/// Validate content size
pub fn validate_content_size(content: &str, max_size: usize) -> Result<(), ValidationError> {
    let size = content.len();
    if size > max_size {
        return Err(ValidationError::ContentTooLarge { size, max: max_size });
    }
    Ok(())
}

/// Validate UTF-8 encoding
/// Note: Rust strings are already UTF-8, but we check for null bytes and other issues
pub fn validate_utf8(content: &str) -> Result<(), ValidationError> {
    // Check for null bytes which might indicate binary data
    if content.as_bytes().contains(&0) {
        return Err(ValidationError::InvalidUtf8);
    }
    
    // Rust String is guaranteed to be valid UTF-8
    // Additional validation could be added here if needed
    Ok(())
}

/// Validate TTL format
pub fn validate_ttl(ttl: &str) -> Result<(), ValidationError> {
    parse_ttl(ttl).map_err(|_| ValidationError::InvalidTtl(ttl.to_string()))?;
    Ok(())
}

/// Validate that content is not empty
pub fn validate_not_empty(content: &str) -> Result<(), ValidationError> {
    if content.trim().is_empty() {
        return Err(ValidationError::EmptyContent);
    }
    Ok(())
}

/// Validate all snippet creation inputs
pub fn validate_snippet_creation(
    content: &str,
    ttl: Option<&str>,
    max_size: usize,
) -> Result<(), ValidationError> {
    validate_content_size(content, max_size)?;
    validate_utf8(content)?;
    
    if let Some(ttl_str) = ttl {
        validate_ttl(ttl_str)?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_content_size_ok() {
        let content = "Hello, World!";
        assert!(validate_content_size(content, 1024).is_ok());
    }

    #[test]
    fn test_validate_content_size_too_large() {
        let content = "x".repeat(1000);
        let result = validate_content_size(&content, 100);
        assert!(matches!(result, Err(ValidationError::ContentTooLarge { .. })));
    }

    #[test]
    fn test_validate_utf8_ok() {
        let content = "Hello, 世界! 🌍";
        assert!(validate_utf8(content).is_ok());
    }

    #[test]
    fn test_validate_utf8_null_byte() {
        let content = "Hello\0World";
        assert!(matches!(validate_utf8(content), Err(ValidationError::InvalidUtf8)));
    }

    #[test]
    fn test_validate_ttl_valid() {
        assert!(validate_ttl("1h").is_ok());
        assert!(validate_ttl("24h").is_ok());
        assert!(validate_ttl("7d").is_ok());
        assert!(validate_ttl("never").is_ok());
    }

    #[test]
    fn test_validate_ttl_invalid() {
        assert!(matches!(validate_ttl("invalid"), Err(ValidationError::InvalidTtl(_))));
        assert!(matches!(validate_ttl("2h"), Err(ValidationError::InvalidTtl(_))));
    }

    #[test]
    fn test_validate_not_empty() {
        assert!(validate_not_empty("content").is_ok());
        assert!(matches!(validate_not_empty(""), Err(ValidationError::EmptyContent)));
        assert!(matches!(validate_not_empty("   "), Err(ValidationError::EmptyContent)));
    }

    #[test]
    fn test_validate_snippet_creation() {
        // Valid input
        assert!(validate_snippet_creation("Hello", Some("1h"), 1024).is_ok());
        assert!(validate_snippet_creation("Hello", None, 1024).is_ok());
        
        // Too large
        let large_content = "x".repeat(1000);
        assert!(validate_snippet_creation(&large_content, None, 100).is_err());
        
        // Invalid TTL
        assert!(validate_snippet_creation("Hello", Some("invalid"), 1024).is_err());
    }
}
