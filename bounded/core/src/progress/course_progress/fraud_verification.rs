use crate::CourseProgress;

const MIN_COMPLETION_RATIO: f64 = 0.2;

impl CourseProgress {
    /// Calculates a fraud risk score based on lesson completion patterns.
    ///
    /// Returns a percentage (0-100) indicating the proportion of suspicious
    /// lesson transitions. A transition is considered suspicious if the time
    /// between starting consecutive lessons is less than 20% of the expected
    /// lesson duration.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{CourseEnded, CourseProgress, LessonProgress};
    /// use education_platform_common::{DateTime, DomainEventDispatcher};
    /// use std::sync::Arc;
    ///
    /// // Lessons with no start dates - no fraud risk
    /// let lesson1 = LessonProgress::new("Lesson 1".to_string(), 900, None, None).unwrap();
    /// let lesson2 = LessonProgress::new("Lesson 2".to_string(), 1200, None, None).unwrap();
    /// let dispatcher = Arc::new(DomainEventDispatcher::<CourseEnded>::new());
    /// let progress = CourseProgress::builder()
    ///     .course_name("Course")
    ///     .user_email("user@example.com")
    ///     .lessons(vec![lesson1, lesson2])
    ///     .event_dispatcher(dispatcher.clone())
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(progress.fraud_risk_score(), 0);
    ///
    /// // Lessons started at same time (suspicious)
    /// // 30-min lesson (1800s) requires 360s (6 min) minimum gap
    /// let start = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
    /// let lesson1 = LessonProgress::new("Lesson 1".to_string(), 1800, Some(start), None).unwrap();
    /// let lesson2 = LessonProgress::new("Lesson 2".to_string(), 1800, Some(start), None).unwrap();
    /// let progress = CourseProgress::builder()
    ///     .course_name("Course")
    ///     .user_email("user@example.com")
    ///     .lessons(vec![lesson1, lesson2])
    ///     .event_dispatcher(dispatcher)
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(progress.fraud_risk_score(), 100);
    /// ```
    #[must_use]
    pub fn fraud_risk_score(&self) -> u64 {
        let lessons = self.lesson_progress();

        if lessons.len() < 2 {
            return 0;
        }

        let (suspicious_count, total_evaluated) = lessons
            .windows(2)
            .filter_map(|window| {
                let current_start = window[0].start_date()?;
                let next_start = window[1].start_date()?;
                let min_expected_gap =
                    (window[0].duration().total_seconds() as f64 * MIN_COMPLETION_RATIO) as i64;
                let actual_gap = current_start.seconds_until(next_start).abs();

                Some(actual_gap < min_expected_gap)
            })
            .fold((0u64, 0u64), |(suspicious, total), is_suspicious| {
                (suspicious + u64::from(is_suspicious), total + 1)
            });

        if total_evaluated == 0 {
            return 0;
        }

        (suspicious_count * 100) / total_evaluated
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CourseEnded, LessonProgress};
    use education_platform_common::{DateTime, DomainEventDispatcher};
    use std::sync::Arc;

    // Realistic lesson durations in seconds
    const DURATION_10_MIN: u64 = 600;
    const DURATION_15_MIN: u64 = 900;
    const DURATION_30_MIN: u64 = 1800;
    const DURATION_45_MIN: u64 = 2700;
    const DURATION_1_HOUR: u64 = 3600;
    const DURATION_2_HOURS: u64 = 7200;

    fn create_test_dispatcher() -> Arc<DomainEventDispatcher<CourseEnded>> {
        Arc::new(DomainEventDispatcher::new())
    }

    fn create_test_lesson(name: &str, duration: u64) -> LessonProgress {
        LessonProgress::new(name.to_string(), duration, None, None).unwrap()
    }

    fn create_lesson_with_start(name: &str, duration: u64, start: DateTime) -> LessonProgress {
        LessonProgress::new(name.to_string(), duration, Some(start), None).unwrap()
    }

    fn create_progress(name: &str, email: &str, lessons: Vec<LessonProgress>) -> CourseProgress {
        CourseProgress::builder()
            .course_name(name)
            .user_email(email)
            .lessons(lessons)
            .event_dispatcher(create_test_dispatcher())
            .build()
            .unwrap()
    }

    mod fraud_risk_score {
        use super::*;

        #[test]
        fn test_single_lesson_returns_zero() {
            let lesson = create_test_lesson("Introduction", DURATION_15_MIN);
            let progress = create_progress("Rust Basics", "student@example.com", vec![lesson]);

            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_no_start_dates_returns_zero() {
            let lesson1 = create_test_lesson("Variables", DURATION_15_MIN);
            let lesson2 = create_test_lesson("Functions", DURATION_30_MIN);
            let progress =
                create_progress("Rust Basics", "student@example.com", vec![lesson1, lesson2]);

            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_same_time_start_is_suspicious() {
            let start = DateTime::new(2024, 1, 15, 14, 0, 0).unwrap();
            let lesson1 = create_lesson_with_start("Variables", DURATION_30_MIN, start);
            let lesson2 = create_lesson_with_start("Functions", DURATION_30_MIN, start);
            let progress =
                create_progress("Rust Basics", "student@example.com", vec![lesson1, lesson2]);

            assert_eq!(progress.fraud_risk_score(), 100);
        }

        #[test]
        fn test_adequate_gap_not_suspicious() {
            let start1 = DateTime::new(2024, 1, 15, 10, 0, 0).unwrap();
            let start2 = DateTime::new(2024, 1, 15, 10, 30, 0).unwrap();
            let lesson1 = create_lesson_with_start("Ownership Basics", DURATION_1_HOUR, start1);
            let lesson2 = create_lesson_with_start("Borrowing Rules", DURATION_1_HOUR, start2);
            let progress =
                create_progress("Rust Memory", "student@example.com", vec![lesson1, lesson2]);

            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_mixed_suspicious_and_normal_transitions() {
            let time1 = DateTime::new(2024, 1, 15, 9, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 15, 9, 10, 0).unwrap();
            let time3 = DateTime::new(2024, 1, 15, 10, 0, 0).unwrap();

            let lesson1 = create_lesson_with_start("Advanced Traits", DURATION_2_HOURS, time1);
            let lesson2 = create_lesson_with_start("Generic Types", DURATION_2_HOURS, time2);
            let lesson3 = create_lesson_with_start("Lifetimes Deep", DURATION_2_HOURS, time3);

            let progress = create_progress(
                "Advanced Rust",
                "student@example.com",
                vec![lesson1, lesson2, lesson3],
            );

            assert_eq!(progress.fraud_risk_score(), 50);
        }

        #[test]
        fn test_partial_start_dates_only_evaluates_complete_pairs() {
            let time1 = DateTime::new(2024, 1, 15, 10, 0, 0).unwrap();
            let lesson1 = create_lesson_with_start("Started Lesson", DURATION_30_MIN, time1);
            let lesson2 = create_test_lesson("Not Started Yet", DURATION_30_MIN);

            let progress =
                create_progress("Mixed Progress", "student@example.com", vec![lesson1, lesson2]);

            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_10_min_lesson_with_adequate_gap() {
            let time1 = DateTime::new(2024, 1, 15, 10, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 15, 10, 3, 0).unwrap();

            let lesson1 = create_lesson_with_start("Quick Intro", DURATION_10_MIN, time1);
            let lesson2 = create_lesson_with_start("Quick Summary", DURATION_10_MIN, time2);

            let progress =
                create_progress("Quick Course", "student@example.com", vec![lesson1, lesson2]);

            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_15_min_lessons_same_time_is_suspicious() {
            let time1 = DateTime::new(2024, 1, 15, 14, 30, 0).unwrap();

            let lesson1 = create_lesson_with_start("Module 1", DURATION_15_MIN, time1);
            let lesson2 = create_lesson_with_start("Module 2", DURATION_15_MIN, time1);

            let progress =
                create_progress("Short Course", "student@example.com", vec![lesson1, lesson2]);

            assert_eq!(progress.fraud_risk_score(), 100);
        }

        #[test]
        fn test_all_suspicious_returns_100() {
            let time1 = DateTime::new(2024, 1, 15, 10, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 15, 10, 2, 0).unwrap();
            let time3 = DateTime::new(2024, 1, 15, 10, 4, 0).unwrap();

            let lesson1 = create_lesson_with_start("Chapter 1", DURATION_45_MIN, time1);
            let lesson2 = create_lesson_with_start("Chapter 2", DURATION_45_MIN, time2);
            let lesson3 = create_lesson_with_start("Chapter 3", DURATION_45_MIN, time3);

            let progress = create_progress(
                "Suspicious Course",
                "student@example.com",
                vec![lesson1, lesson2, lesson3],
            );

            assert_eq!(progress.fraud_risk_score(), 100);
        }

        #[test]
        fn test_none_suspicious_returns_0() {
            let time1 = DateTime::new(2024, 1, 15, 9, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 15, 10, 30, 0).unwrap();
            let time3 = DateTime::new(2024, 1, 15, 12, 0, 0).unwrap();

            let lesson1 = create_lesson_with_start("Morning Session", DURATION_1_HOUR, time1);
            let lesson2 = create_lesson_with_start("Mid-Morning", DURATION_1_HOUR, time2);
            let lesson3 = create_lesson_with_start("Before Lunch", DURATION_1_HOUR, time3);

            let progress = create_progress(
                "Full Day Course",
                "student@example.com",
                vec![lesson1, lesson2, lesson3],
            );

            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_seconds_precision_suspicious() {
            let time1 = DateTime::new(2024, 1, 15, 10, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 15, 10, 1, 30).unwrap();

            let lesson1 = create_lesson_with_start("Quick Tip 1", DURATION_10_MIN, time1);
            let lesson2 = create_lesson_with_start("Quick Tip 2", DURATION_10_MIN, time2);

            let progress =
                create_progress("Tips Course", "student@example.com", vec![lesson1, lesson2]);

            assert_eq!(progress.fraud_risk_score(), 100);
        }

        #[test]
        fn test_seconds_precision_not_suspicious() {
            let time1 = DateTime::new(2024, 1, 15, 10, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 15, 10, 2, 30).unwrap();

            let lesson1 = create_lesson_with_start("Quick Tip 1", DURATION_10_MIN, time1);
            let lesson2 = create_lesson_with_start("Quick Tip 2", DURATION_10_MIN, time2);

            let progress =
                create_progress("Tips Course", "student@example.com", vec![lesson1, lesson2]);

            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_seconds_precision_exactly_at_threshold() {
            let time1 = DateTime::new(2024, 1, 15, 10, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 15, 10, 3, 0).unwrap();

            let lesson1 = create_lesson_with_start("Part 1", DURATION_15_MIN, time1);
            let lesson2 = create_lesson_with_start("Part 2", DURATION_15_MIN, time2);

            let progress =
                create_progress("Two Parts", "student@example.com", vec![lesson1, lesson2]);

            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_30_min_lesson_suspicious_gap() {
            let time1 = DateTime::new(2024, 1, 15, 10, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 15, 10, 5, 0).unwrap();

            let lesson1 = create_lesson_with_start("Basics", DURATION_30_MIN, time1);
            let lesson2 = create_lesson_with_start("Advanced", DURATION_30_MIN, time2);

            let progress =
                create_progress("Complete Course", "student@example.com", vec![lesson1, lesson2]);

            assert_eq!(progress.fraud_risk_score(), 100);
        }

        #[test]
        fn test_30_min_lesson_not_suspicious_gap() {
            let time1 = DateTime::new(2024, 1, 15, 10, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 15, 10, 7, 0).unwrap();

            let lesson1 = create_lesson_with_start("Basics", DURATION_30_MIN, time1);
            let lesson2 = create_lesson_with_start("Advanced", DURATION_30_MIN, time2);

            let progress =
                create_progress("Complete Course", "student@example.com", vec![lesson1, lesson2]);

            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_2_hour_lesson_suspicious_gap() {
            let time1 = DateTime::new(2024, 1, 15, 9, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 15, 9, 15, 0).unwrap();

            let lesson1 = create_lesson_with_start("Deep Dive 1", DURATION_2_HOURS, time1);
            let lesson2 = create_lesson_with_start("Deep Dive 2", DURATION_2_HOURS, time2);

            let progress =
                create_progress("Deep Course", "student@example.com", vec![lesson1, lesson2]);

            assert_eq!(progress.fraud_risk_score(), 100);
        }

        #[test]
        fn test_2_hour_lesson_not_suspicious_gap() {
            let time1 = DateTime::new(2024, 1, 15, 9, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 15, 9, 30, 0).unwrap();

            let lesson1 = create_lesson_with_start("Deep Dive 1", DURATION_2_HOURS, time1);
            let lesson2 = create_lesson_with_start("Deep Dive 2", DURATION_2_HOURS, time2);

            let progress =
                create_progress("Deep Course", "student@example.com", vec![lesson1, lesson2]);

            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_varied_lesson_durations_mixed_results() {
            let time1 = DateTime::new(2024, 1, 15, 9, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 15, 9, 5, 0).unwrap();
            let time3 = DateTime::new(2024, 1, 15, 10, 0, 0).unwrap();
            let time4 = DateTime::new(2024, 1, 15, 10, 5, 0).unwrap();

            let lesson1 = create_lesson_with_start("Intro", DURATION_15_MIN, time1);
            let lesson2 = create_lesson_with_start("Main Content", DURATION_45_MIN, time2);
            let lesson3 = create_lesson_with_start("Practice", DURATION_30_MIN, time3);
            let lesson4 = create_lesson_with_start("Quiz", DURATION_10_MIN, time4);

            let progress = create_progress(
                "Mixed Course",
                "student@example.com",
                vec![lesson1, lesson2, lesson3, lesson4],
            );

            assert_eq!(progress.fraud_risk_score(), 33);
        }
    }
}
