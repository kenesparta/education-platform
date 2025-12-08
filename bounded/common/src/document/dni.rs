use crate::ValidatorError;
use regex::Regex;
use std::fmt;
use std::str::FromStr;
use std::sync::LazyLock;
use thiserror::Error;

/// Error types for DNI validation failures.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DniError {
    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidatorError),

    #[error("Incorrect DNI format (expected: XXXXXXXX-Y where X is digit and Y is 0-9 or A-K)")]
    IncorrectFormat,

    #[error("Invalid length: {0}, should be {1} digits")]
    LengthIsIncorrect(usize, usize),

    #[error("Invalid digit: {0}")]
    DigitNotValid(String),

    #[error(
        "Incorrect validation character: expected {expected_numeric} or {expected_alpha}, received {received}"
    )]
    IncorrectValidationDigit {
        expected_numeric: String,
        expected_alpha: String,
        received: String,
    },
}

const DOCUMENT_SIZE: usize = 8;
const VERIFICATION_ARRAY: [usize; DOCUMENT_SIZE] = [3, 2, 7, 6, 5, 4, 3, 2];
const BASE: usize = 11;
const NUMERIC_SERIES: [&str; BASE] = ["6", "7", "8", "9", "0", "1", "1", "2", "3", "4", "5"];
const ALPHA_SERIES: [&str; BASE] = ["K", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];

static DNI_REGEX: LazyLock<Result<Regex, regex::Error>> =
    LazyLock::new(|| Regex::new(r"^\d{8}-([0-9]|[A-K])$"));

/// Peruvian DNI (Documento Nacional de Identidad) as a Value Object.
///
/// A DNI consists of 8 digits followed by a verification character (digit or letter A-K).
/// The verification character is calculated using a checksum algorithm based on
/// multiplication with a verification array and modulo 11 operation.
///
/// Format: XXXXXXXX-Y where:
/// - X: 8 digits (00000001 to 99999999)
/// - Y: verification character (0-9 or A-K)
///
/// # Examples
///
/// ```
/// use education_platform_common::document::Dni;
///
/// let dni = Dni::new("12345678-1".to_string()).unwrap();
/// assert_eq!(dni.value(), "12345678");
/// assert_eq!(dni.verification_char(), "1");
///
/// // Parse from string
/// let dni: Dni = "12345678-1".parse().unwrap();
/// assert_eq!(dni.to_string(), "12345678-1");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dni {
    value: String,
    verification_char: String,
}

impl Dni {
    /// Creates a new DNI Value Object with validation.
    ///
    /// Validates the format and verification character using the Peruvian DNI algorithm.
    ///
    /// # Errors
    ///
    /// Returns `DniError::IncorrectFormat` if format is invalid.
    /// Returns `DniError::LengthIsIncorrect` if the DNI doesn't have 8 digits.
    /// Returns `DniError::IncorrectValidationDigit` if verification character doesn't match.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::document::Dni;
    ///
    /// let dni = Dni::new("12345678-1".to_string()).unwrap();
    /// assert_eq!(dni.value(), "12345678");
    ///
    /// // Invalid format
    /// assert!(Dni::new("1234567".to_string()).is_err());
    /// assert!(Dni::new("12345678-Z".to_string()).is_err());
    /// ```
    pub fn new(dni: String) -> Result<Self, DniError> {
        let trimmed = dni.trim();
        Self::validate_format(trimmed)?;

        let (dni_value, validation_char) =
            trimmed.split_once('-').ok_or(DniError::IncorrectFormat)?;

        Self::validate_verification_character(dni_value, validation_char)?;

        Ok(Self {
            value: dni_value.to_string(),
            verification_char: validation_char.to_string(),
        })
    }

    /// Returns the 8-digit DNI value without the verification character.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::document::Dni;
    ///
    /// let dni = Dni::new("12345678-1".to_string()).unwrap();
    /// assert_eq!(dni.value(), "12345678");
    /// ```
    #[inline]
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the verification character.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::document::Dni;
    ///
    /// let dni = Dni::new("12345678-1".to_string()).unwrap();
    /// assert_eq!(dni.verification_char(), "1");
    /// ```
    #[inline]
    #[must_use]
    pub fn verification_char(&self) -> &str {
        &self.verification_char
    }

    fn validate_format(dni: &str) -> Result<(), DniError> {
        let regex = DNI_REGEX
            .as_ref()
            .map_err(|e| ValidatorError::RegexError(e.to_string()))?;

        if !regex.is_match(dni) {
            return Err(DniError::IncorrectFormat);
        }

        Ok(())
    }

    fn validate_verification_character(dni: &str, validation_char: &str) -> Result<(), DniError> {
        let dni_len = dni.len();
        if dni_len != DOCUMENT_SIZE {
            return Err(DniError::LengthIsIncorrect(dni_len, DOCUMENT_SIZE));
        }

        let dni_digits: Vec<usize> = dni
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .map(|d| d as usize)
                    .ok_or_else(|| DniError::DigitNotValid(c.to_string()))
            })
            .collect::<Result<Vec<usize>, DniError>>()?;

        let checksum: usize = dni_digits
            .iter()
            .zip(VERIFICATION_ARRAY.iter())
            .map(|(digit, multiplier)| digit * multiplier)
            .sum();

        // Compute check digit index (modulo 11 complement)
        let index = (BASE - checksum % BASE) % BASE;

        let expected_numeric = NUMERIC_SERIES[index];
        let expected_alpha = ALPHA_SERIES[index];
        if validation_char != expected_numeric && validation_char != expected_alpha {
            return Err(DniError::IncorrectValidationDigit {
                expected_numeric: expected_numeric.to_string(),
                expected_alpha: expected_alpha.to_string(),
                received: validation_char.to_string(),
            });
        }

        Ok(())
    }
}

impl fmt::Display for Dni {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.value, self.verification_char)
    }
}

impl FromStr for Dni {
    type Err = DniError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dni_new_with_valid_numeric_verification() {
        let result = Dni::new("17801146-0".to_string());
        assert!(result.is_ok());
        let dni = result.unwrap();
        assert_eq!(dni.value(), "17801146");
        assert_eq!(dni.verification_char(), "0");
    }

    #[test]
    fn test_dni_new_with_valid_alpha_verification() {
        let result = Dni::new("00000001-I".to_string());
        assert!(result.is_ok());
        let dni = result.unwrap();
        assert_eq!(dni.value(), "00000001");
        assert_eq!(dni.verification_char(), "I");
    }

    #[test]
    fn test_dni_new_trims_whitespace() {
        let result = Dni::new("  12345678-1  ".to_string());
        assert!(result.is_ok());
        let dni = result.unwrap();
        assert_eq!(dni.value(), "12345678");
    }

    #[test]
    fn test_dni_display_format() {
        let dni = Dni::new("12345678-1".to_string()).unwrap();
        assert_eq!(dni.to_string(), "12345678-1");
    }

    #[test]
    fn test_dni_from_str() {
        let dni: Dni = "12345678-1".parse().unwrap();
        assert_eq!(dni.value(), "12345678");
        assert_eq!(dni.verification_char(), "1");
    }

    #[test]
    fn test_dni_equality() {
        let dni1 = Dni::new("12345678-1".to_string()).unwrap();
        let dni2 = Dni::new("12345678-1".to_string()).unwrap();
        assert_eq!(dni1, dni2);
    }

    #[test]
    fn test_dni_clone() {
        let dni1 = Dni::new("12345678-1".to_string()).unwrap();
        let dni2 = dni1.clone();
        assert_eq!(dni1, dni2);
    }

    #[test]
    fn test_dni_incorrect_format_missing_dash() {
        let result = Dni::new("123456781".to_string());
        assert!(matches!(result, Err(DniError::IncorrectFormat)));
    }

    #[test]
    fn test_dni_incorrect_format_too_short() {
        let result = Dni::new("1234567-1".to_string());
        assert!(matches!(result, Err(DniError::IncorrectFormat)));
    }

    #[test]
    fn test_dni_incorrect_format_too_long() {
        let result = Dni::new("123456789-1".to_string());
        assert!(matches!(result, Err(DniError::IncorrectFormat)));
    }

    #[test]
    fn test_dni_incorrect_format_invalid_verification_char() {
        let result = Dni::new("12345678-Z".to_string());
        assert!(matches!(result, Err(DniError::IncorrectFormat)));
    }

    #[test]
    fn test_dni_incorrect_format_lowercase_letter() {
        let result = Dni::new("12345678-k".to_string());
        assert!(matches!(result, Err(DniError::IncorrectFormat)));
    }

    #[test]
    fn test_dni_incorrect_format_letter_in_digits() {
        let result = Dni::new("1234567A-1".to_string());
        assert!(matches!(result, Err(DniError::IncorrectFormat)));
    }

    #[test]
    fn test_dni_incorrect_format_special_characters() {
        let result = Dni::new("12345678-@".to_string());
        assert!(matches!(result, Err(DniError::IncorrectFormat)));
    }

    #[test]
    fn test_dni_incorrect_format_empty_string() {
        let result = Dni::new("".to_string());
        assert!(matches!(result, Err(DniError::IncorrectFormat)));
    }

    #[test]
    fn test_dni_incorrect_format_only_whitespace() {
        let result = Dni::new("   ".to_string());
        assert!(matches!(result, Err(DniError::IncorrectFormat)));
    }

    #[test]
    fn test_dni_incorrect_verification_digit() {
        let result = Dni::new("12345678-9".to_string());
        assert!(matches!(
            result,
            Err(DniError::IncorrectValidationDigit { .. })
        ));
    }

    #[test]
    fn test_dni_incorrect_verification_digit_shows_expected() {
        let result = Dni::new("12345678-9".to_string());
        match result {
            Err(DniError::IncorrectValidationDigit {
                expected_numeric,
                expected_alpha,
                received,
            }) => {
                assert_eq!(received, "9");
                assert!(!expected_numeric.is_empty());
                assert!(!expected_alpha.is_empty());
            }
            _ => panic!("Expected IncorrectValidationDigit error"),
        }
    }

    #[test]
    fn test_dni_verification_algorithm_numeric() {
        let result = Dni::new("00000001-4".to_string());
        assert!(result.is_ok(), "DNI 00000001-4 should be valid");
    }

    #[test]
    fn test_dni_verification_algorithm_alpha() {
        let result = Dni::new("00000001-I".to_string());
        assert!(
            result.is_ok(),
            "DNI 00000001-I should be valid (alpha equivalent)"
        );
    }

    #[test]
    fn test_dni_known_valid_examples() {
        let valid_dnis = vec!["00000001-4", "00000001-I", "12345678-1"];

        for dni_str in valid_dnis {
            let result = Dni::new(dni_str.to_string());
            assert!(
                result.is_ok(),
                "DNI {} should be valid, but got error: {:?}",
                dni_str,
                result.err()
            );
        }
    }

    #[test]
    fn test_dni_hash_consistency() {
        use std::collections::HashSet;

        let dni1 = Dni::new("12345678-1".to_string()).unwrap();
        let dni2 = dni1.clone();

        let mut set = HashSet::new();
        set.insert(dni1);
        assert!(set.contains(&dni2));
    }

    #[test]
    fn test_dni_value_object_semantics() {
        let dni1 = Dni::new("12345678-1".to_string()).unwrap();
        let dni2 = Dni::new("12345678-1".to_string()).unwrap();
        let dni3 = Dni::new("87654321-0".to_string()).unwrap();

        assert_eq!(dni1, dni2);
        assert_ne!(dni1, dni3);
    }
}
