use chrono::{DateTime, Duration, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TtlError {
    InvalidFormat(String),
}

impl std::fmt::Display for TtlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TtlError::InvalidFormat(s) => write!(f, "Invalid TTL format: {}", s),
        }
    }
}

impl std::error::Error for TtlError {}

/// Parse TTL string into Duration
/// Supported formats: "1h", "24h", "7d", "never"
pub fn parse_ttl(ttl: &str) -> Result<Option<Duration>, TtlError> {
    match ttl {
        "never" => Ok(None),
        "1h" => Ok(Some(Duration::hours(1))),
        "24h" => Ok(Some(Duration::hours(24))),
        "7d" => Ok(Some(Duration::days(7))),
        _ => Err(TtlError::InvalidFormat(ttl.to_string())),
    }
}

/// Calculate expiration timestamp from creation time and TTL
pub fn calculate_expiration(
    created_at: DateTime<Utc>,
    ttl: Option<Duration>,
) -> Option<DateTime<Utc>> {
    ttl.map(|duration| created_at + duration)
}

/// Calculate expiration from TTL string
pub fn calculate_expiration_from_str(
    created_at: DateTime<Utc>,
    ttl_str: Option<&str>,
) -> Result<Option<DateTime<Utc>>, TtlError> {
    match ttl_str {
        None => Ok(None),
        Some(ttl) => {
            let duration = parse_ttl(ttl)?;
            Ok(calculate_expiration(created_at, duration))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ttl_never() {
        assert_eq!(parse_ttl("never").unwrap(), None);
    }

    #[test]
    fn test_parse_ttl_1h() {
        assert_eq!(parse_ttl("1h").unwrap(), Some(Duration::hours(1)));
    }

    #[test]
    fn test_parse_ttl_24h() {
        assert_eq!(parse_ttl("24h").unwrap(), Some(Duration::hours(24)));
    }

    #[test]
    fn test_parse_ttl_7d() {
        assert_eq!(parse_ttl("7d").unwrap(), Some(Duration::days(7)));
    }

    #[test]
    fn test_parse_ttl_invalid() {
        assert!(parse_ttl("invalid").is_err());
        assert!(parse_ttl("2h").is_err());
        assert!(parse_ttl("1d").is_err());
        assert!(parse_ttl("").is_err());
    }

    #[test]
    fn test_calculate_expiration_never() {
        let created_at = Utc::now();
        let expiration = calculate_expiration(created_at, None);
        assert_eq!(expiration, None);
    }

    #[test]
    fn test_calculate_expiration_1h() {
        let created_at = Utc::now();
        let expiration = calculate_expiration(created_at, Some(Duration::hours(1))).unwrap();
        
        let expected = created_at + Duration::hours(1);
        // Allow small time difference due to test execution time
        let diff = (expiration - expected).num_seconds().abs();
        assert!(diff < 1);
    }

    #[test]
    fn test_calculate_expiration_from_str() {
        let created_at = Utc::now();
        
        // Test "never"
        let result = calculate_expiration_from_str(created_at, Some("never")).unwrap();
        assert_eq!(result, None);
        
        // Test "1h"
        let result = calculate_expiration_from_str(created_at, Some("1h")).unwrap().unwrap();
        let expected = created_at + Duration::hours(1);
        let diff = (result - expected).num_seconds().abs();
        assert!(diff < 1);
        
        // Test None
        let result = calculate_expiration_from_str(created_at, None).unwrap();
        assert_eq!(result, None);
        
        // Test invalid
        assert!(calculate_expiration_from_str(created_at, Some("invalid")).is_err());
    }
}
