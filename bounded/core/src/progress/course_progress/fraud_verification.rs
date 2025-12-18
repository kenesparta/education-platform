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
    /// let lesson1 = LessonProgress::new("Lesson 1".to_string(), 1800, None, None).unwrap();
    /// let lesson2 = LessonProgress::new("Lesson 2".to_string(), 2400, None, None).unwrap();
    /// let progress = CourseProgress::new(
    ///     "Course".to_string(),
    ///     "user@example.com".to_string(),
    ///     vec![lesson1, lesson2],
    ///     None,
    ///     None,
    /// ).unwrap();
    /// assert_eq!(progress.fraud_risk_score(), 0);
    ///
    /// // Lessons started at same time (suspicious for long lessons)
    /// let start = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
    /// let lesson1 = LessonProgress::new("Lesson 1".to_string(), 86400, Some(start), None).unwrap();
    /// let lesson2 = LessonProgress::new("Lesson 2".to_string(), 86400, Some(start), None).unwrap();
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
            let lesson = create_test_lesson("Lesson 1", 1800);
            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_no_start_dates_returns_zero() {
            let lesson1 = create_test_lesson("Lesson 1", 1800);
            let lesson2 = create_test_lesson("Lesson 2", 2400);
            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_same_time_start_is_suspicious() {
            let start = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
            let lesson1 = create_lesson_with_start("Lesson 1", 86400, start);
            let lesson2 = create_lesson_with_start("Lesson 2", 86400, start);
            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.fraud_risk_score(), 100);
        }

        #[test]
        fn test_adequate_gap_not_suspicious() {
            let start1 = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
            let start2 = DateTime::new(2024, 1, 10, 10, 0, 0).unwrap();
            let lesson1 = create_lesson_with_start("Lesson 1", 86400, start1);
            let lesson2 = create_lesson_with_start("Lesson 2", 86400, start2);
            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_mixed_suspicious_and_normal_transitions() {
            let time1 = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 2, 10, 0, 0).unwrap();
            let time3 = DateTime::new(2024, 1, 10, 10, 0, 0).unwrap();

            // 6-day lesson (518400s) requires 20% = 103680s minimum gap
            // time1 -> time2 gap = 86400s < 103680s required -> suspicious
            // time2 -> time3 gap = 8 days (691200s) >= 103680s required -> not suspicious
            let lesson1 = create_lesson_with_start("Lesson 1", 518400, time1);
            let lesson2 = create_lesson_with_start("Lesson 2", 518400, time2);
            let lesson3 = create_lesson_with_start("Lesson 3", 518400, time3);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2, lesson3],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.fraud_risk_score(), 50);
        }

        #[test]
        fn test_partial_start_dates_only_evaluates_complete_pairs() {
            let time1 = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
            let lesson1 = create_lesson_with_start("Lesson 1", 86400, time1);
            let lesson2 = create_test_lesson("Lesson 2", 86400);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_short_lessons_with_adequate_gap() {
            let time1 = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 2, 10, 0, 0).unwrap();

            // 60s lesson requires 12s gap (20% of 60s)
            // 1-day gap (86400s) is way more than 12s
            let lesson1 = create_lesson_with_start("Lesson 1", 60, time1);
            let lesson2 = create_lesson_with_start("Lesson 2", 60, time2);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_short_lessons_same_time_is_suspicious() {
            let time1 = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();

            let lesson1 = create_lesson_with_start("Lesson 1", 60, time1);
            let lesson2 = create_lesson_with_start("Lesson 2", 60, time1);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.fraud_risk_score(), 100);
        }

        #[test]
        fn test_all_suspicious_returns_100() {
            let time1 = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
            let time3 = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();

            // 5-day lesson (432000s) requires 86400s minimum gap
            let lesson1 = create_lesson_with_start("Lesson 1", 432000, time1);
            let lesson2 = create_lesson_with_start("Lesson 2", 432000, time2);
            let lesson3 = create_lesson_with_start("Lesson 3", 432000, time3);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2, lesson3],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.fraud_risk_score(), 100);
        }

        #[test]
        fn test_none_suspicious_returns_0() {
            let time1 = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 15, 10, 0, 0).unwrap();
            let time3 = DateTime::new(2024, 1, 30, 10, 0, 0).unwrap();

            // 5-day lesson (432000s) requires 86400s minimum gap
            // 14-day gaps are more than enough
            let lesson1 = create_lesson_with_start("Lesson 1", 432000, time1);
            let lesson2 = create_lesson_with_start("Lesson 2", 432000, time2);
            let lesson3 = create_lesson_with_start("Lesson 3", 432000, time3);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2, lesson3],
                None,
                None,
            )
            .unwrap();

            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_seconds_precision_suspicious() {
            // 100s lesson requires 20s minimum gap (20% of 100s)
            let time1 = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 1, 10, 0, 10).unwrap(); // 10s gap

            let lesson1 = create_lesson_with_start("Lesson 1", 100, time1);
            let lesson2 = create_lesson_with_start("Lesson 2", 100, time2);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            // 10s gap < 20s required -> suspicious
            assert_eq!(progress.fraud_risk_score(), 100);
        }

        #[test]
        fn test_seconds_precision_not_suspicious() {
            // 100s lesson requires 20s minimum gap (20% of 100s)
            let time1 = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 1, 10, 0, 25).unwrap(); // 25s gap

            let lesson1 = create_lesson_with_start("Lesson 1", 100, time1);
            let lesson2 = create_lesson_with_start("Lesson 2", 100, time2);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            // 25s gap >= 20s required -> not suspicious
            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_seconds_precision_exactly_at_threshold() {
            // 100s lesson requires 20s minimum gap (20% of 100s)
            let time1 = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 1, 10, 0, 20).unwrap(); // exactly 20s gap

            let lesson1 = create_lesson_with_start("Lesson 1", 100, time1);
            let lesson2 = create_lesson_with_start("Lesson 2", 100, time2);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            // 20s gap == 20s required -> not suspicious (actual < min_expected is false)
            assert_eq!(progress.fraud_risk_score(), 0);
        }

        #[test]
        fn test_minutes_precision_suspicious() {
            // 30-min lesson (1800s) requires 360s (6 min) minimum gap
            let time1 = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 1, 10, 5, 0).unwrap(); // 5 min gap

            let lesson1 = create_lesson_with_start("Lesson 1", 1800, time1);
            let lesson2 = create_lesson_with_start("Lesson 2", 1800, time2);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            // 300s gap < 360s required -> suspicious
            assert_eq!(progress.fraud_risk_score(), 100);
        }

        #[test]
        fn test_minutes_precision_not_suspicious() {
            // 30-min lesson (1800s) requires 360s (6 min) minimum gap
            let time1 = DateTime::new(2024, 1, 1, 10, 0, 0).unwrap();
            let time2 = DateTime::new(2024, 1, 1, 10, 7, 0).unwrap(); // 7 min gap

            let lesson1 = create_lesson_with_start("Lesson 1", 1800, time1);
            let lesson2 = create_lesson_with_start("Lesson 2", 1800, time2);

            let progress = CourseProgress::new(
                "Course".to_string(),
                "user@example.com".to_string(),
                vec![lesson1, lesson2],
                None,
                None,
            )
            .unwrap();

            // 420s gap >= 360s required -> not suspicious
            assert_eq!(progress.fraud_risk_score(), 0);
        }
    }
}
