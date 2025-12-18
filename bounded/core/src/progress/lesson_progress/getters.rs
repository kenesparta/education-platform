use super::LessonProgress;
use education_platform_common::{DateTime, Duration, SimpleName};

impl LessonProgress {
    /// Returns the lesson name.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::LessonProgress;
    ///
    /// let progress = LessonProgress::new(
    ///     "Functions".to_string(),
    ///     1800,
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// assert_eq!(progress.lesson_name().as_str(), "Functions");
    /// ```
    #[inline]
    #[must_use]
    pub const fn lesson_name(&self) -> &SimpleName {
        &self.lesson_name
    }

    /// Returns the lesson duration.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::LessonProgress;
    ///
    /// let progress = LessonProgress::new(
    ///     "Modules".to_string(),
    ///     3600,
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// assert_eq!(progress.duration().total_seconds(), 3600);
    /// ```
    #[inline]
    #[must_use]
    pub const fn duration(&self) -> Duration {
        self.duration
    }

    /// Returns the start datetime if the lesson has been started.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::LessonProgress;
    ///
    /// let progress = LessonProgress::new(
    ///     "Structs".to_string(),
    ///     1800,
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// assert!(progress.start_date().is_none());
    ///
    /// let started = progress.start();
    /// assert!(started.start_date().is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn start_date(&self) -> Option<&DateTime> {
        self.start_date.as_ref()
    }

    /// Returns the end datetime if the lesson has been completed.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::LessonProgress;
    ///
    /// let progress = LessonProgress::new(
    ///     "Enums".to_string(),
    ///     1800,
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// assert!(progress.end_date().is_none());
    /// ```
    #[inline]
    #[must_use]
    pub fn end_date(&self) -> Option<&DateTime> {
        self.end_date.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_progress(name: &str, duration: u64) -> LessonProgress {
        LessonProgress::new(name.to_string(), duration, None, None).unwrap()
    }

    mod getters {
        use super::*;

        #[test]
        fn test_lesson_name_returns_name() {
            let progress = create_test_progress("Test Lesson", 1800);

            assert_eq!(progress.lesson_name().as_str(), "Test Lesson");
        }

        #[test]
        fn test_duration_returns_correct_value() {
            let progress = create_test_progress("Duration Test", 3600);

            assert_eq!(progress.duration().total_seconds(), 3600);
        }

        #[test]
        fn test_start_date_returns_none_when_not_started() {
            let progress = create_test_progress("Not Started", 1800);

            assert!(progress.start_date().is_none());
        }

        #[test]
        fn test_end_date_returns_none_when_not_ended() {
            let progress = create_test_progress("Not Ended", 1800);

            assert!(progress.end_date().is_none());
        }

        #[test]
        fn test_id_returns_valid_id() {
            use education_platform_common::Entity;

            let progress = create_test_progress("ID Test", 1800);

            let id = progress.id();
            assert!(id.timestamp_ms() > 0);
        }
    }
}
