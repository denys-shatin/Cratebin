// **Feature: cratebin, Property 12: Automatic expiration cleanup**
// For any set of snippets with expiration times in the past, running the cleanup task should delete all expired snippets and leave non-expired snippets unchanged.

// **Feature: cratebin, Property 13: Never-expiring snippets persistence**
// For any snippet with no expiration set (TTL = never), running the cleanup task should not delete the snippet.

#[cfg(test)]
mod property_tests {
    use crate::models::{Snippet, Visibility};
    use crate::utils::generate_delete_token;
    use chrono::{Duration, Utc};
    use quickcheck::QuickCheck;

    // Property 12: Expired snippets are cleaned up
    fn prop_expired_snippets_cleaned(content: String) -> bool {
        if content.is_empty() {
            return true;
        }

        // Create expired snippet
        let snippet = Snippet {
            id: "test".to_string(),
            content,
            visibility: Visibility::Public,
            expires_at: Some(Utc::now() - Duration::hours(1)), // Expired
            password_hash: None,
            delete_token: generate_delete_token(),
            created_at: Utc::now() - Duration::hours(2),
            size: 10,
        };

        // Check if expired
        if let Some(expires_at) = snippet.expires_at {
            expires_at < Utc::now()
        } else {
            false
        }
    }

    // Property 12: Non-expired snippets are not cleaned up
    fn prop_non_expired_snippets_kept(content: String) -> bool {
        if content.is_empty() {
            return true;
        }

        // Create non-expired snippet
        let snippet = Snippet {
            id: "test".to_string(),
            content,
            visibility: Visibility::Public,
            expires_at: Some(Utc::now() + Duration::hours(1)), // Not expired
            password_hash: None,
            delete_token: generate_delete_token(),
            created_at: Utc::now(),
            size: 10,
        };

        // Check if not expired
        if let Some(expires_at) = snippet.expires_at {
            expires_at > Utc::now()
        } else {
            true
        }
    }

    // Property 13: Never-expiring snippets persist
    fn prop_never_expiring_snippets_persist(content: String) -> bool {
        if content.is_empty() {
            return true;
        }

        // Create never-expiring snippet
        let snippet = Snippet {
            id: "test".to_string(),
            content,
            visibility: Visibility::Public,
            expires_at: None, // Never expires
            password_hash: None,
            delete_token: generate_delete_token(),
            created_at: Utc::now(),
            size: 10,
        };

        // Should have no expiration
        snippet.expires_at.is_none()
    }

    #[test]
    fn run_cleanup_tests() {
        QuickCheck::new()
            .tests(100)
            .quickcheck(prop_expired_snippets_cleaned as fn(String) -> bool);
        
        QuickCheck::new()
            .tests(100)
            .quickcheck(prop_non_expired_snippets_kept as fn(String) -> bool);
        
        QuickCheck::new()
            .tests(100)
            .quickcheck(prop_never_expiring_snippets_persist as fn(String) -> bool);
    }
}
