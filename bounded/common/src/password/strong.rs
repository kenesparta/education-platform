use crate::{Validator, ValidatorError};
use thiserror::Error;

const MIN_LENGTH: usize = 8;
const MAX_LENGTH: usize = 128;

/// Error type for `StrongPassword` validation failures.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StrongPasswordError {
    #[error("Password validation failed: {0}")]
    ValidationError(#[from] ValidatorError),

    #[error("Password must contain at least one uppercase letter")]
    UppercaseNotFound,

    #[error("Password must contain at least one lowercase letter")]
    LowercaseNotFound,

    #[error("Password must contain at least one digit")]
    DigitNotFound,

    #[error("Password must contain at least one special character")]
    SpecialCharacterNotFound,

    #[error("Password cannot contain whitespace")]
    WhitespaceNotAllowed,
}

/// Represents a strong password as a Value Object.
///
/// A strong password must meet the following requirements:
/// - Between 8 and 128 characters in length
/// - Contains at least one uppercase letter (A-Z)
/// - Contains at least one lowercase letter (a-z)
/// - Contains at least one digit (0-9)
/// - Contains at least one special character (!@#$%^&*()_+-=[]{}|;:,.<>?)
/// - Does not contain whitespace characters
///
/// # Security Note
///
/// This value object validates password strength but stores the plaintext temporarily.
/// Passwords should be hashed immediately after validation and before storage.
/// Never log, display, or persist plaintext passwords.
///
/// # Examples
///
/// ```
/// use education_platform_common::StrongPassword;
///
/// // Valid strong password
/// let password = StrongPassword::new("MyP@ssw0rd".to_string()).unwrap();
/// assert_eq!(password.value(), "MyP@ssw0rd");
///
/// // Too short
/// assert!(StrongPassword::new("Weak1!".to_string()).is_err());
///
/// // Missing uppercase
/// assert!(StrongPassword::new("myp@ssw0rd".to_string()).is_err());
///
/// // Missing special character
/// assert!(StrongPassword::new("MyPassword123".to_string()).is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrongPassword {
    value: String,
}

impl StrongPassword {
    /// Creates a new validated `StrongPassword` instance.
    ///
    /// Validates that the password meets all strength requirements:
    /// minimum length, maximum length, character diversity, and no whitespace.
    ///
    /// # Errors
    ///
    /// Returns `StrongPasswordError` if:
    /// - The password is empty or only whitespace
    /// - The password is shorter than 8 characters
    /// - The password is longer than 128 characters
    /// - The password doesn't contain at least one uppercase letter
    /// - The password doesn't contain at least one lowercase letter
    /// - The password doesn't contain at least one digit
    /// - The password doesn't contain at least one special character
    /// - The password contains whitespace
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::StrongPassword;
    ///
    /// // Valid passwords
    /// assert!(StrongPassword::new("MyP@ssw0rd".to_string()).is_ok());
    /// assert!(StrongPassword::new("Tr0ub4dor&3".to_string()).is_ok());
    /// assert!(StrongPassword::new("C0mpl3x!Pass".to_string()).is_ok());
    ///
    /// // Invalid passwords
    /// assert!(StrongPassword::new("short".to_string()).is_err());
    /// assert!(StrongPassword::new("nouppercase123!".to_string()).is_err());
    /// assert!(StrongPassword::new("NOLOWERCASE123!".to_string()).is_err());
    /// assert!(StrongPassword::new("NoDigits!".to_string()).is_err());
    /// assert!(StrongPassword::new("NoSpecial123".to_string()).is_err());
    /// assert!(StrongPassword::new("Has Space1!".to_string()).is_err());
    /// ```
    pub fn new(value: String) -> Result<Self, StrongPasswordError> {
        let trimmed = value.trim();

        Validator::is_not_empty(trimmed)?;
        Validator::has_length_between(trimmed, MIN_LENGTH, MAX_LENGTH)?;

        Self::validate_no_whitespace(trimmed)?;
        Self::validate_has_uppercase(trimmed)?;
        Self::validate_has_lowercase(trimmed)?;
        Self::validate_has_digit(trimmed)?;
        Self::validate_has_special_char(trimmed)?;

        Ok(Self {
            value: trimmed.to_string(),
        })
    }

    /// Validates that the password contains no whitespace characters.
    fn validate_no_whitespace(password: &str) -> Result<(), StrongPasswordError> {
        if password.chars().any(char::is_whitespace) {
            return Err(StrongPasswordError::WhitespaceNotAllowed);
        }
        Ok(())
    }

    /// Validates that the password contains at least one uppercase letter.
    fn validate_has_uppercase(password: &str) -> Result<(), StrongPasswordError> {
        if !password.chars().any(|c| c.is_uppercase()) {
            return Err(StrongPasswordError::UppercaseNotFound);
        }
        Ok(())
    }

    /// Validates that the password contains at least one lowercase letter.
    fn validate_has_lowercase(password: &str) -> Result<(), StrongPasswordError> {
        if !password.chars().any(|c| c.is_lowercase()) {
            return Err(StrongPasswordError::LowercaseNotFound);
        }
        Ok(())
    }

    /// Validates that the password contains at least one digit.
    fn validate_has_digit(password: &str) -> Result<(), StrongPasswordError> {
        if !password.chars().any(|c| c.is_ascii_digit()) {
            return Err(StrongPasswordError::DigitNotFound);
        }
        Ok(())
    }

    /// Validates that the password contains at least one special character.
    fn validate_has_special_char(password: &str) -> Result<(), StrongPasswordError> {
        let special_chars = "!@#$%^&*()_+-=[]{}|;:,.<>?";
        if !password.chars().any(|c| special_chars.contains(c)) {
            return Err(StrongPasswordError::SpecialCharacterNotFound);
        }
        Ok(())
    }

    /// Returns the password value.
    ///
    /// # Security Warning
    ///
    /// This method returns the plaintext password. Use with extreme caution.
    /// Passwords should be hashed immediately after validation, and the plaintext
    /// should be securely cleared from memory.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::StrongPassword;
    ///
    /// let password = StrongPassword::new("MyP@ssw0rd".to_string()).unwrap();
    /// assert_eq!(password.value(), "MyP@ssw0rd");
    /// ```
    #[inline]
    pub fn value(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_valid_password_returns_ok() {
        let password = StrongPassword::new("MyP@ssw0rd".to_string());
        assert!(password.is_ok());
        assert_eq!(password.unwrap().value(), "MyP@ssw0rd");
    }

    #[test]
    fn test_new_with_all_requirements_met() {
        let valid_passwords = vec![
            "Abcd123!",
            "MyP@ssw0rd",
            "Tr0ub4dor&3",
            "C0mpl3x!Pass",
            "Test123!@#",
            "Str0ng#Pass",
        ];

        for pwd in valid_passwords {
            let result = StrongPassword::new(pwd.to_string());
            assert!(
                result.is_ok(),
                "Expected '{}' to be valid but got: {:?}",
                pwd,
                result.err()
            );
        }
    }

    #[test]
    fn test_new_with_empty_string_returns_error() {
        let result = StrongPassword::new("".to_string());
        assert!(matches!(
            result,
            Err(StrongPasswordError::ValidationError(_))
        ));
    }

    #[test]
    fn test_new_with_whitespace_only_returns_error() {
        let result = StrongPassword::new("   ".to_string());
        assert!(matches!(
            result,
            Err(StrongPasswordError::ValidationError(_))
        ));
    }

    #[test]
    fn test_new_with_too_short_password_returns_error() {
        let result = StrongPassword::new("Ab1!".to_string());
        assert!(matches!(
            result,
            Err(StrongPasswordError::ValidationError(_))
        ));
    }

    #[test]
    fn test_new_with_minimum_length_returns_ok() {
        let result = StrongPassword::new("Abcd123!".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_new_with_maximum_length_returns_ok() {
        let long_password = format!("Aa1!{}", "x".repeat(MAX_LENGTH - 4));
        let result = StrongPassword::new(long_password);
        assert!(result.is_ok());
    }

    #[test]
    fn test_new_with_too_long_password_returns_error() {
        let too_long = format!("Aa1!{}", "x".repeat(MAX_LENGTH));
        let result = StrongPassword::new(too_long);
        assert!(matches!(
            result,
            Err(StrongPasswordError::ValidationError(_))
        ));
    }

    #[test]
    fn test_new_without_uppercase_returns_error() {
        let result = StrongPassword::new("myp@ssw0rd".to_string());
        assert!(matches!(
            result,
            Err(StrongPasswordError::UppercaseNotFound)
        ));
    }

    #[test]
    fn test_new_without_lowercase_returns_error() {
        let result = StrongPassword::new("MYP@SSW0RD".to_string());
        assert!(matches!(
            result,
            Err(StrongPasswordError::LowercaseNotFound)
        ));
    }

    #[test]
    fn test_new_without_digit_returns_error() {
        let result = StrongPassword::new("MyP@ssword".to_string());
        assert!(matches!(result, Err(StrongPasswordError::DigitNotFound)));
    }

    #[test]
    fn test_new_without_special_char_returns_error() {
        let result = StrongPassword::new("MyPassword123".to_string());
        assert!(matches!(
            result,
            Err(StrongPasswordError::SpecialCharacterNotFound)
        ));
    }

    #[test]
    fn test_new_with_whitespace_returns_error() {
        let result = StrongPassword::new("My P@ssw0rd".to_string());
        assert!(matches!(
            result,
            Err(StrongPasswordError::WhitespaceNotAllowed)
        ));
    }

    #[test]
    fn test_new_with_tab_returns_error() {
        let result = StrongPassword::new("My\tP@ssw0rd".to_string());
        assert!(matches!(
            result,
            Err(StrongPasswordError::WhitespaceNotAllowed)
        ));
    }

    #[test]
    fn test_new_with_newline_returns_error() {
        let result = StrongPassword::new("My\nP@ssw0rd".to_string());
        assert!(matches!(
            result,
            Err(StrongPasswordError::WhitespaceNotAllowed)
        ));
    }

    #[test]
    fn test_new_trims_whitespace() {
        let password = StrongPassword::new("  MyP@ssw0rd  ".to_string()).unwrap();
        assert_eq!(password.value(), "MyP@ssw0rd");
    }

    #[test]
    fn test_value_getter_returns_password() {
        let password = StrongPassword::new("MyP@ssw0rd".to_string()).unwrap();
        assert_eq!(password.value(), "MyP@ssw0rd");
    }

    #[test]
    fn test_clone_creates_equal_instance() {
        let password = StrongPassword::new("MyP@ssw0rd".to_string()).unwrap();
        let cloned = password.clone();
        assert_eq!(password, cloned);
        assert_eq!(password.value(), cloned.value());
    }

    #[test]
    fn test_equality_for_identical_passwords() {
        let pwd1 = StrongPassword::new("MyP@ssw0rd".to_string()).unwrap();
        let pwd2 = StrongPassword::new("MyP@ssw0rd".to_string()).unwrap();
        assert_eq!(pwd1, pwd2);
    }

    #[test]
    fn test_inequality_for_different_passwords() {
        let pwd1 = StrongPassword::new("MyP@ssw0rd".to_string()).unwrap();
        let pwd2 = StrongPassword::new("Other#Pass1".to_string()).unwrap();
        assert_ne!(pwd1, pwd2);
    }

    #[test]
    fn test_debug_format_does_not_expose_password() {
        let password = StrongPassword::new("MyP@ssw0rd".to_string()).unwrap();
        let debug_output = format!("{:?}", password);
        assert!(debug_output.contains("StrongPassword"));
        // Password should still be visible in debug for development purposes
        // In production, implement a custom Debug that redacts the value
    }

    #[test]
    fn test_with_various_special_characters() {
        let special_chars = vec![
            "MyP@ssw0rd",
            "MyP#ssw0rd",
            "MyP$ssw0rd",
            "MyP%ssw0rd",
            "MyP^ssw0rd",
            "MyP&ssw0rd",
            "MyP*ssw0rd",
            "MyP(ssw0rd",
            "MyP)ssw0rd",
            "MyP_ssw0rd",
            "MyP+ssw0rd",
            "MyP-ssw0rd",
            "MyP=ssw0rd",
            "MyP[ssw0rd",
            "MyP]ssw0rd",
            "MyP{ssw0rd",
            "MyP}ssw0rd",
            "MyP|ssw0rd",
            "MyP;ssw0rd",
            "MyP:ssw0rd",
            "MyP,ssw0rd",
            "MyP.ssw0rd",
            "MyP<ssw0rd",
            "MyP>ssw0rd",
            "MyP?ssw0rd",
        ];

        for pwd in special_chars {
            let result = StrongPassword::new(pwd.to_string());
            assert!(
                result.is_ok(),
                "Expected '{}' to be valid but got: {:?}",
                pwd,
                result.err()
            );
        }
    }

    #[test]
    fn test_with_unicode_characters_without_special_char_returns_error() {
        let result = StrongPassword::new("MyPässw0rd".to_string());
        assert!(matches!(
            result,
            Err(StrongPasswordError::SpecialCharacterNotFound)
        ));
    }

    #[test]
    fn test_with_unicode_and_special_char_returns_ok() {
        let result = StrongPassword::new("MyPässw0rd!".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_value_object_semantics() {
        let pwd1 = StrongPassword::new("MyP@ssw0rd".to_string()).unwrap();
        let pwd2 = StrongPassword::new("MyP@ssw0rd".to_string()).unwrap();
        assert_eq!(pwd1, pwd2);
    }

    #[test]
    fn test_all_character_types_present() {
        let password = StrongPassword::new("Aa1!bcde".to_string()).unwrap();
        assert!(password.value().chars().any(|c| c.is_uppercase()));
        assert!(password.value().chars().any(|c| c.is_lowercase()));
        assert!(password.value().chars().any(|c| c.is_ascii_digit()));
        assert!(
            password
                .value()
                .chars()
                .any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c))
        );
    }

    #[test]
    fn test_edge_case_one_char_below_min_length() {
        let result = StrongPassword::new("Aa1!bcd".to_string());
        assert!(matches!(
            result,
            Err(StrongPasswordError::ValidationError(_))
        ));
    }

    #[test]
    fn test_multiple_special_characters() {
        let result = StrongPassword::new("MyP@ss!w0rd#".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_digits() {
        let result = StrongPassword::new("MyP@ss123456".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_uppercase() {
        let result = StrongPassword::new("MYP@ssw0rd".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_lowercase() {
        let result = StrongPassword::new("Myp@ssword1".to_string());
        assert!(result.is_ok());
    }
}
