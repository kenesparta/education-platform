use super::LessonProgress;

impl LessonProgress {
    /// Returns true if the lesson has been started.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::LessonProgress;
    ///
    /// let mut progress = LessonProgress::new(
    ///     "Error Handling".to_string(),
    ///     2400,
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// assert!(!progress.has_started());
    ///
    /// progress.start();
    /// assert!(progress.has_started());
    /// ```
    #[inline]
    #[must_use]
    pub const fn has_started(&self) -> bool {
        self.start_date.is_some()
    }

    /// Returns true if the lesson has been completed.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::LessonProgress;
    ///
    /// let progress = LessonProgress::new(
    ///     "Traits".to_string(),
    ///     3000,
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// assert!(!progress.has_ended());
    /// ```
    #[inline]
    #[must_use]
    pub const fn has_ended(&self) -> bool {
        self.end_date.is_some()
    }

    /// Returns true if the lesson is in progress (started but not ended).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::LessonProgress;
    ///
    /// let mut progress = LessonProgress::new(
    ///     "Generics".to_string(),
    ///     2700,
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// assert!(!progress.is_in_progress());
    ///
    /// progress.start();
    /// assert!(progress.is_in_progress());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_in_progress(&self) -> bool {
        self.start_date.is_some() && self.end_date.is_none()
    }

    /// Returns true if the lesson is completed (both started and ended).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::LessonProgress;
    ///
    /// let progress = LessonProgress::new(
    ///     "Lifetimes".to_string(),
    ///     3600,
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// assert!(!progress.is_completed());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_completed(&self) -> bool {
        self.end_date.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_progress(name: &str, duration: u64) -> LessonProgress {
        LessonProgress::new(name.to_string(), duration, None, None).unwrap()
    }

    mod status_checks {
        use super::*;

        #[test]
        fn test_has_started_returns_false_initially() {
            let progress = create_test_progress("Not Started", 1800);

            assert!(!progress.has_started());
        }

        #[test]
        fn test_has_started_returns_true_after_start() {
            let mut progress = create_test_progress("Started", 1800);
            progress.start();

            assert!(progress.has_started());
        }

        #[test]
        fn test_has_ended_returns_false_initially() {
            let progress = create_test_progress("Not Ended", 1800);

            assert!(!progress.has_ended());
        }

        #[test]
        fn test_is_in_progress_false_when_not_started() {
            let progress = create_test_progress("Not Started", 1800);

            assert!(!progress.is_in_progress());
        }

        #[test]
        fn test_is_in_progress_true_when_started_not_ended() {
            let mut progress = create_test_progress("In Progress", 1800);
            progress.start();

            assert!(progress.is_in_progress());
        }

        #[test]
        fn test_is_in_progress_false_when_completed() {
            let mut progress = create_test_progress("Completed", 1800);
            progress.start();
            progress.end().unwrap();

            assert!(!progress.is_in_progress());
        }

        #[test]
        fn test_is_completed_false_initially() {
            let progress = create_test_progress("Not Completed", 1800);

            assert!(!progress.is_completed());
        }

        #[test]
        fn test_is_completed_false_when_only_started() {
            let mut progress = create_test_progress("Started Only", 1800);
            progress.start();

            assert!(!progress.is_completed());
        }

        #[test]
        fn test_is_completed_true_when_started_and_ended() {
            let mut progress = create_test_progress("Completed", 1800);
            progress.start();
            progress.end().unwrap();

            assert!(progress.is_completed());
        }
    }
}
