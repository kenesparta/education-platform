use super::{CourseProgress, Duration};
use crate::LessonProgress;
use education_platform_common::DateTime;

impl CourseProgress {
    /// Returns true if all lessons in the course have been completed.
    ///
    /// A course is considered completed when every lesson has both a start
    /// creation_date and an end creation_date.
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

    /// Calculates the end date for a course based on lesson completion status.
    ///
    /// If an explicit `end_date` is provided, it is returned unchanged.
    /// Otherwise, if all lessons are completed, returns the latest lesson end date.
    /// Returns `None` if any lesson is not completed.
    ///
    /// # Arguments
    ///
    /// * `end_date` - An optional explicit end date for the course
    /// * `lessons` - The list of lesson progress records to evaluate
    ///
    /// # Returns
    ///
    /// * `Some(DateTime)` - The course end date (explicit or calculated)
    /// * `None` - If lessons are incomplete or the list is empty
    pub(super) fn calculate_end_date(
        end_date: &Option<DateTime>,
        lessons: &[LessonProgress],
    ) -> Option<DateTime> {
        if let Some(date) = end_date {
            return Some(*date);
        }

        if lessons.is_empty() {
            return None;
        }

        let all_completed = lessons.iter().all(|lesson| lesson.is_completed());
        if !all_completed {
            return None;
        }

        lessons
            .iter()
            .filter_map(|lesson| lesson.end_date())
            .max()
            .copied()
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
        CourseProgress::builder()
            .course_name("Test Course")
            .user_email("test@example.com")
            .lessons(vec![lesson1, lesson2])
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

            let progress = create_progress(vec![lesson1, lesson2]);

            assert!(!progress.is_completed());
        }

        #[test]
        fn test_is_completed_true_when_all_lessons_completed() {
            let lesson1 = create_completed_lesson("Lesson 1", 1800);
            let lesson2 = create_completed_lesson("Lesson 2", 2400);

            let progress = create_progress(vec![lesson1, lesson2]);

            assert!(progress.is_completed());
        }

        #[test]
        fn test_is_completed_single_completed_lesson() {
            let lesson = create_completed_lesson("Only Lesson", 1800);

            let progress = create_progress(vec![lesson]);

            assert!(progress.is_completed());
        }

        #[test]
        fn test_is_completed_started_but_not_ended() {
            let lesson = create_started_lesson("Started Only", 1800);

            let progress = create_progress(vec![lesson]);

            assert!(!progress.is_completed());
        }
    }

    mod total_duration {
        use super::*;

        #[test]
        fn test_total_duration_single_lesson() {
            let lesson = create_test_lesson("Lesson", 3600);

            let progress = create_progress(vec![lesson]);

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

            let progress = create_progress(lessons);

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

            let progress = create_progress(vec![lesson1, lesson2]);

            assert_eq!(progress.duration_lessons_ended().total_seconds(), 1800);
        }

        #[test]
        fn test_duration_lessons_ended_all_ended() {
            let lesson1 = create_completed_lesson("Lesson 1", 1800);
            let lesson2 = create_completed_lesson("Lesson 2", 2400);

            let progress = create_progress(vec![lesson1, lesson2]);

            assert_eq!(progress.duration_lessons_ended().total_seconds(), 4200);
        }

        #[test]
        fn test_duration_lessons_ended_started_not_ended() {
            let lesson = create_started_lesson("Started Only", 1800);

            let progress = create_progress(vec![lesson]);

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

            let progress = create_progress(vec![lesson1, lesson2]);

            assert_eq!(progress.percentage_completed(), 100);
        }

        #[test]
        fn test_percentage_completed_fifty_when_half_ended() {
            let lesson1 = create_completed_lesson("Completed", 1800);
            let lesson2 = create_test_lesson("Not Completed", 1800);

            let progress = create_progress(vec![lesson1, lesson2]);

            assert_eq!(progress.percentage_completed(), 50);
        }

        #[test]
        fn test_percentage_completed_weighted_by_duration() {
            let lesson1 = create_completed_lesson("Short Completed", 1000);
            let lesson2 = create_test_lesson("Long Not Completed", 3000);

            let progress = create_progress(vec![lesson1, lesson2]);

            assert_eq!(progress.percentage_completed(), 25);
        }

        #[test]
        fn test_percentage_completed_rounds_down() {
            let lesson1 = create_completed_lesson("Completed", 1000);
            let lesson2 = create_test_lesson("Not Completed", 3560);

            let progress = create_progress(vec![lesson1, lesson2]);

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

            let progress = create_progress(vec![lesson1, lesson2]);

            assert_eq!(progress.lessons_started_count(), 1);
        }

        #[test]
        fn test_lessons_started_count_all_started() {
            let lesson1 = create_started_lesson("Started 1", 1800);
            let lesson2 = create_started_lesson("Started 2", 2400);

            let progress = create_progress(vec![lesson1, lesson2]);

            assert_eq!(progress.lessons_started_count(), 2);
        }

        #[test]
        fn test_lessons_started_count_includes_completed() {
            let lesson1 = create_completed_lesson("Completed", 1800);
            let lesson2 = create_started_lesson("Started", 2400);

            let progress = create_progress(vec![lesson1, lesson2]);

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

            let progress = create_progress(vec![lesson1, lesson2]);

            assert_eq!(progress.lessons_completed_count(), 1);
        }

        #[test]
        fn test_lessons_completed_count_all_completed() {
            let lesson1 = create_completed_lesson("Completed 1", 1800);
            let lesson2 = create_completed_lesson("Completed 2", 2400);

            let progress = create_progress(vec![lesson1, lesson2]);

            assert_eq!(progress.lessons_completed_count(), 2);
        }

        #[test]
        fn test_lessons_completed_count_excludes_started_only() {
            let lesson1 = create_started_lesson("Started Only", 1800);
            let lesson2 = create_completed_lesson("Completed", 2400);

            let progress = create_progress(vec![lesson1, lesson2]);

            assert_eq!(progress.lessons_completed_count(), 1);
        }
    }

    mod calculate_end_date {
        use super::*;

        fn create_completed_lesson_with_end(
            name: &str,
            duration: u64,
            end: DateTime,
        ) -> LessonProgress {
            let start = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
            LessonProgress::new(name.to_string(), duration, Some(start), Some(end)).unwrap()
        }

        #[test]
        fn test_returns_explicit_end_date_when_provided() {
            let explicit_date = DateTime::new(2024, 6, 15, 12, 0, 0).unwrap();
            let lesson = create_test_lesson("Lesson 1", 1800);

            let result = CourseProgress::calculate_end_date(&Some(explicit_date), &[lesson]);

            assert_eq!(result, Some(explicit_date));
        }

        #[test]
        fn test_returns_explicit_date_even_when_lessons_completed() {
            let explicit_date = DateTime::new(2024, 6, 15, 12, 0, 0).unwrap();
            let lesson = create_completed_lesson("Completed", 1800);

            let result = CourseProgress::calculate_end_date(&Some(explicit_date), &[lesson]);

            assert_eq!(result, Some(explicit_date));
        }

        #[test]
        fn test_returns_none_for_empty_lessons() {
            let result = CourseProgress::calculate_end_date(&None, &[]);

            assert_eq!(result, None);
        }

        #[test]
        fn test_returns_none_when_no_lessons_completed() {
            let lesson1 = create_test_lesson("Lesson 1", 1800);
            let lesson2 = create_test_lesson("Lesson 2", 2400);

            let result = CourseProgress::calculate_end_date(&None, &[lesson1, lesson2]);

            assert_eq!(result, None);
        }

        #[test]
        fn test_returns_none_when_some_lessons_incomplete() {
            let lesson1 = create_completed_lesson("Completed", 1800);
            let lesson2 = create_test_lesson("Not Completed", 2400);

            let result = CourseProgress::calculate_end_date(&None, &[lesson1, lesson2]);

            assert_eq!(result, None);
        }

        #[test]
        fn test_returns_none_when_lessons_started_but_not_ended() {
            let lesson1 = create_started_lesson("Started 1", 1800);
            let lesson2 = create_started_lesson("Started 2", 2400);

            let result = CourseProgress::calculate_end_date(&None, &[lesson1, lesson2]);

            assert_eq!(result, None);
        }

        #[test]
        fn test_returns_latest_end_date_when_all_completed() {
            let early_end = DateTime::new(2024, 1, 10, 10, 0, 0).unwrap();
            let late_end = DateTime::new(2024, 1, 15, 14, 30, 0).unwrap();

            let lesson1 = create_completed_lesson_with_end("Early", 1800, early_end);
            let lesson2 = create_completed_lesson_with_end("Late", 2400, late_end);

            let result = CourseProgress::calculate_end_date(&None, &[lesson1, lesson2]);

            assert_eq!(result, Some(late_end));
        }

        #[test]
        fn test_returns_latest_end_date_regardless_of_order() {
            let early_end = DateTime::new(2024, 1, 5, 9, 0, 0).unwrap();
            let middle_end = DateTime::new(2024, 1, 10, 12, 0, 0).unwrap();
            let late_end = DateTime::new(2024, 1, 20, 18, 0, 0).unwrap();

            let lesson1 = create_completed_lesson_with_end("Late", 1800, late_end);
            let lesson2 = create_completed_lesson_with_end("Early", 2400, early_end);
            let lesson3 = create_completed_lesson_with_end("Middle", 3000, middle_end);

            let result = CourseProgress::calculate_end_date(&None, &[lesson1, lesson2, lesson3]);

            assert_eq!(result, Some(late_end));
        }

        #[test]
        fn test_returns_end_date_for_single_completed_lesson() {
            let end_date = DateTime::new(2024, 3, 20, 16, 45, 0).unwrap();
            let lesson = create_completed_lesson_with_end("Only Lesson", 1800, end_date);

            let result = CourseProgress::calculate_end_date(&None, &[lesson]);

            assert_eq!(result, Some(end_date));
        }

        #[test]
        fn test_handles_same_end_dates() {
            let same_end = DateTime::new(2024, 2, 14, 12, 0, 0).unwrap();
            let lesson1 = create_completed_lesson_with_end("Lesson 1", 1800, same_end);
            let lesson2 = create_completed_lesson_with_end("Lesson 2", 2400, same_end);

            let result = CourseProgress::calculate_end_date(&None, &[lesson1, lesson2]);

            assert_eq!(result, Some(same_end));
        }

        #[test]
        fn test_compares_full_datetime_including_time() {
            let earlier_time = DateTime::new(2024, 5, 1, 10, 0, 0).unwrap();
            let later_time = DateTime::new(2024, 5, 1, 18, 30, 0).unwrap();

            let lesson1 = create_completed_lesson_with_end("Morning", 1800, earlier_time);
            let lesson2 = create_completed_lesson_with_end("Evening", 2400, later_time);

            let result = CourseProgress::calculate_end_date(&None, &[lesson1, lesson2]);

            assert_eq!(result, Some(later_time));
        }
    }
}
