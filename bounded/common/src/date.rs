use chrono::{Datelike, Local, NaiveDate, Weekday};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

/// A date value object representing a calendar date (year, month, day).
///
/// `Date` is an immutable value object that wraps `chrono::NaiveDate` to provide
/// a DDD-compliant date representation. It provides validation for proper calendar
/// dates including leap year handling.
///
/// # Examples
///
/// ```
/// use education_platform_common::Date;
///
/// let date = Date::new(2024, 3, 15).unwrap();
/// assert_eq!(date.year(), 2024);
/// assert_eq!(date.month(), 3);
/// assert_eq!(date.day(), 15);
///
/// // Leap year validation
/// let leap_date = Date::new(2024, 2, 29).unwrap();
/// assert!(Date::new(2023, 2, 29).is_err());
///
/// // Format as ISO 8601
/// assert_eq!(date.to_string(), "2024-03-15");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Date {
    inner: NaiveDate,
}

impl Date {
    /// Creates a new `Date` with validation.
    ///
    /// # Errors
    ///
    /// Returns `DateError::InvalidDate` if the date components don't form a valid date.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Date;
    ///
    /// let date = Date::new(2024, 12, 25).unwrap();
    /// assert_eq!(date.year(), 2024);
    ///
    /// // Invalid month
    /// assert!(Date::new(2024, 13, 1).is_err());
    ///
    /// // Invalid day for February in non-leap year
    /// assert!(Date::new(2023, 2, 29).is_err());
    ///
    /// // Valid leap year date
    /// assert!(Date::new(2024, 2, 29).is_ok());
    /// ```
    pub fn new(year: i32, month: u32, day: u32) -> Result<Self, DateError> {
        NaiveDate::from_ymd_opt(year, month, day)
            .map(|inner| Self { inner })
            .ok_or(DateError::InvalidDate { year, month, day })
    }

    /// Creates a `Date` from a `chrono::NaiveDate`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use education_platform_common::Date;
    ///
    /// let naive = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
    /// let date = Date::from_naive_date(naive);
    /// assert_eq!(date.year(), 2024);
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_naive_date(date: NaiveDate) -> Self {
        Self { inner: date }
    }

    /// Creates a `Date` from an ISO 8601 string (YYYY-MM-DD).
    ///
    /// # Errors
    ///
    /// Returns `DateError::ParseError` if the string is not in valid ISO 8601 format.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Date;
    ///
    /// let date = Date::from_iso("2024-03-15").unwrap();
    /// assert_eq!(date.year(), 2024);
    /// assert_eq!(date.month(), 3);
    /// assert_eq!(date.day(), 15);
    ///
    /// // Invalid format
    /// assert!(Date::from_iso("15/03/2024").is_err());
    /// assert!(Date::from_iso("2024-13-01").is_err());
    /// ```
    pub fn from_iso(s: &str) -> Result<Self, DateError> {
        s.parse()
    }

    /// Returns the current date based on system time.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Date;
    ///
    /// let today = Date::today();
    /// assert!(today.year() >= 2024);
    /// ```
    #[must_use]
    pub fn today() -> Self {
        Self {
            inner: Local::now().date_naive(),
        }
    }

    /// Returns the underlying `chrono::NaiveDate`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::Datelike;
    /// use education_platform_common::Date;
    ///
    /// let date = Date::new(2024, 6, 15).unwrap();
    /// let naive = date.as_naive_date();
    /// assert_eq!(naive.year(), 2024);
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_naive_date(&self) -> &NaiveDate {
        &self.inner
    }

    /// Returns the year component.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Date;
    ///
    /// let date = Date::new(2024, 6, 15).unwrap();
    /// assert_eq!(date.year(), 2024);
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
    /// use education_platform_common::Date;
    ///
    /// let date = Date::new(2024, 6, 15).unwrap();
    /// assert_eq!(date.month(), 6);
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
    /// use education_platform_common::Date;
    ///
    /// let date = Date::new(2024, 6, 15).unwrap();
    /// assert_eq!(date.day(), 15);
    /// ```
    #[inline]
    #[must_use]
    pub fn day(&self) -> u32 {
        self.inner.day()
    }

    /// Returns true if the given year is a leap year.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Date;
    ///
    /// assert!(Date::is_leap_year(2024));
    /// assert!(Date::is_leap_year(2000));
    /// assert!(!Date::is_leap_year(2023));
    /// assert!(!Date::is_leap_year(1900));
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    /// Returns the number of days in the given month for the given year.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Date;
    ///
    /// assert_eq!(Date::days_in_month(2024, 2), 29); // Leap year
    /// assert_eq!(Date::days_in_month(2023, 2), 28); // Non-leap year
    /// assert_eq!(Date::days_in_month(2024, 1), 31);
    /// assert_eq!(Date::days_in_month(2024, 4), 30);
    /// ```
    #[must_use]
    pub const fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if Self::is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            _ => 0,
        }
    }

    /// Returns the day of the year (1-366).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Date;
    ///
    /// let jan_first = Date::new(2024, 1, 1).unwrap();
    /// assert_eq!(jan_first.day_of_year(), 1);
    ///
    /// let dec_last = Date::new(2024, 12, 31).unwrap();
    /// assert_eq!(dec_last.day_of_year(), 366); // Leap year
    ///
    /// let dec_last_non_leap = Date::new(2023, 12, 31).unwrap();
    /// assert_eq!(dec_last_non_leap.day_of_year(), 365);
    /// ```
    #[inline]
    #[must_use]
    pub fn day_of_year(&self) -> u32 {
        self.inner.ordinal()
    }

    /// Returns the ISO 8601 week number (1-53).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Date;
    ///
    /// let date = Date::new(2024, 1, 1).unwrap();
    /// assert_eq!(date.week_number(), 1);
    /// ```
    #[inline]
    #[must_use]
    pub fn week_number(&self) -> u32 {
        self.inner.iso_week().week()
    }

    /// Returns the day of the week (1 = Monday, 7 = Sunday).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Date;
    ///
    /// let monday = Date::new(2024, 12, 9).unwrap();
    /// assert_eq!(monday.weekday(), 1);
    ///
    /// let sunday = Date::new(2024, 12, 15).unwrap();
    /// assert_eq!(sunday.weekday(), 7);
    /// ```
    #[must_use]
    pub fn weekday(&self) -> u32 {
        match self.inner.weekday() {
            Weekday::Mon => 1,
            Weekday::Tue => 2,
            Weekday::Wed => 3,
            Weekday::Thu => 4,
            Weekday::Fri => 5,
            Weekday::Sat => 6,
            Weekday::Sun => 7,
        }
    }

    /// Returns the weekday as a `chrono::Weekday`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::Weekday;
    /// use education_platform_common::Date;
    ///
    /// let monday = Date::new(2024, 12, 9).unwrap();
    /// assert_eq!(monday.weekday_chrono(), Weekday::Mon);
    /// ```
    #[inline]
    #[must_use]
    pub fn weekday_chrono(&self) -> Weekday {
        self.inner.weekday()
    }

    /// Adds a number of days to the date.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Date;
    ///
    /// let date = Date::new(2024, 1, 30).unwrap();
    /// let new_date = date.add_days(5);
    /// assert_eq!(new_date.month(), 2);
    /// assert_eq!(new_date.day(), 4);
    ///
    /// // Handles year boundaries
    /// let dec = Date::new(2024, 12, 30).unwrap();
    /// let jan = dec.add_days(5);
    /// assert_eq!(jan.year(), 2025);
    /// assert_eq!(jan.month(), 1);
    /// ```
    #[must_use]
    pub fn add_days(&self, days: i64) -> Self {
        Self {
            inner: self.inner + chrono::Duration::days(days),
        }
    }

    /// Subtracts a number of days from the date.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Date;
    ///
    /// let date = Date::new(2024, 2, 5).unwrap();
    /// let new_date = date.sub_days(10);
    /// assert_eq!(new_date.month(), 1);
    /// assert_eq!(new_date.day(), 26);
    /// ```
    #[must_use]
    pub fn sub_days(&self, days: i64) -> Self {
        self.add_days(-days)
    }

    /// Returns the number of days between two dates.
    ///
    /// Returns a positive number if `other` is after `self`, negative otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Date;
    ///
    /// let start = Date::new(2024, 1, 1).unwrap();
    /// let end = Date::new(2024, 1, 10).unwrap();
    /// assert_eq!(start.days_until(&end), 9);
    /// assert_eq!(end.days_until(&start), -9);
    /// ```
    #[must_use]
    pub fn days_until(&self, other: &Self) -> i64 {
        (other.inner - self.inner).num_days()
    }

    /// Formats the date as ISO 8601 (YYYY-MM-DD).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Date;
    ///
    /// let date = Date::new(2024, 3, 5).unwrap();
    /// assert_eq!(date.format_iso(), "2024-03-05");
    /// ```
    #[must_use]
    pub fn format_iso(&self) -> String {
        self.inner.format("%Y-%m-%d").to_string()
    }

    /// Formats the date using a custom format string.
    ///
    /// Uses chrono's strftime syntax.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Date;
    ///
    /// let date = Date::new(2024, 3, 15).unwrap();
    /// assert_eq!(date.format("%d/%m/%Y"), "15/03/2024");
    /// assert_eq!(date.format("%B %d, %Y"), "March 15, 2024");
    /// ```
    #[must_use]
    pub fn format(&self, fmt: &str) -> String {
        self.inner.format(fmt).to_string()
    }

    /// Returns true if the date is in the past relative to today.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Date;
    ///
    /// let past = Date::new(2020, 1, 1).unwrap();
    /// assert!(past.is_past());
    /// ```
    #[must_use]
    pub fn is_past(&self) -> bool {
        *self < Self::today()
    }

    /// Returns true if the date is in the future relative to today.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Date;
    ///
    /// let future = Date::new(2099, 12, 31).unwrap();
    /// assert!(future.is_future());
    /// ```
    #[must_use]
    pub fn is_future(&self) -> bool {
        *self > Self::today()
    }

    /// Returns true if this date is today.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Date;
    ///
    /// let today = Date::today();
    /// assert!(today.is_today());
    ///
    /// let yesterday = today.sub_days(1);
    /// assert!(!yesterday.is_today());
    /// ```
    #[must_use]
    pub fn is_today(&self) -> bool {
        *self == Self::today()
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_iso())
    }
}

impl FromStr for Date {
    type Err = DateError;

    /// Parses a date from ISO 8601 format (YYYY-MM-DD).
    ///
    /// # Errors
    ///
    /// Returns `DateError::ParseError` if the string is not in valid format.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Date;
    ///
    /// let date: Date = "2024-03-15".parse().unwrap();
    /// assert_eq!(date.year(), 2024);
    /// assert_eq!(date.month(), 3);
    /// assert_eq!(date.day(), 15);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map(|inner| Self { inner })
            .map_err(|e| DateError::ParseError {
                value: s.to_string(),
                reason: e.to_string(),
            })
    }
}

impl From<NaiveDate> for Date {
    fn from(date: NaiveDate) -> Self {
        Self::from_naive_date(date)
    }
}

impl From<Date> for NaiveDate {
    fn from(date: Date) -> Self {
        date.inner
    }
}

/// Error types for Date operations.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DateError {
    #[error("Invalid date: {year}-{month}-{day}")]
    InvalidDate { year: i32, month: u32, day: u32 },

    #[error("Failed to parse date from '{value}': {reason}")]
    ParseError { value: String, reason: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    mod constructors {
        use super::*;

        #[test]
        fn test_new_creates_valid_date() {
            let date = Date::new(2024, 6, 15).unwrap();
            assert_eq!(date.year(), 2024);
            assert_eq!(date.month(), 6);
            assert_eq!(date.day(), 15);
        }

        #[test]
        fn test_new_with_first_day_of_year() {
            let date = Date::new(2024, 1, 1).unwrap();
            assert_eq!(date.year(), 2024);
            assert_eq!(date.month(), 1);
            assert_eq!(date.day(), 1);
        }

        #[test]
        fn test_new_with_last_day_of_year() {
            let date = Date::new(2024, 12, 31).unwrap();
            assert_eq!(date.year(), 2024);
            assert_eq!(date.month(), 12);
            assert_eq!(date.day(), 31);
        }

        #[test]
        fn test_new_rejects_month_zero() {
            let result = Date::new(2024, 0, 15);
            assert!(matches!(result, Err(DateError::InvalidDate { .. })));
        }

        #[test]
        fn test_new_rejects_month_13() {
            let result = Date::new(2024, 13, 15);
            assert!(matches!(result, Err(DateError::InvalidDate { .. })));
        }

        #[test]
        fn test_new_rejects_day_zero() {
            let result = Date::new(2024, 6, 0);
            assert!(matches!(result, Err(DateError::InvalidDate { .. })));
        }

        #[test]
        fn test_new_rejects_day_32() {
            let result = Date::new(2024, 1, 32);
            assert!(matches!(result, Err(DateError::InvalidDate { .. })));
        }

        #[test]
        fn test_new_rejects_feb_30() {
            let result = Date::new(2024, 2, 30);
            assert!(matches!(result, Err(DateError::InvalidDate { .. })));
        }

        #[test]
        fn test_new_accepts_feb_29_leap_year() {
            let date = Date::new(2024, 2, 29).unwrap();
            assert_eq!(date.day(), 29);
        }

        #[test]
        fn test_new_rejects_feb_29_non_leap_year() {
            let result = Date::new(2023, 2, 29);
            assert!(matches!(result, Err(DateError::InvalidDate { .. })));
        }

        #[test]
        fn test_today_returns_valid_date() {
            let today = Date::today();
            assert!(today.year() >= 2024);
            assert!((1..=12).contains(&today.month()));
            assert!((1..=31).contains(&today.day()));
        }

        #[test]
        fn test_from_naive_date() {
            let naive = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
            let date = Date::from_naive_date(naive);
            assert_eq!(date.year(), 2024);
            assert_eq!(date.month(), 6);
            assert_eq!(date.day(), 15);
        }

        #[test]
        fn test_from_iso_valid() {
            let date = Date::from_iso("2024-03-15").unwrap();
            assert_eq!(date.year(), 2024);
            assert_eq!(date.month(), 3);
            assert_eq!(date.day(), 15);
        }

        #[test]
        fn test_from_iso_with_leading_zeros() {
            let date = Date::from_iso("2024-01-05").unwrap();
            assert_eq!(date.month(), 1);
            assert_eq!(date.day(), 5);
        }

        #[test]
        fn test_from_iso_invalid_format_slash() {
            let result = Date::from_iso("15/03/2024");
            assert!(matches!(result, Err(DateError::ParseError { .. })));
        }

        #[test]
        fn test_from_iso_invalid_month() {
            let result = Date::from_iso("2024-13-01");
            assert!(matches!(result, Err(DateError::ParseError { .. })));
        }

        #[test]
        fn test_from_iso_invalid_day() {
            let result = Date::from_iso("2024-02-30");
            assert!(matches!(result, Err(DateError::ParseError { .. })));
        }

        #[test]
        fn test_from_iso_empty_string() {
            let result = Date::from_iso("");
            assert!(matches!(result, Err(DateError::ParseError { .. })));
        }

        #[test]
        fn test_from_iso_roundtrip() {
            let original = Date::new(2024, 6, 15).unwrap();
            let parsed = Date::from_iso(&original.format_iso()).unwrap();
            assert_eq!(original, parsed);
        }
    }

    mod leap_year {
        use super::*;

        #[test]
        fn test_is_leap_year_divisible_by_4() {
            assert!(Date::is_leap_year(2024));
            assert!(Date::is_leap_year(2028));
        }

        #[test]
        fn test_is_leap_year_century_not_leap() {
            assert!(!Date::is_leap_year(1900));
            assert!(!Date::is_leap_year(2100));
        }

        #[test]
        fn test_is_leap_year_400_is_leap() {
            assert!(Date::is_leap_year(2000));
            assert!(Date::is_leap_year(1600));
        }

        #[test]
        fn test_is_not_leap_year() {
            assert!(!Date::is_leap_year(2023));
            assert!(!Date::is_leap_year(2021));
        }
    }

    mod days_in_month {
        use super::*;

        #[test]
        fn test_january_has_31_days() {
            assert_eq!(Date::days_in_month(2024, 1), 31);
        }

        #[test]
        fn test_february_leap_year_has_29_days() {
            assert_eq!(Date::days_in_month(2024, 2), 29);
        }

        #[test]
        fn test_february_non_leap_year_has_28_days() {
            assert_eq!(Date::days_in_month(2023, 2), 28);
        }

        #[test]
        fn test_april_has_30_days() {
            assert_eq!(Date::days_in_month(2024, 4), 30);
        }

        #[test]
        fn test_december_has_31_days() {
            assert_eq!(Date::days_in_month(2024, 12), 31);
        }

        #[test]
        fn test_invalid_month_returns_zero() {
            assert_eq!(Date::days_in_month(2024, 0), 0);
            assert_eq!(Date::days_in_month(2024, 13), 0);
        }
    }

    mod day_of_year {
        use super::*;

        #[test]
        fn test_january_first() {
            let date = Date::new(2024, 1, 1).unwrap();
            assert_eq!(date.day_of_year(), 1);
        }

        #[test]
        fn test_january_31() {
            let date = Date::new(2024, 1, 31).unwrap();
            assert_eq!(date.day_of_year(), 31);
        }

        #[test]
        fn test_february_first() {
            let date = Date::new(2024, 2, 1).unwrap();
            assert_eq!(date.day_of_year(), 32);
        }

        #[test]
        fn test_december_31_leap_year() {
            let date = Date::new(2024, 12, 31).unwrap();
            assert_eq!(date.day_of_year(), 366);
        }

        #[test]
        fn test_december_31_non_leap_year() {
            let date = Date::new(2023, 12, 31).unwrap();
            assert_eq!(date.day_of_year(), 365);
        }
    }

    mod weekday {
        use super::*;

        #[test]
        fn test_known_monday() {
            let date = Date::new(2024, 12, 9).unwrap();
            assert_eq!(date.weekday(), 1);
        }

        #[test]
        fn test_known_sunday() {
            let date = Date::new(2024, 12, 15).unwrap();
            assert_eq!(date.weekday(), 7);
        }

        #[test]
        fn test_known_wednesday() {
            let date = Date::new(2024, 12, 11).unwrap();
            assert_eq!(date.weekday(), 3);
        }

        #[test]
        fn test_new_years_day_2024() {
            let date = Date::new(2024, 1, 1).unwrap();
            assert_eq!(date.weekday(), 1); // Monday
        }

        #[test]
        fn test_weekday_chrono() {
            let monday = Date::new(2024, 12, 9).unwrap();
            assert_eq!(monday.weekday_chrono(), Weekday::Mon);
        }
    }

    mod arithmetic {
        use super::*;

        #[test]
        fn test_add_days_same_month() {
            let date = Date::new(2024, 6, 10).unwrap();
            let new_date = date.add_days(5);
            assert_eq!(new_date.day(), 15);
            assert_eq!(new_date.month(), 6);
        }

        #[test]
        fn test_add_days_cross_month() {
            let date = Date::new(2024, 1, 30).unwrap();
            let new_date = date.add_days(5);
            assert_eq!(new_date.month(), 2);
            assert_eq!(new_date.day(), 4);
        }

        #[test]
        fn test_add_days_cross_year() {
            let date = Date::new(2024, 12, 30).unwrap();
            let new_date = date.add_days(5);
            assert_eq!(new_date.year(), 2025);
            assert_eq!(new_date.month(), 1);
            assert_eq!(new_date.day(), 4);
        }

        #[test]
        fn test_sub_days() {
            let date = Date::new(2024, 2, 5).unwrap();
            let new_date = date.sub_days(10);
            assert_eq!(new_date.month(), 1);
            assert_eq!(new_date.day(), 26);
        }

        #[test]
        fn test_sub_days_cross_year() {
            let date = Date::new(2024, 1, 5).unwrap();
            let new_date = date.sub_days(10);
            assert_eq!(new_date.year(), 2023);
            assert_eq!(new_date.month(), 12);
            assert_eq!(new_date.day(), 26);
        }

        #[test]
        fn test_days_until_positive() {
            let start = Date::new(2024, 1, 1).unwrap();
            let end = Date::new(2024, 1, 10).unwrap();
            assert_eq!(start.days_until(&end), 9);
        }

        #[test]
        fn test_days_until_negative() {
            let start = Date::new(2024, 1, 10).unwrap();
            let end = Date::new(2024, 1, 1).unwrap();
            assert_eq!(start.days_until(&end), -9);
        }

        #[test]
        fn test_days_until_same_date() {
            let date = Date::new(2024, 6, 15).unwrap();
            assert_eq!(date.days_until(&date), 0);
        }

        #[test]
        fn test_days_until_cross_year() {
            let start = Date::new(2023, 12, 31).unwrap();
            let end = Date::new(2024, 1, 1).unwrap();
            assert_eq!(start.days_until(&end), 1);
        }
    }

    mod formatting {
        use super::*;

        #[test]
        fn test_format_iso() {
            let date = Date::new(2024, 3, 5).unwrap();
            assert_eq!(date.format_iso(), "2024-03-05");
        }

        #[test]
        fn test_format_iso_pads_month_and_day() {
            let date = Date::new(2024, 1, 1).unwrap();
            assert_eq!(date.format_iso(), "2024-01-01");
        }

        #[test]
        fn test_display_trait() {
            let date = Date::new(2024, 12, 25).unwrap();
            assert_eq!(format!("{}", date), "2024-12-25");
        }

        #[test]
        fn test_debug_trait() {
            let date = Date::new(2024, 6, 15).unwrap();
            let debug = format!("{:?}", date);
            assert!(debug.contains("Date"));
        }

        #[test]
        fn test_custom_format() {
            let date = Date::new(2024, 3, 15).unwrap();
            assert_eq!(date.format("%d/%m/%Y"), "15/03/2024");
        }

        #[test]
        fn test_format_month_name() {
            let date = Date::new(2024, 3, 15).unwrap();
            assert_eq!(date.format("%B %d, %Y"), "March 15, 2024");
        }
    }

    mod parsing {
        use super::*;

        #[test]
        fn test_from_str_valid() {
            let date: Date = "2024-03-15".parse().unwrap();
            assert_eq!(date.year(), 2024);
            assert_eq!(date.month(), 3);
            assert_eq!(date.day(), 15);
        }

        #[test]
        fn test_from_str_invalid_format() {
            let result: Result<Date, _> = "2024/03/15".parse();
            assert!(matches!(result, Err(DateError::ParseError { .. })));
        }

        #[test]
        fn test_from_str_invalid_month() {
            let result: Result<Date, _> = "2024-13-15".parse();
            assert!(matches!(result, Err(DateError::ParseError { .. })));
        }

        #[test]
        fn test_from_str_invalid_day() {
            let result: Result<Date, _> = "2024-02-30".parse();
            assert!(matches!(result, Err(DateError::ParseError { .. })));
        }

        #[test]
        fn test_from_str_roundtrip() {
            let original = Date::new(2024, 6, 15).unwrap();
            let parsed: Date = original.to_string().parse().unwrap();
            assert_eq!(original, parsed);
        }
    }

    mod value_object_semantics {
        use super::*;

        #[test]
        fn test_equality_for_same_date() {
            let d1 = Date::new(2024, 6, 15).unwrap();
            let d2 = Date::new(2024, 6, 15).unwrap();
            assert_eq!(d1, d2);
        }

        #[test]
        fn test_inequality_for_different_dates() {
            let d1 = Date::new(2024, 6, 15).unwrap();
            let d2 = Date::new(2024, 6, 16).unwrap();
            assert_ne!(d1, d2);
        }

        #[test]
        fn test_copy_creates_equal_instance() {
            let d1 = Date::new(2024, 6, 15).unwrap();
            let d2 = d1;
            assert_eq!(d1, d2);
        }

        #[test]
        fn test_clone_creates_equal_instance() {
            let d1 = Date::new(2024, 6, 15).unwrap();
            let d2 = d1.clone();
            assert_eq!(d1, d2);
        }

        #[test]
        fn test_hash_consistency() {
            use std::collections::HashSet;
            let mut set = HashSet::new();
            let d1 = Date::new(2024, 6, 15).unwrap();
            let d2 = Date::new(2024, 6, 15).unwrap();
            set.insert(d1);
            assert!(set.contains(&d2));
        }

        #[test]
        fn test_ordering() {
            let d1 = Date::new(2024, 1, 1).unwrap();
            let d2 = Date::new(2024, 6, 15).unwrap();
            let d3 = Date::new(2025, 1, 1).unwrap();
            assert!(d1 < d2);
            assert!(d2 < d3);
            assert!(d1 < d3);
        }

        #[test]
        fn test_ordering_same_year_month() {
            let d1 = Date::new(2024, 6, 10).unwrap();
            let d2 = Date::new(2024, 6, 20).unwrap();
            assert!(d1 < d2);
        }
    }

    mod temporal_checks {
        use super::*;

        #[test]
        fn test_is_past_for_old_date() {
            let date = Date::new(2020, 1, 1).unwrap();
            assert!(date.is_past());
        }

        #[test]
        fn test_is_future_for_future_date() {
            let date = Date::new(2099, 12, 31).unwrap();
            assert!(date.is_future());
        }

        #[test]
        fn test_is_today() {
            let today = Date::today();
            assert!(today.is_today());
            assert!(!today.is_past());
            assert!(!today.is_future());
        }

        #[test]
        fn test_yesterday_is_not_today() {
            let yesterday = Date::today().sub_days(1);
            assert!(!yesterday.is_today());
            assert!(yesterday.is_past());
        }
    }

    mod conversions {
        use super::*;

        #[test]
        fn test_from_naive_date_trait() {
            let naive = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
            let date: Date = naive.into();
            assert_eq!(date.year(), 2024);
        }

        #[test]
        fn test_into_naive_date() {
            let date = Date::new(2024, 6, 15).unwrap();
            let naive: NaiveDate = date.into();
            assert_eq!(naive.year(), 2024);
            assert_eq!(naive.month(), 6);
            assert_eq!(naive.day(), 15);
        }

        #[test]
        fn test_as_naive_date() {
            let date = Date::new(2024, 6, 15).unwrap();
            let naive = date.as_naive_date();
            assert_eq!(naive.year(), 2024);
        }
    }

    mod edge_cases {
        use super::*;

        #[test]
        fn test_negative_year() {
            let date = Date::new(-100, 6, 15).unwrap();
            assert_eq!(date.year(), -100);
        }

        #[test]
        fn test_year_zero() {
            let date = Date::new(0, 6, 15).unwrap();
            assert_eq!(date.year(), 0);
        }

        #[test]
        fn test_each_month_max_days() {
            for month in 1..=12 {
                let max_day = Date::days_in_month(2024, month);
                let date = Date::new(2024, month, max_day);
                assert!(date.is_ok(), "Failed for month {}", month);
            }
        }
    }
}
