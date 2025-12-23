use crate::{CourseProgress, CourseProgressError};
use education_platform_common::Entity;

impl CourseProgress {
    /// Starts the currently selected lesson.
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
    /// let mut progress = CourseProgress::builder()
    ///     .course_name("Course")
    ///     .user_email("user@example.com")
    ///     .lessons(vec![lesson])
    ///     .event_dispatcher(dispatcher)
    ///     .build()
    ///     .unwrap();
    ///
    /// progress.start_selected_lesson();
    /// assert!(progress.selected_lesson().has_started());
    /// ```
    pub fn start_selected_lesson(&mut self) {
        self.start_lesson(self.selected_lesson.id());
    }

    /// Ends the currently selected lesson.
    ///
    /// # Errors
    ///
    /// Returns `CourseProgressError::LessonError` if the lesson hasn't been started.
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
    /// let mut progress = CourseProgress::builder()
    ///     .course_name("Course")
    ///     .user_email("user@example.com")
    ///     .lessons(vec![lesson])
    ///     .event_dispatcher(dispatcher)
    ///     .build()
    ///     .unwrap();
    ///
    /// progress.start_selected_lesson();
    /// progress.end_selected_lesson().unwrap();
    /// assert!(progress.selected_lesson().has_ended());
    /// ```
    pub fn end_selected_lesson(&mut self) -> Result<(), CourseProgressError> {
        self.end_lesson(self.selected_lesson.id())
    }

    /// Ends the currently selected lesson and selects the next one.
    ///
    /// # Errors
    ///
    /// Returns `CourseProgressError::LessonError` if the lesson hasn't been started.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{CourseEnded, CourseProgress, LessonProgress};
    /// use education_platform_common::{DomainEventDispatcher, Entity};
    /// use std::sync::Arc;
    ///
    /// let lesson1 = LessonProgress::new("Lesson 1".to_string(), 1800, None, None).unwrap();
    /// let lesson2 = LessonProgress::new("Lesson 2".to_string(), 2400, None, None).unwrap();
    /// let lesson2_id = lesson2.id();
    ///
    /// let dispatcher = Arc::new(DomainEventDispatcher::<CourseEnded>::new());
    /// let mut progress = CourseProgress::builder()
    ///     .course_name("Course")
    ///     .user_email("user@example.com")
    ///     .lessons(vec![lesson1, lesson2])
    ///     .event_dispatcher(dispatcher)
    ///     .build()
    ///     .unwrap();
    ///
    /// progress.start_selected_lesson();
    /// progress.end_and_select_next_lesson().unwrap();
    /// assert_eq!(progress.selected_lesson().id(), lesson2_id);
    /// ```
    pub fn end_and_select_next_lesson(&mut self) -> Result<(), CourseProgressError> {
        self.end_selected_lesson()?;
        self.select_next_lesson();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CourseEnded, LessonProgress};
    use education_platform_common::{DateTime, DomainEventDispatcher};
    use std::sync::Arc;

    fn create_test_dispatcher() -> Arc<DomainEventDispatcher<CourseEnded>> {
        Arc::new(DomainEventDispatcher::new())
    }

    fn create_test_lesson(name: &str, duration: u64) -> LessonProgress {
        LessonProgress::new(name.to_string(), duration, None, None).unwrap()
    }

    fn create_started_lesson(name: &str, duration: u64) -> LessonProgress {
        let start = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
        LessonProgress::new(name.to_string(), duration, Some(start), None).unwrap()
    }

    fn create_test_progress() -> CourseProgress {
        let lesson1 = create_test_lesson("Lesson 1", 1800);
        let lesson2 = create_test_lesson("Lesson 2", 2400);
        let lesson3 = create_test_lesson("Lesson 3", 3000);
        CourseProgress::builder()
            .course_name("Test Course")
            .user_email("test@example.com")
            .lessons(vec![lesson1, lesson2, lesson3])
            .event_dispatcher(create_test_dispatcher())
            .build()
            .unwrap()
    }

    fn create_progress(lessons: Vec<LessonProgress>) -> CourseProgress {
        CourseProgress::builder()
            .course_name("Course")
            .user_email("user@example.com")
            .lessons(lessons)
            .event_dispatcher(create_test_dispatcher())
            .build()
            .unwrap()
    }

    mod start_selected_lesson {
        use super::*;

        #[test]
        fn test_start_selected_lesson_starts_current() {
            let mut progress = create_test_progress();

            progress.start_selected_lesson();

            assert!(progress.lesson_progress()[0].has_started());
        }

        #[test]
        fn test_start_selected_lesson_preserves_selection() {
            let mut progress = create_test_progress();
            let selected_id = progress.selected_lesson().id();

            progress.start_selected_lesson();

            assert_eq!(progress.selected_lesson().id(), selected_id);
        }
    }

    mod end_selected_lesson {
        use super::*;

        #[test]
        fn test_end_selected_lesson_ends_current() {
            let lesson = create_started_lesson("Lesson", 1800);
            let mut progress = create_progress(vec![lesson]);

            progress.end_selected_lesson().unwrap();

            assert!(progress.lesson_progress()[0].has_ended());
        }

        #[test]
        fn test_end_selected_lesson_fails_if_not_started() {
            let mut progress = create_test_progress();

            let result = progress.end_selected_lesson();

            assert!(result.is_err());
        }
    }

    mod end_and_select_next_lesson {
        use super::*;

        #[test]
        fn test_end_and_select_next_lesson() {
            let lesson1 = create_started_lesson("Lesson 1", 1800);
            let lesson2 = create_test_lesson("Lesson 2", 2400);
            let lesson2_id = lesson2.id();

            let mut progress = create_progress(vec![lesson1, lesson2]);

            progress.end_and_select_next_lesson().unwrap();

            assert!(progress.lesson_progress()[0].has_ended());
            assert_eq!(progress.selected_lesson().id(), lesson2_id);
        }

        #[test]
        fn test_end_and_select_next_lesson_fails_if_not_started() {
            let mut progress = create_test_progress();

            let result = progress.end_and_select_next_lesson();

            assert!(result.is_err());
        }

        #[test]
        fn test_end_and_select_next_lesson_at_last_stays() {
            let lesson1 = create_started_lesson("Lesson 1", 1800);
            let lesson1_id = lesson1.id();

            let mut progress = create_progress(vec![lesson1]);

            progress.end_and_select_next_lesson().unwrap();

            assert!(progress.lesson_progress()[0].has_ended());
            assert_eq!(progress.selected_lesson().id(), lesson1_id);
        }
    }
}
