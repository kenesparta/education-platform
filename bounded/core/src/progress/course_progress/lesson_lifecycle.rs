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
    /// use education_platform_common::{DateTime, Entity};
    ///
    /// let start = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
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
        let lesson = self.one_lesson_progress(lesson_id)?;

        if !lesson.is_completed() {
            return self.end_lesson(lesson_id);
        }

        Ok(self.restart_lesson(lesson_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LessonProgress;
    use education_platform_common::DateTime;

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
}
