use crate::{Name, NameError};
use thiserror::Error;

/// Error type for `PersonName` validation failures.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PersonNameError {
    #[error("Name validation failed: {0}")]
    NameError(#[from] NameError),
}

/// Represents a person's name with first, optional middle, last, and optional second last name components.
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
///     "Doe".to_string(),
///     None
/// ).unwrap();
/// assert_eq!(name.full_name(), "John Michael Doe");
///
/// let spanish = PersonName::new(
///     "María".to_string(),
///     None,
///     "García".to_string(),
///     Some("Rodríguez".to_string())
/// ).unwrap();
/// assert_eq!(spanish.full_name(), "María García Rodríguez");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersonName {
    first_name: Name,
    middle_name: Option<Name>,
    last_name: Name,
    second_last_name: Option<Name>,
}

impl PersonName {
    /// Creates a new `PersonName` instance with validated name components.
    ///
    /// # Errors
    ///
    /// Returns error if any name component is empty, contains only whitespace,
    /// or exceeds length constraints (1-100 characters).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::PersonName;
    ///
    /// let name = PersonName::new(
    ///     "John".to_string(),
    ///     Some("Michael".to_string()),
    ///     "Doe".to_string(),
    ///     None
    /// ).unwrap();
    /// assert_eq!(name.first_name(), "John");
    ///
    /// let spanish = PersonName::new(
    ///     "María".to_string(),
    ///     None,
    ///     "García".to_string(),
    ///     Some("Rodríguez".to_string())
    /// ).unwrap();
    /// assert_eq!(spanish.second_last_name(), Some("Rodríguez"));
    ///
    /// let invalid = PersonName::new("".to_string(), None, "Doe".to_string(), None);
    /// assert!(invalid.is_err());
    /// ```
    pub fn new(
        first_name: String,
        middle_name: Option<String>,
        last_name: String,
        second_last_name: Option<String>,
    ) -> Result<Self, PersonNameError> {
        Ok(Self {
            first_name: Name::new(first_name)?,
            middle_name: middle_name.map(Name::new).transpose()?,
            last_name: Name::new(last_name)?,
            second_last_name: second_last_name.map(Name::new).transpose()?,
        })
    }

    /// Returns the first name.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::PersonName;
    ///
    /// let name = PersonName::new("John".to_string(), None, "Doe".to_string(), None).unwrap();
    /// assert_eq!(name.first_name(), "John");
    /// ```
    #[inline]
    #[must_use]
    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    /// Returns the middle name if present.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::PersonName;
    ///
    /// let with_middle = PersonName::new(
    ///     "John".to_string(),
    ///     Some("Michael".to_string()),
    ///     "Doe".to_string(),
    ///     None
    /// ).unwrap();
    /// assert_eq!(with_middle.middle_name(), Some("Michael"));
    ///
    /// let without_middle = PersonName::new("Jane".to_string(), None, "Smith".to_string(), None).unwrap();
    /// assert_eq!(without_middle.middle_name(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn middle_name(&self) -> Option<&str> {
        self.middle_name.as_deref()
    }

    /// Returns the last name.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::PersonName;
    ///
    /// let name = PersonName::new("John".to_string(), None, "Doe".to_string(), None).unwrap();
    /// assert_eq!(name.last_name(), "Doe");
    /// ```
    #[inline]
    #[must_use]
    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    /// Returns the second last name if present.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::PersonName;
    ///
    /// let spanish = PersonName::new(
    ///     "María".to_string(),
    ///     None,
    ///     "García".to_string(),
    ///     Some("Rodríguez".to_string())
    /// ).unwrap();
    /// assert_eq!(spanish.second_last_name(), Some("Rodríguez"));
    ///
    /// let simple = PersonName::new("John".to_string(), None, "Doe".to_string(), None).unwrap();
    /// assert_eq!(simple.second_last_name(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn second_last_name(&self) -> Option<&str> {
        self.second_last_name.as_deref()
    }

    /// Returns the full name formatted as a single string.
    ///
    /// Includes all name components separated by spaces.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::PersonName;
    ///
    /// let name = PersonName::new(
    ///     "John".to_string(),
    ///     Some("Michael".to_string()),
    ///     "Doe".to_string(),
    ///     None
    /// ).unwrap();
    /// assert_eq!(name.full_name(), "John Michael Doe");
    ///
    /// let spanish = PersonName::new(
    ///     "María".to_string(),
    ///     None,
    ///     "García".to_string(),
    ///     Some("Rodríguez".to_string())
    /// ).unwrap();
    /// assert_eq!(spanish.full_name(), "María García Rodríguez");
    /// ```
    #[must_use]
    pub fn full_name(&self) -> String {
        let mut parts = vec![self.first_name.as_str()];
        if let Some(middle) = &self.middle_name {
            parts.push(middle.as_str());
        }
        parts.push(self.last_name.as_str());
        if let Some(second_last) = &self.second_last_name {
            parts.push(second_last.as_str());
        }
        parts.join(" ")
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
            None,
        );

        assert!(result.is_ok());
        let name = result.unwrap();
        assert_eq!(name.first_name(), "John");
        assert_eq!(name.middle_name(), Some("Michael"));
        assert_eq!(name.last_name(), "Doe");
    }

    #[test]
    fn test_new_without_middle_name_returns_ok() {
        let result = PersonName::new("Jane".to_string(), None, "Smith".to_string(), None);

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
            None,
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
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_whitespace_only_first_name_returns_error() {
        let result = PersonName::new(
            "   ".to_string(),
            Some("Michael".to_string()),
            "Doe".to_string(),
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_empty_last_name_returns_error() {
        let result = PersonName::new(
            "John".to_string(),
            Some("Michael".to_string()),
            "".to_string(),
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_whitespace_only_last_name_returns_error() {
        let result = PersonName::new(
            "John".to_string(),
            Some("Michael".to_string()),
            "   ".to_string(),
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_empty_middle_name_returns_error() {
        let result = PersonName::new(
            "John".to_string(),
            Some("".to_string()),
            "Doe".to_string(),
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_whitespace_only_middle_name_returns_error() {
        let result = PersonName::new(
            "John".to_string(),
            Some("   ".to_string()),
            "Doe".to_string(),
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_full_name_with_middle_name() {
        let name = PersonName::new(
            "John".to_string(),
            Some("Michael".to_string()),
            "Doe".to_string(),
            None,
        )
        .unwrap();

        assert_eq!(name.full_name(), "John Michael Doe");
    }

    #[test]
    fn test_full_name_without_middle_name() {
        let name = PersonName::new("Jane".to_string(), None, "Smith".to_string(), None).unwrap();

        assert_eq!(name.full_name(), "Jane Smith");
    }

    #[test]
    fn test_clone_creates_equal_instance() {
        let original = PersonName::new(
            "John".to_string(),
            Some("Michael".to_string()),
            "Doe".to_string(),
            None,
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
            None,
        )
        .unwrap();

        let name2 = PersonName::new(
            "John".to_string(),
            Some("Michael".to_string()),
            "Doe".to_string(),
            None,
        )
        .unwrap();

        assert_eq!(name1, name2);
    }

    #[test]
    fn test_inequality_for_different_first_names() {
        let name1 = PersonName::new("John".to_string(), None, "Doe".to_string(), None).unwrap();

        let name2 = PersonName::new("Jane".to_string(), None, "Doe".to_string(), None).unwrap();

        assert_ne!(name1, name2);
    }

    #[test]
    fn test_new_with_all_names_empty_returns_error() {
        let result = PersonName::new("".to_string(), Some("".to_string()), "".to_string(), None);

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_all_names_whitespace_only_returns_error() {
        let result = PersonName::new(
            "   ".to_string(),
            Some("   ".to_string()),
            "   ".to_string(),
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_mixed_whitespace_and_empty_returns_error() {
        let result = PersonName::new(
            "".to_string(),
            Some("   ".to_string()),
            "".to_string(),
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_empty_first_and_last_returns_error() {
        let result = PersonName::new("".to_string(), None, "".to_string(), None);

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_tabs_only_in_first_name_returns_error() {
        let result = PersonName::new("\t\t".to_string(), None, "Doe".to_string(), None);

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_newlines_only_in_last_name_returns_error() {
        let result = PersonName::new("John".to_string(), None, "\n\n".to_string(), None);

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_mixed_whitespace_in_middle_name_returns_error() {
        let result = PersonName::new(
            "John".to_string(),
            Some(" \t\n ".to_string()),
            "Doe".to_string(),
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_two_character_first_name_returns_ok() {
        let result = PersonName::new("Jo".to_string(), None, "Doe".to_string(), None);

        assert!(result.is_ok());
    }

    #[test]
    fn test_new_with_two_character_last_name_returns_ok() {
        let result = PersonName::new("John".to_string(), None, "Do".to_string(), None);

        assert!(result.is_ok());
    }

    #[test]
    fn test_new_with_two_character_middle_name_returns_ok() {
        let result = PersonName::new(
            "John".to_string(),
            Some("Ma".to_string()),
            "Doe".to_string(),
            None,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_new_with_first_name_at_max_length_returns_ok() {
        let long_name = "a".repeat(101);
        let result = PersonName::new(long_name, None, "Doe".to_string(), None);

        assert!(result.is_ok());
    }

    #[test]
    fn test_new_with_first_name_exceeding_max_length_returns_error() {
        let too_long_name = "a".repeat(102);
        let result = PersonName::new(too_long_name, None, "Doe".to_string(), None);

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_last_name_at_max_length_returns_ok() {
        let long_name = "a".repeat(101);
        let result = PersonName::new("John".to_string(), None, long_name, None);

        assert!(result.is_ok());
    }

    #[test]
    fn test_new_with_last_name_exceeding_max_length_returns_error() {
        let too_long_name = "a".repeat(102);
        let result = PersonName::new("John".to_string(), None, too_long_name, None);

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_middle_name_at_max_length_returns_ok() {
        let long_name = "a".repeat(101);
        let result = PersonName::new("John".to_string(), Some(long_name), "Doe".to_string(), None);

        assert!(result.is_ok());
    }

    #[test]
    fn test_new_with_middle_name_exceeding_max_length_returns_error() {
        let too_long_name = "a".repeat(102);
        let result = PersonName::new(
            "John".to_string(),
            Some(too_long_name),
            "Doe".to_string(),
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_all_names_at_minimum_valid_length_returns_ok() {
        let result = PersonName::new(
            "Jo".to_string(),
            Some("Ma".to_string()),
            "Do".to_string(),
            None,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_new_with_all_names_at_maximum_valid_length_returns_ok() {
        let long_name = "a".repeat(101);
        let result = PersonName::new(long_name.clone(), Some(long_name.clone()), long_name, None);

        assert!(result.is_ok());
    }

    #[test]
    fn test_new_with_first_name_too_long_after_trim_returns_error() {
        let too_long_name = format!("  {}  ", "a".repeat(102));
        let result = PersonName::new(too_long_name, None, "Doe".to_string(), None);

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_last_name_too_long_after_trim_returns_error() {
        let too_long_name = format!("  {}  ", "a".repeat(102));
        let result = PersonName::new("John".to_string(), None, too_long_name, None);

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_middle_name_too_long_after_trim_returns_error() {
        let too_long_name = format!("  {}  ", "a".repeat(102));
        let result = PersonName::new(
            "John".to_string(),
            Some(too_long_name),
            "Doe".to_string(),
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_second_last_name_returns_ok() {
        let result = PersonName::new(
            "María".to_string(),
            None,
            "García".to_string(),
            Some("Rodríguez".to_string()),
        );

        assert!(result.is_ok());
        let name = result.unwrap();
        assert_eq!(name.first_name(), "María");
        assert_eq!(name.last_name(), "García");
        assert_eq!(name.second_last_name(), Some("Rodríguez"));
        assert_eq!(name.full_name(), "María García Rodríguez");
    }

    #[test]
    fn test_new_with_empty_second_last_name_returns_error() {
        let result = PersonName::new(
            "María".to_string(),
            None,
            "García".to_string(),
            Some("".to_string()),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_full_name_with_all_components() {
        let name = PersonName::new(
            "Juan".to_string(),
            Some("Carlos".to_string()),
            "García".to_string(),
            Some("Rodríguez".to_string()),
        )
        .unwrap();

        assert_eq!(name.full_name(), "Juan Carlos García Rodríguez");
    }
}
