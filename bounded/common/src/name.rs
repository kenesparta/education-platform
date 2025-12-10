use crate::{Validator, ValidatorError};
use std::fmt;
use std::ops::Deref;
use thiserror::Error;

const MAX_LENGTH: usize = 101;
const MIN_LENGTH: usize = 2;

/// Error type for `Name` validation failures.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum NameError {
    /// Validation error occurred during name validation.
    #[error("Name validation failed: {0}")]
    ValidationError(#[from] ValidatorError),

    #[error("Name is empty")]
    EmptyValue,

    #[error("Name characters are not valid")]
    CharactersNotValid,
}

/// Configuration for name validation rules.
///
/// # Examples
///
/// ```
/// use education_platform_common::NameConfig;
///
/// let config = NameConfig::default();
/// assert_eq!(config.min_length(), 2);
/// assert_eq!(config.max_length(), 101);
///
/// let custom = NameConfig::builder()
///     .min_length(2)
///     .max_length(50)
///     .build();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NameConfig {
    min_length: usize,
    max_length: usize,
}

impl NameConfig {
    /// Creates a new `NameConfig` with the specified constraints.
    ///
    /// # Panics
    ///
    /// Panics if `min_length` is greater than `max_length`.
    #[must_use]
    pub const fn new(min_length: usize, max_length: usize) -> Self {
        assert!(
            min_length <= max_length,
            "min_length must be less than or equal to max_length"
        );
        Self {
            min_length,
            max_length,
        }
    }

    /// Creates a builder for `NameConfig`.
    #[must_use]
    pub const fn builder() -> NameConfigBuilder {
        NameConfigBuilder::new()
    }

    /// Returns the minimum allowed length.
    #[inline]
    #[must_use]
    pub const fn min_length(&self) -> usize {
        self.min_length
    }

    /// Returns the maximum allowed length.
    #[inline]
    #[must_use]
    pub const fn max_length(&self) -> usize {
        self.max_length
    }
}

impl Default for NameConfig {
    /// Creates a default configuration with min_length=2 and max_length=101.
    fn default() -> Self {
        Self::new(MIN_LENGTH, MAX_LENGTH)
    }
}

/// Builder for `NameConfig`.
#[derive(Debug, Clone, Copy)]
pub struct NameConfigBuilder {
    min_length: usize,
    max_length: usize,
}

impl NameConfigBuilder {
    /// Creates a new builder with default values.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            min_length: MIN_LENGTH,
            max_length: MAX_LENGTH,
        }
    }

    /// Sets the minimum length constraint.
    #[must_use]
    pub const fn min_length(mut self, min: usize) -> Self {
        self.min_length = min;
        self
    }

    /// Sets the maximum length constraint.
    #[must_use]
    pub const fn max_length(mut self, max: usize) -> Self {
        self.max_length = max;
        self
    }

    /// Builds the `NameConfig`.
    ///
    /// # Panics
    ///
    /// Panics if `min_length` is greater than `max_length`.
    #[must_use]
    pub const fn build(self) -> NameConfig {
        NameConfig::new(self.min_length, self.max_length)
    }
}

impl Default for NameConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// A validated name string with configurable constraints.
///
/// This is a newtype wrapper around `String` that ensures the name
/// meets validation requirements at construction time.
///
/// # Examples
///
/// ```
/// use education_platform_common::{Name, NameConfig};
///
/// // Using the default configuration (2-101 characters)
/// let name = Name::new("John".to_string()).unwrap();
/// assert_eq!(name.as_str(), "John");
///
/// // Using custom configuration
/// let config = NameConfig::builder().min_length(2).max_length(50).build();
/// let name = Name::with_config("Jane".to_string(), config).unwrap();
/// assert_eq!(name.as_str(), "Jane");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Name {
    inner: String,
    config: NameConfig,
}

impl Name {
    /// Creates a new `Name` with default validation rules.
    ///
    /// The name will be trimmed and validated to:
    /// - Not be empty after trimming
    /// - Have length >= 2 characters
    /// - Have length <= 101 characters
    /// - Contain only valid Latin characters (letters, spaces, hyphens, apostrophes)
    ///
    /// # Errors
    ///
    /// Returns `NameError::ValidationError` if validation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Name;
    ///
    /// let name = Name::new("John".to_string()).unwrap();
    /// assert_eq!(name.as_str(), "John");
    ///
    /// // Whitespace is trimmed
    /// let trimmed = Name::new("  Alice  ".to_string()).unwrap();
    /// assert_eq!(trimmed.as_str(), "Alice");
    /// ```
    pub fn new(name: String) -> Result<Self, NameError> {
        Self::with_config(name, NameConfig::default())
    }

    /// Creates a new `Name` with custom validation configuration.
    ///
    /// # Errors
    ///
    /// Returns `NameError::ValidationError` if validation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::{Name, NameConfig};
    ///
    /// let config = NameConfig::builder()
    ///     .min_length(3)
    ///     .max_length(20)
    ///     .build();
    ///
    /// let name = Name::with_config("Bob".to_string(), config).unwrap();
    /// assert_eq!(name.as_str(), "Bob");
    ///
    /// // Too short for this config
    /// let result = Name::with_config("Jo".to_string(), config);
    /// assert!(result.is_err());
    /// ```
    pub fn with_config(name: String, config: NameConfig) -> Result<Self, NameError> {
        let trimmed = name.trim();

        Validator::is_not_empty(trimmed)?;
        Validator::has_min_length(trimmed, config.min_length)?;
        Validator::has_max_length(trimmed, config.max_length)?;
        Self::is_valid_latin_name(trimmed)?;

        Ok(Self {
            inner: trimmed.to_string(),
            config,
        })
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
    ///
    /// ```
    /// use education_platform_common::Name;
    ///
    /// assert!(Name::is_valid_latin_name("José").is_ok());
    /// assert!(Name::is_valid_latin_name("María García").is_ok());
    /// assert!(Name::is_valid_latin_name("O'Brien").is_ok());
    /// assert!(Name::is_valid_latin_name("Jean-Pierre").is_ok());
    /// assert!(Name::is_valid_latin_name("João").is_ok());
    /// assert!(Name::is_valid_latin_name("Nuñez").is_ok());
    ///
    /// assert!(Name::is_valid_latin_name("John123").is_err());
    /// assert!(Name::is_valid_latin_name("José@email").is_err());
    /// assert!(Name::is_valid_latin_name("Test$Name").is_err());
    /// ```
    pub fn is_valid_latin_name(name: &str) -> Result<(), NameError> {
        if name.is_empty() {
            return Err(NameError::EmptyValue);
        }

        if !name
            .chars()
            .all(|c| c.is_alphabetic() || c == ' ' || c == '-' || c == '\'')
        {
            return Err(NameError::CharactersNotValid);
        }

        Ok(())
    }

    /// Returns the name as a string slice.
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.inner
    }

    /// Returns the configuration used for this name.
    #[inline]
    #[must_use]
    pub const fn config(&self) -> &NameConfig {
        &self.config
    }

    /// Consumes the `Name` and returns the inner `String`.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> String {
        self.inner
    }
}

impl Deref for Name {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl From<Name> for String {
    fn from(name: Name) -> Self {
        name.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_new_with_valid_name_returns_ok() {
        let result = Name::new("John".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "John");
    }

    #[test]
    fn test_name_new_trims_whitespace() {
        let result = Name::new("  Alice  ".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "Alice");
    }

    #[test]
    fn test_name_new_with_empty_string_returns_error() {
        let result = Name::new("".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_name_new_with_whitespace_only_returns_error() {
        let result = Name::new("   ".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_name_with_custom_config() {
        let config = NameConfig::builder().min_length(3).max_length(20).build();

        let result = Name::with_config("Bob".to_string(), config);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "Bob");
    }

    #[test]
    fn test_name_with_custom_config_too_short_returns_error() {
        let config = NameConfig::builder().min_length(5).max_length(20).build();

        let result = Name::with_config("Bob".to_string(), config);
        assert!(result.is_err());
    }

    #[test]
    fn test_name_with_custom_config_too_long_returns_error() {
        let config = NameConfig::builder().min_length(1).max_length(5).build();

        let result = Name::with_config("Extremely".to_string(), config);
        assert!(result.is_err());
    }

    #[test]
    fn test_name_config_default() {
        let config = NameConfig::default();
        assert_eq!(config.min_length(), 2);
        assert_eq!(config.max_length(), 101);
    }

    #[test]
    fn test_name_config_builder() {
        let config = NameConfig::builder().min_length(2).max_length(50).build();
        assert_eq!(config.min_length(), 2);
        assert_eq!(config.max_length(), 50);
    }

    #[test]
    fn test_name_deref() {
        let name = Name::new("Test".to_string()).unwrap();
        assert_eq!(name.len(), 4);
        assert!(name.starts_with("T"));
    }

    #[test]
    fn test_name_display() {
        let name = Name::new("Display".to_string()).unwrap();
        assert_eq!(format!("{}", name), "Display");
    }

    #[test]
    fn test_name_into_string() {
        let name = Name::new("Convert".to_string()).unwrap();
        let string: String = name.into();
        assert_eq!(string, "Convert");
    }

    #[test]
    fn test_name_as_ref() {
        let name = Name::new("Reference".to_string()).unwrap();
        let s: &str = name.as_ref();
        assert_eq!(s, "Reference");
    }

    #[test]
    fn test_name_equality() {
        let name1 = Name::new("Same".to_string()).unwrap();
        let name2 = Name::new("Same".to_string()).unwrap();
        assert_eq!(name1, name2);
    }

    #[test]
    fn test_name_clone() {
        let name1 = Name::new("Original".to_string()).unwrap();
        let name2 = name1.clone();
        assert_eq!(name1, name2);
    }

    #[test]
    fn test_name_at_min_boundary() {
        let name = Name::new("Jo".to_string()).unwrap();
        assert_eq!(name.as_str(), "Jo");
    }

    #[test]
    fn test_name_at_max_boundary() {
        let long_name = "a".repeat(101);
        let name = Name::new(long_name.clone()).unwrap();
        assert_eq!(name.as_str(), &long_name);
    }

    #[test]
    fn test_name_exceeds_max_boundary_returns_error() {
        let too_long = "a".repeat(102);
        let result = Name::new(too_long);
        assert!(result.is_err());
    }

    #[test]
    fn test_name_with_spanish_characters_returns_ok() {
        assert!(Name::new("José".to_string()).is_ok());
        assert!(Name::new("María".to_string()).is_ok());
        assert!(Name::new("Ángel".to_string()).is_ok());
        assert!(Name::new("Nuñez".to_string()).is_ok());
        assert!(Name::new("García".to_string()).is_ok());
        assert!(Name::new("Rodríguez".to_string()).is_ok());
    }

    #[test]
    fn test_name_with_portuguese_characters_returns_ok() {
        assert!(Name::new("João".to_string()).is_ok());
        assert!(Name::new("António".to_string()).is_ok());
        assert!(Name::new("Conceição".to_string()).is_ok());
        assert!(Name::new("José".to_string()).is_ok());
    }

    #[test]
    fn test_name_with_hyphen_returns_ok() {
        assert!(Name::new("Mary-Jane".to_string()).is_ok());
        assert!(Name::new("María-José".to_string()).is_ok());
        assert!(Name::new("Jean-Pierre".to_string()).is_ok());
    }

    #[test]
    fn test_name_with_apostrophe_returns_ok() {
        assert!(Name::new("O'Brien".to_string()).is_ok());
        assert!(Name::new("D'Angelo".to_string()).is_ok());
    }

    #[test]
    fn test_name_with_numbers_returns_error() {
        assert!(Name::new("John123".to_string()).is_err());
        assert!(Name::new("María2".to_string()).is_err());
        assert!(Name::new("Test123Name".to_string()).is_err());
        assert!(Name::new("1John".to_string()).is_err());
    }

    #[test]
    fn test_name_with_special_characters_returns_error() {
        assert!(Name::new("John@Doe".to_string()).is_err());
        assert!(Name::new("Test$Name".to_string()).is_err());
        assert!(Name::new("Name!".to_string()).is_err());
        assert!(Name::new("José#García".to_string()).is_err());
        assert!(Name::new("Test.Name".to_string()).is_err());
        assert!(Name::new("Name_Test".to_string()).is_err());
        assert!(Name::new("Test&Name".to_string()).is_err());
        assert!(Name::new("Name%Test".to_string()).is_err());
    }

    #[test]
    fn test_name_with_multiple_words_returns_ok() {
        assert!(Name::new("Mary Jane".to_string()).is_ok());
        assert!(Name::new("María García".to_string()).is_ok());
        assert!(Name::new("Anne Marie".to_string()).is_ok());
    }

    #[test]
    fn test_valid_latin_names_english() {
        assert!(Name::is_valid_latin_name("John").is_ok());
        assert!(Name::is_valid_latin_name("Mary Jane").is_ok());
        assert!(Name::is_valid_latin_name("O'Brien").is_ok());
        assert!(Name::is_valid_latin_name("Mary-Jane").is_ok());
        assert!(Name::is_valid_latin_name("Anne Marie").is_ok());
    }

    #[test]
    fn test_valid_latin_names_spanish() {
        assert!(Name::is_valid_latin_name("José").is_ok());
        assert!(Name::is_valid_latin_name("María").is_ok());
        assert!(Name::is_valid_latin_name("Nuñez").is_ok());
        assert!(Name::is_valid_latin_name("García").is_ok());
        assert!(Name::is_valid_latin_name("Rodríguez").is_ok());
        assert!(Name::is_valid_latin_name("María José").is_ok());
        assert!(Name::is_valid_latin_name("María-José").is_ok());
        assert!(Name::is_valid_latin_name("Ángel").is_ok());
        assert!(Name::is_valid_latin_name("Mónica").is_ok());
    }

    #[test]
    fn test_valid_latin_names_portuguese() {
        assert!(Name::is_valid_latin_name("João").is_ok());
        assert!(Name::is_valid_latin_name("José").is_ok());
        assert!(Name::is_valid_latin_name("António").is_ok());
        assert!(Name::is_valid_latin_name("Conceição").is_ok());
        assert!(Name::is_valid_latin_name("São Paulo").is_ok());
    }

    #[test]
    fn test_invalid_latin_names_with_numbers() {
        assert!(matches!(
            Name::is_valid_latin_name("John123"),
            Err(NameError::CharactersNotValid)
        ));
        assert!(matches!(
            Name::is_valid_latin_name("María2"),
            Err(NameError::CharactersNotValid)
        ));
        assert!(matches!(
            Name::is_valid_latin_name("Test123Name"),
            Err(NameError::CharactersNotValid)
        ));
    }

    #[test]
    fn test_invalid_latin_names_with_special_characters() {
        assert!(matches!(
            Name::is_valid_latin_name("John@Doe"),
            Err(NameError::CharactersNotValid)
        ));
        assert!(matches!(
            Name::is_valid_latin_name("Test$Name"),
            Err(NameError::CharactersNotValid)
        ));
        assert!(matches!(
            Name::is_valid_latin_name("Name!"),
            Err(NameError::CharactersNotValid)
        ));
        assert!(matches!(
            Name::is_valid_latin_name("José#García"),
            Err(NameError::CharactersNotValid)
        ));
        assert!(matches!(
            Name::is_valid_latin_name("Test.Name"),
            Err(NameError::CharactersNotValid)
        ));
        assert!(matches!(
            Name::is_valid_latin_name("Name_Test"),
            Err(NameError::CharactersNotValid)
        ));
    }

    #[test]
    fn test_invalid_latin_names_empty() {
        assert!(matches!(
            Name::is_valid_latin_name(""),
            Err(NameError::EmptyValue)
        ));
    }
}
