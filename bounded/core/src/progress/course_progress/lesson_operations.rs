use crate::{CourseProgress, CourseProgressError};
use education_platform_common::{Date, Entity, Id};

impl CourseProgress {
    /// Starts a lesson by setting its start date to today.
    ///
    /// If the lesson is already started, it remains unchanged.
    /// If the lesson ID is not found, the progress is returned unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{CourseProgress, LessonProgress};
    /// use education_platform_common::Entity;
    ///
    /// let lesson = LessonProgress::new("Intro".to_string(), 1800, None, None).unwrap();
    /// let lesson_id = lesson.id();
    /// let progress = CourseProgress::new(
    ///     "Course".to_string(),
    ///     "user@example.com".to_string(),
    ///     vec![lesson],
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// let updated = progress.start_lesson(lesson_id);
    /// assert!(updated.lesson_progress()[0].has_started());
    /// ```
    #[must_use]
    pub fn start_lesson(&self, lesson_id: Id) -> Self {
        let mut new_self = self.clone();
        if let Some(lesson) = new_self
            .lesson_progress
            .iter_mut()
            .find(|lp| lp.id() == lesson_id)
        {
            let started = lesson.start();
            if new_self.selected_lesson.id() == lesson_id {
                new_self.selected_lesson = started.clone();
            }
            *lesson = started;
        }

        new_self.date = Some(Date::today());
        new_self
    }

    /// Ends a lesson by setting its end date to today.
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
    /// use education_platform_core::{CourseProgress, LessonProgress};
    /// use education_platform_common::Entity;
    ///
    /// let lesson = LessonProgress::new("Intro".to_string(), 1800, None, None).unwrap();
    /// let lesson_id = lesson.id();
    /// let progress = CourseProgress::new(
    ///     "Course".to_string(),
    ///     "user@example.com".to_string(),
    ///     vec![lesson],
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// // Must start before ending
    /// let started = progress.start_lesson(lesson_id);
    /// let ended = started.end_lesson(lesson_id).unwrap();
    /// assert!(ended.lesson_progress()[0].has_ended());
    /// ```
    pub fn end_lesson(&self, lesson_id: Id) -> Result<Self, CourseProgressError> {
        let mut new_self = self.clone();
        if self.is_completed() {
            return Ok(new_self);
        }

        if let Some(lesson) = new_self
            .lesson_progress
            .iter_mut()
            .find(|lp| lp.id() == lesson_id)
        {
            let ended = lesson.end()?;
            if new_self.selected_lesson.id() == lesson_id {
                new_self.selected_lesson = ended.clone();
            }
            *lesson = ended;
        }

        new_self.date = Some(Date::today());
        Ok(new_self)
    }

    /// Restarts a lesson by clearing its start and end dates.
    ///
    /// If the lesson hasn't been started, it remains unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{CourseProgress, LessonProgress};
    /// use education_platform_common::Entity;
    ///
    /// let lesson = LessonProgress::new("Intro".to_string(), 1800, None, None).unwrap();
    /// let lesson_id = lesson.id();
    /// let progress = CourseProgress::new(
    ///     "Course".to_string(),
    ///     "user@example.com".to_string(),
    ///     vec![lesson],
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// let started = progress.start_lesson(lesson_id);
    /// let restarted = started.restart_lesson(lesson_id);
    /// assert!(!restarted.lesson_progress()[0].has_started());
    /// ```
    #[must_use]
    pub fn restart_lesson(&self, lesson_id: Id) -> Self {
        let mut new_self = self.clone();

        if let Some(lesson) = new_self
            .lesson_progress
            .iter_mut()
            .find(|lp| lp.id() == lesson_id)
        {
            let restarted = lesson.restart();
            if new_self.selected_lesson.id() == lesson_id {
                new_self.selected_lesson = restarted.clone();
            }
            *lesson = restarted;
        }

        new_self.date = Some(Date::today());
        new_self
    }

    /// Toggles a lesson's completion status.
    ///
    /// If the lesson is not completed, it will be ended.
    /// If the lesson is completed, it will be restarted.
    ///
    /// # Errors
    ///
    /// Returns `CourseProgressError::LessonNotFound` if the lesson ID doesn't exist.
    /// Returns `CourseProgressError::LessonError` if ending fails (lesson not started).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{CourseProgress, LessonProgress};
    /// use education_platform_common::{Date, Entity};
    ///
    /// let start = Date::new(2024, 1, 1).unwrap();
    /// let lesson = LessonProgress::new("Intro".to_string(), 1800, Some(start), None).unwrap();
    /// let lesson_id = lesson.id();
    /// let progress = CourseProgress::new(
    ///     "Course".to_string(),
    ///     "user@example.com".to_string(),
    ///     vec![lesson],
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// // Toggle to complete
    /// let completed = progress.toggle_lesson_completion(lesson_id).unwrap();
    /// assert!(completed.lesson_progress()[0].is_completed());
    /// ```
    pub fn toggle_lesson_completion(&self, lesson_id: Id) -> Result<Self, CourseProgressError> {
        let lesson = self
            .lesson_progress
            .iter()
            .find(|lp| lp.id() == lesson_id)
            .ok_or_else(|| CourseProgressError::LessonNotFound(lesson_id.to_string()))?;

        if !lesson.is_completed() {
            return self.end_lesson(lesson_id);
        }

        Ok(self.restart_lesson(lesson_id))
    }

    /// Starts the currently selected lesson.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{CourseProgress, LessonProgress};
    ///
    /// let lesson = LessonProgress::new("Intro".to_string(), 1800, None, None).unwrap();
    /// let progress = CourseProgress::new(
    ///     "Course".to_string(),
    ///     "user@example.com".to_string(),
    ///     vec![lesson],
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// let started = progress.start_selected_lesson();
    /// assert!(started.selected_lesson().has_started());
    /// ```
    #[must_use]
    pub fn start_selected_lesson(&self) -> Self {
        self.start_lesson(self.selected_lesson.id())
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
    /// use education_platform_core::{CourseProgress, LessonProgress};
    ///
    /// let lesson = LessonProgress::new("Intro".to_string(), 1800, None, None).unwrap();
    /// let progress = CourseProgress::new(
    ///     "Course".to_string(),
    ///     "user@example.com".to_string(),
    ///     vec![lesson],
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// let started = progress.start_selected_lesson();
    /// let ended = started.end_selected_lesson().unwrap();
    /// assert!(ended.selected_lesson().has_ended());
    /// ```
    pub fn end_selected_lesson(&self) -> Result<Self, CourseProgressError> {
        self.end_lesson(self.selected_lesson.id())
    }

    /// Selects a different lesson by ID.
    ///
    /// # Errors
    ///
    /// Returns `CourseProgressError::LessonNotFound` if no lesson with the given ID exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{CourseProgress, LessonProgress};
    /// use education_platform_common::Entity;
    ///
    /// let lesson1 = LessonProgress::new("Lesson 1".to_string(), 1800, None, None).unwrap();
    /// let lesson2 = LessonProgress::new("Lesson 2".to_string(), 2400, None, None).unwrap();
    /// let lesson2_id = lesson2.id();
    ///
    /// let progress = CourseProgress::new(
    ///     "Course".to_string(),
    ///     "user@example.com".to_string(),
    ///     vec![lesson1, lesson2],
    ///     None,
    ///     None,
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
    /// use education_platform_core::{CourseProgress, LessonProgress};
    /// use education_platform_common::Entity;
    ///
    /// let lesson1 = LessonProgress::new("Lesson 1".to_string(), 1800, None, None).unwrap();
    /// let lesson2 = LessonProgress::new("Lesson 2".to_string(), 2400, None, None).unwrap();
    /// let lesson2_id = lesson2.id();
    ///
    /// let progress = CourseProgress::new(
    ///     "Course".to_string(),
    ///     "user@example.com".to_string(),
    ///     vec![lesson1, lesson2],
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// let next = progress.select_next_lesson();
    /// assert_eq!(next.selected_lesson().id(), lesson2_id);
    /// ```
    #[must_use]
    pub fn select_next_lesson(&self) -> Self {
        let current_index = self.find_selected_lesson_index();

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
    /// use education_platform_core::{CourseProgress, LessonProgress};
    /// use education_platform_common::Entity;
    ///
    /// let lesson1 = LessonProgress::new("Lesson 1".to_string(), 1800, None, None).unwrap();
    /// let lesson1_id = lesson1.id();
    /// let lesson2 = LessonProgress::new("Lesson 2".to_string(), 2400, None, None).unwrap();
    /// let lesson2_id = lesson2.id();
    ///
    /// let progress = CourseProgress::new(
    ///     "Course".to_string(),
    ///     "user@example.com".to_string(),
    ///     vec![lesson1, lesson2],
    ///     None,
    ///     Some(lesson2_id),
    /// ).unwrap();
    ///
    /// let prev = progress.select_previous_lesson();
    /// assert_eq!(prev.selected_lesson().id(), lesson1_id);
    /// ```
    #[must_use]
    pub fn select_previous_lesson(&self) -> Self {
        let current_index = self.find_selected_lesson_index();

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

    /// Returns true if the selected lesson is the first lesson.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{CourseProgress, LessonProgress};
    ///
    /// let lesson1 = LessonProgress::new("Lesson 1".to_string(), 1800, None, None).unwrap();
    /// let lesson2 = LessonProgress::new("Lesson 2".to_string(), 2400, None, None).unwrap();
    ///
    /// let progress = CourseProgress::new(
    ///     "Course".to_string(),
    ///     "user@example.com".to_string(),
    ///     vec![lesson1, lesson2],
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// assert!(progress.is_first_lesson_selected());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_first_lesson_selected(&self) -> bool {
        self.find_selected_lesson_index() == 0
    }

    /// Returns true if the selected lesson is the last lesson.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{CourseProgress, LessonProgress};
    /// use education_platform_common::Entity;
    ///
    /// let lesson1 = LessonProgress::new("Lesson 1".to_string(), 1800, None, None).unwrap();
    /// let lesson2 = LessonProgress::new("Lesson 2".to_string(), 2400, None, None).unwrap();
    /// let lesson2_id = lesson2.id();
    ///
    /// let progress = CourseProgress::new(
    ///     "Course".to_string(),
    ///     "user@example.com".to_string(),
    ///     vec![lesson1, lesson2],
    ///     None,
    ///     Some(lesson2_id),
    /// ).unwrap();
    ///
    /// assert!(progress.is_last_lesson_selected());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_last_lesson_selected(&self) -> bool {
        self.find_selected_lesson_index() == self.lesson_progress.len().saturating_sub(1)
    }

    /// Finds the index of the currently selected lesson.
    fn find_selected_lesson_index(&self) -> usize {
        self.lesson_progress
            .iter()
            .position(|lp| lp.id() == self.selected_lesson.id())
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LessonProgress;
    use education_platform_common::Date;

    fn create_test_lesson(name: &str, duration: u64) -> LessonProgress {
        LessonProgress::new(name.to_string(), duration, None, None).unwrap()
    }

    fn create_started_lesson(name: &str, duration: u64) -> LessonProgress {
        let start = Date::new(2024, 1, 1).unwrap();
        LessonProgress::new(name.to_string(), duration, Some(start), None).unwrap()
    }

    fn create_completed_lesson(name: &str, duration: u64) -> LessonProgress {
        let start = Date::new(2024, 1, 1).unwrap();
        let end = Date::new(2024, 1, 2).unwrap();
        LessonProgress::new(name.to_string(), duration, Some(start), Some(end)).unwrap()
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
        )
        .unwrap()
    }

    mod start_lesson {
        use super::*;

        #[test]
        fn test_start_lesson_sets_start_date() {
            let lesson = create_test_lesson("Lesson", 1800);
            let lesson_id = lesson.id();
            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

            let updated = progress.start_lesson(lesson_id);

            assert!(updated.lesson_progress()[0].has_started());
        }

        #[test]
        fn test_start_lesson_updates_date() {
            let lesson = create_test_lesson("Lesson", 1800);
            let lesson_id = lesson.id();
            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

            let updated = progress.start_lesson(lesson_id);

            assert!(updated.conclusion_date().is_some());
        }

        #[test]
        fn test_start_lesson_unknown_id_returns_unchanged() {
            let progress = create_test_progress();
            let unknown_id = Id::new();

            let updated = progress.start_lesson(unknown_id);

            assert!(!updated.lesson_progress()[0].has_started());
        }

        #[test]
        fn test_start_lesson_idempotent() {
            let lesson = create_test_lesson("Lesson", 1800);
            let lesson_id = lesson.id();
            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

            let started1 = progress.start_lesson(lesson_id);
            let started2 = started1.start_lesson(lesson_id);

            assert_eq!(
                started1.lesson_progress()[0].start_date(),
                started2.lesson_progress()[0].start_date()
            );
        }

        #[test]
        fn test_start_lesson_does_not_modify_original() {
            let lesson = create_test_lesson("Lesson", 1800);
            let lesson_id = lesson.id();
            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

            let _updated = progress.start_lesson(lesson_id);

            assert!(!progress.lesson_progress()[0].has_started());
        }
    }

    mod end_lesson {
        use super::*;

        #[test]
        fn test_end_lesson_sets_end_date() {
            let lesson = create_started_lesson("Lesson", 1800);
            let lesson_id = lesson.id();
            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

            let updated = progress.end_lesson(lesson_id).unwrap();

            assert!(updated.lesson_progress()[0].has_ended());
        }

        #[test]
        fn test_end_lesson_fails_if_not_started() {
            let lesson = create_test_lesson("Lesson", 1800);
            let lesson_id = lesson.id();
            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

            let result = progress.end_lesson(lesson_id);

            assert!(result.is_err());
        }

        #[test]
        fn test_end_lesson_returns_unchanged_if_course_completed() {
            let lesson = create_completed_lesson("Lesson", 1800);
            let lesson_id = lesson.id();
            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

            let updated = progress.end_lesson(lesson_id).unwrap();

            assert!(updated.is_completed());
        }

        #[test]
        fn test_end_lesson_updates_date() {
            let lesson = create_started_lesson("Lesson", 1800);
            let lesson_id = lesson.id();
            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

            let updated = progress.end_lesson(lesson_id).unwrap();

            assert!(updated.conclusion_date().is_some());
        }
    }

    mod restart_lesson {
        use super::*;

        #[test]
        fn test_restart_lesson_clears_dates() {
            let lesson = create_completed_lesson("Lesson", 1800);
            let lesson_id = lesson.id();
            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

            let updated = progress.restart_lesson(lesson_id);

            assert!(!updated.lesson_progress()[0].has_started());
            assert!(!updated.lesson_progress()[0].has_ended());
        }

        #[test]
        fn test_restart_lesson_not_started_returns_unchanged() {
            let lesson = create_test_lesson("Lesson", 1800);
            let lesson_id = lesson.id();
            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

            let updated = progress.restart_lesson(lesson_id);

            assert!(!updated.lesson_progress()[0].has_started());
        }

        #[test]
        fn test_restart_lesson_updates_date() {
            let lesson = create_completed_lesson("Lesson", 1800);
            let lesson_id = lesson.id();
            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

            let updated = progress.restart_lesson(lesson_id);

            assert!(updated.conclusion_date().is_some());
        }
    }

    mod toggle_lesson_completion {
        use super::*;

        #[test]
        fn test_toggle_ends_started_lesson() {
            let lesson = create_started_lesson("Lesson", 1800);
            let lesson_id = lesson.id();
            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

            let updated = progress.toggle_lesson_completion(lesson_id).unwrap();

            assert!(updated.lesson_progress()[0].is_completed());
        }

        #[test]
        fn test_toggle_restarts_completed_lesson() {
            let lesson = create_completed_lesson("Lesson", 1800);
            let lesson_id = lesson.id();
            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

            let updated = progress.toggle_lesson_completion(lesson_id).unwrap();

            assert!(!updated.lesson_progress()[0].is_completed());
        }

        #[test]
        fn test_toggle_fails_for_unknown_id() {
            let progress = create_test_progress();
            let unknown_id = Id::new();

            let result = progress.toggle_lesson_completion(unknown_id);

            assert!(matches!(result, Err(CourseProgressError::LessonNotFound(_))));
        }
    }

    mod start_selected_lesson {
        use super::*;

        #[test]
        fn test_start_selected_lesson_starts_current() {
            let progress = create_test_progress();

            let updated = progress.start_selected_lesson();

            assert!(updated.lesson_progress()[0].has_started());
        }

        #[test]
        fn test_start_selected_lesson_preserves_selection() {
            let progress = create_test_progress();
            let selected_id = progress.selected_lesson().id();

            let updated = progress.start_selected_lesson();

            assert_eq!(updated.selected_lesson().id(), selected_id);
        }
    }

    mod end_selected_lesson {
        use super::*;

        #[test]
        fn test_end_selected_lesson_ends_current() {
            let lesson = create_started_lesson("Lesson", 1800);
            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

            let updated = progress.end_selected_lesson().unwrap();

            assert!(updated.lesson_progress()[0].has_ended());
        }

        #[test]
        fn test_end_selected_lesson_fails_if_not_started() {
            let progress = create_test_progress();

            let result = progress.end_selected_lesson();

            assert!(result.is_err());
        }
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
            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

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
            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

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

    mod is_first_lesson_selected {
        use super::*;

        #[test]
        fn test_is_first_lesson_selected_true_at_start() {
            let progress = create_test_progress();

            assert!(progress.is_first_lesson_selected());
        }

        #[test]
        fn test_is_first_lesson_selected_false_after_next() {
            let progress = create_test_progress();

            let updated = progress.select_next_lesson();

            assert!(!updated.is_first_lesson_selected());
        }

        #[test]
        fn test_is_first_lesson_selected_true_single_lesson() {
            let lesson = create_test_lesson("Only", 1800);
            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

            assert!(progress.is_first_lesson_selected());
        }
    }

    mod is_last_lesson_selected {
        use super::*;

        #[test]
        fn test_is_last_lesson_selected_false_at_start() {
            let progress = create_test_progress();

            assert!(!progress.is_last_lesson_selected());
        }

        #[test]
        fn test_is_last_lesson_selected_true_at_end() {
            let progress = create_test_progress();

            let at_end = progress.select_next_lesson().select_next_lesson();

            assert!(at_end.is_last_lesson_selected());
        }

        #[test]
        fn test_is_last_lesson_selected_true_single_lesson() {
            let lesson = create_test_lesson("Only", 1800);
            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

            assert!(progress.is_last_lesson_selected());
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

        #[test]
        fn test_boundary_conditions() {
            let progress = create_test_progress();
            let first_id = progress.lesson_progress()[0].id();
            let last_id = progress.lesson_progress()[2].id();

            let at_start = progress.select_previous_lesson();
            assert_eq!(at_start.selected_lesson().id(), first_id);
            assert!(at_start.is_first_lesson_selected());

            let at_end = progress
                .select_next_lesson()
                .select_next_lesson()
                .select_next_lesson();
            assert_eq!(at_end.selected_lesson().id(), last_id);
            assert!(at_end.is_last_lesson_selected());
        }
    }
}
