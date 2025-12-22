use super::{CourseProgress, Email, LessonProgress, SimpleName};
use education_platform_common::DateTime;

impl CourseProgress {
    /// Returns the course name.
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
    /// let progress = CourseProgress::builder()
    ///     .course_name("My Course")
    ///     .user_email("user@example.com")
    ///     .lessons(vec![lesson])
    ///     .event_dispatcher(dispatcher)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(progress.course_name().as_str(), "My Course");
    /// ```
    #[inline]
    #[must_use]
    pub const fn course_name(&self) -> &SimpleName {
        &self.course_name
    }

    /// Returns the user's email.
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
    /// let progress = CourseProgress::builder()
    ///     .course_name("My Course")
    ///     .user_email("student@example.com")
    ///     .lessons(vec![lesson])
    ///     .event_dispatcher(dispatcher)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(progress.user_email().address(), "student@example.com");
    /// ```
    #[inline]
    #[must_use]
    pub const fn user_email(&self) -> &Email {
        &self.user_email
    }

    /// Returns the conclusion creation_date if the course has been completed.
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
    /// let progress = CourseProgress::builder()
    ///     .course_name("My Course")
    ///     .user_email("user@example.com")
    ///     .lessons(vec![lesson])
    ///     .event_dispatcher(dispatcher)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert!(progress.creation_date().is_none());
    /// ```
    #[inline]
    #[must_use]
    pub const fn creation_date(&self) -> Option<DateTime> {
        self.creation_date
    }

    /// Returns the end date if the course has been completed.
    ///
    /// The end date is set when all lessons are completed, representing
    /// the date when the last lesson was finished.
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
    /// let progress = CourseProgress::builder()
    ///     .course_name("My Course")
    ///     .user_email("user@example.com")
    ///     .lessons(vec![lesson])
    ///     .event_dispatcher(dispatcher)
    ///     .build()
    ///     .unwrap();
    ///
    /// // Course isn't completed yet, so end_date is None
    /// assert!(progress.end_date().is_none());
    /// ```
    #[inline]
    #[must_use]
    pub const fn end_date(&self) -> Option<DateTime> {
        self.end_date
    }

    /// Returns a reference to all lesson progress records.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{CourseEnded, CourseProgress, LessonProgress};
    /// use education_platform_common::DomainEventDispatcher;
    /// use std::sync::Arc;
    ///
    /// let lesson1 = LessonProgress::new("Lesson 1".to_string(), 1800, None, None).unwrap();
    /// let lesson2 = LessonProgress::new("Lesson 2".to_string(), 2400, None, None).unwrap();
    /// let dispatcher = Arc::new(DomainEventDispatcher::<CourseEnded>::new());
    /// let progress = CourseProgress::builder()
    ///     .course_name("My Course")
    ///     .user_email("user@example.com")
    ///     .lessons(vec![lesson1, lesson2])
    ///     .event_dispatcher(dispatcher)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(progress.lesson_progress().len(), 2);
    /// ```
    #[inline]
    #[must_use]
    pub fn lesson_progress(&self) -> &[LessonProgress] {
        &self.lesson_progress
    }

    /// Returns the currently selected lesson.
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
    /// let progress = CourseProgress::builder()
    ///     .course_name("My Course")
    ///     .user_email("user@example.com")
    ///     .lessons(vec![lesson])
    ///     .event_dispatcher(dispatcher)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(progress.selected_lesson().lesson_name().as_str(), "Intro");
    /// ```
    #[inline]
    #[must_use]
    pub const fn selected_lesson(&self) -> &LessonProgress {
        &self.selected_lesson
    }

    /// Returns the number of lessons in the course.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{CourseEnded, CourseProgress, LessonProgress};
    /// use education_platform_common::DomainEventDispatcher;
    /// use std::sync::Arc;
    ///
    /// let lesson1 = LessonProgress::new("Lesson 1".to_string(), 1800, None, None).unwrap();
    /// let lesson2 = LessonProgress::new("Lesson 2".to_string(), 2400, None, None).unwrap();
    /// let dispatcher = Arc::new(DomainEventDispatcher::<CourseEnded>::new());
    /// let progress = CourseProgress::builder()
    ///     .course_name("My Course")
    ///     .user_email("user@example.com")
    ///     .lessons(vec![lesson1, lesson2])
    ///     .event_dispatcher(dispatcher)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(progress.lesson_count(), 2);
    /// ```
    #[inline]
    #[must_use]
    pub fn lesson_count(&self) -> usize {
        self.lesson_progress.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CourseEnded;
    use education_platform_common::{DomainEventDispatcher, Entity};
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
        CourseProgress::builder()
            .course_name("Test Course")
            .user_email("test@example.com")
            .lessons(vec![lesson1, lesson2])
            .event_dispatcher(create_test_dispatcher())
            .build()
            .unwrap()
    }

    mod course_name {
        use super::*;

        #[test]
        fn test_course_name_returns_name() {
            let progress = create_test_progress();

            assert_eq!(progress.course_name().as_str(), "Test Course");
        }

        #[test]
        fn test_course_name_with_special_characters() {
            let lesson = create_test_lesson("Intro", 1800);
            let progress = CourseProgress::builder()
                .course_name("Rust 101 - Basics")
                .user_email("user@example.com")
                .lessons(vec![lesson])
                .event_dispatcher(create_test_dispatcher())
                .build()
                .unwrap();

            assert_eq!(progress.course_name().as_str(), "Rust 101 - Basics");
        }
    }

    mod user_email {
        use super::*;

        #[test]
        fn test_user_email_returns_email() {
            let progress = create_test_progress();

            assert_eq!(progress.user_email().address(), "test@example.com");
        }

        #[test]
        fn test_user_email_with_subdomain() {
            let lesson = create_test_lesson("Intro", 1800);
            let progress = CourseProgress::builder()
                .course_name("Course")
                .user_email("user@mail.example.com")
                .lessons(vec![lesson])
                .event_dispatcher(create_test_dispatcher())
                .build()
                .unwrap();

            assert_eq!(progress.user_email().address(), "user@mail.example.com");
        }
    }

    mod conclusion_date {
        use super::*;
        use education_platform_common::DateTime;

        #[test]
        fn test_conclusion_date_returns_none_when_not_set() {
            let progress = create_test_progress();

            assert!(progress.creation_date().is_none());
        }

        #[test]
        fn test_conclusion_date_returns_date_when_set() {
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
    }

    mod end_date {
        use super::*;
        use education_platform_common::DateTime;

        fn create_completed_lesson_with_end(
            name: &str,
            duration: u64,
            end: DateTime,
        ) -> LessonProgress {
            let start = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
            LessonProgress::new(name.to_string(), duration, Some(start), Some(end)).unwrap()
        }

        #[test]
        fn test_end_date_returns_none_when_not_completed() {
            let progress = create_test_progress();

            assert!(progress.end_date().is_none());
        }

        #[test]
        fn test_end_date_returns_none_when_lessons_not_started() {
            let lesson = create_test_lesson("Not Started", 1800);
            let progress = CourseProgress::builder()
                .course_name("Course")
                .user_email("user@example.com")
                .lessons(vec![lesson])
                .event_dispatcher(create_test_dispatcher())
                .build()
                .unwrap();

            assert!(progress.end_date().is_none());
        }

        #[test]
        fn test_end_date_returns_explicit_date_when_set() {
            let lesson = create_test_lesson("Lesson", 1800);
            let explicit_end = DateTime::new(2024, 6, 15, 14, 30, 0).unwrap();
            let progress = CourseProgress::builder()
                .course_name("Course")
                .user_email("user@example.com")
                .lessons(vec![lesson])
                .end_date(explicit_end)
                .event_dispatcher(create_test_dispatcher())
                .build()
                .unwrap();

            assert_eq!(progress.end_date(), Some(explicit_end));
        }

        #[test]
        fn test_end_date_calculated_when_all_lessons_completed() {
            let end1 = DateTime::new(2024, 3, 10, 10, 0, 0).unwrap();
            let end2 = DateTime::new(2024, 3, 15, 14, 30, 0).unwrap();

            let lesson1 = create_completed_lesson_with_end("Lesson 1", 1800, end1);
            let lesson2 = create_completed_lesson_with_end("Lesson 2", 2400, end2);

            let progress = CourseProgress::builder()
                .course_name("Course")
                .user_email("user@example.com")
                .lessons(vec![lesson1, lesson2])
                .event_dispatcher(create_test_dispatcher())
                .build()
                .unwrap();

            assert_eq!(progress.end_date(), Some(end2));
        }

        #[test]
        fn test_end_date_returns_latest_lesson_end_date() {
            let early = DateTime::new(2024, 1, 5, 9, 0, 0).unwrap();
            let middle = DateTime::new(2024, 1, 10, 12, 0, 0).unwrap();
            let late = DateTime::new(2024, 1, 20, 18, 0, 0).unwrap();

            let lesson1 = create_completed_lesson_with_end("Early", 1800, early);
            let lesson2 = create_completed_lesson_with_end("Late", 2400, late);
            let lesson3 = create_completed_lesson_with_end("Middle", 3000, middle);

            let progress = CourseProgress::builder()
                .course_name("Course")
                .user_email("user@example.com")
                .lessons(vec![lesson1, lesson2, lesson3])
                .event_dispatcher(create_test_dispatcher())
                .build()
                .unwrap();

            assert_eq!(progress.end_date(), Some(late));
        }

        #[test]
        fn test_end_date_single_completed_lesson() {
            let end = DateTime::new(2024, 5, 20, 16, 45, 0).unwrap();
            let lesson = create_completed_lesson_with_end("Only Lesson", 1800, end);

            let progress = CourseProgress::builder()
                .course_name("Course")
                .user_email("user@example.com")
                .lessons(vec![lesson])
                .event_dispatcher(create_test_dispatcher())
                .build()
                .unwrap();

            assert_eq!(progress.end_date(), Some(end));
        }

        #[test]
        fn test_end_date_none_when_some_lessons_incomplete() {
            let end = DateTime::new(2024, 3, 15, 14, 30, 0).unwrap();
            let completed = create_completed_lesson_with_end("Completed", 1800, end);
            let not_completed = create_test_lesson("Not Completed", 2400);

            let progress = CourseProgress::builder()
                .course_name("Course")
                .user_email("user@example.com")
                .lessons(vec![completed, not_completed])
                .event_dispatcher(create_test_dispatcher())
                .build()
                .unwrap();

            assert!(progress.end_date().is_none());
        }

        #[test]
        fn test_explicit_end_date_takes_priority_over_calculated() {
            let lesson_end = DateTime::new(2024, 3, 15, 14, 30, 0).unwrap();
            let explicit_end = DateTime::new(2024, 6, 1, 12, 0, 0).unwrap();
            let lesson = create_completed_lesson_with_end("Completed", 1800, lesson_end);

            let progress = CourseProgress::builder()
                .course_name("Course")
                .user_email("user@example.com")
                .lessons(vec![lesson])
                .end_date(explicit_end)
                .event_dispatcher(create_test_dispatcher())
                .build()
                .unwrap();

            assert_eq!(progress.end_date(), Some(explicit_end));
        }
    }

    mod lesson_progress {
        use super::*;

        #[test]
        fn test_lesson_progress_returns_all_lessons() {
            let progress = create_test_progress();

            assert_eq!(progress.lesson_progress().len(), 2);
        }

        #[test]
        fn test_lesson_progress_preserves_order() {
            let lesson1 = create_test_lesson("First", 1800);
            let lesson2 = create_test_lesson("Second", 2400);
            let lesson3 = create_test_lesson("Third", 3000);

            let progress = CourseProgress::builder()
                .course_name("Course")
                .user_email("user@example.com")
                .lessons(vec![lesson1, lesson2, lesson3])
                .event_dispatcher(create_test_dispatcher())
                .build()
                .unwrap();

            assert_eq!(progress.lesson_progress()[0].lesson_name().as_str(), "First");
            assert_eq!(progress.lesson_progress()[1].lesson_name().as_str(), "Second");
            assert_eq!(progress.lesson_progress()[2].lesson_name().as_str(), "Third");
        }

        #[test]
        fn test_lesson_progress_single_lesson() {
            let lesson = create_test_lesson("Only Lesson", 1800);
            let progress = CourseProgress::builder()
                .course_name("Course")
                .user_email("user@example.com")
                .lessons(vec![lesson])
                .event_dispatcher(create_test_dispatcher())
                .build()
                .unwrap();

            assert_eq!(progress.lesson_progress().len(), 1);
        }
    }

    mod selected_lesson {
        use super::*;

        #[test]
        fn test_selected_lesson_returns_first_by_default() {
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
        fn test_selected_lesson_with_explicit_selection() {
            let lesson1 = create_test_lesson("First", 1800);
            let lesson2 = create_test_lesson("Second", 2400);
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
    }

    mod lesson_count {
        use super::*;

        #[test]
        fn test_lesson_count_returns_correct_count() {
            let progress = create_test_progress();

            assert_eq!(progress.lesson_count(), 2);
        }

        #[test]
        fn test_lesson_count_single_lesson() {
            let lesson = create_test_lesson("Only", 1800);
            let progress = CourseProgress::builder()
                .course_name("Course")
                .user_email("user@example.com")
                .lessons(vec![lesson])
                .event_dispatcher(create_test_dispatcher())
                .build()
                .unwrap();

            assert_eq!(progress.lesson_count(), 1);
        }

        #[test]
        fn test_lesson_count_many_lessons() {
            let lessons: Vec<LessonProgress> = (1..=10)
                .map(|i| create_test_lesson(&format!("Lesson {}", i), 1800))
                .collect();

            let progress = CourseProgress::builder()
                .course_name("Course")
                .user_email("user@example.com")
                .lessons(lessons)
                .event_dispatcher(create_test_dispatcher())
                .build()
                .unwrap();

            assert_eq!(progress.lesson_count(), 10);
        }
    }

    mod id {
        use super::*;

        #[test]
        fn test_id_returns_valid_id() {
            let progress = create_test_progress();

            assert!(progress.id().timestamp_ms() > 0);
        }

        #[test]
        fn test_each_instance_has_unique_id() {
            let progress1 = create_test_progress();
            let progress2 = create_test_progress();

            assert_ne!(progress1.id(), progress2.id());
        }
    }
}
