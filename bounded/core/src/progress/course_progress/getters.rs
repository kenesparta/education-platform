use super::{CourseProgress, Date, Email, LessonProgress, SimpleName};

impl CourseProgress {
    /// Returns the course name.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{CourseProgress, LessonProgress};
    ///
    /// let lesson = LessonProgress::new("Intro".to_string(), 1800, None, None).unwrap();
    /// let progress = CourseProgress::new(
    ///     "My Course".to_string(),
    ///     "user@example.com".to_string(),
    ///     vec![lesson],
    ///     None,
    ///     None,
    /// ).unwrap();
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
    /// use education_platform_core::{CourseProgress, LessonProgress};
    ///
    /// let lesson = LessonProgress::new("Intro".to_string(), 1800, None, None).unwrap();
    /// let progress = CourseProgress::new(
    ///     "My Course".to_string(),
    ///     "student@example.com".to_string(),
    ///     vec![lesson],
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// assert_eq!(progress.user_email().address(), "student@example.com");
    /// ```
    #[inline]
    #[must_use]
    pub const fn user_email(&self) -> &Email {
        &self.user_email
    }

    /// Returns the conclusion date if the course has been completed.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{CourseProgress, LessonProgress};
    ///
    /// let lesson = LessonProgress::new("Intro".to_string(), 1800, None, None).unwrap();
    /// let progress = CourseProgress::new(
    ///     "My Course".to_string(),
    ///     "user@example.com".to_string(),
    ///     vec![lesson],
    ///     None,
    ///     None,
    /// ).unwrap();
    ///
    /// assert!(progress.conclusion_date().is_none());
    /// ```
    #[inline]
    #[must_use]
    pub const fn conclusion_date(&self) -> Option<Date> {
        self.date
    }

    /// Returns a reference to all lesson progress records.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{CourseProgress, LessonProgress};
    ///
    /// let lesson1 = LessonProgress::new("Lesson 1".to_string(), 1800, None, None).unwrap();
    /// let lesson2 = LessonProgress::new("Lesson 2".to_string(), 2400, None, None).unwrap();
    /// let progress = CourseProgress::new(
    ///     "My Course".to_string(),
    ///     "user@example.com".to_string(),
    ///     vec![lesson1, lesson2],
    ///     None,
    ///     None,
    /// ).unwrap();
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
    /// use education_platform_core::{CourseProgress, LessonProgress};
    ///
    /// let lesson = LessonProgress::new("Intro".to_string(), 1800, None, None).unwrap();
    /// let progress = CourseProgress::new(
    ///     "My Course".to_string(),
    ///     "user@example.com".to_string(),
    ///     vec![lesson],
    ///     None,
    ///     None,
    /// ).unwrap();
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
    /// use education_platform_core::{CourseProgress, LessonProgress};
    ///
    /// let lesson1 = LessonProgress::new("Lesson 1".to_string(), 1800, None, None).unwrap();
    /// let lesson2 = LessonProgress::new("Lesson 2".to_string(), 2400, None, None).unwrap();
    /// let progress = CourseProgress::new(
    ///     "My Course".to_string(),
    ///     "user@example.com".to_string(),
    ///     vec![lesson1, lesson2],
    ///     None,
    ///     None,
    /// ).unwrap();
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
    use education_platform_common::Entity;

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
            let progress = CourseProgress::new(
                "Rust 101 - Basics".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
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
            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@mail.example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.user_email().address(), "user@mail.example.com");
        }
    }

    mod conclusion_date {
        use super::*;

        #[test]
        fn test_conclusion_date_returns_none_when_not_set() {
            let progress = create_test_progress();

            assert!(progress.conclusion_date().is_none());
        }

        #[test]
        fn test_conclusion_date_returns_date_when_set() {
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

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2, lesson3],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.lesson_progress()[0].lesson_name().as_str(), "First");
            assert_eq!(progress.lesson_progress()[1].lesson_name().as_str(), "Second");
            assert_eq!(progress.lesson_progress()[2].lesson_name().as_str(), "Third");
        }

        #[test]
        fn test_lesson_progress_single_lesson() {
            let lesson = create_test_lesson("Only Lesson", 1800);
            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
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
        fn test_selected_lesson_with_explicit_selection() {
            let lesson1 = create_test_lesson("First", 1800);
            let lesson2 = create_test_lesson("Second", 2400);
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
            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.lesson_count(), 1);
        }

        #[test]
        fn test_lesson_count_many_lessons() {
            let lessons: Vec<LessonProgress> = (1..=10)
                .map(|i| create_test_lesson(&format!("Lesson {}", i), 1800))
                .collect();

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                lessons,
                None,
                None,
            )
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
