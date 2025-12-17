use super::{CourseProgress, Duration};

impl CourseProgress {
    /// Returns true if all lessons in the course have been completed.
    ///
    /// A course is considered completed when every lesson has both a start
    /// date and an end date.
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
    /// assert!(!progress.is_completed());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_completed(&self) -> bool {
        self.lesson_progress.iter().all(|p| p.is_completed())
    }

    /// Returns the total duration of all lessons in the course.
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
    /// assert_eq!(progress.total_duration().total_seconds(), 4200);
    /// ```
    #[must_use]
    pub fn total_duration(&self) -> Duration {
        self.lesson_progress
            .iter()
            .fold(Duration::default(), |acc, lp| acc.add(&lp.duration()))
    }

    /// Returns the total duration of all completed (ended) lessons.
    ///
    /// Only lessons that have been ended are included in this calculation.
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
    /// // No lessons ended yet
    /// assert_eq!(progress.duration_lessons_ended().total_seconds(), 0);
    /// ```
    #[must_use]
    pub fn duration_lessons_ended(&self) -> Duration {
        self.lesson_progress
            .iter()
            .filter(|lp| lp.has_ended())
            .fold(Duration::default(), |acc, lp| acc.add(&lp.duration()))
    }

    /// Returns the percentage of the course that has been completed.
    ///
    /// The percentage is calculated based on the duration of ended lessons
    /// divided by the total duration of all lessons.
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
    /// // No lessons ended yet
    /// assert_eq!(progress.percentage_completed(), 0);
    /// ```
    #[must_use]
    pub fn percentage_completed(&self) -> u64 {
        let total_duration = self.total_duration().total_seconds();
        if total_duration == 0 {
            return 0;
        }

        let duration_lessons_ended = self.duration_lessons_ended().total_seconds();
        duration_lessons_ended * 100 / total_duration
    }

    /// Returns the number of lessons that have been started.
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
    /// assert_eq!(progress.lessons_started_count(), 0);
    /// ```
    #[inline]
    #[must_use]
    pub fn lessons_started_count(&self) -> usize {
        self.lesson_progress
            .iter()
            .filter(|lp| lp.has_started())
            .count()
    }

    /// Returns the number of lessons that have been completed.
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
    /// assert_eq!(progress.lessons_completed_count(), 0);
    /// ```
    #[inline]
    #[must_use]
    pub fn lessons_completed_count(&self) -> usize {
        self.lesson_progress
            .iter()
            .filter(|lp| lp.is_completed())
            .count()
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
        CourseProgress::new(
            "Test Course".to_string(),
            "test@example.com".to_string(),
            vec![lesson1, lesson2],
            None,
            None,
        )
        .unwrap()
    }

    mod is_completed {
        use super::*;

        #[test]
        fn test_is_completed_false_when_no_lessons_completed() {
            let progress = create_test_progress();

            assert!(!progress.is_completed());
        }

        #[test]
        fn test_is_completed_false_when_some_lessons_completed() {
            let lesson1 = create_completed_lesson("Completed", 1800);
            let lesson2 = create_test_lesson("Not Completed", 2400);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            assert!(!progress.is_completed());
        }

        #[test]
        fn test_is_completed_true_when_all_lessons_completed() {
            let lesson1 = create_completed_lesson("Lesson 1", 1800);
            let lesson2 = create_completed_lesson("Lesson 2", 2400);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            assert!(progress.is_completed());
        }

        #[test]
        fn test_is_completed_single_completed_lesson() {
            let lesson = create_completed_lesson("Only Lesson", 1800);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

            assert!(progress.is_completed());
        }

        #[test]
        fn test_is_completed_started_but_not_ended() {
            let lesson = create_started_lesson("Started Only", 1800);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

            assert!(!progress.is_completed());
        }
    }

    mod total_duration {
        use super::*;

        #[test]
        fn test_total_duration_single_lesson() {
            let lesson = create_test_lesson("Lesson", 3600);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.total_duration().total_seconds(), 3600);
        }

        #[test]
        fn test_total_duration_multiple_lessons() {
            let progress = create_test_progress();

            assert_eq!(progress.total_duration().total_seconds(), 4200);
        }

        #[test]
        fn test_total_duration_many_lessons() {
            let lessons: Vec<LessonProgress> = (1..=5)
                .map(|i| create_test_lesson(&format!("Lesson {}", i), 1000))
                .collect();

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                lessons,
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.total_duration().total_seconds(), 5000);
        }
    }

    mod duration_lessons_ended {
        use super::*;

        #[test]
        fn test_duration_lessons_ended_none_ended() {
            let progress = create_test_progress();

            assert_eq!(progress.duration_lessons_ended().total_seconds(), 0);
        }

        #[test]
        fn test_duration_lessons_ended_some_ended() {
            let lesson1 = create_completed_lesson("Completed", 1800);
            let lesson2 = create_test_lesson("Not Completed", 2400);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.duration_lessons_ended().total_seconds(), 1800);
        }

        #[test]
        fn test_duration_lessons_ended_all_ended() {
            let lesson1 = create_completed_lesson("Lesson 1", 1800);
            let lesson2 = create_completed_lesson("Lesson 2", 2400);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.duration_lessons_ended().total_seconds(), 4200);
        }

        #[test]
        fn test_duration_lessons_ended_started_not_ended() {
            let lesson = create_started_lesson("Started Only", 1800);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.duration_lessons_ended().total_seconds(), 0);
        }
    }

    mod percentage_completed {
        use super::*;

        #[test]
        fn test_percentage_completed_zero_when_none_ended() {
            let progress = create_test_progress();

            assert_eq!(progress.percentage_completed(), 0);
        }

        #[test]
        fn test_percentage_completed_hundred_when_all_ended() {
            let lesson1 = create_completed_lesson("Lesson 1", 1800);
            let lesson2 = create_completed_lesson("Lesson 2", 1800);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.percentage_completed(), 100);
        }

        #[test]
        fn test_percentage_completed_fifty_when_half_ended() {
            let lesson1 = create_completed_lesson("Completed", 1800);
            let lesson2 = create_test_lesson("Not Completed", 1800);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.percentage_completed(), 50);
        }

        #[test]
        fn test_percentage_completed_weighted_by_duration() {
            let lesson1 = create_completed_lesson("Short Completed", 1000);
            let lesson2 = create_test_lesson("Long Not Completed", 3000);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.percentage_completed(), 25);
        }

        #[test]
        fn test_percentage_completed_rounds_down() {
            let lesson1 = create_completed_lesson("Completed", 1000);
            let lesson2 = create_test_lesson("Not Completed", 3560);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.percentage_completed(), 21);
        }
    }

    mod lessons_started_count {
        use super::*;

        #[test]
        fn test_lessons_started_count_zero_when_none_started() {
            let progress = create_test_progress();

            assert_eq!(progress.lessons_started_count(), 0);
        }

        #[test]
        fn test_lessons_started_count_some_started() {
            let lesson1 = create_started_lesson("Started", 1800);
            let lesson2 = create_test_lesson("Not Started", 2400);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.lessons_started_count(), 1);
        }

        #[test]
        fn test_lessons_started_count_all_started() {
            let lesson1 = create_started_lesson("Started 1", 1800);
            let lesson2 = create_started_lesson("Started 2", 2400);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.lessons_started_count(), 2);
        }

        #[test]
        fn test_lessons_started_count_includes_completed() {
            let lesson1 = create_completed_lesson("Completed", 1800);
            let lesson2 = create_started_lesson("Started", 2400);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.lessons_started_count(), 2);
        }
    }

    mod lessons_completed_count {
        use super::*;

        #[test]
        fn test_lessons_completed_count_zero_when_none_completed() {
            let progress = create_test_progress();

            assert_eq!(progress.lessons_completed_count(), 0);
        }

        #[test]
        fn test_lessons_completed_count_some_completed() {
            let lesson1 = create_completed_lesson("Completed", 1800);
            let lesson2 = create_test_lesson("Not Completed", 2400);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.lessons_completed_count(), 1);
        }

        #[test]
        fn test_lessons_completed_count_all_completed() {
            let lesson1 = create_completed_lesson("Completed 1", 1800);
            let lesson2 = create_completed_lesson("Completed 2", 2400);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.lessons_completed_count(), 2);
        }

        #[test]
        fn test_lessons_completed_count_excludes_started_only() {
            let lesson1 = create_started_lesson("Started Only", 1800);
            let lesson2 = create_completed_lesson("Completed", 2400);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.lessons_completed_count(), 1);
        }
    }
}
