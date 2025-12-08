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
}
