use education_platform_common::{
    Date, Duration, Entity, Id, SimpleName, SimpleNameConfig, SimpleNameError,
};
use thiserror::Error;

/// Error types for Lesson Progress validation failures.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LessonProgressError {
    #[error("Lesson name validation failed: {0}")]
    NameError(#[from] SimpleNameError),

    #[error("Duration must be different from zero")]
    DurationCantBeZero,

    #[error("Cannot end a lesson that has not started")]
    CannotEndUnstartedLesson,
}

/// Tracks progress of a lesson within a course.
///
/// `LessonProgress` is an entity that records whether a student has started
/// and/or completed a specific lesson. It tracks the duration of the lesson
/// and the dates when it was started and ended.
///
/// # Examples
///
/// ```
/// use education_platform_core::LessonProgress;
///
/// let progress = LessonProgress::new(
///     "Introduction to Rust".to_string(),
///     1800,
///     None,
///     None,
/// ).unwrap();
///
/// assert!(!progress.has_started());
/// assert!(!progress.has_ended());
///
/// let started = progress.start();
/// assert!(started.has_started());
/// ```
#[derive(Debug, Clone)]
pub struct LessonProgress {
    id: Id,
    lesson_name: SimpleName,
    duration: Duration,
    start_date: Option<Date>,
    end_date: Option<Date>,
}

impl LessonProgress {
    /// Creates a new `LessonProgress` instance with validated parameters.
    ///
    /// # Errors
    ///
    /// Returns `LessonProgressError::NameError` if the lesson name fails validation.
    /// Returns `LessonProgressError::DurationCantBeZero` if duration is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::LessonProgress;
    ///
    /// let progress = LessonProgress::new(
    ///     "Variables and Types".to_string(),
    ///     3600,
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// // Name too short
    /// let invalid = LessonProgress::new("AB".to_string(), 100, None, None);
    /// assert!(invalid.is_err());
    ///
    /// // Zero duration
    /// let zero_duration = LessonProgress::new("Valid Name".to_string(), 0, None, None);
    /// assert!(zero_duration.is_err());
    /// ```
    pub fn new(
        lesson_name: String,
        duration: u64,
        start_date: Option<Date>,
        end_date: Option<Date>,
    ) -> Result<Self, LessonProgressError> {
        let lesson_name = SimpleName::with_config(lesson_name, SimpleNameConfig::new(3, 50))?;
        let duration = Duration::from_seconds(duration);
        if duration.is_zero() {
            return Err(LessonProgressError::DurationCantBeZero);
        }

        Ok(Self {
            id: Id::default(),
            lesson_name,
            duration,
            start_date,
            end_date,
        })
    }

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

    /// Returns the start date if the lesson has been started.
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
    pub const fn start_date(&self) -> Option<Date> {
        self.start_date
    }

    /// Returns the end date if the lesson has been completed.
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
    pub const fn end_date(&self) -> Option<Date> {
        self.end_date
    }

    /// Returns true if the lesson has been started.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::LessonProgress;
    ///
    /// let progress = LessonProgress::new(
    ///     "Error Handling".to_string(),
    ///     2400,
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// assert!(!progress.has_started());
    ///
    /// let started = progress.start();
    /// assert!(started.has_started());
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
    /// let progress = LessonProgress::new(
    ///     "Generics".to_string(),
    ///     2700,
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// assert!(!progress.is_in_progress());
    ///
    /// let started = progress.start();
    /// assert!(started.is_in_progress());
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

    /// Starts the lesson by setting the start date to today.
    ///
    /// If the lesson is already started, returns a clone with the same start date.
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
            start_date: Some(Date::today()),
            ..self.clone()
        }
    }

    /// Ends the lesson by setting the end date to today.
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
            end_date: Some(Date::today()),
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

impl Entity for LessonProgress {
    fn id(&self) -> Id {
        self.id
    }
}

impl PartialEq for LessonProgress {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for LessonProgress {}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_progress(name: &str, duration: u64) -> LessonProgress {
        LessonProgress::new(name.to_string(), duration, None, None).unwrap()
    }

    mod constructors {
        use super::*;

        #[test]
        fn test_new_creates_valid_progress() {
            let progress =
                LessonProgress::new("Introduction".to_string(), 1800, None, None).unwrap();

            assert_eq!(progress.lesson_name().as_str(), "Introduction");
            assert_eq!(progress.duration().total_seconds(), 1800);
            assert!(!progress.has_started());
            assert!(!progress.has_ended());
        }

        #[test]
        fn test_new_with_start_date() {
            let start = Date::new(2024, 1, 15).unwrap();
            let progress =
                LessonProgress::new("Variables".to_string(), 3600, Some(start), None).unwrap();

            assert!(progress.has_started());
            assert!(!progress.has_ended());
            assert_eq!(progress.start_date(), Some(start));
        }

        #[test]
        fn test_new_with_both_dates() {
            let start = Date::new(2024, 1, 15).unwrap();
            let end = Date::new(2024, 1, 16).unwrap();
            let progress =
                LessonProgress::new("Completed Lesson".to_string(), 1800, Some(start), Some(end))
                    .unwrap();

            assert!(progress.has_started());
            assert!(progress.has_ended());
            assert!(progress.is_completed());
        }

        #[test]
        fn test_new_rejects_short_name() {
            let result = LessonProgress::new("AB".to_string(), 1800, None, None);

            assert!(matches!(result, Err(LessonProgressError::NameError(_))));
        }

        #[test]
        fn test_new_rejects_empty_name() {
            let result = LessonProgress::new(String::new(), 1800, None, None);

            assert!(matches!(result, Err(LessonProgressError::NameError(_))));
        }

        #[test]
        fn test_new_rejects_zero_duration() {
            let result = LessonProgress::new("Valid Name".to_string(), 0, None, None);

            assert!(matches!(result, Err(LessonProgressError::DurationCantBeZero)));
        }

        #[test]
        fn test_new_accepts_minimum_valid_name() {
            let result = LessonProgress::new("ABC".to_string(), 1, None, None);

            assert!(result.is_ok());
        }

        #[test]
        fn test_new_accepts_maximum_valid_name() {
            let name = "A".repeat(50);
            let result = LessonProgress::new(name, 1, None, None);

            assert!(result.is_ok());
        }

        #[test]
        fn test_new_rejects_name_too_long() {
            let name = "A".repeat(51);
            let result = LessonProgress::new(name, 1, None, None);

            assert!(matches!(result, Err(LessonProgressError::NameError(_))));
        }
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
            let progress = create_test_progress("ID Test", 1800);

            let id = progress.id();
            assert!(id.timestamp_ms() > 0);
        }
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
            let progress = create_test_progress("Started", 1800);
            let started = progress.start();

            assert!(started.has_started());
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
            let progress = create_test_progress("In Progress", 1800);
            let started = progress.start();

            assert!(started.is_in_progress());
        }

        #[test]
        fn test_is_in_progress_false_when_completed() {
            let progress = create_test_progress("Completed", 1800);
            let started = progress.start();
            let ended = started.end().unwrap();

            assert!(!ended.is_in_progress());
        }

        #[test]
        fn test_is_completed_false_initially() {
            let progress = create_test_progress("Not Completed", 1800);

            assert!(!progress.is_completed());
        }

        #[test]
        fn test_is_completed_false_when_only_started() {
            let progress = create_test_progress("Started Only", 1800);
            let started = progress.start();

            assert!(!started.is_completed());
        }

        #[test]
        fn test_is_completed_true_when_started_and_ended() {
            let progress = create_test_progress("Completed", 1800);
            let started = progress.start();
            let ended = started.end().unwrap();

            assert!(ended.is_completed());
        }
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

            assert!(matches!(result, Err(LessonProgressError::CannotEndUnstartedLesson)));
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

    mod entity_trait {
        use super::*;

        #[test]
        fn test_implements_entity_trait() {
            let progress = create_test_progress("Entity Test", 1800);
            let id = progress.id();

            assert!(id.timestamp_ms() > 0);
        }

        #[test]
        fn test_each_instance_has_unique_id() {
            let progress1 = create_test_progress("First", 1800);
            let progress2 = create_test_progress("Second", 1800);

            assert_ne!(progress1.id(), progress2.id());
        }
    }

    mod equality {
        use super::*;

        #[test]
        fn test_equality_based_on_id() {
            let progress = create_test_progress("Equality Test", 1800);
            let cloned = progress.clone();

            assert_eq!(progress, cloned);
        }

        #[test]
        fn test_different_ids_not_equal() {
            let progress1 = create_test_progress("First", 1800);
            let progress2 = create_test_progress("First", 1800);

            assert_ne!(progress1, progress2);
        }

        #[test]
        fn test_started_version_equals_original() {
            let progress = create_test_progress("Started", 1800);
            let started = progress.start();

            assert_eq!(progress, started);
        }
    }

    mod error_messages {
        use super::*;

        #[test]
        fn test_duration_error_message() {
            let error = LessonProgressError::DurationCantBeZero;

            assert_eq!(error.to_string(), "Duration must be different from zero");
        }

        #[test]
        fn test_cannot_end_unstarted_error_message() {
            let error = LessonProgressError::CannotEndUnstartedLesson;

            assert_eq!(error.to_string(), "Cannot end a lesson that has not started");
        }
    }
}
