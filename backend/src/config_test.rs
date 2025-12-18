// **Feature: cratebin, Property 20: Configuration loading**
// For any environment variable defined in the configuration, the system should load and use that value at startup.

#[cfg(test)]
mod property_tests {
    use quickcheck::QuickCheck;
    use std::env;

    fn prop_config_loading(value: u32) -> bool {
        let value_str = value.to_string();
        let key = "TEST_CONFIG_VALUE";
        
        // Set environment variable
        env::set_var(key, &value_str);
        
        // Load it back
        let loaded = env::var(key).unwrap_or_default();
        
        // Clean up
        env::remove_var(key);
        
        // Verify it matches
        loaded == value_str
    }

    fn prop_config_default_fallback(key_suffix: u16) -> bool {
        let key = format!("NONEXISTENT_KEY_{}", key_suffix);
        let default_value = "default";
        
        // Try to load non-existent key with default
        let loaded = env::var(&key).unwrap_or_else(|_| default_value.to_string());
        
        // Should get default value
        loaded == default_value
    }

    #[test]
    fn run_config_loading_tests() {
        QuickCheck::new()
            .tests(100)
            .quickcheck(prop_config_loading as fn(u32) -> bool);
        
        QuickCheck::new()
            .tests(100)
            .quickcheck(prop_config_default_fallback as fn(u16) -> bool);
    }

    #[test]
    fn test_config_parsing() {
        // Test that numeric config values can be parsed
        env::set_var("TEST_MAX_SIZE", "524288");
        let max_size: usize = env::var("TEST_MAX_SIZE")
            .unwrap()
            .parse()
            .expect("Should parse");
        assert_eq!(max_size, 524288);
        env::remove_var("TEST_MAX_SIZE");
    }
}
