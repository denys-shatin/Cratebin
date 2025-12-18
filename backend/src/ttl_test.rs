// **Feature: cratebin, Property 3: TTL calculation correctness**
// For any snippet created with a TTL value (1h, 24h, 7d), the expires_at timestamp should equal created_at plus the specified duration.

#[cfg(test)]
mod property_tests {
    use crate::ttl::{calculate_expiration, parse_ttl};
    use chrono::{DateTime, Duration, Utc};
    use quickcheck::{Arbitrary, Gen, QuickCheck};

    #[derive(Debug, Clone)]
    struct TtlValue(String);

    impl Arbitrary for TtlValue {
        fn arbitrary(g: &mut Gen) -> Self {
            let choices = ["1h", "24h", "7d"];
            TtlValue(g.choose(&choices).unwrap().to_string())
        }
    }

    #[derive(Debug, Clone)]
    struct ArbitraryDateTime(DateTime<Utc>);

    impl Arbitrary for ArbitraryDateTime {
        fn arbitrary(g: &mut Gen) -> Self {
            // Generate a timestamp within a reasonable range
            let timestamp = i64::arbitrary(g) % (365 * 24 * 3600 * 10); // Within 10 years
            let base = Utc::now();
            ArbitraryDateTime(base + Duration::seconds(timestamp))
        }
    }

    fn prop_ttl_calculation_correctness(created_at: ArbitraryDateTime, ttl: TtlValue) -> bool {
        let created_at = created_at.0;
        let ttl_str = ttl.0;
        
        // Parse TTL to get duration
        let duration = match parse_ttl(&ttl_str) {
            Ok(Some(d)) => d,
            _ => return false,
        };
        
        // Calculate expiration
        let expires_at = match calculate_expiration(created_at, Some(duration)) {
            Some(exp) => exp,
            None => return false,
        };
        
        // Verify: expires_at should equal created_at + duration
        let expected = created_at + duration;
        
        // Allow small difference due to computation time (< 1 second)
        let diff = (expires_at - expected).num_seconds().abs();
        diff < 1
    }

    fn prop_ttl_never_no_expiration(created_at: ArbitraryDateTime) -> bool {
        let created_at = created_at.0;
        
        // Parse "never" TTL
        let duration = match parse_ttl("never") {
            Ok(None) => None,
            _ => return false,
        };
        
        // Calculate expiration
        let expires_at = calculate_expiration(created_at, duration);
        
        // Should be None for "never"
        expires_at.is_none()
    }

    #[test]
    fn run_ttl_calculation_tests() {
        QuickCheck::new()
            .tests(100)
            .quickcheck(prop_ttl_calculation_correctness as fn(ArbitraryDateTime, TtlValue) -> bool);
        
        QuickCheck::new()
            .tests(100)
            .quickcheck(prop_ttl_never_no_expiration as fn(ArbitraryDateTime) -> bool);
    }
}
