mod fraud_verification;
mod getters;
mod lesson_lifecycle;
mod lesson_navigation;
mod progress_calculations;
mod selected_lesson;

use crate::{LessonProgress, LessonProgressError};
use education_platform_common::{
    Date, Duration, Email, EmailError, Entity, Id, SimpleName, SimpleNameConfig, SimpleNameError,
};
use thiserror::Error;

/// Error types for Course Progress validation failures.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CourseProgressError {
    #[error("Course name validation failed: {0}")]
    NameError(#[from] SimpleNameError),

    #[error("Email validation failed: {0}")]
    EmailError(#[from] EmailError),

    #[error("Lesson progress error validation failed: {0}")]
    LessonError(#[from] LessonProgressError),

    #[error("Lessons can't be empty. At least one lesson must be added to the course.")]
    LessonsCantBeEmpty,

    #[error("Lesson with id {0} not found")]
    LessonNotFound(String),
}

/// Tracks a user's progress through a course.
///
/// `CourseProgress` is an entity that records which lessons a user has started
/// or completed within a course. It maintains a list of lesson progress records
/// and tracks which lesson is currently selected.
///
/// # Examples
///
/// ```
/// use education_platform_core::{CourseProgress, LessonProgress};
///
/// let lesson1 = LessonProgress::new("Intro".to_string(), 1800, None, None).unwrap();
/// let lesson2 = LessonProgress::new("Basics".to_string(), 2400, None, None).unwrap();
///
/// let progress = CourseProgress::new(
///     "Rust Fundamentals".to_string(),
///     "user@example.com".to_string(),
///     vec![lesson1, lesson2],
///     None,
///     None,
/// ).unwrap();
///
/// assert_eq!(progress.course_name().as_str(), "Rust Fundamentals");
/// ```
#[derive(Debug, Clone)]
pub struct CourseProgress {
    id: Id,
    course_name: SimpleName,
    user_email: Email,
    date: Option<Date>,
    lesson_progress: Vec<LessonProgress>,
    selected_lesson: LessonProgress,
}

impl CourseProgress {
    /// Creates a new `CourseProgress` instance with validated parameters.
    ///
    /// # Errors
    ///
    /// Returns `CourseProgressError::NameError` if the course name fails validation.
    /// Returns `CourseProgressError::EmailError` if the email fails validation.
    /// Returns `CourseProgressError::LessonsCantBeEmpty` if no lessons are provided.
    /// Returns `CourseProgressError::LessonNotFound` if the specified lesson ID doesn't exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{CourseProgress, LessonProgress};
    ///
    /// let lesson = LessonProgress::new("Variables".to_string(), 1800, None, None).unwrap();
    ///
    /// let progress = CourseProgress::new(
    ///     "Rust Course".to_string(),
    ///     "student@example.com".to_string(),
    ///     vec![lesson],
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// // Empty lessons rejected
    /// let empty = CourseProgress::new(
    ///     "Empty Course".to_string(),
    ///     "user@example.com".to_string(),
    ///     vec![],
    ///     None,
    ///     None,
    /// );
    /// assert!(empty.is_err());
    /// ```
    pub fn new(
        course_name: String,
        user_email: String,
        lesson_progress: Vec<LessonProgress>,
        date: Option<Date>,
        selected_lesson_id: Option<Id>,
    ) -> Result<Self, CourseProgressError> {
        let selected_lesson = Self::find_lesson_by_id(selected_lesson_id, &lesson_progress)?;
        let course_name = SimpleName::with_config(course_name, SimpleNameConfig::new(3, 50))?;
        let user_email = Email::new(user_email)?;

        Ok(Self {
            id: Id::default(),
            course_name,
            user_email,
            date,
            lesson_progress,
            selected_lesson,
        })
    }

    /// Finds a lesson by ID, or returns the first lesson if no ID is provided.
    fn find_lesson_by_id(
        lesson_id: Option<Id>,
        lessons: &[LessonProgress],
    ) -> Result<LessonProgress, CourseProgressError> {
        match lesson_id {
            Some(id) => lessons
                .iter()
                .find(|p| p.id() == id)
                .cloned()
                .ok_or_else(|| CourseProgressError::LessonNotFound(id.to_string())),
            None => lessons
                .first()
                .cloned()
                .ok_or(CourseProgressError::LessonsCantBeEmpty),
        }
    }
}

impl Entity for CourseProgress {
    fn id(&self) -> Id {
        self.id
    }
}

impl PartialEq for CourseProgress {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for CourseProgress {}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_lesson(name: &str, duration: u64) -> LessonProgress {
        LessonProgress::new(name.to_string(), duration, None, None).unwrap()
    }

    fn create_test_progress() -> CourseProgress {
        let lesson1 = create_test_lesson("Lesson 1", 1800);
        let lesson2 = create_test_lesson("Lesson 2", 2400);
        CourseProgress::new(
            "Test Course".to_string(),
            "test@example.com".to_string(),
            vec![lesson1, lesson2],
            None,
            None,
        )
        .unwrap()
    }

    mod constructors {
        use super::*;

        #[test]
        fn test_new_creates_valid_progress() {
            let lesson = create_test_lesson("Introduction", 1800);
            let progress = CourseProgress::new(
                "Rust Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.course_name().as_str(), "Rust Course");
            assert_eq!(progress.user_email().address(), "user@example.com");
            assert!(progress.conclusion_date().is_none());
        }

        #[test]
        fn test_new_with_conclusion_date() {
            let lesson = create_test_lesson("Intro", 1800);
            let date = Date::new(2024, 12, 25).unwrap();
            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                Some(date),
                None,
            )
            .unwrap();

            assert_eq!(progress.conclusion_date(), Some(date));
        }

        #[test]
        fn test_new_with_selected_lesson_id() {
            let lesson1 = create_test_lesson("Lesson 1", 1800);
            let lesson2 = create_test_lesson("Lesson 2", 2400);
            let lesson2_id = lesson2.id();

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                Some(lesson2_id),
            )
            .unwrap();

            assert_eq!(progress.selected_lesson().id(), lesson2_id);
        }

        #[test]
        fn test_new_rejects_empty_lessons() {
            let result = CourseProgress::new(
                "Empty Course".to_string(),
                "user@example.com".to_string(),
                vec![],
                None,
                None,
            );

            assert!(matches!(result, Err(CourseProgressError::LessonsCantBeEmpty)));
        }

        #[test]
        fn test_new_rejects_short_name() {
            let lesson = create_test_lesson("Intro", 1800);
            let result = CourseProgress::new(
                "AB".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            );

            assert!(matches!(result, Err(CourseProgressError::NameError(_))));
        }

        #[test]
        fn test_new_rejects_invalid_email() {
            let lesson = create_test_lesson("Intro", 1800);
            let result = CourseProgress::new(
                "Valid Course".to_string(),
                "invalid-email".to_string(),
                vec![lesson],
                None,
                None,
            );

            assert!(matches!(result, Err(CourseProgressError::EmailError(_))));
        }

        #[test]
        fn test_new_rejects_unknown_lesson_id() {
            let lesson = create_test_lesson("Intro", 1800);
            let unknown_id = Id::new();

            let result = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                Some(unknown_id),
            );

            assert!(matches!(result, Err(CourseProgressError::LessonNotFound(_))));
        }

        #[test]
        fn test_new_selects_first_lesson_by_default() {
            let lesson1 = create_test_lesson("First", 1800);
            let lesson1_id = lesson1.id();
            let lesson2 = create_test_lesson("Second", 2400);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.selected_lesson().id(), lesson1_id);
        }

        #[test]
        fn test_new_generates_unique_id() {
            let progress1 = create_test_progress();
            let progress2 = create_test_progress();

            assert_ne!(progress1.id(), progress2.id());
        }

        #[test]
        fn test_new_with_name_at_min_length() {
            let lesson = create_test_lesson("Intro", 1800);
            let progress = CourseProgress::new(
                "ABC".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.course_name().as_str(), "ABC");
        }

        #[test]
        fn test_new_with_name_at_max_length() {
            let lesson = create_test_lesson("Intro", 1800);
            let name = "A".repeat(50);
            let progress = CourseProgress::new(
                name.clone(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.course_name().as_str(), name);
        }

        #[test]
        fn test_new_with_name_too_long_returns_error() {
            let lesson = create_test_lesson("Intro", 1800);
            let name = "A".repeat(51);
            let result =
                CourseProgress::new(name, "user@example.com".to_string(), vec![lesson], None, None);

            assert!(matches!(result, Err(CourseProgressError::NameError(_))));
        }
    }

    mod entity_trait {
        use super::*;

        #[test]
        fn test_implements_entity_trait() {
            let progress = create_test_progress();

            assert!(progress.id().timestamp_ms() > 0);
        }
    }

    mod equality {
        use super::*;

        #[test]
        fn test_equality_based_on_id() {
            let progress = create_test_progress();
            let cloned = progress.clone();

            assert_eq!(progress, cloned);
        }

        #[test]
        fn test_different_ids_not_equal() {
            let progress1 = create_test_progress();
            let progress2 = create_test_progress();

            assert_ne!(progress1, progress2);
        }
    }

    mod error_messages {
        use super::*;

        #[test]
        fn test_lessons_cant_be_empty_message() {
            let error = CourseProgressError::LessonsCantBeEmpty;

            assert_eq!(
                error.to_string(),
                "Lessons can't be empty. At least one lesson must be added to the course."
            );
        }

        #[test]
        fn test_lesson_not_found_message() {
            let error = CourseProgressError::LessonNotFound("ABC123".to_string());

            assert_eq!(error.to_string(), "Lesson with id ABC123 not found");
        }
    }
}
