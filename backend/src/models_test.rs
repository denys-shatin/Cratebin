// **Feature: cratebin, Property 2: Visibility preservation**
// For any snippet created with a specific visibility level, retrieving that snippet should return the same visibility level.

#[cfg(test)]
mod property_tests {
    use crate::models::{Snippet, Visibility};
    use quickcheck::{Arbitrary, Gen, QuickCheck};

    impl Arbitrary for Visibility {
        fn arbitrary(g: &mut Gen) -> Self {
            let choices = [Visibility::Public, Visibility::Unlisted, Visibility::Private];
            *g.choose(&choices).unwrap()
        }
    }

    fn prop_visibility_preservation(visibility: Visibility) -> bool {
        // Create a snippet with a specific visibility
        let snippet = Snippet {
            id: "test".to_string(),
            content: "content".to_string(),
            visibility,
            expires_at: None,
            password_hash: None,
            delete_token: "token".to_string(),
            created_at: chrono::Utc::now(),
            size: 7,
        };

        // Serialize and deserialize to simulate storage/retrieval
        let json = serde_json::to_string(&snippet).unwrap();
        let retrieved: Snippet = serde_json::from_str(&json).unwrap();

        // Verify visibility is preserved
        retrieved.visibility == visibility
    }

    fn prop_visibility_roundtrip_string(visibility: Visibility) -> bool {
        // Test that visibility can be converted to string and back
        let as_string = visibility.to_string();
        let parsed: Visibility = serde_json::from_str(&format!("\"{}\"", as_string)).unwrap();
        parsed == visibility
    }

    #[test]
    fn run_visibility_preservation_tests() {
        QuickCheck::new()
            .tests(100)
            .quickcheck(prop_visibility_preservation as fn(Visibility) -> bool);
        
        QuickCheck::new()
            .tests(100)
            .quickcheck(prop_visibility_roundtrip_string as fn(Visibility) -> bool);
    }
}
