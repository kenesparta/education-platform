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
    /// let progress = CourseProgress::new(
    ///     "Course".to_string(),
    ///     "user@example.com".to_string(),
    ///     vec![lesson1, lesson2],
    ///     None,
    ///     None,
    ///     dispatcher,
    /// ).unwrap();
    ///
    /// let updated = progress.select_lesson(lesson2_id).unwrap();
    /// assert_eq!(updated.selected_lesson().id(), lesson2_id);
    /// ```
    pub fn select_lesson(&self, lesson_id: Id) -> Result<Self, CourseProgressError> {
        let selected_lesson = Self::find_lesson_by_id(Some(lesson_id), &self.lesson_progress)?;
        Ok(Self {
            selected_lesson,
            ..self.clone()
        })
    }

    /// Selects the next lesson in the course.
    ///
    /// If the current lesson is the last one, returns unchanged (stays on last lesson).
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
    /// let progress = CourseProgress::new(
    ///     "Course".to_string(),
    ///     "user@example.com".to_string(),
    ///     vec![lesson1, lesson2],
    ///     None,
    ///     None,
    ///     dispatcher,
    /// ).unwrap();
    ///
    /// let next = progress.select_next_lesson();
    /// assert_eq!(next.selected_lesson().id(), lesson2_id);
    /// ```
    #[must_use]
    pub fn select_next_lesson(&self) -> Self {
        let current_index = self
            .lesson_progress
            .iter()
            .position(|lp| lp.id() == self.selected_lesson.id())
            .unwrap_or(0);

        match self.lesson_progress.get(current_index + 1) {
            Some(next_lesson) => Self {
                selected_lesson: next_lesson.clone(),
                ..self.clone()
            },
            None => self.clone(),
        }
    }

    /// Selects the previous lesson in the course.
    ///
    /// If the current lesson is the first one, returns unchanged (stays on first lesson).
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
    /// let progress = CourseProgress::new(
    ///     "Course".to_string(),
    ///     "user@example.com".to_string(),
    ///     vec![lesson1, lesson2],
    ///     None,
    ///     Some(lesson2_id),
    ///     dispatcher,
    /// ).unwrap();
    ///
    /// let prev = progress.select_previous_lesson();
    /// assert_eq!(prev.selected_lesson().id(), lesson1_id);
    /// ```
    #[must_use]
    pub fn select_previous_lesson(&self) -> Self {
        let current_index = self
            .lesson_progress
            .iter()
            .position(|lp| lp.id() == self.selected_lesson.id())
            .unwrap_or(0);

        if current_index == 0 {
            return self.clone();
        }

        match self.lesson_progress.get(current_index - 1) {
            Some(prev_lesson) => Self {
                selected_lesson: prev_lesson.clone(),
                ..self.clone()
            },
            None => self.clone(),
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
    /// let progress = CourseProgress::new(
    ///     "Course".to_string(),
    ///     "user@example.com".to_string(),
    ///     vec![lesson],
    ///     None,
    ///     None,
    ///     dispatcher,
    /// ).unwrap();
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
        CourseProgress::new(
            "Test Course".to_string(),
            "test@example.com".to_string(),
            vec![lesson1, lesson2, lesson3],
            None,
            None,
            create_test_dispatcher(),
        )
        .unwrap()
    }

    fn create_progress(lessons: Vec<LessonProgress>) -> CourseProgress {
        CourseProgress::new(
            "Course".to_string(),
            "user@example.com".to_string(),
            lessons,
            None,
            None,
            create_test_dispatcher(),
        )
        .unwrap()
    }

    mod select_lesson {
        use super::*;

        #[test]
        fn test_select_lesson_changes_selection() {
            let progress = create_test_progress();
            let second_lesson_id = progress.lesson_progress()[1].id();

            let updated = progress.select_lesson(second_lesson_id).unwrap();

            assert_eq!(updated.selected_lesson().id(), second_lesson_id);
        }

        #[test]
        fn test_select_lesson_fails_for_unknown_id() {
            let progress = create_test_progress();
            let unknown_id = Id::new();

            let result = progress.select_lesson(unknown_id);

            assert!(matches!(result, Err(CourseProgressError::LessonNotFound(_))));
        }

        #[test]
        fn test_select_lesson_preserves_other_fields() {
            let progress = create_test_progress();
            let second_lesson_id = progress.lesson_progress()[1].id();

            let updated = progress.select_lesson(second_lesson_id).unwrap();

            assert_eq!(updated.course_name().as_str(), progress.course_name().as_str());
            assert_eq!(updated.lesson_progress().len(), 3);
        }
    }

    mod select_next_lesson {
        use super::*;

        #[test]
        fn test_select_next_lesson_moves_to_next() {
            let progress = create_test_progress();
            let second_lesson_id = progress.lesson_progress()[1].id();

            let updated = progress.select_next_lesson();

            assert_eq!(updated.selected_lesson().id(), second_lesson_id);
        }

        #[test]
        fn test_select_next_lesson_twice_moves_to_third() {
            let progress = create_test_progress();
            let third_lesson_id = progress.lesson_progress()[2].id();

            let updated = progress.select_next_lesson().select_next_lesson();

            assert_eq!(updated.selected_lesson().id(), third_lesson_id);
        }

        #[test]
        fn test_select_next_lesson_at_end_stays_at_end() {
            let progress = create_test_progress();
            let last_lesson_id = progress.lesson_progress()[2].id();

            let at_end = progress.select_next_lesson().select_next_lesson();
            let still_at_end = at_end.select_next_lesson();

            assert_eq!(still_at_end.selected_lesson().id(), last_lesson_id);
        }

        #[test]
        fn test_select_next_lesson_single_lesson_stays() {
            let lesson = create_test_lesson("Only", 1800);
            let lesson_id = lesson.id();
            let progress = create_progress(vec![lesson]);

            let updated = progress.select_next_lesson();

            assert_eq!(updated.selected_lesson().id(), lesson_id);
        }

        #[test]
        fn test_select_next_lesson_preserves_other_fields() {
            let progress = create_test_progress();

            let updated = progress.select_next_lesson();

            assert_eq!(updated.course_name().as_str(), progress.course_name().as_str());
            assert_eq!(updated.lesson_progress().len(), 3);
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

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                Some(lesson2_id),
                create_test_dispatcher(),
            )
            .unwrap();

            let updated = progress.select_previous_lesson();

            assert_eq!(updated.selected_lesson().id(), lesson1_id);
        }

        #[test]
        fn test_select_previous_lesson_at_start_stays_at_start() {
            let progress = create_test_progress();
            let first_lesson_id = progress.lesson_progress()[0].id();

            let updated = progress.select_previous_lesson();

            assert_eq!(updated.selected_lesson().id(), first_lesson_id);
        }

        #[test]
        fn test_select_previous_lesson_single_lesson_stays() {
            let lesson = create_test_lesson("Only", 1800);
            let lesson_id = lesson.id();
            let progress = create_progress(vec![lesson]);

            let updated = progress.select_previous_lesson();

            assert_eq!(updated.selected_lesson().id(), lesson_id);
        }

        #[test]
        fn test_select_previous_from_middle_moves_back() {
            let progress = create_test_progress();
            let first_id = progress.lesson_progress()[0].id();
            let second_id = progress.lesson_progress()[1].id();

            let at_second = progress.select_next_lesson();
            assert_eq!(at_second.selected_lesson().id(), second_id);

            let back_to_first = at_second.select_previous_lesson();
            assert_eq!(back_to_first.selected_lesson().id(), first_id);
        }
    }

    mod navigation_integration {
        use super::*;

        #[test]
        fn test_navigate_forward_and_back() {
            let progress = create_test_progress();
            let first_id = progress.lesson_progress()[0].id();
            let second_id = progress.lesson_progress()[1].id();
            let third_id = progress.lesson_progress()[2].id();

            let step1 = progress.select_next_lesson();
            assert_eq!(step1.selected_lesson().id(), second_id);

            let step2 = step1.select_next_lesson();
            assert_eq!(step2.selected_lesson().id(), third_id);

            let step3 = step2.select_previous_lesson();
            assert_eq!(step3.selected_lesson().id(), second_id);

            let step4 = step3.select_previous_lesson();
            assert_eq!(step4.selected_lesson().id(), first_id);
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
