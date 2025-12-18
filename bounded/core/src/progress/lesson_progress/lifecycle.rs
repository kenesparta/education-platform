use super::{LessonProgress, LessonProgressError};
use education_platform_common::DateTime;

impl LessonProgress {
    /// Starts the lesson by setting the start datetime to now.
    ///
    /// If the lesson is already started, returns a clone with the same start datetime.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::LessonProgress;
    ///
    /// let progress = LessonProgress::new(
    ///     "Closures".to_string(),
    ///     1800,
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// let started = progress.start();
    /// assert!(started.has_started());
    /// assert!(started.start_date().is_some());
    /// ```
    #[must_use]
    pub fn start(&self) -> Self {
        if self.has_started() {
            return self.clone();
        }

        Self {
            start_date: Some(DateTime::now()),
            ..self.clone()
        }
    }

    /// Ends the lesson by setting the end datetime to now.
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
    /// let progress = LessonProgress::new(
    ///     "Iterators".to_string(),
    ///     2100,
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// // Cannot end unstarted lesson
    /// let result = progress.end();
    /// assert!(result.is_err());
    ///
    /// // Start first, then end
    /// let started = progress.start();
    /// let ended = started.end().unwrap();
    /// assert!(ended.has_ended());
    /// ```
    pub fn end(&self) -> Result<Self, LessonProgressError> {
        if !self.has_started() {
            return Err(LessonProgressError::CannotEndUnstartedLesson);
        }

        if self.has_ended() {
            return Ok(self.clone());
        }

        Ok(Self {
            end_date: Some(DateTime::now()),
            ..self.clone()
        })
    }

    /// Restarts the lesson by clearing both start and end dates.
    ///
    /// If the lesson has not been started, returns a clone unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::LessonProgress;
    ///
    /// let progress = LessonProgress::new(
    ///     "Smart Pointers".to_string(),
    ///     2400,
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// let started = progress.start();
    /// assert!(started.has_started());
    ///
    /// let restarted = started.restart();
    /// assert!(!restarted.has_started());
    /// assert!(!restarted.has_ended());
    /// ```
    #[must_use]
    pub fn restart(&self) -> Self {
        if !self.has_started() {
            return self.clone();
        }

        Self {
            start_date: None,
            end_date: None,
            ..self.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_progress(name: &str, duration: u64) -> LessonProgress {
        LessonProgress::new(name.to_string(), duration, None, None).unwrap()
    }

    mod start_operation {
        use super::*;

        #[test]
        fn test_start_sets_start_date() {
            let progress = create_test_progress("Start Test", 1800);
            let started = progress.start();

            assert!(started.start_date().is_some());
            assert!(started.has_started());
        }

        #[test]
        fn test_start_preserves_other_fields() {
            let progress = create_test_progress("Preserved", 3600);
            let started = progress.start();

            assert_eq!(started.lesson_name().as_str(), "Preserved");
            assert_eq!(started.duration().total_seconds(), 3600);
        }

        #[test]
        fn test_start_is_idempotent() {
            let progress = create_test_progress("Idempotent", 1800);
            let started1 = progress.start();
            let started2 = started1.start();

            assert_eq!(started1.start_date(), started2.start_date());
        }

        #[test]
        fn test_start_does_not_modify_original() {
            let progress = create_test_progress("Original", 1800);
            let _started = progress.start();

            assert!(!progress.has_started());
        }
    }

    mod end_operation {
        use super::*;

        #[test]
        fn test_end_fails_when_not_started() {
            let progress = create_test_progress("Not Started", 1800);
            let result = progress.end();

            assert!(matches!(
                result,
                Err(LessonProgressError::CannotEndUnstartedLesson)
            ));
        }

        #[test]
        fn test_end_succeeds_when_started() {
            let progress = create_test_progress("Started", 1800);
            let started = progress.start();
            let ended = started.end();

            assert!(ended.is_ok());
            assert!(ended.unwrap().has_ended());
        }

        #[test]
        fn test_end_sets_end_date() {
            let progress = create_test_progress("End Test", 1800);
            let started = progress.start();
            let ended = started.end().unwrap();

            assert!(ended.end_date().is_some());
        }

        #[test]
        fn test_end_preserves_start_date() {
            let progress = create_test_progress("Preserve Start", 1800);
            let started = progress.start();
            let start_date = started.start_date();
            let ended = started.end().unwrap();

            assert_eq!(ended.start_date(), start_date);
        }

        #[test]
        fn test_end_is_idempotent() {
            let progress = create_test_progress("Idempotent End", 1800);
            let started = progress.start();
            let ended1 = started.end().unwrap();
            let ended2 = ended1.end().unwrap();

            assert_eq!(ended1.end_date(), ended2.end_date());
        }

        #[test]
        fn test_end_does_not_modify_original() {
            let progress = create_test_progress("Original", 1800);
            let started = progress.start();
            let _ended = started.end().unwrap();

            assert!(!started.has_ended());
        }
    }

    mod restart_operation {
        use super::*;

        #[test]
        fn test_restart_clears_start_date() {
            let progress = create_test_progress("Restart", 1800);
            let started = progress.start();
            let restarted = started.restart();

            assert!(!restarted.has_started());
            assert!(restarted.start_date().is_none());
        }

        #[test]
        fn test_restart_clears_end_date() {
            let progress = create_test_progress("Restart End", 1800);
            let started = progress.start();
            let ended = started.end().unwrap();
            let restarted = ended.restart();

            assert!(!restarted.has_ended());
            assert!(restarted.end_date().is_none());
        }

        #[test]
        fn test_restart_preserves_other_fields() {
            let progress = create_test_progress("Preserved Fields", 3600);
            let started = progress.start();
            let restarted = started.restart();

            assert_eq!(restarted.lesson_name().as_str(), "Preserved Fields");
            assert_eq!(restarted.duration().total_seconds(), 3600);
        }

        #[test]
        fn test_restart_is_noop_when_not_started() {
            let progress = create_test_progress("Not Started", 1800);
            let restarted = progress.restart();

            assert!(!restarted.has_started());
            assert!(!restarted.has_ended());
        }

        #[test]
        fn test_restart_does_not_modify_original() {
            let progress = create_test_progress("Original", 1800);
            let started = progress.start();
            let _restarted = started.restart();

            assert!(started.has_started());
        }
    }
}
