use crate::{CourseProgress, CourseProgressError};
use education_platform_common::{DateTime, Entity, Id};

impl CourseProgress {
    /// Starts a lesson by setting its start creation_date today.
    ///
    /// If the lesson is already started, it remains unchanged.
    /// If the lesson ID is not found, the progress is returned unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{CourseEnded, CourseProgress, LessonProgress};
    /// use education_platform_common::{DomainEventDispatcher, Entity};
    /// use std::sync::Arc;
    ///
    /// let lesson = LessonProgress::new("Intro".to_string(), 1800, None, None).unwrap();
    /// let lesson_id = lesson.id();
    /// let dispatcher = Arc::new(DomainEventDispatcher::<CourseEnded>::new());
    /// let mut progress = CourseProgress::builder()
    ///     .course_name("Course")
    ///     .user_email("user@example.com")
    ///     .lessons(vec![lesson])
    ///     .event_dispatcher(dispatcher)
    ///     .build()
    ///     .unwrap();
    ///
    /// progress.start_lesson(lesson_id);
    /// assert!(progress.lesson_progress()[0].has_started());
    /// ```
    pub fn start_lesson(&mut self, lesson_id: Id) {
        if let Some(lesson) = self
            .lesson_progress
            .iter_mut()
            .find(|lp| lp.id() == lesson_id)
        {
            lesson.start();
            if self.selected_lesson.id() == lesson_id {
                self.selected_lesson = lesson.clone();
            }
        }

        self.creation_date = Some(DateTime::today());
    }

    /// Ends a lesson by setting its end creation_date today.
    ///
    /// Returns an error if the lesson hasn't been started yet.
    /// If the course is already completed, returns unchanged.
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
    /// let lesson = LessonProgress::new("Intro".to_string(), 1800, None, None).unwrap();
    /// let lesson_id = lesson.id();
    /// let dispatcher = Arc::new(DomainEventDispatcher::<CourseEnded>::new());
    /// let mut progress = CourseProgress::builder()
    ///     .course_name("Course")
    ///     .user_email("user@example.com")
    ///     .lessons(vec![lesson])
    ///     .event_dispatcher(dispatcher)
    ///     .build()
    ///     .unwrap();
    ///
    /// // Must start before ending
    /// progress.start_lesson(lesson_id);
    /// progress.end_lesson(lesson_id).unwrap();
    /// assert!(progress.lesson_progress()[0].has_ended());
    /// ```
    pub fn end_lesson(&mut self, lesson_id: Id) -> Result<(), CourseProgressError> {
        if self.is_completed() {
            return Ok(());
        }

        if let Some(lesson) = self
            .lesson_progress
            .iter_mut()
            .find(|lp| lp.id() == lesson_id)
        {
            lesson.end()?;
            if self.selected_lesson.id() == lesson_id {
                self.selected_lesson = lesson.clone();
            }
        }

        self.creation_date = Some(DateTime::today());
        Ok(())
    }

    /// Restarts a lesson by clearing its start and end dates.
    ///
    /// If the lesson hasn't been started, it remains unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{CourseEnded, CourseProgress, LessonProgress};
    /// use education_platform_common::{DomainEventDispatcher, Entity};
    /// use std::sync::Arc;
    ///
    /// let lesson = LessonProgress::new("Intro".to_string(), 1800, None, None).unwrap();
    /// let lesson_id = lesson.id();
    /// let dispatcher = Arc::new(DomainEventDispatcher::<CourseEnded>::new());
    /// let mut progress = CourseProgress::builder()
    ///     .course_name("Course")
    ///     .user_email("user@example.com")
    ///     .lessons(vec![lesson])
    ///     .event_dispatcher(dispatcher)
    ///     .build()
    ///     .unwrap();
    ///
    /// progress.start_lesson(lesson_id);
    /// progress.restart_lesson(lesson_id);
    /// assert!(!progress.lesson_progress()[0].has_started());
    /// ```
    pub fn restart_lesson(&mut self, lesson_id: Id) {
        if let Some(lesson) = self
            .lesson_progress
            .iter_mut()
            .find(|lp| lp.id() == lesson_id)
        {
            lesson.restart();
            if self.selected_lesson.id() == lesson_id {
                self.selected_lesson = lesson.clone();
            }
        }

        self.creation_date = Some(DateTime::today());
    }

    /// Toggles a lesson's completion status.
    ///
    /// If the lesson is not completed, it will be ended.
    /// If the lesson is completed, it will be restarted.
    ///
    /// # Errors
    ///
    /// Returns `CourseProgressError::LessonNotFound` if the lesson ID doesn't exist.
    /// Returns `CourseProgressError::LessonError` if ending fails (a lesson not started).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{CourseEnded, CourseProgress, LessonProgress};
    /// use education_platform_common::{DateTime, DomainEventDispatcher, Entity};
    /// use std::sync::Arc;
    ///
    /// let start = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
    /// let lesson = LessonProgress::new("Intro".to_string(), 1800, Some(start), None).unwrap();
    /// let lesson_id = lesson.id();
    /// let dispatcher = Arc::new(DomainEventDispatcher::<CourseEnded>::new());
    /// let mut progress = CourseProgress::builder()
    ///     .course_name("Course")
    ///     .user_email("user@example.com")
    ///     .lessons(vec![lesson])
    ///     .event_dispatcher(dispatcher)
    ///     .build()
    ///     .unwrap();
    ///
    /// // Toggle to complete
    /// progress.toggle_lesson_completion(lesson_id).unwrap();
    /// assert!(progress.lesson_progress()[0].is_completed());
    /// ```
    pub fn toggle_lesson_completion(&mut self, lesson_id: Id) -> Result<(), CourseProgressError> {
        let lesson = self.one_lesson_progress(lesson_id)?;

        if !lesson.is_completed() {
            return self.end_lesson(lesson_id);
        }

        self.restart_lesson(lesson_id);
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

    fn create_completed_lesson(name: &str, duration: u64) -> LessonProgress {
        let start = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
        let end = DateTime::new(2024, 1, 2, 10, 0, 0).unwrap();
        LessonProgress::new(name.to_string(), duration, Some(start), Some(end)).unwrap()
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

    mod start_lesson {
        use super::*;

        #[test]
        fn test_start_lesson_sets_start_date() {
            let lesson = create_test_lesson("Lesson", 1800);
            let lesson_id = lesson.id();
            let mut progress = create_progress(vec![lesson]);

            progress.start_lesson(lesson_id);

            assert!(progress.lesson_progress()[0].has_started());
        }

        #[test]
        fn test_start_lesson_updates_date() {
            let lesson = create_test_lesson("Lesson", 1800);
            let lesson_id = lesson.id();
            let mut progress = create_progress(vec![lesson]);

            progress.start_lesson(lesson_id);

            assert!(progress.creation_date().is_some());
        }

        #[test]
        fn test_start_lesson_unknown_id_returns_unchanged() {
            let mut progress = create_test_progress();
            let unknown_id = Id::new();

            progress.start_lesson(unknown_id);

            assert!(!progress.lesson_progress()[0].has_started());
        }

        #[test]
        fn test_start_lesson_idempotent() {
            let lesson = create_test_lesson("Lesson", 1800);
            let lesson_id = lesson.id();
            let mut progress = create_progress(vec![lesson]);

            progress.start_lesson(lesson_id);
            let first_start = progress.lesson_progress()[0].start_date().cloned();
            progress.start_lesson(lesson_id);

            assert_eq!(first_start, progress.lesson_progress()[0].start_date().cloned());
        }
    }

    mod end_lesson {
        use super::*;

        #[test]
        fn test_end_lesson_sets_end_date() {
            let lesson = create_started_lesson("Lesson", 1800);
            let lesson_id = lesson.id();
            let mut progress = create_progress(vec![lesson]);

            progress.end_lesson(lesson_id).unwrap();

            assert!(progress.lesson_progress()[0].has_ended());
        }

        #[test]
        fn test_end_lesson_fails_if_not_started() {
            let lesson = create_test_lesson("Lesson", 1800);
            let lesson_id = lesson.id();
            let mut progress = create_progress(vec![lesson]);

            let result = progress.end_lesson(lesson_id);

            assert!(result.is_err());
        }

        #[test]
        fn test_end_lesson_returns_unchanged_if_course_completed() {
            let lesson = create_completed_lesson("Lesson", 1800);
            let lesson_id = lesson.id();
            let mut progress = create_progress(vec![lesson]);

            progress.end_lesson(lesson_id).unwrap();

            assert!(progress.is_completed());
        }

        #[test]
        fn test_end_lesson_updates_date() {
            let lesson = create_started_lesson("Lesson", 1800);
            let lesson_id = lesson.id();
            let mut progress = create_progress(vec![lesson]);

            progress.end_lesson(lesson_id).unwrap();

            assert!(progress.creation_date().is_some());
        }
    }

    mod restart_lesson {
        use super::*;

        #[test]
        fn test_restart_lesson_clears_dates() {
            let lesson = create_completed_lesson("Lesson", 1800);
            let lesson_id = lesson.id();
            let mut progress = create_progress(vec![lesson]);

            progress.restart_lesson(lesson_id);

            assert!(!progress.lesson_progress()[0].has_started());
            assert!(!progress.lesson_progress()[0].has_ended());
        }

        #[test]
        fn test_restart_lesson_not_started_returns_unchanged() {
            let lesson = create_test_lesson("Lesson", 1800);
            let lesson_id = lesson.id();
            let mut progress = create_progress(vec![lesson]);

            progress.restart_lesson(lesson_id);

            assert!(!progress.lesson_progress()[0].has_started());
        }

        #[test]
        fn test_restart_lesson_updates_date() {
            let lesson = create_completed_lesson("Lesson", 1800);
            let lesson_id = lesson.id();
            let mut progress = create_progress(vec![lesson]);

            progress.restart_lesson(lesson_id);

            assert!(progress.creation_date().is_some());
        }
    }

    mod toggle_lesson_completion {
        use super::*;

        #[test]
        fn test_toggle_ends_started_lesson() {
            let lesson = create_started_lesson("Lesson", 1800);
            let lesson_id = lesson.id();
            let mut progress = create_progress(vec![lesson]);

            progress.toggle_lesson_completion(lesson_id).unwrap();

            assert!(progress.lesson_progress()[0].is_completed());
        }

        #[test]
        fn test_toggle_restarts_completed_lesson() {
            let lesson = create_completed_lesson("Lesson", 1800);
            let lesson_id = lesson.id();
            let mut progress = create_progress(vec![lesson]);

            progress.toggle_lesson_completion(lesson_id).unwrap();

            assert!(!progress.lesson_progress()[0].is_completed());
        }

        #[test]
        fn test_toggle_fails_for_unknown_id() {
            let mut progress = create_test_progress();
            let unknown_id = Id::new();

            let result = progress.toggle_lesson_completion(unknown_id);

            assert!(matches!(result, Err(CourseProgressError::LessonNotFound(_))));
        }
    }
}
