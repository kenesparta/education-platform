use crate::{Validator, ValidatorError};
use std::fmt;
use thiserror::Error;

const EMAIL_PARTS: usize = 2;

/// Error type for `Email` validation failures.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum EmailError {
    #[error("Email validation failed: {0}")]
    ValidationError(#[from] ValidatorError),

    #[error("Email format is not valid")]
    FormatNotValid,

    #[error("Missing domain in email address")]
    MissingDomain,

    #[error("Internal inconsistency: email structure is corrupted")]
    InconsistentState,
}

/// Represents a validated email address as a Value Object.
///
/// Email addresses are immutable and self-validating. The validation ensures:
/// - The email is not empty after trimming
/// - The email contains exactly one '@' symbol
/// - Both local part and domain are non-empty
/// - The domain contains at least one dot
/// - The email uses valid characters
///
/// # Examples
///
/// ```
/// use education_platform_common::Email;
///
/// let email = Email::new("user@example.com".to_string()).unwrap();
/// assert_eq!(email.address(), "user@example.com");
/// assert_eq!(email.domain().unwrap(), "example.com");
/// assert_eq!(email.local_part().unwrap(), "user");
///
/// // Invalid emails return errors
/// assert!(Email::new("invalid".to_string()).is_err());
/// assert!(Email::new("".to_string()).is_err());
/// assert!(Email::new("@example.com".to_string()).is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email {
    address: String,
}

impl Email {
    /// Creates a new validated `Email` instance.
    ///
    /// The email address is trimmed and validated to ensure it meets basic
    /// email format requirements.
    ///
    /// # Errors
    ///
    /// Returns `EmailError` if:
    /// - The email is empty or only whitespace
    /// - The email doesn't contain exactly one '@' symbol
    /// - The local part or domain is empty
    /// - The domain doesn't contain at least one dot
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Email;
    ///
    /// let email = Email::new("user@example.com".to_string()).unwrap();
    /// assert_eq!(email.address(), "user@example.com");
    ///
    /// let with_subdomain = Email::new("user@mail.example.com".to_string()).unwrap();
    /// assert_eq!(with_subdomain.domain().unwrap(), "mail.example.com");
    ///
    /// // Invalid examples
    /// assert!(Email::new("".to_string()).is_err());
    /// assert!(Email::new("invalid".to_string()).is_err());
    /// assert!(Email::new("@example.com".to_string()).is_err());
    /// assert!(Email::new("user@".to_string()).is_err());
    /// assert!(Email::new("user@domain".to_string()).is_err());
    /// ```
    pub fn new(address: String) -> Result<Self, EmailError> {
        let trimmed = address.trim();

        Validator::is_not_empty(trimmed)?;

        let parts: Vec<&str> = trimmed.split('@').collect();

        if parts.len() != EMAIL_PARTS {
            return Err(EmailError::FormatNotValid);
        }

        let local_part = parts[0];
        let domain = parts[1];

        Validator::is_not_empty(local_part)?;
        Validator::is_not_empty(domain)?;

        if !domain.contains('.') {
            return Err(EmailError::MissingDomain);
        }

        Self::validate_characters(local_part, domain)?;

        Ok(Self {
            address: trimmed.to_string(),
        })
    }

    /// Validates that the email address contains only valid characters.
    fn validate_characters(local_part: &str, domain: &str) -> Result<(), EmailError> {
        if !local_part.chars().all(|c| {
            c.is_alphanumeric() || c == '.' || c == '_' || c == '%' || c == '+' || c == '-'
        }) {
            return Err(EmailError::FormatNotValid);
        }

        if !domain
            .chars()
            .all(|c| c.is_alphanumeric() || c == '.' || c == '-')
        {
            return Err(EmailError::FormatNotValid);
        }

        Ok(())
    }

    /// Returns the full email address.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Email;
    ///
    /// let email = Email::new("user@example.com".to_string()).unwrap();
    /// assert_eq!(email.address(), "user@example.com");
    /// ```
    #[inline]
    #[must_use]
    pub fn address(&self) -> &str {
        &self.address
    }

    /// Returns the domain part of the email address.
    ///
    /// # Errors
    ///
    /// Returns `EmailError::InconsistentState` if the email structure is corrupted
    /// (should never happen if constructed via `new()`).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Email;
    ///
    /// let email = Email::new("user@example.com".to_string()).unwrap();
    /// assert_eq!(email.domain().unwrap(), "example.com");
    ///
    /// let subdomain = Email::new("user@mail.example.org".to_string()).unwrap();
    /// assert_eq!(subdomain.domain().unwrap(), "mail.example.org");
    /// ```
    #[inline]
    pub fn domain(&self) -> Result<&str, EmailError> {
        self.address
            .split('@')
            .nth(1)
            .ok_or(EmailError::InconsistentState)
    }

    /// Returns the local part (username) of the email address.
    ///
    /// # Errors
    ///
    /// Returns `EmailError::InconsistentState` if the email structure is corrupted
    /// (should never happen if constructed via `new()`).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Email;
    ///
    /// let email = Email::new("user@example.com".to_string()).unwrap();
    /// assert_eq!(email.local_part().unwrap(), "user");
    ///
    /// let with_plus = Email::new("user+tag@example.com".to_string()).unwrap();
    /// assert_eq!(with_plus.local_part().unwrap(), "user+tag");
    /// ```
    #[inline]
    pub fn local_part(&self) -> Result<&str, EmailError> {
        self.address
            .split('@')
            .next()
            .ok_or(EmailError::InconsistentState)
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_valid_email_returns_ok() {
        let email = Email::new("user@example.com".to_string());
        assert!(email.is_ok());
        assert_eq!(email.unwrap().address(), "user@example.com");
    }

    #[test]
    fn test_new_with_subdomain_returns_ok() {
        let email = Email::new("user@mail.example.com".to_string());
        assert!(email.is_ok());
        assert_eq!(email.unwrap().domain().unwrap(), "mail.example.com");
    }

    #[test]
    fn test_new_with_plus_addressing_returns_ok() {
        let email = Email::new("user+tag@example.com".to_string());
        assert!(email.is_ok());
        assert_eq!(email.unwrap().local_part().unwrap(), "user+tag");
    }

    #[test]
    fn test_new_with_dots_in_local_part_returns_ok() {
        let email = Email::new("first.last@example.com".to_string());
        assert!(email.is_ok());
        assert_eq!(email.unwrap().local_part().unwrap(), "first.last");
    }

    #[test]
    fn test_new_with_underscore_returns_ok() {
        let email = Email::new("user_name@example.com".to_string());
        assert!(email.is_ok());
    }

    #[test]
    fn test_new_with_hyphen_returns_ok() {
        let email = Email::new("user-name@example.com".to_string());
        assert!(email.is_ok());
    }

    #[test]
    fn test_new_with_numbers_returns_ok() {
        let email = Email::new("user123@example.com".to_string());
        assert!(email.is_ok());
    }

    #[test]
    fn test_new_with_multiple_tlds_returns_ok() {
        let email = Email::new("user@example.co.uk".to_string());
        assert!(email.is_ok());
        assert_eq!(email.unwrap().domain().unwrap(), "example.co.uk");
    }

    #[test]
    fn test_new_trims_whitespace() {
        let email = Email::new("  user@example.com  ".to_string()).unwrap();
        assert_eq!(email.address(), "user@example.com");
    }

    #[test]
    fn test_new_with_empty_string_returns_error() {
        let result = Email::new("".to_string());
        assert!(result.is_err());
        assert!(matches!(result, Err(EmailError::ValidationError(_))));
    }

    #[test]
    fn test_new_with_whitespace_only_returns_error() {
        let result = Email::new("   ".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_new_without_at_symbol_returns_error() {
        let result = Email::new("userexample.com".to_string());
        assert!(matches!(result, Err(EmailError::FormatNotValid)));
    }

    #[test]
    fn test_new_with_multiple_at_symbols_returns_error() {
        let result = Email::new("user@@example.com".to_string());
        assert!(matches!(result, Err(EmailError::FormatNotValid)));
    }

    #[test]
    fn test_new_with_empty_local_part_returns_error() {
        let result = Email::new("@example.com".to_string());
        assert!(matches!(result, Err(EmailError::ValidationError(_))));
    }

    #[test]
    fn test_new_with_empty_domain_returns_error() {
        let result = Email::new("user@".to_string());
        assert!(matches!(result, Err(EmailError::ValidationError(_))));
    }

    #[test]
    fn test_new_without_domain_dot_returns_error() {
        let result = Email::new("user@domain".to_string());
        assert!(matches!(result, Err(EmailError::MissingDomain)));
    }

    #[test]
    fn test_new_with_invalid_local_characters_returns_error() {
        let result = Email::new("user!name@example.com".to_string());
        assert!(matches!(result, Err(EmailError::FormatNotValid)));
    }

    #[test]
    fn test_new_with_invalid_domain_characters_returns_error() {
        let result = Email::new("user@exam!ple.com".to_string());
        assert!(matches!(result, Err(EmailError::FormatNotValid)));
    }

    #[test]
    fn test_new_with_spaces_in_email_returns_error() {
        let result = Email::new("user name@example.com".to_string());
        assert!(matches!(result, Err(EmailError::FormatNotValid)));
    }

    #[test]
    fn test_address_getter_returns_full_email() {
        let email = Email::new("user@example.com".to_string()).unwrap();
        assert_eq!(email.address(), "user@example.com");
    }

    #[test]
    fn test_domain_getter_returns_correct_domain() {
        let email = Email::new("user@example.com".to_string()).unwrap();
        assert_eq!(email.domain().unwrap(), "example.com");
    }

    #[test]
    fn test_domain_getter_with_subdomain() {
        let email = Email::new("user@mail.example.com".to_string()).unwrap();
        assert_eq!(email.domain().unwrap(), "mail.example.com");
    }

    #[test]
    fn test_local_part_getter_returns_username() {
        let email = Email::new("user@example.com".to_string()).unwrap();
        assert_eq!(email.local_part().unwrap(), "user");
    }

    #[test]
    fn test_local_part_getter_with_plus() {
        let email = Email::new("user+tag@example.com".to_string()).unwrap();
        assert_eq!(email.local_part().unwrap(), "user+tag");
    }

    #[test]
    fn test_display_format() {
        let email = Email::new("user@example.com".to_string()).unwrap();
        assert_eq!(format!("{}", email), "user@example.com");
    }

    #[test]
    fn test_debug_format() {
        let email = Email::new("user@example.com".to_string()).unwrap();
        let debug_output = format!("{:?}", email);
        assert!(debug_output.contains("Email"));
        assert!(debug_output.contains("user@example.com"));
    }

    #[test]
    fn test_clone() {
        let email = Email::new("user@example.com".to_string()).unwrap();
        let cloned = email.clone();
        assert_eq!(email, cloned);
        assert_eq!(email.address(), cloned.address());
    }

    #[test]
    fn test_equality_for_identical_emails() {
        let email1 = Email::new("user@example.com".to_string()).unwrap();
        let email2 = Email::new("user@example.com".to_string()).unwrap();
        assert_eq!(email1, email2);
    }

    #[test]
    fn test_inequality_for_different_emails() {
        let email1 = Email::new("user1@example.com".to_string()).unwrap();
        let email2 = Email::new("user2@example.com".to_string()).unwrap();
        assert_ne!(email1, email2);
    }

    #[test]
    fn test_inequality_for_different_domains() {
        let email1 = Email::new("user@example.com".to_string()).unwrap();
        let email2 = Email::new("user@other.com".to_string()).unwrap();
        assert_ne!(email1, email2);
    }

    #[test]
    fn test_hash_consistency() {
        use std::collections::HashSet;

        let email1 = Email::new("user@example.com".to_string()).unwrap();
        let email2 = Email::new("user@example.com".to_string()).unwrap();

        let mut set = HashSet::new();
        set.insert(email1.clone());
        assert!(set.contains(&email2));
    }

    #[test]
    fn test_value_object_semantics() {
        let email1 = Email::new("user@example.com".to_string()).unwrap();
        let email2 = Email::new("user@example.com".to_string()).unwrap();

        assert_eq!(email1, email2);
    }

    #[test]
    fn test_case_sensitive_local_part() {
        let email1 = Email::new("User@example.com".to_string()).unwrap();
        let email2 = Email::new("user@example.com".to_string()).unwrap();

        assert_ne!(email1.local_part(), email2.local_part());
    }

    #[test]
    fn test_case_sensitive_domain() {
        let email1 = Email::new("user@Example.com".to_string()).unwrap();
        let email2 = Email::new("user@example.com".to_string()).unwrap();

        assert_ne!(email1.domain(), email2.domain());
    }

    #[test]
    fn test_with_percent_in_local_part() {
        let email = Email::new("user%test@example.com".to_string());
        assert!(email.is_ok());
    }

    #[test]
    fn test_real_world_email_addresses() {
        let valid_emails = vec![
            "john.doe@example.com",
            "jane_smith@company.co.uk",
            "user+tag@mail.example.org",
            "admin123@test-domain.com",
            "first.last@subdomain.example.com",
        ];

        for email_str in valid_emails {
            let email = Email::new(email_str.to_string());
            assert!(
                email.is_ok(),
                "Expected '{}' to be valid but got: {:?}",
                email_str,
                email.err()
            );
        }
    }

    #[test]
    fn test_invalid_real_world_patterns() {
        let invalid_emails = vec![
            "plaintext",
            "@example.com",
            "user@",
            "user@@example.com",
            "user@domain",
            "user name@example.com",
            "user@exam ple.com",
            "user!invalid@example.com",
            "",
            "   ",
        ];

        for email_str in invalid_emails {
            let email = Email::new(email_str.to_string());
            assert!(
                email.is_err(),
                "Expected '{}' to be invalid but it was accepted",
                email_str
            );
        }
    }
}
