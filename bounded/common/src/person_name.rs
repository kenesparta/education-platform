use crate::{Validator, ValidatorError};
use thiserror::Error;

/// Error type for `PersonName` validation failures.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PersonNameError {
    /// Validation error occurred during name validation.
    #[error("Name validation failed: {0}")]
    ValidationError(#[from] ValidatorError),
}

/// Represents a person's name with first, optional middle, and last name components.
///
/// All name components are validated to be non-empty and are automatically trimmed.
/// Name length must be between 1 and 100 characters (inclusive) after trimming.
///
/// # Examples
///
/// ```
/// use education_platform_common::PersonName;
///
/// let name = PersonName::new(
///     "John".to_string(),
///     Some("Michael".to_string()),
///     "Doe".to_string()
/// ).unwrap();
///
/// assert_eq!(name.full_name(), "John Michael Doe");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersonName {
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
}

impl PersonName {
    /// Creates a new `PersonName` instance.
    ///
    /// # Arguments
    ///
    /// * `first_name` - The person's first name (will be trimmed)
    /// * `middle_name` - Optional middle name (will be trimmed if provided)
    /// * `last_name` - The person's last name (will be trimmed)
    ///
    /// # Errors
    ///
    /// Returns `PersonNameError::ValidationError` if:
    /// - Any name component is empty after trimming
    /// - Any name component is less than 1 character after trimming
    /// - Any name component exceeds 100 characters after trimming
    ///
    /// # Example
    ///
    /// ```
    /// use education_platform_common::PersonName;
    ///
    /// let name = PersonName::new(
    ///     "John".to_string(),
    ///     Some("Michael".to_string()),
    ///     "Doe".to_string()
    /// ).unwrap();
    /// assert_eq!(name.first_name(), "John");
    /// ```
    pub fn new(
        first_name: String,
        middle_name: Option<String>,
        last_name: String,
    ) -> Result<Self, PersonNameError> {
        Validator::is_not_empty(first_name.trim())?;
        Validator::is_greater_than(first_name.trim(), 1)?;
        Validator::is_less_than(first_name.trim(), 100)?;

        Validator::is_not_empty(last_name.trim())?;
        Validator::is_greater_than(last_name.trim(), 1)?;
        Validator::is_less_than(last_name.trim(), 100)?;

        let trimmed_middle = match &middle_name {
            Some(name) => {
                let trimmed = name.trim();
                Validator::is_not_empty(trimmed)?;
                Validator::is_greater_than(trimmed, 1)?;
                Validator::is_less_than(trimmed, 100)?;
                Some(trimmed.to_string())
            }
            None => None,
        };

        Ok(Self {
            first_name: first_name.trim().to_string(),
            middle_name: trimmed_middle,
            last_name: last_name.trim().to_string(),
        })
    }

    /// Returns the first name.
    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    /// Returns the middle name if present.
    pub fn middle_name(&self) -> Option<&str> {
        self.middle_name.as_deref()
    }

    /// Returns the last name.
    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    /// Returns the full name as a single string.
    ///
    /// If a middle name is present, it will be included in the format "First Middle Last".
    /// Otherwise, the format will be "First Last".
    pub fn full_name(&self) -> String {
        match &self.middle_name {
            Some(middle) => format!("{} {} {}", self.first_name, middle, self.last_name),
            None => format!("{} {}", self.first_name, self.last_name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_all_names_returns_ok() {
        let result = PersonName::new(
            "John".to_string(),
            Some("Michael".to_string()),
            "Doe".to_string(),
        );

        assert!(result.is_ok());
        let name = result.unwrap();
        assert_eq!(name.first_name(), "John");
        assert_eq!(name.middle_name(), Some("Michael"));
        assert_eq!(name.last_name(), "Doe");
    }

    #[test]
    fn test_new_without_middle_name_returns_ok() {
        let result = PersonName::new("Jane".to_string(), None, "Smith".to_string());

        assert!(result.is_ok());
        let name = result.unwrap();
        assert_eq!(name.first_name(), "Jane");
        assert_eq!(name.middle_name(), None);
        assert_eq!(name.last_name(), "Smith");
    }

    #[test]
    fn test_new_trims_whitespace_from_names() {
        let result = PersonName::new(
            "  John  ".to_string(),
            Some("  Michael  ".to_string()),
            "  Doe  ".to_string(),
        );

        assert!(result.is_ok());
        let name = result.unwrap();
        assert_eq!(name.first_name(), "John");
        assert_eq!(name.middle_name(), Some("Michael"));
        assert_eq!(name.last_name(), "Doe");
    }

    #[test]
    fn test_new_with_empty_first_name_returns_error() {
        let result = PersonName::new(
            "".to_string(),
            Some("Michael".to_string()),
            "Doe".to_string(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_whitespace_only_first_name_returns_error() {
        let result = PersonName::new(
            "   ".to_string(),
            Some("Michael".to_string()),
            "Doe".to_string(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_empty_last_name_returns_error() {
        let result = PersonName::new(
            "John".to_string(),
            Some("Michael".to_string()),
            "".to_string(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_whitespace_only_last_name_returns_error() {
        let result = PersonName::new(
            "John".to_string(),
            Some("Michael".to_string()),
            "   ".to_string(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_empty_middle_name_returns_error() {
        let result = PersonName::new("John".to_string(), Some("".to_string()), "Doe".to_string());

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_whitespace_only_middle_name_returns_error() {
        let result = PersonName::new(
            "John".to_string(),
            Some("   ".to_string()),
            "Doe".to_string(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_full_name_with_middle_name() {
        let name = PersonName::new(
            "John".to_string(),
            Some("Michael".to_string()),
            "Doe".to_string(),
        )
        .unwrap();

        assert_eq!(name.full_name(), "John Michael Doe");
    }

    #[test]
    fn test_full_name_without_middle_name() {
        let name = PersonName::new("Jane".to_string(), None, "Smith".to_string()).unwrap();

        assert_eq!(name.full_name(), "Jane Smith");
    }

    #[test]
    fn test_clone_creates_equal_instance() {
        let original = PersonName::new(
            "John".to_string(),
            Some("Michael".to_string()),
            "Doe".to_string(),
        )
        .unwrap();

        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(original.first_name(), cloned.first_name());
        assert_eq!(original.middle_name(), cloned.middle_name());
        assert_eq!(original.last_name(), cloned.last_name());
    }

    #[test]
    fn test_equality_for_identical_names() {
        let name1 = PersonName::new(
            "John".to_string(),
            Some("Michael".to_string()),
            "Doe".to_string(),
        )
        .unwrap();

        let name2 = PersonName::new(
            "John".to_string(),
            Some("Michael".to_string()),
            "Doe".to_string(),
        )
        .unwrap();

        assert_eq!(name1, name2);
    }

    #[test]
    fn test_inequality_for_different_first_names() {
        let name1 = PersonName::new("John".to_string(), None, "Doe".to_string()).unwrap();

        let name2 = PersonName::new("Jane".to_string(), None, "Doe".to_string()).unwrap();

        assert_ne!(name1, name2);
    }

    #[test]
    fn test_new_with_all_names_empty_returns_error() {
        let result = PersonName::new("".to_string(), Some("".to_string()), "".to_string());

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_all_names_whitespace_only_returns_error() {
        let result = PersonName::new(
            "   ".to_string(),
            Some("   ".to_string()),
            "   ".to_string(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_mixed_whitespace_and_empty_returns_error() {
        let result = PersonName::new("".to_string(), Some("   ".to_string()), "".to_string());

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_empty_first_and_last_returns_error() {
        let result = PersonName::new("".to_string(), None, "".to_string());

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_tabs_only_in_first_name_returns_error() {
        let result = PersonName::new("\t\t".to_string(), None, "Doe".to_string());

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_newlines_only_in_last_name_returns_error() {
        let result = PersonName::new("John".to_string(), None, "\n\n".to_string());

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_mixed_whitespace_in_middle_name_returns_error() {
        let result = PersonName::new(
            "John".to_string(),
            Some(" \t\n ".to_string()),
            "Doe".to_string(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_single_character_first_name_returns_ok() {
        let result = PersonName::new("J".to_string(), None, "Doe".to_string());

        assert!(result.is_ok());
    }

    #[test]
    fn test_new_with_single_character_last_name_returns_ok() {
        let result = PersonName::new("John".to_string(), None, "D".to_string());

        assert!(result.is_ok());
    }

    #[test]
    fn test_new_with_single_character_middle_name_returns_ok() {
        let result = PersonName::new("John".to_string(), Some("M".to_string()), "Doe".to_string());

        assert!(result.is_ok());
    }

    #[test]
    fn test_new_with_first_name_at_max_length_returns_ok() {
        let long_name = "a".repeat(100);
        let result = PersonName::new(long_name, None, "Doe".to_string());

        assert!(result.is_ok());
    }

    #[test]
    fn test_new_with_first_name_exceeding_max_length_returns_error() {
        let too_long_name = "a".repeat(101);
        let result = PersonName::new(too_long_name, None, "Doe".to_string());

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_last_name_at_max_length_returns_ok() {
        let long_name = "a".repeat(100);
        let result = PersonName::new("John".to_string(), None, long_name);

        assert!(result.is_ok());
    }

    #[test]
    fn test_new_with_last_name_exceeding_max_length_returns_error() {
        let too_long_name = "a".repeat(101);
        let result = PersonName::new("John".to_string(), None, too_long_name);

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_middle_name_at_max_length_returns_ok() {
        let long_name = "a".repeat(100);
        let result = PersonName::new("John".to_string(), Some(long_name), "Doe".to_string());

        assert!(result.is_ok());
    }

    #[test]
    fn test_new_with_middle_name_exceeding_max_length_returns_error() {
        let too_long_name = "a".repeat(101);
        let result = PersonName::new("John".to_string(), Some(too_long_name), "Doe".to_string());

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_all_names_at_minimum_valid_length_returns_ok() {
        let result = PersonName::new("J".to_string(), Some("M".to_string()), "D".to_string());

        assert!(result.is_ok());
    }

    #[test]
    fn test_new_with_all_names_at_maximum_valid_length_returns_ok() {
        let long_name = "a".repeat(100);
        let result = PersonName::new(
            long_name.clone(),
            Some(long_name.clone()),
            long_name,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_new_with_first_name_too_long_after_trim_returns_error() {
        let too_long_name = format!("  {}  ", "a".repeat(101));
        let result = PersonName::new(too_long_name, None, "Doe".to_string());

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_last_name_too_long_after_trim_returns_error() {
        let too_long_name = format!("  {}  ", "a".repeat(101));
        let result = PersonName::new("John".to_string(), None, too_long_name);

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_middle_name_too_long_after_trim_returns_error() {
        let too_long_name = format!("  {}  ", "a".repeat(101));
        let result = PersonName::new("John".to_string(), Some(too_long_name), "Doe".to_string());

        assert!(result.is_err());
    }
}
