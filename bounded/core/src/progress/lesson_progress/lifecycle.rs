use super::{LessonProgress, LessonProgressError};
use education_platform_common::DateTime;

impl LessonProgress {
    /// Starts the lesson by setting the start datetime to today.
    ///
    /// If the lesson is already started, this is a no-op.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::LessonProgress;
    ///
    /// let mut progress = LessonProgress::new(
    ///     "Closures".to_string(),
    ///     1800,
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// progress.start();
    /// assert!(progress.has_started());
    /// assert!(progress.start_date().is_some());
    /// ```
    pub fn start(&mut self) {
        if self.has_started() {
            return;
        }

        self.start_date = Some(DateTime::today());
    }

    /// Ends the lesson by setting the end datetime to today.
    ///
    /// If the lesson is already ended, this is a no-op.
    ///
    /// # Errors
    ///
    /// Returns `LessonProgressError::CannotEndUnstartedLesson` if the lesson
    /// has not been started yet.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::LessonProgress;
    ///
    /// let mut progress = LessonProgress::new(
    ///     "Iterators".to_string(),
    ///     2100,
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// // Cannot end an unstarted lesson
    /// let result = progress.end();
    /// assert!(result.is_err());
    ///
    /// // Start first, then end
    /// progress.start();
    /// progress.end().unwrap();
    /// assert!(progress.has_ended());
    /// ```
    pub fn end(&mut self) -> Result<(), LessonProgressError> {
        if !self.has_started() {
            return Err(LessonProgressError::CannotEndUnstartedLesson);
        }

        if self.has_ended() {
            return Ok(());
        }

        self.end_date = Some(DateTime::today());
        Ok(())
    }

    /// Restarts the lesson by clearing both start and end dates.
    ///
    /// If the lesson has not been started, this is a no-op.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::LessonProgress;
    ///
    /// let mut progress = LessonProgress::new(
    ///     "Smart Pointers".to_string(),
    ///     2400,
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// progress.start();
    /// assert!(progress.has_started());
    ///
    /// progress.restart();
    /// assert!(!progress.has_started());
    /// assert!(!progress.has_ended());
    /// ```
    pub fn restart(&mut self) {
        if !self.has_started() {
            return;
        }

        self.start_date = None;
        self.end_date = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use education_platform_common::Entity;

    fn create_test_progress(name: &str, duration: u64) -> LessonProgress {
        LessonProgress::new(name.to_string(), duration, None, None).unwrap()
    }

    mod start_operation {
        use super::*;

        #[test]
        fn test_start_sets_start_date() {
            let mut progress = create_test_progress("Start Test", 1800);
            progress.start();

            assert!(progress.start_date().is_some());
            assert!(progress.has_started());
        }

        #[test]
        fn test_start_preserves_other_fields() {
            let mut progress = create_test_progress("Preserved", 3600);
            progress.start();

            assert_eq!(progress.lesson_name().as_str(), "Preserved");
            assert_eq!(progress.duration().total_seconds(), 3600);
        }

        #[test]
        fn test_start_is_idempotent() {
            let mut progress = create_test_progress("Idempotent", 1800);
            progress.start();
            let first_start_date = progress.start_date().cloned();
            progress.start();

            assert_eq!(progress.start_date(), first_start_date.as_ref());
        }

        #[test]
        fn test_start_preserves_id() {
            let mut progress = create_test_progress("ID Test", 1800);
            let original_id = progress.id();
            progress.start();

            assert_eq!(progress.id(), original_id);
        }
    }

    mod end_operation {
        use super::*;

        #[test]
        fn test_end_fails_when_not_started() {
            let mut progress = create_test_progress("Not Started", 1800);
            let result = progress.end();

            assert!(matches!(result, Err(LessonProgressError::CannotEndUnstartedLesson)));
        }

        #[test]
        fn test_end_succeeds_when_started() {
            let mut progress = create_test_progress("Started", 1800);
            progress.start();
            let result = progress.end();

            assert!(result.is_ok());
            assert!(progress.has_ended());
        }

        #[test]
        fn test_end_sets_end_date() {
            let mut progress = create_test_progress("End Test", 1800);
            progress.start();
            progress.end().unwrap();

            assert!(progress.end_date().is_some());
        }

        #[test]
        fn test_end_preserves_start_date() {
            let mut progress = create_test_progress("Preserve Start", 1800);
            progress.start();
            let start_date = progress.start_date().cloned();
            progress.end().unwrap();

            assert_eq!(progress.start_date(), start_date.as_ref());
        }

        #[test]
        fn test_end_is_idempotent() {
            let mut progress = create_test_progress("Idempotent End", 1800);
            progress.start();
            progress.end().unwrap();
            let first_end_date = progress.end_date().cloned();
            progress.end().unwrap();

            assert_eq!(progress.end_date(), first_end_date.as_ref());
        }

        #[test]
        fn test_end_preserves_id() {
            let mut progress = create_test_progress("ID Test", 1800);
            let original_id = progress.id();
            progress.start();
            progress.end().unwrap();

            assert_eq!(progress.id(), original_id);
        }
    }

    mod restart_operation {
        use super::*;

        #[test]
        fn test_restart_clears_start_date() {
            let mut progress = create_test_progress("Restart", 1800);
            progress.start();
            progress.restart();

            assert!(!progress.has_started());
            assert!(progress.start_date().is_none());
        }

        #[test]
        fn test_restart_clears_end_date() {
            let mut progress = create_test_progress("Restart End", 1800);
            progress.start();
            progress.end().unwrap();
            progress.restart();

            assert!(!progress.has_ended());
            assert!(progress.end_date().is_none());
        }

        #[test]
        fn test_restart_preserves_other_fields() {
            let mut progress = create_test_progress("Preserved Fields", 3600);
            progress.start();
            progress.restart();

            assert_eq!(progress.lesson_name().as_str(), "Preserved Fields");
            assert_eq!(progress.duration().total_seconds(), 3600);
        }

        #[test]
        fn test_restart_is_noop_when_not_started() {
            let mut progress = create_test_progress("Not Started", 1800);
            progress.restart();

            assert!(!progress.has_started());
            assert!(!progress.has_ended());
        }

        #[test]
        fn test_restart_preserves_id() {
            let mut progress = create_test_progress("ID Test", 1800);
            let original_id = progress.id();
            progress.start();
            progress.restart();

            assert_eq!(progress.id(), original_id);
        }
    }
}
