/// A duration value object representing a time span in seconds.
///
/// `Duration` is an immutable value object that represents a time duration.
/// It provides methods to format the duration in human-readable formats
/// (hours, minutes, seconds) and perform arithmetic operations.
///
/// # Examples
///
/// ```
/// use education_platform_common::Duration;
///
/// let duration = Duration::from_seconds(3665);
/// assert_eq!(duration.total_seconds(), 3665);
/// assert_eq!(duration.format_hours(), "01h 01m 05s");
///
/// let d1 = Duration::from_seconds(60);
/// let d2 = Duration::from_seconds(30);
/// let sum = d1.add(&d2);
/// assert_eq!(sum.total_seconds(), 90);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Duration {
    seconds: u64,
}

const SECONDS_PER_MINUTE: u64 = 60;
const SECONDS_PER_HOUR: u64 = 3600;

impl Duration {
    /// Creates a new `Duration` from seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Duration;
    ///
    /// let duration = Duration::from_seconds(120);
    /// assert_eq!(duration.total_seconds(), 120);
    ///
    /// let zero = Duration::from_seconds(0);
    /// assert_eq!(zero.total_seconds(), 0);
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_seconds(seconds: u64) -> Self {
        Self { seconds }
    }

    /// Creates a new `Duration` from minutes.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Duration;
    ///
    /// let duration = Duration::from_minutes(5);
    /// assert_eq!(duration.total_seconds(), 300);
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_minutes(minutes: u64) -> Self {
        Self {
            seconds: minutes * SECONDS_PER_MINUTE,
        }
    }

    /// Creates a new `Duration` from hours.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Duration;
    ///
    /// let duration = Duration::from_hours(2);
    /// assert_eq!(duration.total_seconds(), 7200);
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_hours(hours: u64) -> Self {
        Self {
            seconds: hours * SECONDS_PER_HOUR,
        }
    }

    /// Creates a new `Duration` from hours, minutes, and seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Duration;
    ///
    /// let duration = Duration::from_hms(1, 30, 45);
    /// assert_eq!(duration.total_seconds(), 5445);
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_hms(hours: u64, minutes: u64, seconds: u64) -> Self {
        Self {
            seconds: hours * SECONDS_PER_HOUR + minutes * SECONDS_PER_MINUTE + seconds,
        }
    }

    /// Returns the total number of seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Duration;
    ///
    /// let duration = Duration::from_minutes(3);
    /// assert_eq!(duration.total_seconds(), 180);
    /// ```
    #[inline]
    #[must_use]
    pub const fn total_seconds(&self) -> u64 {
        self.seconds
    }

    /// Returns the hours component.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Duration;
    ///
    /// let duration = Duration::from_seconds(7265);
    /// assert_eq!(duration.hours(), 2);
    /// ```
    #[inline]
    #[must_use]
    pub const fn hours(&self) -> u64 {
        self.seconds / SECONDS_PER_HOUR
    }

    /// Returns the minutes component (0-59).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Duration;
    ///
    /// let duration = Duration::from_seconds(7265);
    /// assert_eq!(duration.minutes(), 1);
    /// ```
    #[inline]
    #[must_use]
    pub const fn minutes(&self) -> u64 {
        (self.seconds % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE
    }

    /// Returns the seconds component (0-59).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Duration;
    ///
    /// let duration = Duration::from_seconds(7265);
    /// assert_eq!(duration.seconds(), 5);
    /// ```
    #[inline]
    #[must_use]
    pub const fn seconds(&self) -> u64 {
        self.seconds % SECONDS_PER_MINUTE
    }

    /// Adds two durations together.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Duration;
    ///
    /// let d1 = Duration::from_minutes(5);
    /// let d2 = Duration::from_seconds(30);
    /// let sum = d1.add(&d2);
    /// assert_eq!(sum.total_seconds(), 330);
    /// ```
    #[inline]
    #[must_use]
    pub const fn add(&self, other: &Self) -> Self {
        Self {
            seconds: self.seconds + other.seconds,
        }
    }

    /// Formats the duration as "HHh MMm" (hours and minutes only).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Duration;
    ///
    /// let duration = Duration::from_seconds(3665);
    /// assert_eq!(duration.format_minutes(), "01h 01m");
    ///
    /// let minutes_only = Duration::from_seconds(300);
    /// assert_eq!(minutes_only.format_minutes(), "00h 05m");
    /// ```
    #[must_use]
    pub fn format_minutes(&self) -> String {
        format!("{:02}h {:02}m", self.hours(), self.minutes())
    }

    /// Formats the duration in the most appropriate format.
    ///
    /// - If less than 1 hour: "MMm SSs"
    /// - If exactly on the hour: "HHh"
    /// - If hours and minutes (no seconds): "HHh MMm"
    /// - Otherwise: "HHh MMm SSs"
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Duration;
    ///
    /// let short = Duration::from_seconds(125);
    /// assert_eq!(short.format_hours(), "02m 05s");
    ///
    /// let exact_hour = Duration::from_hours(2);
    /// assert_eq!(exact_hour.format_hours(), "02h");
    ///
    /// let hours_and_minutes = Duration::from_seconds(3660);
    /// assert_eq!(hours_and_minutes.format_hours(), "01h 01m");
    ///
    /// let full = Duration::from_seconds(3665);
    /// assert_eq!(full.format_hours(), "01h 01m 05s");
    /// ```
    #[must_use]
    pub fn format_hours(&self) -> String {
        let h = self.hours();
        let m = self.minutes();
        let s = self.seconds();

        match (h, m, s) {
            (0, m, s) => format!("{:02}m {:02}s", m, s),
            (h, 0, 0) => format!("{:02}h", h),
            (h, m, 0) => format!("{:02}h {:02}m", h, m),
            (h, m, s) => format!("{:02}h {:02}m {:02}s", h, m, s),
        }
    }

    /// Returns true if the duration is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Duration;
    ///
    /// let zero = Duration::from_seconds(0);
    /// assert!(zero.is_zero());
    ///
    /// let non_zero = Duration::from_seconds(1);
    /// assert!(!non_zero.is_zero());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.seconds == 0
    }
}

impl Default for Duration {
    /// Creates a zero duration.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Duration;
    ///
    /// let duration = Duration::default();
    /// assert_eq!(duration.total_seconds(), 0);
    /// ```
    #[inline]
    fn default() -> Self {
        Self::from_seconds(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod constructors {
        use super::*;

        #[test]
        fn test_from_seconds_creates_duration() {
            let duration = Duration::from_seconds(120);
            assert_eq!(duration.total_seconds(), 120);
        }

        #[test]
        fn test_from_seconds_with_zero() {
            let duration = Duration::from_seconds(0);
            assert_eq!(duration.total_seconds(), 0);
        }

        #[test]
        fn test_from_seconds_with_large_value() {
            let duration = Duration::from_seconds(86400);
            assert_eq!(duration.total_seconds(), 86400);
        }

        #[test]
        fn test_from_minutes_creates_correct_seconds() {
            let duration = Duration::from_minutes(5);
            assert_eq!(duration.total_seconds(), 300);
        }

        #[test]
        fn test_from_minutes_with_zero() {
            let duration = Duration::from_minutes(0);
            assert_eq!(duration.total_seconds(), 0);
        }

        #[test]
        fn test_from_hours_creates_correct_seconds() {
            let duration = Duration::from_hours(2);
            assert_eq!(duration.total_seconds(), 7200);
        }

        #[test]
        fn test_from_hours_with_zero() {
            let duration = Duration::from_hours(0);
            assert_eq!(duration.total_seconds(), 0);
        }

        #[test]
        fn test_from_hms_combines_all_components() {
            let duration = Duration::from_hms(1, 30, 45);
            assert_eq!(duration.total_seconds(), 5445);
        }

        #[test]
        fn test_from_hms_with_zeros() {
            let duration = Duration::from_hms(0, 0, 0);
            assert_eq!(duration.total_seconds(), 0);
        }

        #[test]
        fn test_from_hms_with_only_hours() {
            let duration = Duration::from_hms(2, 0, 0);
            assert_eq!(duration.total_seconds(), 7200);
        }

        #[test]
        fn test_from_hms_with_only_minutes() {
            let duration = Duration::from_hms(0, 45, 0);
            assert_eq!(duration.total_seconds(), 2700);
        }

        #[test]
        fn test_from_hms_with_only_seconds() {
            let duration = Duration::from_hms(0, 0, 30);
            assert_eq!(duration.total_seconds(), 30);
        }

        #[test]
        fn test_default_creates_zero_duration() {
            let duration = Duration::default();
            assert_eq!(duration.total_seconds(), 0);
        }
    }

    mod getters {
        use super::*;

        #[test]
        fn test_hours_returns_correct_value() {
            let duration = Duration::from_seconds(7265);
            assert_eq!(duration.hours(), 2);
        }

        #[test]
        fn test_hours_with_exact_hours() {
            let duration = Duration::from_hours(3);
            assert_eq!(duration.hours(), 3);
        }

        #[test]
        fn test_hours_with_zero() {
            let duration = Duration::from_seconds(59);
            assert_eq!(duration.hours(), 0);
        }

        #[test]
        fn test_minutes_returns_correct_value() {
            let duration = Duration::from_seconds(7265);
            assert_eq!(duration.minutes(), 1);
        }

        #[test]
        fn test_minutes_with_exact_minutes() {
            let duration = Duration::from_minutes(45);
            assert_eq!(duration.minutes(), 45);
        }

        #[test]
        fn test_minutes_wraps_at_60() {
            let duration = Duration::from_seconds(3720);
            assert_eq!(duration.minutes(), 2);
        }

        #[test]
        fn test_minutes_with_zero() {
            let duration = Duration::from_seconds(7200);
            assert_eq!(duration.minutes(), 0);
        }

        #[test]
        fn test_seconds_returns_correct_value() {
            let duration = Duration::from_seconds(7265);
            assert_eq!(duration.seconds(), 5);
        }

        #[test]
        fn test_seconds_with_exact_seconds() {
            let duration = Duration::from_seconds(45);
            assert_eq!(duration.seconds(), 45);
        }

        #[test]
        fn test_seconds_wraps_at_60() {
            let duration = Duration::from_seconds(125);
            assert_eq!(duration.seconds(), 5);
        }

        #[test]
        fn test_seconds_with_zero() {
            let duration = Duration::from_seconds(120);
            assert_eq!(duration.seconds(), 0);
        }

        #[test]
        fn test_is_zero_returns_true_for_zero() {
            let duration = Duration::from_seconds(0);
            assert!(duration.is_zero());
        }

        #[test]
        fn test_is_zero_returns_false_for_non_zero() {
            let duration = Duration::from_seconds(1);
            assert!(!duration.is_zero());
        }
    }

    mod arithmetic {
        use super::*;

        #[test]
        fn test_add_combines_durations() {
            let d1 = Duration::from_minutes(5);
            let d2 = Duration::from_seconds(30);
            let sum = d1.add(&d2);
            assert_eq!(sum.total_seconds(), 330);
        }

        #[test]
        fn test_add_with_zero() {
            let d1 = Duration::from_minutes(5);
            let d2 = Duration::from_seconds(0);
            let sum = d1.add(&d2);
            assert_eq!(sum.total_seconds(), 300);
        }

        #[test]
        fn test_add_is_commutative() {
            let d1 = Duration::from_minutes(5);
            let d2 = Duration::from_seconds(30);
            assert_eq!(d1.add(&d2), d2.add(&d1));
        }
    }

    mod formatting {
        use super::*;

        #[test]
        fn test_format_minutes_with_hours_and_minutes() {
            let duration = Duration::from_seconds(3665);
            assert_eq!(duration.format_minutes(), "01h 01m");
        }

        #[test]
        fn test_format_minutes_with_only_minutes() {
            let duration = Duration::from_seconds(300);
            assert_eq!(duration.format_minutes(), "00h 05m");
        }

        #[test]
        fn test_format_minutes_with_zero() {
            let duration = Duration::from_seconds(0);
            assert_eq!(duration.format_minutes(), "00h 00m");
        }

        #[test]
        fn test_format_minutes_ignores_seconds() {
            let duration = Duration::from_seconds(3665);
            assert_eq!(duration.format_minutes(), "01h 01m");
        }

        #[test]
        fn test_format_hours_less_than_one_hour() {
            let duration = Duration::from_seconds(125);
            assert_eq!(duration.format_hours(), "02m 05s");
        }

        #[test]
        fn test_format_hours_exact_hour() {
            let duration = Duration::from_hours(2);
            assert_eq!(duration.format_hours(), "02h");
        }

        #[test]
        fn test_format_hours_full_format() {
            let duration = Duration::from_seconds(3665);
            assert_eq!(duration.format_hours(), "01h 01m 05s");
        }

        #[test]
        fn test_format_hours_with_hours_and_minutes_no_seconds() {
            let duration = Duration::from_seconds(3660);
            assert_eq!(duration.format_hours(), "01h 01m");
        }

        #[test]
        fn test_format_hours_with_zero() {
            let duration = Duration::from_seconds(0);
            assert_eq!(duration.format_hours(), "00m 00s");
        }

        #[test]
        fn test_format_hours_only_seconds() {
            let duration = Duration::from_seconds(45);
            assert_eq!(duration.format_hours(), "00m 45s");
        }

        #[test]
        fn test_format_hours_only_minutes() {
            let duration = Duration::from_seconds(180);
            assert_eq!(duration.format_hours(), "03m 00s");
        }
    }

    mod value_object_semantics {
        use super::*;

        #[test]
        fn test_equality_for_same_duration() {
            let d1 = Duration::from_seconds(120);
            let d2 = Duration::from_seconds(120);
            assert_eq!(d1, d2);
        }

        #[test]
        fn test_inequality_for_different_durations() {
            let d1 = Duration::from_seconds(120);
            let d2 = Duration::from_seconds(121);
            assert_ne!(d1, d2);
        }

        #[test]
        #[allow(clippy::clone_on_copy)]
        fn test_clone_creates_equal_instance() {
            let d1 = Duration::from_seconds(120);
            let d2 = d1.clone();
            assert_eq!(d1, d2);
        }

        #[test]
        fn test_copy_creates_equal_instance() {
            let d1 = Duration::from_seconds(120);
            let d2 = d1;
            assert_eq!(d1, d2);
        }

        #[test]
        fn test_hash_consistency() {
            use std::collections::HashSet;
            let mut set = HashSet::new();
            let d1 = Duration::from_seconds(120);
            let d2 = Duration::from_seconds(120);
            set.insert(d1);
            assert!(set.contains(&d2));
        }

        #[test]
        fn test_ordering() {
            let d1 = Duration::from_seconds(100);
            let d2 = Duration::from_seconds(200);
            assert!(d1 < d2);
            assert!(d2 > d1);
        }

        #[test]
        fn test_debug_format() {
            let duration = Duration::from_seconds(120);
            let debug = format!("{:?}", duration);
            assert!(debug.contains("Duration"));
        }
    }

    mod edge_cases {
        use super::*;

        #[test]
        fn test_max_u64_value() {
            let duration = Duration::from_seconds(u64::MAX);
            assert_eq!(duration.total_seconds(), u64::MAX);
        }

        #[test]
        fn test_boundary_values_59_seconds() {
            let duration = Duration::from_seconds(59);
            assert_eq!(duration.hours(), 0);
            assert_eq!(duration.minutes(), 0);
            assert_eq!(duration.seconds(), 59);
        }

        #[test]
        fn test_boundary_values_60_seconds() {
            let duration = Duration::from_seconds(60);
            assert_eq!(duration.hours(), 0);
            assert_eq!(duration.minutes(), 1);
            assert_eq!(duration.seconds(), 0);
        }

        #[test]
        fn test_boundary_values_3599_seconds() {
            let duration = Duration::from_seconds(3599);
            assert_eq!(duration.hours(), 0);
            assert_eq!(duration.minutes(), 59);
            assert_eq!(duration.seconds(), 59);
        }

        #[test]
        fn test_boundary_values_3600_seconds() {
            let duration = Duration::from_seconds(3600);
            assert_eq!(duration.hours(), 1);
            assert_eq!(duration.minutes(), 0);
            assert_eq!(duration.seconds(), 0);
        }
    }
}
