use chrono::{Datelike, Local, NaiveDateTime, Timelike};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

/// A datetime value object representing a calendar date with time (year, month, day, hour, minute, second).
///
/// `DateTime` is an immutable value object that wraps `chrono::NaiveDateTime` to provide
/// a DDD-compliant datetime representation with seconds precision.
///
/// # Examples
///
/// ```
/// use education_platform_common::DateTime;
///
/// let dt = DateTime::new(2024, 3, 15, 10, 30, 45).unwrap();
/// assert_eq!(dt.year(), 2024);
/// assert_eq!(dt.month(), 3);
/// assert_eq!(dt.day(), 15);
/// assert_eq!(dt.hour(), 10);
/// assert_eq!(dt.minute(), 30);
/// assert_eq!(dt.second(), 45);
///
/// // Format as ISO 8601
/// assert_eq!(dt.to_string(), "2024-03-15T10:30:45");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DateTime {
    inner: NaiveDateTime,
}

impl DateTime {
    /// Creates a new `DateTime` with validation.
    ///
    /// # Errors
    ///
    /// Returns `DateTimeError::InvalidDateTime` if the components don't form a valid datetime.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::DateTime;
    ///
    /// let dt = DateTime::new(2024, 12, 25, 14, 30, 0).unwrap();
    /// assert_eq!(dt.year(), 2024);
    /// assert_eq!(dt.hour(), 14);
    ///
    /// // Invalid hour
    /// assert!(DateTime::new(2024, 1, 1, 25, 0, 0).is_err());
    ///
    /// // Invalid month
    /// assert!(DateTime::new(2024, 13, 1, 0, 0, 0).is_err());
    /// ```
    pub fn new(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> Result<Self, DateTimeError> {
        NaiveDateTime::parse_from_str(
            &format!(
                "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
                year, month, day, hour, minute, second
            ),
            "%Y-%m-%dT%H:%M:%S",
        )
        .map(|inner| Self { inner })
        .map_err(|_| DateTimeError::InvalidDateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
        })
    }

    /// Creates a `DateTime` from a `chrono::NaiveDateTime`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::NaiveDateTime;
    /// use education_platform_common::DateTime;
    ///
    /// let naive = NaiveDateTime::parse_from_str("2024-06-15T10:30:00", "%Y-%m-%dT%H:%M:%S").unwrap();
    /// let dt = DateTime::from_naive_datetime(naive);
    /// assert_eq!(dt.year(), 2024);
    /// assert_eq!(dt.hour(), 10);
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_naive_datetime(datetime: NaiveDateTime) -> Self {
        Self { inner: datetime }
    }

    /// Creates a `DateTime` from an ISO 8601 string (YYYY-MM-DDTHH:MM:SS).
    ///
    /// # Errors
    ///
    /// Returns `DateTimeError::ParseError` if the string is not in valid ISO 8601 format.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::DateTime;
    ///
    /// let dt = DateTime::from_iso("2024-03-15T10:30:45").unwrap();
    /// assert_eq!(dt.year(), 2024);
    /// assert_eq!(dt.hour(), 10);
    /// assert_eq!(dt.minute(), 30);
    /// assert_eq!(dt.second(), 45);
    ///
    /// // Invalid format
    /// assert!(DateTime::from_iso("2024-03-15 10:30:45").is_err());
    /// ```
    pub fn from_iso(s: &str) -> Result<Self, DateTimeError> {
        s.parse()
    }

    /// Returns the current datetime based on system time.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::DateTime;
    ///
    /// let now = DateTime::now();
    /// assert!(now.year() >= 2024);
    /// ```
    #[must_use]
    pub fn now() -> Self {
        Self {
            inner: Local::now().naive_local(),
        }
    }

    /// Returns the underlying `chrono::NaiveDateTime`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::Datelike;
    /// use education_platform_common::DateTime;
    ///
    /// let dt = DateTime::new(2024, 6, 15, 10, 0, 0).unwrap();
    /// let naive = dt.as_naive_datetime();
    /// assert_eq!(naive.year(), 2024);
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_naive_datetime(&self) -> &NaiveDateTime {
        &self.inner
    }

    /// Returns the year component.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::DateTime;
    ///
    /// let dt = DateTime::new(2024, 6, 15, 10, 30, 0).unwrap();
    /// assert_eq!(dt.year(), 2024);
    /// ```
    #[inline]
    #[must_use]
    pub fn year(&self) -> i32 {
        self.inner.year()
    }

    /// Returns the month component (1-12).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::DateTime;
    ///
    /// let dt = DateTime::new(2024, 6, 15, 10, 30, 0).unwrap();
    /// assert_eq!(dt.month(), 6);
    /// ```
    #[inline]
    #[must_use]
    pub fn month(&self) -> u32 {
        self.inner.month()
    }

    /// Returns the day component (1-31).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::DateTime;
    ///
    /// let dt = DateTime::new(2024, 6, 15, 10, 30, 0).unwrap();
    /// assert_eq!(dt.day(), 15);
    /// ```
    #[inline]
    #[must_use]
    pub fn day(&self) -> u32 {
        self.inner.day()
    }

    /// Returns the hour component (0-23).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::DateTime;
    ///
    /// let dt = DateTime::new(2024, 6, 15, 14, 30, 0).unwrap();
    /// assert_eq!(dt.hour(), 14);
    /// ```
    #[inline]
    #[must_use]
    pub fn hour(&self) -> u32 {
        self.inner.hour()
    }

    /// Returns the minute component (0-59).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::DateTime;
    ///
    /// let dt = DateTime::new(2024, 6, 15, 10, 45, 0).unwrap();
    /// assert_eq!(dt.minute(), 45);
    /// ```
    #[inline]
    #[must_use]
    pub fn minute(&self) -> u32 {
        self.inner.minute()
    }

    /// Returns the second component (0-59).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::DateTime;
    ///
    /// let dt = DateTime::new(2024, 6, 15, 10, 30, 55).unwrap();
    /// assert_eq!(dt.second(), 55);
    /// ```
    #[inline]
    #[must_use]
    pub fn second(&self) -> u32 {
        self.inner.second()
    }

    /// Returns the number of seconds between two datetimes.
    ///
    /// Returns a positive number if `other` is after `self`, negative otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::DateTime;
    ///
    /// let start = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
    /// let end = DateTime::new(2024, 1, 1, 10, 0, 30).unwrap();
    /// assert_eq!(start.seconds_until(&end), 30);
    /// assert_eq!(end.seconds_until(&start), -30);
    ///
    /// let next_hour = DateTime::new(2024, 1, 1, 11, 0, 0).unwrap();
    /// assert_eq!(start.seconds_until(&next_hour), 3600);
    /// ```
    #[must_use]
    pub fn seconds_until(&self, other: &Self) -> i64 {
        (other.inner - self.inner).num_seconds()
    }

    /// Adds a number of seconds to the datetime.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::DateTime;
    ///
    /// let dt = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
    /// let later = dt.add_seconds(90);
    /// assert_eq!(later.minute(), 1);
    /// assert_eq!(later.second(), 30);
    /// ```
    #[must_use]
    pub fn add_seconds(&self, seconds: i64) -> Self {
        Self {
            inner: self.inner + chrono::Duration::seconds(seconds),
        }
    }

    /// Subtracts a number of seconds from the datetime.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::DateTime;
    ///
    /// let dt = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
    /// let earlier = dt.sub_seconds(90);
    /// assert_eq!(earlier.hour(), 9);
    /// assert_eq!(earlier.minute(), 58);
    /// assert_eq!(earlier.second(), 30);
    /// ```
    #[must_use]
    pub fn sub_seconds(&self, seconds: i64) -> Self {
        self.add_seconds(-seconds)
    }

    /// Formats the datetime as ISO 8601 (YYYY-MM-DDTHH:MM:SS).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::DateTime;
    ///
    /// let dt = DateTime::new(2024, 3, 5, 9, 5, 3).unwrap();
    /// assert_eq!(dt.format_iso(), "2024-03-05T09:05:03");
    /// ```
    #[must_use]
    pub fn format_iso(&self) -> String {
        self.inner.format("%Y-%m-%dT%H:%M:%S").to_string()
    }

    /// Formats the datetime using a custom format string.
    ///
    /// Uses chrono's strftime syntax.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::DateTime;
    ///
    /// let dt = DateTime::new(2024, 3, 15, 14, 30, 0).unwrap();
    /// assert_eq!(dt.format("%d/%m/%Y %H:%M"), "15/03/2024 14:30");
    /// ```
    #[must_use]
    pub fn format(&self, fmt: &str) -> String {
        self.inner.format(fmt).to_string()
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_iso())
    }
}

impl FromStr for DateTime {
    type Err = DateTimeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S")
            .map(|inner| Self { inner })
            .map_err(|e| DateTimeError::ParseError {
                value: s.to_string(),
                reason: e.to_string(),
            })
    }
}

impl From<NaiveDateTime> for DateTime {
    fn from(datetime: NaiveDateTime) -> Self {
        Self::from_naive_datetime(datetime)
    }
}

impl From<DateTime> for NaiveDateTime {
    fn from(datetime: DateTime) -> Self {
        datetime.inner
    }
}

/// Error types for DateTime operations.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DateTimeError {
    #[error("Invalid datetime: {year}-{month}-{day}T{hour}:{minute}:{second}")]
    InvalidDateTime {
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    },

    #[error("Failed to parse datetime from '{value}': {reason}")]
    ParseError { value: String, reason: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    mod constructors {
        use super::*;

        #[test]
        fn test_new_creates_valid_datetime() {
            let dt = DateTime::new(2024, 6, 15, 10, 30, 45).unwrap();
            assert_eq!(dt.year(), 2024);
            assert_eq!(dt.month(), 6);
            assert_eq!(dt.day(), 15);
            assert_eq!(dt.hour(), 10);
            assert_eq!(dt.minute(), 30);
            assert_eq!(dt.second(), 45);
        }

        #[test]
        fn test_new_with_midnight() {
            let dt = DateTime::new(2024, 1, 1, 0, 0, 0).unwrap();
            assert_eq!(dt.hour(), 0);
            assert_eq!(dt.minute(), 0);
            assert_eq!(dt.second(), 0);
        }

        #[test]
        fn test_new_with_end_of_day() {
            let dt = DateTime::new(2024, 1, 1, 23, 59, 59).unwrap();
            assert_eq!(dt.hour(), 23);
            assert_eq!(dt.minute(), 59);
            assert_eq!(dt.second(), 59);
        }

        #[test]
        fn test_new_rejects_invalid_hour() {
            let result = DateTime::new(2024, 1, 1, 24, 0, 0);
            assert!(matches!(result, Err(DateTimeError::InvalidDateTime { .. })));
        }

        #[test]
        fn test_new_rejects_invalid_minute() {
            let result = DateTime::new(2024, 1, 1, 10, 60, 0);
            assert!(matches!(result, Err(DateTimeError::InvalidDateTime { .. })));
        }

        #[test]
        fn test_new_rejects_invalid_second() {
            // Note: chrono allows second 60 for leap seconds, so we use 61
            let result = DateTime::new(2024, 1, 1, 10, 30, 61);
            assert!(matches!(result, Err(DateTimeError::InvalidDateTime { .. })));
        }

        #[test]
        fn test_new_rejects_invalid_month() {
            let result = DateTime::new(2024, 13, 1, 10, 0, 0);
            assert!(matches!(result, Err(DateTimeError::InvalidDateTime { .. })));
        }

        #[test]
        fn test_new_rejects_invalid_day() {
            let result = DateTime::new(2024, 2, 30, 10, 0, 0);
            assert!(matches!(result, Err(DateTimeError::InvalidDateTime { .. })));
        }

        #[test]
        fn test_now_returns_valid_datetime() {
            let now = DateTime::now();
            assert!(now.year() >= 2024);
            assert!((1..=12).contains(&now.month()));
            assert!((1..=31).contains(&now.day()));
            assert!(now.hour() < 24);
            assert!(now.minute() < 60);
            assert!(now.second() < 60);
        }

        #[test]
        fn test_from_iso_valid() {
            let dt = DateTime::from_iso("2024-03-15T10:30:45").unwrap();
            assert_eq!(dt.year(), 2024);
            assert_eq!(dt.month(), 3);
            assert_eq!(dt.day(), 15);
            assert_eq!(dt.hour(), 10);
            assert_eq!(dt.minute(), 30);
            assert_eq!(dt.second(), 45);
        }

        #[test]
        fn test_from_iso_invalid_format() {
            let result = DateTime::from_iso("2024-03-15 10:30:45");
            assert!(matches!(result, Err(DateTimeError::ParseError { .. })));
        }
    }

    mod seconds_until {
        use super::*;

        #[test]
        fn test_seconds_until_same_datetime() {
            let dt = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
            assert_eq!(dt.seconds_until(&dt), 0);
        }

        #[test]
        fn test_seconds_until_30_seconds_later() {
            let start = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
            let end = DateTime::new(2024, 1, 1, 10, 0, 30).unwrap();
            assert_eq!(start.seconds_until(&end), 30);
        }

        #[test]
        fn test_seconds_until_negative() {
            let start = DateTime::new(2024, 1, 1, 10, 0, 30).unwrap();
            let end = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
            assert_eq!(start.seconds_until(&end), -30);
        }

        #[test]
        fn test_seconds_until_one_hour() {
            let start = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
            let end = DateTime::new(2024, 1, 1, 11, 0, 0).unwrap();
            assert_eq!(start.seconds_until(&end), 3600);
        }

        #[test]
        fn test_seconds_until_one_day() {
            let start = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
            let end = DateTime::new(2024, 1, 2, 10, 0, 0).unwrap();
            assert_eq!(start.seconds_until(&end), 86400);
        }

        #[test]
        fn test_seconds_until_complex_difference() {
            let start = DateTime::new(2024, 1, 1, 10, 30, 15).unwrap();
            let end = DateTime::new(2024, 1, 1, 11, 45, 30).unwrap();
            // 1 hour 15 minutes 15 seconds = 3600 + 900 + 15 = 4515 seconds
            assert_eq!(start.seconds_until(&end), 4515);
        }
    }

    mod arithmetic {
        use super::*;

        #[test]
        fn test_add_seconds() {
            let dt = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
            let later = dt.add_seconds(90);
            assert_eq!(later.minute(), 1);
            assert_eq!(later.second(), 30);
        }

        #[test]
        fn test_add_seconds_crosses_hour() {
            let dt = DateTime::new(2024, 1, 1, 10, 59, 30).unwrap();
            let later = dt.add_seconds(60);
            assert_eq!(later.hour(), 11);
            assert_eq!(later.minute(), 0);
            assert_eq!(later.second(), 30);
        }

        #[test]
        fn test_sub_seconds() {
            let dt = DateTime::new(2024, 1, 1, 10, 1, 30).unwrap();
            let earlier = dt.sub_seconds(90);
            assert_eq!(earlier.minute(), 0);
            assert_eq!(earlier.second(), 0);
        }

        #[test]
        fn test_sub_seconds_crosses_hour() {
            let dt = DateTime::new(2024, 1, 1, 10, 0, 30).unwrap();
            let earlier = dt.sub_seconds(60);
            assert_eq!(earlier.hour(), 9);
            assert_eq!(earlier.minute(), 59);
            assert_eq!(earlier.second(), 30);
        }
    }

    mod formatting {
        use super::*;

        #[test]
        fn test_format_iso() {
            let dt = DateTime::new(2024, 3, 5, 9, 5, 3).unwrap();
            assert_eq!(dt.format_iso(), "2024-03-05T09:05:03");
        }

        #[test]
        fn test_display_trait() {
            let dt = DateTime::new(2024, 12, 25, 14, 30, 0).unwrap();
            assert_eq!(format!("{}", dt), "2024-12-25T14:30:00");
        }

        #[test]
        fn test_custom_format() {
            let dt = DateTime::new(2024, 3, 15, 14, 30, 0).unwrap();
            assert_eq!(dt.format("%d/%m/%Y %H:%M"), "15/03/2024 14:30");
        }
    }

    mod value_object_semantics {
        use super::*;

        #[test]
        fn test_equality_for_same_datetime() {
            let dt1 = DateTime::new(2024, 6, 15, 10, 30, 0).unwrap();
            let dt2 = DateTime::new(2024, 6, 15, 10, 30, 0).unwrap();
            assert_eq!(dt1, dt2);
        }

        #[test]
        fn test_inequality_for_different_seconds() {
            let dt1 = DateTime::new(2024, 6, 15, 10, 30, 0).unwrap();
            let dt2 = DateTime::new(2024, 6, 15, 10, 30, 1).unwrap();
            assert_ne!(dt1, dt2);
        }

        #[test]
        fn test_ordering() {
            let dt1 = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
            let dt2 = DateTime::new(2024, 1, 1, 10, 0, 1).unwrap();
            let dt3 = DateTime::new(2024, 1, 1, 10, 1, 0).unwrap();
            assert!(dt1 < dt2);
            assert!(dt2 < dt3);
            assert!(dt1 < dt3);
        }

        #[test]
        fn test_hash_consistency() {
            use std::collections::HashSet;
            let mut set = HashSet::new();
            let dt1 = DateTime::new(2024, 6, 15, 10, 30, 0).unwrap();
            let dt2 = DateTime::new(2024, 6, 15, 10, 30, 0).unwrap();
            set.insert(dt1);
            assert!(set.contains(&dt2));
        }
    }

    mod conversions {
        use super::*;

        #[test]
        fn test_from_naive_datetime() {
            let naive =
                NaiveDateTime::parse_from_str("2024-06-15T10:30:00", "%Y-%m-%dT%H:%M:%S").unwrap();
            let dt: DateTime = naive.into();
            assert_eq!(dt.year(), 2024);
            assert_eq!(dt.hour(), 10);
        }

        #[test]
        fn test_into_naive_datetime() {
            let dt = DateTime::new(2024, 6, 15, 10, 30, 45).unwrap();
            let naive: NaiveDateTime = dt.into();
            assert_eq!(naive.year(), 2024);
            assert_eq!(naive.hour(), 10);
            assert_eq!(naive.second(), 45);
        }
    }
}
