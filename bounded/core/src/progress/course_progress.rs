mod events;
mod fraud_verification;
mod getters;
mod lesson_lifecycle;
mod lesson_navigation;
mod progress_calculations;
mod selected_lesson;

pub use events::CourseEnded;

use crate::{LessonProgress, LessonProgressError};
use education_platform_common::{
    DateTime, DomainEventDispatcher, Duration, Email, EmailError, Entity, Id, SimpleName,
    SimpleNameConfig, SimpleNameError,
};
use std::sync::Arc;
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
/// use education_platform_core::{CourseEnded, CourseProgress, LessonProgress};
/// use education_platform_common::DomainEventDispatcher;
/// use std::sync::Arc;
///
/// let lesson1 = LessonProgress::new("Intro".to_string(), 1800, None, None).unwrap();
/// let lesson2 = LessonProgress::new("Basics".to_string(), 2400, None, None).unwrap();
/// let dispatcher = Arc::new(DomainEventDispatcher::<CourseEnded>::new());
///
/// let progress = CourseProgress::builder()
///     .course_name("Rust Fundamentals")
///     .user_email("user@example.com")
///     .lessons(vec![lesson1, lesson2])
///     .event_dispatcher(dispatcher)
///     .build()
///     .unwrap();
///
/// assert_eq!(progress.course_name().as_str(), "Rust Fundamentals");
/// ```
#[derive(Debug, Clone)]
pub struct CourseProgress {
    id: Id,
    course_name: SimpleName,
    user_email: Email,
    creation_date: Option<DateTime>,
    end_date: Option<DateTime>,
    lesson_progress: Vec<LessonProgress>,
    selected_lesson: LessonProgress,
    event_dispatcher: Arc<DomainEventDispatcher<CourseEnded>>,
}

/// Builder for creating `CourseProgress` instances.
///
/// # Examples
///
/// ```
/// use education_platform_core::{CourseEnded, CourseProgress, CourseProgressBuilder, LessonProgress};
/// use education_platform_common::DomainEventDispatcher;
/// use std::sync::Arc;
///
/// let lesson = LessonProgress::new("Intro".to_string(), 1800, None, None).unwrap();
/// let dispatcher = Arc::new(DomainEventDispatcher::<CourseEnded>::new());
///
/// let progress = CourseProgressBuilder::new()
///     .course_name("Rust Course")
///     .user_email("user@example.com")
///     .lessons(vec![lesson])
///     .event_dispatcher(dispatcher)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct CourseProgressBuilder {
    course_name: Option<String>,
    user_email: Option<String>,
    lessons: Option<Vec<LessonProgress>>,
    creation_date: Option<DateTime>,
    end_date: Option<DateTime>,
    selected_lesson_id: Option<Id>,
    event_dispatcher: Option<Arc<DomainEventDispatcher<CourseEnded>>>,
}

impl Default for CourseProgressBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CourseProgressBuilder {
    /// Creates a new builder instance.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            course_name: None,
            user_email: None,
            lessons: None,
            creation_date: None,
            end_date: None,
            selected_lesson_id: None,
            event_dispatcher: None,
        }
    }

    /// Sets the course name.
    #[must_use]
    pub fn course_name(mut self, name: impl Into<String>) -> Self {
        self.course_name = Some(name.into());
        self
    }

    /// Sets the user email.
    #[must_use]
    pub fn user_email(mut self, email: impl Into<String>) -> Self {
        self.user_email = Some(email.into());
        self
    }

    /// Sets the lesson progress list.
    #[must_use]
    pub fn lessons(mut self, lessons: Vec<LessonProgress>) -> Self {
        self.lessons = Some(lessons);
        self
    }

    /// Sets the creation date.
    #[must_use]
    pub fn creation_date(mut self, date: DateTime) -> Self {
        self.creation_date = Some(date);
        self
    }

    /// Sets the end date.
    #[must_use]
    pub fn end_date(mut self, date: DateTime) -> Self {
        self.end_date = Some(date);
        self
    }

    /// Sets the initially selected lesson by ID.
    #[must_use]
    pub fn selected_lesson_id(mut self, id: Id) -> Self {
        self.selected_lesson_id = Some(id);
        self
    }

    /// Sets the event dispatcher.
    #[must_use]
    pub fn event_dispatcher(mut self, dispatcher: Arc<DomainEventDispatcher<CourseEnded>>) -> Self {
        self.event_dispatcher = Some(dispatcher);
        self
    }

    /// Builds the `CourseProgress` instance.
    ///
    /// # Errors
    ///
    /// Returns `CourseProgressError` if:
    /// - Required fields (course_name, user_email, lessons, event_dispatcher) are missing
    /// - Course name fails validation
    /// - Email fails validation
    /// - A Lessons list is empty
    /// - Selected lesson ID doesn't exist in the lessons list
    pub fn build(self) -> Result<CourseProgress, CourseProgressError> {
        let course_name = self
            .course_name
            .ok_or(SimpleNameError::EmptyValue)
            .and_then(|name| SimpleName::with_config(name, SimpleNameConfig::new(3, 50)))?;

        let user_email = self
            .user_email
            .ok_or(EmailError::FormatNotValid)
            .and_then(Email::new)?;

        let lessons = self.lessons.unwrap_or_default();
        let selected_lesson = CourseProgress::find_lesson_by_id(self.selected_lesson_id, &lessons)?;
        let calculated_end_date = CourseProgress::calculate_end_date(&self.end_date, &lessons);
        let should_publish_ended = calculated_end_date.is_some() && self.end_date.is_none();

        let event_dispatcher = self
            .event_dispatcher
            .unwrap_or_else(|| Arc::new(DomainEventDispatcher::new()));

        let course_progress = CourseProgress {
            id: Id::default(),
            course_name,
            user_email,
            creation_date: self.creation_date,
            end_date: calculated_end_date,
            lesson_progress: lessons,
            selected_lesson,
            event_dispatcher,
        };

        if should_publish_ended {
            course_progress.publish_ended();
        }

        Ok(course_progress)
    }
}

impl CourseProgress {
    /// Creates a new builder for `CourseProgress`.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{CourseEnded, CourseProgress, LessonProgress};
    /// use education_platform_common::DomainEventDispatcher;
    /// use std::sync::Arc;
    ///
    /// let lesson = LessonProgress::new("Intro".to_string(), 1800, None, None).unwrap();
    /// let dispatcher = Arc::new(DomainEventDispatcher::<CourseEnded>::new());
    ///
    /// let progress = CourseProgress::builder()
    ///     .course_name("Rust Course")
    ///     .user_email("user@example.com")
    ///     .lessons(vec![lesson])
    ///     .event_dispatcher(dispatcher)
    ///     .build()
    ///     .unwrap();
    /// ```
    #[inline]
    #[must_use]
    pub fn builder() -> CourseProgressBuilder {
        CourseProgressBuilder::new()
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

    fn create_test_dispatcher() -> Arc<DomainEventDispatcher<CourseEnded>> {
        Arc::new(DomainEventDispatcher::new())
    }

    fn create_test_lesson(name: &str, duration: u64) -> LessonProgress {
        LessonProgress::new(name.to_string(), duration, None, None).unwrap()
    }

    fn create_test_progress() -> CourseProgress {
        let lesson1 = create_test_lesson("Lesson 1", 1800);
        let lesson2 = create_test_lesson("Lesson 2", 2400);
        CourseProgress::builder()
            .course_name("Test Course")
            .user_email("test@example.com")
            .lessons(vec![lesson1, lesson2])
            .event_dispatcher(create_test_dispatcher())
            .build()
            .unwrap()
    }

    mod constructors {
        use super::*;

        #[test]
        fn test_builder_creates_valid_progress() {
            let lesson = create_test_lesson("Introduction", 1800);
            let progress = CourseProgress::builder()
                .course_name("Rust Course")
                .user_email("user@example.com")
                .lessons(vec![lesson])
                .event_dispatcher(create_test_dispatcher())
                .build()
                .unwrap();

            assert_eq!(progress.course_name().as_str(), "Rust Course");
            assert_eq!(progress.user_email().address(), "user@example.com");
            assert!(progress.creation_date().is_none());
        }

        #[test]
        fn test_builder_with_creation_date() {
            let lesson = create_test_lesson("Intro", 1800);
            let date = DateTime::new(2024, 12, 25, 12, 12, 12).unwrap();
            let progress = CourseProgress::builder()
                .course_name("Course")
                .user_email("user@example.com")
                .lessons(vec![lesson])
                .creation_date(date)
                .event_dispatcher(create_test_dispatcher())
                .build()
                .unwrap();

            assert_eq!(progress.creation_date(), Some(date));
        }

        #[test]
        fn test_builder_with_selected_lesson_id() {
            let lesson1 = create_test_lesson("Lesson 1", 1800);
            let lesson2 = create_test_lesson("Lesson 2", 2400);
            let lesson2_id = lesson2.id();

            let progress = CourseProgress::builder()
                .course_name("Course")
                .user_email("user@example.com")
                .lessons(vec![lesson1, lesson2])
                .selected_lesson_id(lesson2_id)
                .event_dispatcher(create_test_dispatcher())
                .build()
                .unwrap();

            assert_eq!(progress.selected_lesson().id(), lesson2_id);
        }

        #[test]
        fn test_builder_rejects_empty_lessons() {
            let result = CourseProgress::builder()
                .course_name("Empty Course")
                .user_email("user@example.com")
                .lessons(vec![])
                .event_dispatcher(create_test_dispatcher())
                .build();

            assert!(matches!(result, Err(CourseProgressError::LessonsCantBeEmpty)));
        }

        #[test]
        fn test_builder_rejects_short_name() {
            let lesson = create_test_lesson("Intro", 1800);
            let result = CourseProgress::builder()
                .course_name("AB")
                .user_email("user@example.com")
                .lessons(vec![lesson])
                .event_dispatcher(create_test_dispatcher())
                .build();

            assert!(matches!(result, Err(CourseProgressError::NameError(_))));
        }

        #[test]
        fn test_builder_rejects_invalid_email() {
            let lesson = create_test_lesson("Intro", 1800);
            let result = CourseProgress::builder()
                .course_name("Valid Course")
                .user_email("invalid-email")
                .lessons(vec![lesson])
                .event_dispatcher(create_test_dispatcher())
                .build();

            assert!(matches!(result, Err(CourseProgressError::EmailError(_))));
        }

        #[test]
        fn test_builder_rejects_unknown_lesson_id() {
            let lesson = create_test_lesson("Intro", 1800);
            let unknown_id = Id::new();

            let result = CourseProgress::builder()
                .course_name("Course")
                .user_email("user@example.com")
                .lessons(vec![lesson])
                .selected_lesson_id(unknown_id)
                .event_dispatcher(create_test_dispatcher())
                .build();

            assert!(matches!(result, Err(CourseProgressError::LessonNotFound(_))));
        }

        #[test]
        fn test_builder_selects_first_lesson_by_default() {
            let lesson1 = create_test_lesson("First", 1800);
            let lesson1_id = lesson1.id();
            let lesson2 = create_test_lesson("Second", 2400);

            let progress = CourseProgress::builder()
                .course_name("Course")
                .user_email("user@example.com")
                .lessons(vec![lesson1, lesson2])
                .event_dispatcher(create_test_dispatcher())
                .build()
                .unwrap();

            assert_eq!(progress.selected_lesson().id(), lesson1_id);
        }

        #[test]
        fn test_builder_generates_unique_id() {
            let progress1 = create_test_progress();
            let progress2 = create_test_progress();

            assert_ne!(progress1.id(), progress2.id());
        }

        #[test]
        fn test_builder_with_name_at_min_length() {
            let lesson = create_test_lesson("Intro", 1800);
            let progress = CourseProgress::builder()
                .course_name("ABC")
                .user_email("user@example.com")
                .lessons(vec![lesson])
                .event_dispatcher(create_test_dispatcher())
                .build()
                .unwrap();

            assert_eq!(progress.course_name().as_str(), "ABC");
        }

        #[test]
        fn test_builder_with_name_at_max_length() {
            let lesson = create_test_lesson("Intro", 1800);
            let name = "A".repeat(50);
            let progress = CourseProgress::builder()
                .course_name(name.clone())
                .user_email("user@example.com")
                .lessons(vec![lesson])
                .event_dispatcher(create_test_dispatcher())
                .build()
                .unwrap();

            assert_eq!(progress.course_name().as_str(), name);
        }

        #[test]
        fn test_builder_with_name_too_long_returns_error() {
            let lesson = create_test_lesson("Intro", 1800);
            let name = "A".repeat(51);
            let result = CourseProgress::builder()
                .course_name(name)
                .user_email("user@example.com")
                .lessons(vec![lesson])
                .event_dispatcher(create_test_dispatcher())
                .build();

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
