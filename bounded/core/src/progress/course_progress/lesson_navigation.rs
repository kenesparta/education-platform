use crate::{CourseProgress, CourseProgressError, LessonProgress};
use education_platform_common::{Entity, Id};

impl CourseProgress {
    /// Selects a different lesson by ID.
    ///
    /// # Errors
    ///
    /// Returns `CourseProgressError::LessonNotFound` if no lesson with the given ID exists.
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
    /// progress.select_lesson(lesson2_id).unwrap();
    /// assert_eq!(progress.selected_lesson().id(), lesson2_id);
    /// ```
    pub fn select_lesson(&mut self, lesson_id: Id) -> Result<(), CourseProgressError> {
        self.selected_lesson = Self::find_lesson_by_id(Some(lesson_id), &self.lesson_progress)?;
        Ok(())
    }

    /// Selects the next lesson in the course.
    ///
    /// If the current lesson is the last one, stays on the last lesson (no change).
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
    /// progress.select_next_lesson();
    /// assert_eq!(progress.selected_lesson().id(), lesson2_id);
    /// ```
    pub fn select_next_lesson(&mut self) {
        let current_index = self
            .lesson_progress
            .iter()
            .position(|lp| lp.id() == self.selected_lesson.id())
            .unwrap_or(0);

        if let Some(next_lesson) = self.lesson_progress.get(current_index + 1) {
            self.selected_lesson = next_lesson.clone();
        }
    }

    /// Selects the previous lesson in the course.
    ///
    /// If the current lesson is the first one, stays on the first lesson (no change).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{CourseEnded, CourseProgress, LessonProgress};
    /// use education_platform_common::{DomainEventDispatcher, Entity};
    /// use std::sync::Arc;
    ///
    /// let lesson1 = LessonProgress::new("Lesson 1".to_string(), 1800, None, None).unwrap();
    /// let lesson1_id = lesson1.id();
    /// let lesson2 = LessonProgress::new("Lesson 2".to_string(), 2400, None, None).unwrap();
    /// let lesson2_id = lesson2.id();
    ///
    /// let dispatcher = Arc::new(DomainEventDispatcher::<CourseEnded>::new());
    /// let mut progress = CourseProgress::builder()
    ///     .course_name("Course")
    ///     .user_email("user@example.com")
    ///     .lessons(vec![lesson1, lesson2])
    ///     .selected_lesson_id(lesson2_id)
    ///     .event_dispatcher(dispatcher)
    ///     .build()
    ///     .unwrap();
    ///
    /// progress.select_previous_lesson();
    /// assert_eq!(progress.selected_lesson().id(), lesson1_id);
    /// ```
    pub fn select_previous_lesson(&mut self) {
        let current_index = self
            .lesson_progress
            .iter()
            .position(|lp| lp.id() == self.selected_lesson.id())
            .unwrap_or(0);

        if current_index == 0 {
            return;
        }

        if let Some(prev_lesson) = self.lesson_progress.get(current_index - 1) {
            self.selected_lesson = prev_lesson.clone();
        }
    }

    /// Retrieves a single lesson progress by ID.
    ///
    /// # Errors
    ///
    /// Returns `CourseProgressError::LessonNotFound` if the lesson ID doesn't exist.
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
    /// let progress = CourseProgress::builder()
    ///     .course_name("Course")
    ///     .user_email("user@example.com")
    ///     .lessons(vec![lesson])
    ///     .event_dispatcher(dispatcher)
    ///     .build()
    ///     .unwrap();
    ///
    /// let found = progress.one_lesson_progress(lesson_id).unwrap();
    /// assert_eq!(found.id(), lesson_id);
    /// ```
    pub fn one_lesson_progress(
        &self,
        lesson_id: Id,
    ) -> Result<LessonProgress, CourseProgressError> {
        self.lesson_progress
            .iter()
            .find(|lp| lp.id() == lesson_id)
            .ok_or_else(|| CourseProgressError::LessonNotFound(lesson_id.to_string()))
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CourseEnded, LessonProgress};
    use education_platform_common::DomainEventDispatcher;
    use std::sync::Arc;

    fn create_test_dispatcher() -> Arc<DomainEventDispatcher<CourseEnded>> {
        Arc::new(DomainEventDispatcher::new())
    }

    fn create_test_lesson(name: &str, duration: u64) -> LessonProgress {
        LessonProgress::new(name.to_string(), duration, None, None).unwrap()
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

    mod select_lesson {
        use super::*;

        #[test]
        fn test_select_lesson_changes_selection() {
            let mut progress = create_test_progress();
            let second_lesson_id = progress.lesson_progress()[1].id();

            progress.select_lesson(second_lesson_id).unwrap();

            assert_eq!(progress.selected_lesson().id(), second_lesson_id);
        }

        #[test]
        fn test_select_lesson_fails_for_unknown_id() {
            let mut progress = create_test_progress();
            let unknown_id = Id::new();

            let result = progress.select_lesson(unknown_id);

            assert!(matches!(result, Err(CourseProgressError::LessonNotFound(_))));
        }

        #[test]
        fn test_select_lesson_preserves_other_fields() {
            let mut progress = create_test_progress();
            let original_name = progress.course_name().as_str().to_string();
            let second_lesson_id = progress.lesson_progress()[1].id();

            progress.select_lesson(second_lesson_id).unwrap();

            assert_eq!(progress.course_name().as_str(), original_name);
            assert_eq!(progress.lesson_progress().len(), 3);
        }
    }

    mod select_next_lesson {
        use super::*;

        #[test]
        fn test_select_next_lesson_moves_to_next() {
            let mut progress = create_test_progress();
            let second_lesson_id = progress.lesson_progress()[1].id();

            progress.select_next_lesson();

            assert_eq!(progress.selected_lesson().id(), second_lesson_id);
        }

        #[test]
        fn test_select_next_lesson_twice_moves_to_third() {
            let mut progress = create_test_progress();
            let third_lesson_id = progress.lesson_progress()[2].id();

            progress.select_next_lesson();
            progress.select_next_lesson();

            assert_eq!(progress.selected_lesson().id(), third_lesson_id);
        }

        #[test]
        fn test_select_next_lesson_at_end_stays_at_end() {
            let mut progress = create_test_progress();
            let last_lesson_id = progress.lesson_progress()[2].id();

            progress.select_next_lesson();
            progress.select_next_lesson();
            progress.select_next_lesson();

            assert_eq!(progress.selected_lesson().id(), last_lesson_id);
        }

        #[test]
        fn test_select_next_lesson_single_lesson_stays() {
            let lesson = create_test_lesson("Only", 1800);
            let lesson_id = lesson.id();
            let mut progress = create_progress(vec![lesson]);

            progress.select_next_lesson();

            assert_eq!(progress.selected_lesson().id(), lesson_id);
        }

        #[test]
        fn test_select_next_lesson_preserves_other_fields() {
            let mut progress = create_test_progress();
            let original_name = progress.course_name().as_str().to_string();

            progress.select_next_lesson();

            assert_eq!(progress.course_name().as_str(), original_name);
            assert_eq!(progress.lesson_progress().len(), 3);
        }
    }

    mod select_previous_lesson {
        use super::*;

        #[test]
        fn test_select_previous_lesson_moves_to_previous() {
            let lesson1 = create_test_lesson("Lesson 1", 1800);
            let lesson1_id = lesson1.id();
            let lesson2 = create_test_lesson("Lesson 2", 2400);
            let lesson2_id = lesson2.id();

            let mut progress = CourseProgress::builder()
                .course_name("Course")
                .user_email("user@example.com")
                .lessons(vec![lesson1, lesson2])
                .selected_lesson_id(lesson2_id)
                .event_dispatcher(create_test_dispatcher())
                .build()
                .unwrap();

            progress.select_previous_lesson();

            assert_eq!(progress.selected_lesson().id(), lesson1_id);
        }

        #[test]
        fn test_select_previous_lesson_at_start_stays_at_start() {
            let mut progress = create_test_progress();
            let first_lesson_id = progress.lesson_progress()[0].id();

            progress.select_previous_lesson();

            assert_eq!(progress.selected_lesson().id(), first_lesson_id);
        }

        #[test]
        fn test_select_previous_lesson_single_lesson_stays() {
            let lesson = create_test_lesson("Only", 1800);
            let lesson_id = lesson.id();
            let mut progress = create_progress(vec![lesson]);

            progress.select_previous_lesson();

            assert_eq!(progress.selected_lesson().id(), lesson_id);
        }

        #[test]
        fn test_select_previous_from_middle_moves_back() {
            let mut progress = create_test_progress();
            let first_id = progress.lesson_progress()[0].id();
            let second_id = progress.lesson_progress()[1].id();

            progress.select_next_lesson();
            assert_eq!(progress.selected_lesson().id(), second_id);

            progress.select_previous_lesson();
            assert_eq!(progress.selected_lesson().id(), first_id);
        }
    }

    mod navigation_integration {
        use super::*;

        #[test]
        fn test_navigate_forward_and_back() {
            let mut progress = create_test_progress();
            let first_id = progress.lesson_progress()[0].id();
            let second_id = progress.lesson_progress()[1].id();
            let third_id = progress.lesson_progress()[2].id();

            progress.select_next_lesson();
            assert_eq!(progress.selected_lesson().id(), second_id);

            progress.select_next_lesson();
            assert_eq!(progress.selected_lesson().id(), third_id);

            progress.select_previous_lesson();
            assert_eq!(progress.selected_lesson().id(), second_id);

            progress.select_previous_lesson();
            assert_eq!(progress.selected_lesson().id(), first_id);
        }
    }

    mod one_lesson_progress {
        use super::*;

        #[test]
        fn test_one_lesson_progress_finds_lesson() {
            let progress = create_test_progress();
            let lesson_id = progress.lesson_progress()[0].id();

            let found = progress.one_lesson_progress(lesson_id).unwrap();

            assert_eq!(found.id(), lesson_id);
        }

        #[test]
        fn test_one_lesson_progress_fails_for_unknown_id() {
            let progress = create_test_progress();
            let unknown_id = Id::new();

            let result = progress.one_lesson_progress(unknown_id);

            assert!(matches!(result, Err(CourseProgressError::LessonNotFound(_))));
        }
    }
}
