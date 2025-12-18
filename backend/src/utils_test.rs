// **Feature: cratebin, Property 1: Unique non-sequential identifiers**
// For any set of created snippets, all snippet IDs should be unique and non-sequential (not incrementing by 1).

// **Feature: cratebin, Property 4: Password hashing**
// For any snippet created with a password, the stored password_hash should be a valid Argon2 hash and should not equal the original password.

#[cfg(test)]
mod property_tests {
    use crate::utils::{generate_id, hash_password, verify_password};
    use quickcheck::QuickCheck;
    use std::collections::HashSet;

    fn prop_ids_are_unique(count: u8) -> bool {
        let count = count.max(2) as usize; // At least 2 IDs
        let mut ids = HashSet::new();
        
        for _ in 0..count {
            let id = generate_id();
            ids.insert(id);
        }
        
        // All IDs should be unique
        ids.len() == count
    }

    fn prop_ids_are_non_sequential(count: u8) -> bool {
        let count = count.max(3) as usize; // At least 3 IDs to check sequence
        let ids: Vec<String> = (0..count).map(|_| generate_id()).collect();
        
        // Check that IDs are not sequential (not incrementing by 1)
        // For UUIDs, we check that they're not simple increments
        for i in 0..ids.len().saturating_sub(1) {
            // If IDs were sequential, parsing as numbers would show increment pattern
            // UUIDs should not be parseable as simple integers
            if let (Ok(n1), Ok(n2)) = (ids[i].parse::<u64>(), ids[i + 1].parse::<u64>()) {
                if n2 == n1 + 1 {
                    return false; // Found sequential pattern
                }
            }
        }
        
        true
    }

    fn prop_password_hash_not_plaintext(password: String) -> bool {
        if password.is_empty() {
            return true; // Skip empty passwords
        }
        
        match hash_password(&password) {
            Ok(hash) => {
                // Hash should not equal plaintext
                if hash == password {
                    return false;
                }
                
                // Hash should start with Argon2 identifier
                if !hash.starts_with("$argon2") {
                    return false;
                }
                
                // Hash should be verifiable
                verify_password(&password, &hash).unwrap_or(false)
            }
            Err(_) => false,
        }
    }

    fn prop_password_hash_different_salts(password: String) -> bool {
        if password.is_empty() {
            return true; // Skip empty passwords
        }
        
        match (hash_password(&password), hash_password(&password)) {
            (Ok(hash1), Ok(hash2)) => {
                // Same password should produce different hashes due to different salts
                hash1 != hash2
            }
            _ => false,
        }
    }

    #[test]
    fn run_unique_id_tests() {
        QuickCheck::new()
            .tests(100)
            .quickcheck(prop_ids_are_unique as fn(u8) -> bool);
    }

    #[test]
    fn run_non_sequential_id_tests() {
        QuickCheck::new()
            .tests(100)
            .quickcheck(prop_ids_are_non_sequential as fn(u8) -> bool);
    }

    #[test]
    fn run_password_hash_tests() {
        QuickCheck::new()
            .tests(100)
            .quickcheck(prop_password_hash_not_plaintext as fn(String) -> bool);
        
        QuickCheck::new()
            .tests(100)
            .quickcheck(prop_password_hash_different_salts as fn(String) -> bool);
    }
}
