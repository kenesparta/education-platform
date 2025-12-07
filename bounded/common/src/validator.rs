use regex::Regex;
use std::sync::LazyLock;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ValidatorError {
    #[error("Invalid regex pattern: {0}")]
    RegexError(String),

    #[error("email format not valid")]
    NotValidEmail,

    #[error("name cannot be empty")]
    EmptyName,

    #[error("value must be greater than {min}")]
    GreaterThan { min: usize },

    #[error("value must be less than {max}")]
    LessThan { max: usize },

    #[error("name contains invalid characters")]
    InvalidNameCharacters,
}

static EMAIL_REGEX: LazyLock<Result<Regex, regex::Error>> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}$"));

static LATIN_NAME_REGEX: LazyLock<Result<Regex, regex::Error>> =
    LazyLock::new(|| Regex::new(r"^[\p{L}\s'\-]+$"));

pub struct Validator;

impl Validator {
    pub fn is_valid_email(email: &str) -> Result<(), ValidatorError> {
        let regex = EMAIL_REGEX
            .as_ref()
            .map_err(|e| ValidatorError::RegexError(e.to_string()))?;

        if !regex.is_match(email) {
            return Err(ValidatorError::NotValidEmail);
        }

        Ok(())
    }

    pub fn is_not_empty(v: &str) -> Result<(), ValidatorError> {
        if v.trim().is_empty() {
            return Err(ValidatorError::EmptyName);
        }

        Ok(())
    }

    pub fn is_greater_than(v: &str, min: usize) -> Result<(), ValidatorError> {
        if v.len() < min {
            return Err(ValidatorError::GreaterThan { min });
        }

        Ok(())
    }

    pub fn is_less_than(v: &str, max: usize) -> Result<(), ValidatorError> {
        if v.len() > max {
            return Err(ValidatorError::LessThan { max });
        }

        Ok(())
    }

    /// Validates that a name contains only Latin characters.
    ///
    /// Accepts:
    /// - English letters (a-z, A-Z)
    /// - Spanish letters with diacritics (á, é, í, ó, ú, ñ, ü, etc.)
    /// - Portuguese letters with diacritics (á, à, â, ã, ç, etc.)
    /// - Spaces (for multi-word names)
    /// - Hyphens (for compound names like "María-José")
    /// - Apostrophes (for names like "O'Brien")
    ///
    /// Rejects:
    /// - Numbers (0-9)
    /// - Special characters (!@#$%^&*()+=[]{}|;:",.<>?/)
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Validator;
    ///
    /// assert!(Validator::is_valid_latin_name("José").is_ok());
    /// assert!(Validator::is_valid_latin_name("María García").is_ok());
    /// assert!(Validator::is_valid_latin_name("O'Brien").is_ok());
    /// assert!(Validator::is_valid_latin_name("Jean-Pierre").is_ok());
    /// assert!(Validator::is_valid_latin_name("João").is_ok());
    /// assert!(Validator::is_valid_latin_name("Nuñez").is_ok());
    ///
    /// assert!(Validator::is_valid_latin_name("John123").is_err());
    /// assert!(Validator::is_valid_latin_name("José@email").is_err());
    /// assert!(Validator::is_valid_latin_name("Test$Name").is_err());
    /// ```
    pub fn is_valid_latin_name(name: &str) -> Result<(), ValidatorError> {
        let regex = LATIN_NAME_REGEX
            .as_ref()
            .map_err(|e| ValidatorError::RegexError(e.to_string()))?;

        if !regex.is_match(name) {
            return Err(ValidatorError::InvalidNameCharacters);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_emails() {
        assert!(Validator::is_valid_email("user@example.com").is_ok());
        assert!(Validator::is_valid_email("test.user@domain.co.uk").is_ok());
        assert!(Validator::is_valid_email("user+tag@example.com").is_ok());
        assert!(Validator::is_valid_email("user_name@example.org").is_ok());
        assert!(Validator::is_valid_email("user.name@example.org").is_ok());
    }

    #[test]
    fn test_invalid_emails() {
        assert!(matches!(
            Validator::is_valid_email("invalid"),
            Err(ValidatorError::NotValidEmail)
        ));
        assert!(matches!(
            Validator::is_valid_email("@example.com"),
            Err(ValidatorError::NotValidEmail)
        ));
        assert!(matches!(
            Validator::is_valid_email("user@"),
            Err(ValidatorError::NotValidEmail)
        ));
        assert!(matches!(
            Validator::is_valid_email("user@domain"),
            Err(ValidatorError::NotValidEmail)
        ));
        assert!(matches!(
            Validator::is_valid_email("user @example.com"),
            Err(ValidatorError::NotValidEmail)
        ));
        assert!(matches!(
            Validator::is_valid_email("user<@example.com"),
            Err(ValidatorError::NotValidEmail)
        ));
        assert!(matches!(
            Validator::is_valid_email(""),
            Err(ValidatorError::NotValidEmail)
        ));
    }

    #[test]
    fn test_valid_latin_names_english() {
        assert!(Validator::is_valid_latin_name("John").is_ok());
        assert!(Validator::is_valid_latin_name("Mary Jane").is_ok());
        assert!(Validator::is_valid_latin_name("O'Brien").is_ok());
        assert!(Validator::is_valid_latin_name("Mary-Jane").is_ok());
        assert!(Validator::is_valid_latin_name("Anne Marie").is_ok());
    }

    #[test]
    fn test_valid_latin_names_spanish() {
        assert!(Validator::is_valid_latin_name("José").is_ok());
        assert!(Validator::is_valid_latin_name("María").is_ok());
        assert!(Validator::is_valid_latin_name("Nuñez").is_ok());
        assert!(Validator::is_valid_latin_name("García").is_ok());
        assert!(Validator::is_valid_latin_name("Rodríguez").is_ok());
        assert!(Validator::is_valid_latin_name("María José").is_ok());
        assert!(Validator::is_valid_latin_name("María-José").is_ok());
        assert!(Validator::is_valid_latin_name("Ángel").is_ok());
        assert!(Validator::is_valid_latin_name("Mónica").is_ok());
    }

    #[test]
    fn test_valid_latin_names_portuguese() {
        assert!(Validator::is_valid_latin_name("João").is_ok());
        assert!(Validator::is_valid_latin_name("José").is_ok());
        assert!(Validator::is_valid_latin_name("António").is_ok());
        assert!(Validator::is_valid_latin_name("Conceição").is_ok());
        assert!(Validator::is_valid_latin_name("São Paulo").is_ok());
    }

    #[test]
    fn test_invalid_latin_names_with_numbers() {
        assert!(matches!(
            Validator::is_valid_latin_name("John123"),
            Err(ValidatorError::InvalidNameCharacters)
        ));
        assert!(matches!(
            Validator::is_valid_latin_name("María2"),
            Err(ValidatorError::InvalidNameCharacters)
        ));
        assert!(matches!(
            Validator::is_valid_latin_name("Test123Name"),
            Err(ValidatorError::InvalidNameCharacters)
        ));
    }

    #[test]
    fn test_invalid_latin_names_with_special_characters() {
        assert!(matches!(
            Validator::is_valid_latin_name("John@Doe"),
            Err(ValidatorError::InvalidNameCharacters)
        ));
        assert!(matches!(
            Validator::is_valid_latin_name("Test$Name"),
            Err(ValidatorError::InvalidNameCharacters)
        ));
        assert!(matches!(
            Validator::is_valid_latin_name("Name!"),
            Err(ValidatorError::InvalidNameCharacters)
        ));
        assert!(matches!(
            Validator::is_valid_latin_name("José#García"),
            Err(ValidatorError::InvalidNameCharacters)
        ));
        assert!(matches!(
            Validator::is_valid_latin_name("Test.Name"),
            Err(ValidatorError::InvalidNameCharacters)
        ));
        assert!(matches!(
            Validator::is_valid_latin_name("Name_Test"),
            Err(ValidatorError::InvalidNameCharacters)
        ));
    }

    #[test]
    fn test_invalid_latin_names_empty() {
        assert!(matches!(
            Validator::is_valid_latin_name(""),
            Err(ValidatorError::InvalidNameCharacters)
        ));
    }
}
