use std::fmt;
use thiserror::Error;

const VALID_BCRYPT_PREFIXES: [&str; 4] = ["$2a$", "$2b$", "$2x$", "$2y$"];
const BCRYPT_LENGTH: usize = 60;

/// Error types for hashed password validation failures.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum HashedPasswordError {
    #[error("Hash format is invalid for {algorithm}: {reason}")]
    FormatNotValid { algorithm: String, reason: String },

    #[error("Hash string is empty")]
    EmptyHash,

    #[error("Hash length is invalid for {algorithm}: expected {expected}, got {actual}")]
    LengthNotValid {
        algorithm: String,
        expected: String,
        actual: usize,
    },

    #[error("Hash contains invalid characters for {algorithm}")]
    CharactersNotValid { algorithm: String },
}

/// Represents a securely hashed password using a specific algorithm.
///
/// This value object encapsulates a password hash and validates its format
/// according to the specified hashing algorithm. It ensures that only properly
/// formatted hashes are stored.
///
/// # Examples
///
/// ```
/// use education_platform_common::{HashedPassword, HashingAlgorithm};
///
/// let bcrypt_hash = "$2b$12$R9h/cIPz0gi.URNNX3kh2OPST9/PgBkqquzi.Ss7KIUgO2t0jWMUW";
/// let hashed = HashedPassword::new(
///     bcrypt_hash.to_string(),
///     HashingAlgorithm::Bcrypt
/// ).unwrap();
///
/// assert_eq!(hashed.value(), bcrypt_hash);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HashedPassword {
    value: String,
    hashing_algorithm: HashingAlgorithm,
}

/// Supported password hashing algorithms.
///
/// Each variant represents a cryptographically secure password hashing algorithm
/// with specific format requirements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HashingAlgorithm {
    /// Bcrypt algorithm (all variants: $2a$, $2b$, $2x$, $2y$)
    Bcrypt,

    /// Argon2 algorithm family
    Argon(ArgonVariant),
}

impl fmt::Display for HashingAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bcrypt => write!(f, "bcrypt"),
            Self::Argon(variant) => write!(f, "{variant}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArgonVariant {
    /// Argon2d variant (data-dependent)
    Argon2d,

    /// Argon2i variant (data-independent)
    Argon2i,

    /// Argon2id variant (hybrid mode, recommended)
    Argon2id,
}

impl fmt::Display for ArgonVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgonVariant::Argon2d => write!(f, "argon2d"),
            ArgonVariant::Argon2i => write!(f, "argon2i"),
            ArgonVariant::Argon2id => write!(f, "argon2id"),
        }
    }
}

impl HashedPassword {
    /// Creates a new `HashedPassword` instance with format validation.
    ///
    /// Validates the hash string format according to the specified algorithm's
    /// requirements, including structure, length, and character set.
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Hash string is empty
    /// - A Hash format doesn't match algorithm specification
    /// - Hash length is invalid
    /// - Hash contains invalid characters
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::{HashedPassword, HashingAlgorithm};
    ///
    /// // Valid bcrypt hash
    /// let hash = "$2b$12$R9h/cIPz0gi.URNNX3kh2OPST9/PgBkqquzi.Ss7KIUgO2t0jWMUW";
    /// let hashed = HashedPassword::new(hash.to_string(), HashingAlgorithm::Bcrypt);
    /// assert!(hashed.is_ok());
    ///
    /// // Invalid format
    /// let invalid = HashedPassword::new("invalid".to_string(), HashingAlgorithm::Bcrypt);
    /// assert!(invalid.is_err());
    /// ```
    pub fn new(
        value: String,
        hashing_algorithm: HashingAlgorithm,
    ) -> Result<Self, HashedPasswordError> {
        Self::validate_format(&value, hashing_algorithm)?;
        Ok(Self {
            value,
            hashing_algorithm,
        })
    }

    /// Returns the hash string value.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::{HashedPassword, HashingAlgorithm};
    ///
    /// let hash = "$2b$12$R9h/cIPz0gi.URNNX3kh2OPST9/PgBkqquzi.Ss7KIUgO2t0jWMUW";
    /// let hashed = HashedPassword::new(hash.to_string(), HashingAlgorithm::Bcrypt).unwrap();
    /// assert_eq!(hashed.value(), hash);
    /// ```
    #[inline]
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the hashing algorithm used.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::{HashedPassword, HashingAlgorithm};
    ///
    /// let hash = "$2b$12$R9h/cIPz0gi.URNNX3kh2OPST9/PgBkqquzi.Ss7KIUgO2t0jWMUW";
    /// let hashed = HashedPassword::new(hash.to_string(), HashingAlgorithm::Bcrypt).unwrap();
    /// assert_eq!(hashed.algorithm(), HashingAlgorithm::Bcrypt);
    /// ```
    #[inline]
    #[must_use]
    pub const fn algorithm(&self) -> HashingAlgorithm {
        self.hashing_algorithm
    }

    /// Validates a hash format according to algorithm specification.
    ///
    /// # Errors
    ///
    /// Returns error if a hash format is invalid for the specified algorithm.
    fn validate_format(hash: &str, algorithm: HashingAlgorithm) -> Result<(), HashedPasswordError> {
        if hash.is_empty() {
            return Err(HashedPasswordError::EmptyHash);
        }

        match algorithm {
            HashingAlgorithm::Bcrypt => Self::validate_bcrypt(hash),
            HashingAlgorithm::Argon(variant) => Self::validate_argon2(hash, variant),
        }
    }

    /// Validates bcrypt hash format.
    ///
    /// Bcrypt format: `$2[a/b/x/y]$[cost]$[22-char salt][31-char hash]`
    /// Total length: 60 characters
    fn validate_bcrypt(hash: &str) -> Result<(), HashedPasswordError> {
        if hash.len() != BCRYPT_LENGTH {
            return Err(HashedPasswordError::LengthNotValid {
                algorithm: HashingAlgorithm::Bcrypt.to_string(),
                expected: format!("{BCRYPT_LENGTH}"),
                actual: hash.len(),
            });
        }

        if !VALID_BCRYPT_PREFIXES
            .iter()
            .any(|prefix| hash.starts_with(prefix))
        {
            return Err(HashedPasswordError::FormatNotValid {
                algorithm: HashingAlgorithm::Bcrypt.to_string(),
                reason: format!("must start with one of: {}", VALID_BCRYPT_PREFIXES.join(", ")),
            });
        }

        let parts: Vec<&str> = hash.split('$').collect();
        if parts.len() != 4 {
            return Err(HashedPasswordError::FormatNotValid {
                algorithm: HashingAlgorithm::Bcrypt.to_string(),
                reason: "must have format $2x$cost$salthash".to_string(),
            });
        }

        // Validate cost parameter (4-31)
        let cost = parts[2]
            .parse::<u32>()
            .map_err(|_| HashedPasswordError::FormatNotValid {
                algorithm: HashingAlgorithm::Bcrypt.to_string(),
                reason: "cost parameter must be a valid number".to_string(),
            })?;

        if !(4..=31).contains(&cost) {
            return Err(HashedPasswordError::FormatNotValid {
                algorithm: HashingAlgorithm::Bcrypt.to_string(),
                reason: format!("cost must be between 4 and 31, got {cost}"),
            });
        }

        // Validate salt+hash (22 + 31 = 53 characters)
        if parts[3].len() != 53 {
            return Err(HashedPasswordError::LengthNotValid {
                algorithm: HashingAlgorithm::Bcrypt.to_string(),
                expected: "53 (salt+hash)".to_string(),
                actual: parts[3].len(),
            });
        }

        // Validate base64 characters (bcrypt uses custom ./A-Za-z0-9 alphabet)
        if !parts[3]
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '/')
        {
            return Err(HashedPasswordError::CharactersNotValid {
                algorithm: HashingAlgorithm::Bcrypt.to_string(),
            });
        }

        Ok(())
    }

    /// Validates an Argon2 hash format.
    ///
    /// Argon2 format: `$argon2[d/i/id]$v=19$m=X,t=Y,p=Z$salt$hash`
    fn validate_argon2(hash: &str, variant: ArgonVariant) -> Result<(), HashedPasswordError> {
        let expected_prefix = format!("${variant}$");
        if !hash.starts_with(&expected_prefix) {
            return Err(HashedPasswordError::FormatNotValid {
                algorithm: variant.to_string(),
                reason: format!("must start with {expected_prefix}"),
            });
        }

        let parts: Vec<&str> = hash.split('$').collect();
        if parts.len() < 5 {
            return Err(HashedPasswordError::FormatNotValid {
                algorithm: variant.to_string(),
                reason: format!("must have at least 5 parts separated by '$', got {}", parts.len()),
            });
        }

        // Validate version parameter (v=19 is standard)
        if !parts[2].starts_with("v=") {
            return Err(HashedPasswordError::FormatNotValid {
                algorithm: variant.to_string(),
                reason: "must include version parameter (v=19)".to_string(),
            });
        }

        // Validate parameters format (m=X,t=Y,p=Z)
        if !parts[3].contains("m=") || !parts[3].contains("t=") || !parts[3].contains("p=") {
            return Err(HashedPasswordError::FormatNotValid {
                algorithm: variant.to_string(),
                reason: "must include memory (m), time (t), and parallelism (p) parameters"
                    .to_string(),
            });
        }

        // Validate salt exists
        if parts.get(4).is_none_or(|s| s.is_empty()) {
            return Err(HashedPasswordError::FormatNotValid {
                algorithm: variant.to_string(),
                reason: "salt is missing or empty".to_string(),
            });
        }

        // Validate hash exists
        if parts.get(5).is_none_or(|h| h.is_empty()) {
            return Err(HashedPasswordError::FormatNotValid {
                algorithm: variant.to_string(),
                reason: "hash is missing or empty".to_string(),
            });
        }

        // Validate base64 characters in salt and hash
        for part in [parts.get(4), parts.get(5)].iter() {
            if let Some(section) = part
                && !section
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=')
            {
                return Err(HashedPasswordError::CharactersNotValid {
                    algorithm: variant.to_string(),
                });
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod bcrypt {
        use super::*;

        #[test]
        fn test_valid_bcrypt_2a() {
            let hash = "$2a$12$psJT/efQo8reGDiil3M2GOBc5qCxCxJEnNRuX3gdYPvx4pXICZ8Kq";
            let result = HashedPassword::new(hash.to_string(), HashingAlgorithm::Bcrypt);
            assert!(result.is_ok());
        }

        #[test]
        fn test_valid_bcrypt_2b() {
            let hash = "$2b$10$N9qo8uLOickgx2ZMRZoMye.IjefuNEo5JkeOlV5YPrj5Qv2s5KeNK";
            let result = HashedPassword::new(hash.to_string(), HashingAlgorithm::Bcrypt);
            assert!(result.is_ok());
        }

        #[test]
        fn test_valid_bcrypt_2y() {
            let hash = "$2y$12$R9h/cIPz0gi.URNNX3kh2OPST9/PgBkqquzi.Ss7KIUgO2t0jWMUW";
            let result = HashedPassword::new(hash.to_string(), HashingAlgorithm::Bcrypt);
            assert!(result.is_ok());
        }

        #[test]
        fn test_invalid_bcrypt_prefix() {
            let hash = "$3a$12$R9h/cIPz0gi.URNNX3kh2OPST9/PgBkqquzi.Ss7KIUgO2t0jWMUW";
            let result = HashedPassword::new(hash.to_string(), HashingAlgorithm::Bcrypt);
            assert!(matches!(result, Err(HashedPasswordError::FormatNotValid { .. })));
        }

        #[test]
        fn test_invalid_bcrypt_length_too_short() {
            let hash = "$2a$12$short";
            let result = HashedPassword::new(hash.to_string(), HashingAlgorithm::Bcrypt);
            assert!(matches!(result, Err(HashedPasswordError::LengthNotValid { .. })));
        }

        #[test]
        fn test_invalid_bcrypt_length_too_long() {
            let hash = "$2a$12$R9h/cIPz0gi.URNNX3kh2OPST9/PgBkqquzi.Ss7KIUgO2t0jWMUWExtra";
            let result = HashedPassword::new(hash.to_string(), HashingAlgorithm::Bcrypt);
            assert!(matches!(result, Err(HashedPasswordError::LengthNotValid { .. })));
        }

        #[test]
        fn test_invalid_bcrypt_cost_too_low() {
            let hash = "$2a$03$R9h/cIPz0gi.URNNX3kh2OPST9/PgBkqquzi.Ss7KIUgO2t0jWMUW";
            let result = HashedPassword::new(hash.to_string(), HashingAlgorithm::Bcrypt);
            assert!(matches!(result, Err(HashedPasswordError::FormatNotValid { .. })));
        }

        #[test]
        fn test_invalid_bcrypt_cost_too_high() {
            let hash = "$2a$32$R9h/cIPz0gi.URNNX3kh2OPST9/PgBkqquzi.Ss7KIUgO2t0jWMUW";
            let result = HashedPassword::new(hash.to_string(), HashingAlgorithm::Bcrypt);
            assert!(matches!(result, Err(HashedPasswordError::FormatNotValid { .. })));
        }

        #[test]
        fn test_invalid_bcrypt_characters() {
            let hash = "$2a$12$R9h/cIPz0gi.URNNX3kh2OPST9/PgBkqquzi.Ss7KIUgO2t0jWM@#";
            let result = HashedPassword::new(hash.to_string(), HashingAlgorithm::Bcrypt);
            match result {
                Err(HashedPasswordError::CharactersNotValid { .. }) => (),
                other => panic!("Expected CharactersNotValid error, got: {other:?}"),
            }
        }

        #[test]
        fn test_bcrypt_getters() {
            let hash = "$2b$10$N9qo8uLOickgx2ZMRZoMye.IjefuNEo5JkeOlV5YPrj5Qv2s5KeNK";
            let hashed = HashedPassword::new(hash.to_string(), HashingAlgorithm::Bcrypt).unwrap();
            assert_eq!(hashed.value(), hash);
            assert_eq!(hashed.algorithm(), HashingAlgorithm::Bcrypt);
        }
    }

    mod argon2 {
        use super::*;

        #[test]
        fn test_valid_argon2id() {
            let hash =
                "$argon2id$v=19$m=65536,t=3,p=4$c29tZXNhbHQ$RdescudvJCsgt3ub+b+dWRWJTmaaJObG";
            let result = HashedPassword::new(
                hash.to_string(),
                HashingAlgorithm::Argon(ArgonVariant::Argon2id),
            );
            assert!(result.is_ok());
            assert!(result.is_ok());
        }

        #[test]
        fn test_valid_argon2i() {
            let hash = "$argon2i$v=19$m=16,t=10,p=1$MTIzNDU2Nzg$MCCNT76IgG8UJpMhLAdfJQ";
            let result = HashedPassword::new(
                hash.to_string(),
                HashingAlgorithm::Argon(ArgonVariant::Argon2i),
            );
            assert!(result.is_ok());
        }

        #[test]
        fn test_valid_argon2d() {
            let hash = "$argon2d$v=19$m=16,t=10,p=1$MTIzNDU2Nzg$glqugeSL8QVEMSxC4D0LBA";
            let result = HashedPassword::new(
                hash.to_string(),
                HashingAlgorithm::Argon(ArgonVariant::Argon2d),
            );
            assert!(result.is_ok());
        }

        #[test]
        fn test_invalid_argon2_prefix() {
            let hash = "$argon3$v=19$m=65536,t=3,p=4$c29tZXNhbHQ$RdescudvJCsgt3ub+b+dWRWJTmaaJObG";
            let result = HashedPassword::new(
                hash.to_string(),
                HashingAlgorithm::Argon(ArgonVariant::Argon2id),
            );
            assert!(matches!(result, Err(HashedPasswordError::FormatNotValid { .. })));
        }

        #[test]
        fn test_invalid_argon2_missing_version() {
            let hash = "$argon2id$m=65536,t=3,p=4$c29tZXNhbHQ$RdescudvJCsgt3ub+b+dWRWJTmaaJObG";
            let result = HashedPassword::new(
                hash.to_string(),
                HashingAlgorithm::Argon(ArgonVariant::Argon2id),
            );
            assert!(matches!(result, Err(HashedPasswordError::FormatNotValid { .. })));
        }

        #[test]
        fn test_invalid_argon2_missing_parameters() {
            let hash = "$argon2id$v=19$m=65536$c29tZXNhbHQ$RdescudvJCsgt3ub+b+dWRWJTmaaJObG";
            let result = HashedPassword::new(
                hash.to_string(),
                HashingAlgorithm::Argon(ArgonVariant::Argon2id),
            );
            assert!(matches!(result, Err(HashedPasswordError::FormatNotValid { .. })));
        }

        #[test]
        fn test_invalid_argon2_missing_salt() {
            let hash = "$argon2id$v=19$m=65536,t=3,p=4$$RdescudvJCsgt3ub+b+dWRWJTmaaJObG";
            let result = HashedPassword::new(
                hash.to_string(),
                HashingAlgorithm::Argon(ArgonVariant::Argon2id),
            );
            assert!(matches!(result, Err(HashedPasswordError::FormatNotValid { .. })));
        }

        #[test]
        fn test_invalid_argon2_missing_hash() {
            let hash = "$argon2id$v=19$m=65536,t=3,p=4$c29tZXNhbHQ$";
            let result = HashedPassword::new(
                hash.to_string(),
                HashingAlgorithm::Argon(ArgonVariant::Argon2id),
            );
            assert!(matches!(result, Err(HashedPasswordError::FormatNotValid { .. })));
        }

        #[test]
        fn test_argon2_getters() {
            let hash =
                "$argon2id$v=19$m=65536,t=3,p=4$c29tZXNhbHQ$RdescudvJCsgt3ub+b+dWRWJTmaaJObG";
            let hashed = HashedPassword::new(
                hash.to_string(),
                HashingAlgorithm::Argon(ArgonVariant::Argon2id),
            )
            .unwrap();
            assert_eq!(hashed.value(), hash);
            assert_eq!(hashed.algorithm(), HashingAlgorithm::Argon(ArgonVariant::Argon2id));
        }
    }

    mod general {
        use super::*;

        #[test]
        fn test_empty_hash() {
            let result = HashedPassword::new(String::new(), HashingAlgorithm::Bcrypt);
            assert!(matches!(result, Err(HashedPasswordError::EmptyHash)));
        }

        #[test]
        fn test_hashing_algorithm_display() {
            assert_eq!(HashingAlgorithm::Bcrypt.to_string(), "bcrypt");
            assert_eq!(HashingAlgorithm::Argon(ArgonVariant::Argon2d).to_string(), "argon2d");
            assert_eq!(HashingAlgorithm::Argon(ArgonVariant::Argon2i).to_string(), "argon2i");
            assert_eq!(
                HashingAlgorithm::Argon(ArgonVariant::Argon2id).to_string(),
                "argon2id"
            );
        }

        #[test]
        fn test_hashed_password_equality() {
            let hash = "$2a$12$LLaDD.UAV9pa/dPEhg.DH.xtT4cNycxsIG4Ws7yFNj7DlpweqIX5e";
            let hashed1 = HashedPassword::new(hash.to_string(), HashingAlgorithm::Bcrypt).unwrap();
            let hashed2 = HashedPassword::new(hash.to_string(), HashingAlgorithm::Bcrypt).unwrap();
            assert_eq!(hashed1, hashed2);
        }

        #[test]
        fn test_hashed_password_clone() {
            let hash = "$2a$12$Ia7McyoljzSFhM0K1MvNeulaBxPtAmNXs/1IUdVs7giTB4DQ5XNrW";
            let hashed = HashedPassword::new(hash.to_string(), HashingAlgorithm::Bcrypt).unwrap();
            let cloned = hashed.clone();
            assert_eq!(hashed, cloned);
        }
    }
}
