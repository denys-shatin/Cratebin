use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use uuid::Uuid;

/// Generate a unique non-sequential identifier using UUID v4
pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

/// Generate a random delete token
pub fn generate_delete_token() -> String {
    Uuid::new_v4().to_string()
}

/// Hash a password using Argon2id
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

/// Verify a password against an Argon2 hash using constant-time comparison
pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    let argon2 = Argon2::default();
    
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(argon2::password_hash::Error::Password) => Ok(false),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_id_format() {
        let id = generate_id();
        // UUID v4 format: 8-4-4-4-12 hex digits
        assert_eq!(id.len(), 36);
        assert!(id.contains('-'));
        // Verify it's a valid UUID
        assert!(Uuid::parse_str(&id).is_ok());
    }

    #[test]
    fn test_generate_id_uniqueness() {
        let id1 = generate_id();
        let id2 = generate_id();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_generate_delete_token_format() {
        let token = generate_delete_token();
        assert_eq!(token.len(), 36);
        assert!(Uuid::parse_str(&token).is_ok());
    }

    #[test]
    fn test_hash_password_not_plaintext() {
        let password = "my_secret_password";
        let hash = hash_password(password).unwrap();
        
        // Hash should not equal plaintext
        assert_ne!(hash, password);
        // Hash should start with Argon2 identifier
        assert!(hash.starts_with("$argon2"));
    }

    #[test]
    fn test_hash_password_different_salts() {
        let password = "same_password";
        let hash1 = hash_password(password).unwrap();
        let hash2 = hash_password(password).unwrap();
        
        // Same password should produce different hashes due to different salts
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_verify_password_correct() {
        let password = "correct_password";
        let hash = hash_password(password).unwrap();
        
        assert!(verify_password(password, &hash).unwrap());
    }

    #[test]
    fn test_verify_password_incorrect() {
        let password = "correct_password";
        let wrong_password = "wrong_password";
        let hash = hash_password(password).unwrap();
        
        assert!(!verify_password(wrong_password, &hash).unwrap());
    }

    #[test]
    fn test_verify_password_invalid_hash() {
        let result = verify_password("password", "invalid_hash");
        assert!(result.is_err());
    }
}
