use crate::CourseProgress;

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
    /// use education_platform_core::{CourseProgress, LessonProgress};
    /// use education_platform_common::DateTime;
    ///
    /// // Lessons with no start dates - no fraud risk
    /// let lesson1 = LessonProgress::new("Lesson 1".to_string(), 900, None, None).unwrap();
    /// let lesson2 = LessonProgress::new("Lesson 2".to_string(), 1200, None, None).unwrap();
    /// let progress = CourseProgress::new(
    ///     "Course".to_string(),
    ///     "user@example.com".to_string(),
    ///     vec![lesson1, lesson2],
    ///     None,
    ///     None,
    /// ).unwrap();
    /// assert_eq!(progress.fraud_risk_score(), 0);
    ///
    /// // Lessons started at same time (suspicious)
    /// // 30-min lesson (1800s) requires 360s (6 min) minimum gap
    /// let start = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
    /// let lesson1 = LessonProgress::new("Lesson 1".to_string(), 1800, Some(start), None).unwrap();
    /// let lesson2 = LessonProgress::new("Lesson 2".to_string(), 1800, Some(start), None).unwrap();
    /// let progress = CourseProgress::new(
    ///     "Course".to_string(),
    ///     "user@example.com".to_string(),
    ///     vec![lesson1, lesson2],
    ///     None,
    ///     None,
    /// ).unwrap();
    /// assert_eq!(progress.fraud_risk_score(), 100);
    /// ```
    #[must_use]
    pub fn fraud_risk_score(&self) -> u64 {
        let lessons = self.lesson_progress();

        if lessons.len() < 2 {
            return 0;
        }

        let (suspicious, evaluated) = lessons
            .windows(2)
            .filter_map(|window| {
                let current_start = window[0].start_date()?;
                let next_start = window[1].start_date()?;

                let min_expected = (window[0].duration().total_seconds() as f64 * 0.2) as i64;
                let actual = current_start.seconds_until(next_start).abs();

                Some(actual < min_expected)
            })
            .fold((0u64, 0u64), |(suspicious, total), is_suspicious| {
                (suspicious + u64::from(is_suspicious), total + 1)
            });

        match evaluated {
            0 => 0,
            _ => (suspicious * 100) / evaluated,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LessonProgress;
    use education_platform_common::DateTime;

    // Realistic lesson durations in seconds
    const DURATION_10_MIN: u64 = 600;
    const DURATION_15_MIN: u64 = 900;
    const DURATION_30_MIN: u64 = 1800;
    const DURATION_45_MIN: u64 = 2700;
    const DURATION_1_HOUR: u64 = 3600;
    const DURATION_2_HOURS: u64 = 7200;

    fn create_test_lesson(name: &str, duration: u64) -> LessonProgress {
        LessonProgress::new(name.to_string(), duration, None, None).unwrap()
    }

    fn create_lesson_with_start(name: &str, duration: u64, start: DateTime) -> LessonProgress {
        LessonProgress::new(name.to_string(), duration, Some(start), None).unwrap()
    }

    mod fraud_risk_score {
        use super::*;

        #[test]
        fn test_single_lesson_returns_zero() {
            let lesson = create_test_lesson("Introduction", DURATION_15_MIN);
            let progress = CourseProgress::new(
                "Rust Basics".to_string(),
                "student@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_no_start_dates_returns_zero() {
            let lesson1 = create_test_lesson("Variables", DURATION_15_MIN);
            let lesson2 = create_test_lesson("Functions", DURATION_30_MIN);
            let progress = CourseProgress::new(
                "Rust Basics".to_string(),
                "student@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_same_time_start_is_suspicious() {
            // 30-min lesson requires 6 min (360s) minimum gap
            // Starting both at the same time = 0s gap -> suspicious
            let start = DateTime::new(2024, 1, 15, 14, 0, 0).unwrap();
            let lesson1 = create_lesson_with_start("Variables", DURATION_30_MIN, start);
            let lesson2 = create_lesson_with_start("Functions", DURATION_30_MIN, start);
            let progress = CourseProgress::new(
                "Rust Basics".to_string(),
                "student@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.fraud_risk_score(), 100);
        }

        #[test]
        fn test_adequate_gap_not_suspicious() {
            // 1-hour lesson requires 12 min (720s) minimum gap
            // 30 min gap (1800s) is adequate
            let start1 = DateTime::new(2024, 1, 15, 10, 0, 0).unwrap();
            let start2 = DateTime::new(2024, 1, 15, 10, 30, 0).unwrap();
            let lesson1 = create_lesson_with_start("Ownership Basics", DURATION_1_HOUR, start1);
            let lesson2 = create_lesson_with_start("Borrowing Rules", DURATION_1_HOUR, start2);
            let progress = CourseProgress::new(
                "Rust Memory".to_string(),
                "student@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_mixed_suspicious_and_normal_transitions() {
            // 2-hour lesson (7200s) requires 20% = 1440s (24 min) minimum gap
            let time1 = DateTime::new(2024, 1, 15, 9, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 15, 9, 10, 0).unwrap(); // 10 min gap -> suspicious
            let time3 = DateTime::new(2024, 1, 15, 10, 0, 0).unwrap(); // 50 min gap -> not suspicious

            let lesson1 = create_lesson_with_start("Advanced Traits", DURATION_2_HOURS, time1);
            let lesson2 = create_lesson_with_start("Generic Types", DURATION_2_HOURS, time2);
            let lesson3 = create_lesson_with_start("Lifetimes Deep", DURATION_2_HOURS, time3);

            let progress = CourseProgress::new(
                "Advanced Rust".to_string(),
                "student@example.com".to_string(),
                vec![lesson1, lesson2, lesson3],
                None,
                None,
            )
            .unwrap();

            // First transition: 600s < 1440s required -> suspicious
            // Second transition: 3000s >= 1440s required -> not suspicious
            assert_eq!(progress.fraud_risk_score(), 50);
        }

        #[test]
        fn test_partial_start_dates_only_evaluates_complete_pairs() {
            let time1 = DateTime::new(2024, 1, 15, 10, 0, 0).unwrap();
            let lesson1 = create_lesson_with_start("Started Lesson", DURATION_30_MIN, time1);
            let lesson2 = create_test_lesson("Not Started Yet", DURATION_30_MIN);

            let progress = CourseProgress::new(
                "Mixed Progress".to_string(),
                "student@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_10_min_lesson_with_adequate_gap() {
            // 10-min lesson (600s) requires 120s (2 min) minimum gap
            let time1 = DateTime::new(2024, 1, 15, 10, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 15, 10, 3, 0).unwrap(); // 3 min gap

            let lesson1 = create_lesson_with_start("Quick Intro", DURATION_10_MIN, time1);
            let lesson2 = create_lesson_with_start("Quick Summary", DURATION_10_MIN, time2);

            let progress = CourseProgress::new(
                "Quick Course".to_string(),
                "student@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            // 180s gap >= 120s required -> not suspicious
            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_15_min_lessons_same_time_is_suspicious() {
            // 15-min lesson (900s) requires 180s (3 min) minimum gap
            let time1 = DateTime::new(2024, 1, 15, 14, 30, 0).unwrap();

            let lesson1 = create_lesson_with_start("Module 1", DURATION_15_MIN, time1);
            let lesson2 = create_lesson_with_start("Module 2", DURATION_15_MIN, time1);

            let progress = CourseProgress::new(
                "Short Course".to_string(),
                "student@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.fraud_risk_score(), 100);
        }

        #[test]
        fn test_all_suspicious_returns_100() {
            // 45-min lessons (2700s) require 540s (9 min) minimum gap
            let time1 = DateTime::new(2024, 1, 15, 10, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 15, 10, 2, 0).unwrap(); // 2 min gap
            let time3 = DateTime::new(2024, 1, 15, 10, 4, 0).unwrap(); // 2 min gap

            let lesson1 = create_lesson_with_start("Chapter 1", DURATION_45_MIN, time1);
            let lesson2 = create_lesson_with_start("Chapter 2", DURATION_45_MIN, time2);
            let lesson3 = create_lesson_with_start("Chapter 3", DURATION_45_MIN, time3);

            let progress = CourseProgress::new(
                "Suspicious Course".to_string(),
                "student@example.com".to_string(),
                vec![lesson1, lesson2, lesson3],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.fraud_risk_score(), 100);
        }

        #[test]
        fn test_none_suspicious_returns_0() {
            // 1-hour lessons (3600s) require 720s (12 min) minimum gap
            let time1 = DateTime::new(2024, 1, 15, 9, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 15, 10, 30, 0).unwrap(); // 90 min gap
            let time3 = DateTime::new(2024, 1, 15, 12, 0, 0).unwrap(); // 90 min gap

            let lesson1 = create_lesson_with_start("Morning Session", DURATION_1_HOUR, time1);
            let lesson2 = create_lesson_with_start("Mid-Morning", DURATION_1_HOUR, time2);
            let lesson3 = create_lesson_with_start("Before Lunch", DURATION_1_HOUR, time3);

            let progress = CourseProgress::new(
                "Full Day Course".to_string(),
                "student@example.com".to_string(),
                vec![lesson1, lesson2, lesson3],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_seconds_precision_suspicious() {
            // 10-min lesson (600s) requires 120s (2 min) minimum gap
            let time1 = DateTime::new(2024, 1, 15, 10, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 15, 10, 1, 30).unwrap(); // 90s gap

            let lesson1 = create_lesson_with_start("Quick Tip 1", DURATION_10_MIN, time1);
            let lesson2 = create_lesson_with_start("Quick Tip 2", DURATION_10_MIN, time2);

            let progress = CourseProgress::new(
                "Tips Course".to_string(),
                "student@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            // 90s gap < 120s required -> suspicious
            assert_eq!(progress.fraud_risk_score(), 100);
        }

        #[test]
        fn test_seconds_precision_not_suspicious() {
            // 10-min lesson (600s) requires 120s (2 min) minimum gap
            let time1 = DateTime::new(2024, 1, 15, 10, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 15, 10, 2, 30).unwrap(); // 150s gap

            let lesson1 = create_lesson_with_start("Quick Tip 1", DURATION_10_MIN, time1);
            let lesson2 = create_lesson_with_start("Quick Tip 2", DURATION_10_MIN, time2);

            let progress = CourseProgress::new(
                "Tips Course".to_string(),
                "student@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            // 150s gap >= 120s required -> not suspicious
            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_seconds_precision_exactly_at_threshold() {
            // 15-min lesson (900s) requires 180s (3 min) minimum gap
            let time1 = DateTime::new(2024, 1, 15, 10, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 15, 10, 3, 0).unwrap(); // exactly 180s gap

            let lesson1 = create_lesson_with_start("Part 1", DURATION_15_MIN, time1);
            let lesson2 = create_lesson_with_start("Part 2", DURATION_15_MIN, time2);

            let progress = CourseProgress::new(
                "Two Parts".to_string(),
                "student@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            // 180s gap == 180s required -> not suspicious (actual < min_expected is false)
            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_30_min_lesson_suspicious_gap() {
            // 30-min lesson (1800s) requires 360s (6 min) minimum gap
            let time1 = DateTime::new(2024, 1, 15, 10, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 15, 10, 5, 0).unwrap(); // 5 min gap

            let lesson1 = create_lesson_with_start("Basics", DURATION_30_MIN, time1);
            let lesson2 = create_lesson_with_start("Advanced", DURATION_30_MIN, time2);

            let progress = CourseProgress::new(
                "Complete Course".to_string(),
                "student@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            // 300s gap < 360s required -> suspicious
            assert_eq!(progress.fraud_risk_score(), 100);
        }

        #[test]
        fn test_30_min_lesson_not_suspicious_gap() {
            // 30-min lesson (1800s) requires 360s (6 min) minimum gap
            let time1 = DateTime::new(2024, 1, 15, 10, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 15, 10, 7, 0).unwrap(); // 7 min gap

            let lesson1 = create_lesson_with_start("Basics", DURATION_30_MIN, time1);
            let lesson2 = create_lesson_with_start("Advanced", DURATION_30_MIN, time2);

            let progress = CourseProgress::new(
                "Complete Course".to_string(),
                "student@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            // 420s gap >= 360s required -> not suspicious
            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_2_hour_lesson_suspicious_gap() {
            // 2-hour lesson (7200s) requires 1440s (24 min) minimum gap
            let time1 = DateTime::new(2024, 1, 15, 9, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 15, 9, 15, 0).unwrap(); // 15 min gap

            let lesson1 = create_lesson_with_start("Deep Dive 1", DURATION_2_HOURS, time1);
            let lesson2 = create_lesson_with_start("Deep Dive 2", DURATION_2_HOURS, time2);

            let progress = CourseProgress::new(
                "Deep Course".to_string(),
                "student@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            // 900s gap < 1440s required -> suspicious
            assert_eq!(progress.fraud_risk_score(), 100);
        }

        #[test]
        fn test_2_hour_lesson_not_suspicious_gap() {
            // 2-hour lesson (7200s) requires 1440s (24 min) minimum gap
            let time1 = DateTime::new(2024, 1, 15, 9, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 15, 9, 30, 0).unwrap(); // 30 min gap

            let lesson1 = create_lesson_with_start("Deep Dive 1", DURATION_2_HOURS, time1);
            let lesson2 = create_lesson_with_start("Deep Dive 2", DURATION_2_HOURS, time2);

            let progress = CourseProgress::new(
                "Deep Course".to_string(),
                "student@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            // 1800s gap >= 1440s required -> not suspicious
            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_varied_lesson_durations_mixed_results() {
            // Mixed lesson durations simulating a real course
            let time1 = DateTime::new(2024, 1, 15, 9, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 15, 9, 5, 0).unwrap(); // 5 min gap
            let time3 = DateTime::new(2024, 1, 15, 10, 0, 0).unwrap(); // 55 min gap
            let time4 = DateTime::new(2024, 1, 15, 10, 5, 0).unwrap(); // 5 min gap

            // 15 min lesson requires 3 min gap -> 5 min OK
            let lesson1 = create_lesson_with_start("Intro", DURATION_15_MIN, time1);
            // 45 min lesson requires 9 min gap -> 55 min OK
            let lesson2 = create_lesson_with_start("Main Content", DURATION_45_MIN, time2);
            // 30 min lesson requires 6 min gap -> 5 min NOT OK
            let lesson3 = create_lesson_with_start("Practice", DURATION_30_MIN, time3);
            let lesson4 = create_lesson_with_start("Quiz", DURATION_10_MIN, time4);

            let progress = CourseProgress::new(
                "Mixed Course".to_string(),
                "student@example.com".to_string(),
                vec![lesson1, lesson2, lesson3, lesson4],
                None,
                None,
            )
            .unwrap();

            // Transitions:
            // 1->2: 300s gap, 15min lesson needs 180s -> OK
            // 2->3: 3300s gap, 45min lesson needs 540s -> OK
            // 3->4: 300s gap, 30min lesson needs 360s -> suspicious
            // 1 suspicious out of 3 = 33%
            assert_eq!(progress.fraud_risk_score(), 33);
        }
    }
}
