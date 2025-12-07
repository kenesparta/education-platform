use crate::{Validator, ValidatorError};
use std::fmt;
use std::ops::Deref;
use thiserror::Error;

/// Error type for `Name` validation failures.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum NameError {
    /// Validation error occurred during name validation.
    #[error("Name validation failed: {0}")]
    ValidationError(#[from] ValidatorError),
}

/// Configuration for name validation rules.
///
/// # Examples
///
/// ```
/// use education_platform_common::NameConfig;
///
/// let config = NameConfig::default();
/// assert_eq!(config.min_length(), 1);
/// assert_eq!(config.max_length(), 100);
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
    /// Creates a default configuration with min_length=1 and max_length=100.
    fn default() -> Self {
        Self::new(1, 100)
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
            min_length: 1,
            max_length: 100,
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
/// // Using the default configuration (1-100 characters)
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
    /// - Have length >= 1 character
    /// - Have length <= 100 characters
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
        Validator::is_greater_than(trimmed, config.min_length)?;
        Validator::is_less_than(trimmed, config.max_length)?;

        Ok(Self {
            inner: trimmed.to_string(),
            config,
        })
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
        assert_eq!(config.min_length(), 1);
        assert_eq!(config.max_length(), 100);
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
        let name = Name::new("J".to_string()).unwrap();
        assert_eq!(name.as_str(), "J");
    }

    #[test]
    fn test_name_at_max_boundary() {
        let long_name = "a".repeat(100);
        let name = Name::new(long_name.clone()).unwrap();
        assert_eq!(name.as_str(), &long_name);
    }

    #[test]
    fn test_name_exceeds_max_boundary_returns_error() {
        let too_long = "a".repeat(101);
        let result = Name::new(too_long);
        assert!(result.is_err());
    }
}
