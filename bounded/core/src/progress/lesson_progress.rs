mod getters;
mod lifecycle;
mod status_checks;

use education_platform_common::{
    DateTime, Duration, Entity, Id, SimpleName, SimpleNameConfig, SimpleNameError,
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
    start_date: Option<DateTime>,
    end_date: Option<DateTime>,
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
        start_date: Option<DateTime>,
        end_date: Option<DateTime>,
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
            let start = DateTime::new(2024, 1, 15, 10, 30, 0).unwrap();
            let progress =
                LessonProgress::new("Variables".to_string(), 3600, Some(start), None).unwrap();

            assert!(progress.has_started());
            assert!(!progress.has_ended());
            assert_eq!(progress.start_date(), Some(&start));
        }

        #[test]
        fn test_new_with_both_dates() {
            let start = DateTime::new(2024, 1, 15, 10, 0, 0).unwrap();
            let end = DateTime::new(2024, 1, 16, 11, 0, 0).unwrap();
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
