use super::{Chapter, ChapterError, Duration, Lesson, SimpleName};
use education_platform_common::Index;

impl Chapter {
    /// Returns the chapter name.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Chapter, Lesson};
    ///
    /// let lesson = Lesson::new(
    ///     "Intro".to_string(),
    ///     600,
    ///     "https://example.com/intro.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let chapter = Chapter::new(
    ///     "Fundamentals".to_string(),
    ///     0,
    ///     vec![lesson],
    /// ).unwrap();
    ///
    /// assert_eq!(chapter.name().as_str(), "Fundamentals");
    /// ```
    #[inline]
    #[must_use]
    pub const fn name(&self) -> &SimpleName {
        &self.name
    }

    /// Returns the chapter index (position within the course).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Chapter, Lesson};
    ///
    /// let lesson = Lesson::new(
    ///     "Lesson".to_string(),
    ///     1200,
    ///     "https://example.com/lesson.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let chapter = Chapter::new(
    ///     "Advanced Topics".to_string(),
    ///     5,
    ///     vec![lesson],
    /// ).unwrap();
    ///
    /// assert_eq!(chapter.index().value(), 5);
    /// ```
    #[inline]
    #[must_use]
    pub const fn index(&self) -> Index {
        self.index
    }

    /// Returns a reference to the lessons in this chapter.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Chapter, Lesson};
    ///
    /// let lesson1 = Lesson::new(
    ///     "Part 1".to_string(),
    ///     900,
    ///     "https://example.com/part1.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let lesson2 = Lesson::new(
    ///     "Part 2".to_string(),
    ///     1200,
    ///     "https://example.com/part2.mp4".to_string(),
    ///     1,
    /// ).unwrap();
    ///
    /// let chapter = Chapter::new(
    ///     "Introduction".to_string(),
    ///     0,
    ///     vec![lesson1, lesson2],
    /// ).unwrap();
    ///
    /// assert_eq!(chapter.lessons().len(), 2);
    /// ```
    #[inline]
    #[must_use]
    pub fn lessons(&self) -> &[Lesson] {
        &self.lessons
    }

    /// Calculates the total duration of all lessons in this chapter.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Chapter, Lesson};
    ///
    /// let lesson1 = Lesson::new(
    ///     "Part 1".to_string(),
    ///     1800,
    ///     "https://example.com/part1.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let lesson2 = Lesson::new(
    ///     "Part 2".to_string(),
    ///     1200,
    ///     "https://example.com/part2.mp4".to_string(),
    ///     1,
    /// ).unwrap();
    ///
    /// let chapter = Chapter::new(
    ///     "Introduction".to_string(),
    ///     0,
    ///     vec![lesson1, lesson2],
    /// ).unwrap();
    ///
    /// let total = chapter.total_duration();
    /// assert_eq!(total.total_seconds(), 3000);
    /// ```
    #[must_use]
    pub fn total_duration(&self) -> Duration {
        self.lessons
            .iter()
            .fold(Duration::default(), |acc, lesson| acc.add(&lesson.duration()))
    }

    /// Returns the number of lessons in this chapter.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Chapter, Lesson};
    ///
    /// let lesson1 = Lesson::new(
    ///     "Part 1".to_string(),
    ///     900,
    ///     "https://example.com/part1.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let lesson2 = Lesson::new(
    ///     "Part 2".to_string(),
    ///     1200,
    ///     "https://example.com/part2.mp4".to_string(),
    ///     1,
    /// ).unwrap();
    ///
    /// let chapter = Chapter::new(
    ///     "Introduction".to_string(),
    ///     0,
    ///     vec![lesson1, lesson2],
    /// ).unwrap();
    ///
    /// assert_eq!(chapter.lesson_quantity(), 2);
    /// ```
    #[inline]
    #[must_use]
    pub fn lesson_quantity(&self) -> usize {
        self.lessons.len()
    }

    /// Returns a reference to the first lessons in this chapter.
    ///
    /// # Errors
    ///
    /// Returns `ChapterError::ChapterWithEmptyLessons` if the chapter has no lessons.
    /// Note: This error should not occur with a properly constructed `Chapter`,
    /// as the constructor validates that at least one lesson is provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Chapter, Lesson};
    ///
    /// let lesson = Lesson::new(
    ///     "First Lesson".to_string(),
    ///     1800,
    ///     "https://example.com/first.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let chapter = Chapter::new(
    ///     "Getting Started".to_string(),
    ///     0,
    ///     vec![lesson],
    /// ).unwrap();
    ///
    /// let first = chapter.first_lesson().unwrap();
    /// assert_eq!(first.name().as_str(), "First Lesson");
    /// ```
    #[inline]
    pub fn first_lesson(&self) -> Result<&Lesson, ChapterError> {
        self.lessons
            .first()
            .ok_or(ChapterError::ChapterWithEmptyLessons)
    }

    /// Returns a reference to the last lesson in this chapter.
    ///
    /// # Errors
    ///
    /// Returns `ChapterError::ChapterWithEmptyLessons` if the chapter has no lessons.
    /// Note: This error should not occur with a properly constructed `Chapter`,
    /// as the constructor validates that at least one lesson is provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Chapter, Lesson};
    ///
    /// let lesson1 = Lesson::new(
    ///     "First Lesson".to_string(),
    ///     1800,
    ///     "https://example.com/first.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let lesson2 = Lesson::new(
    ///     "Last Lesson".to_string(),
    ///     1200,
    ///     "https://example.com/last.mp4".to_string(),
    ///     1,
    /// ).unwrap();
    ///
    /// let chapter = Chapter::new(
    ///     "Getting Started".to_string(),
    ///     0,
    ///     vec![lesson1, lesson2],
    /// ).unwrap();
    ///
    /// let last = chapter.last_lesson().unwrap();
    /// assert_eq!(last.name().as_str(), "Last Lesson");
    /// ```
    #[inline]
    pub fn last_lesson(&self) -> Result<&Lesson, ChapterError> {
        self.lessons
            .last()
            .ok_or(ChapterError::ChapterWithEmptyLessons)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_lesson(name: &str, index: usize) -> Lesson {
        Lesson::new(
            name.to_string(),
            1800,
            format!("https://example.com/{}.mp4", index),
            index,
        )
        .unwrap()
    }

    mod getters {
        use super::*;

        #[test]
        fn test_name_returns_simple_name() {
            let lesson = create_test_lesson("Test Lesson", 0);
            let chapter = Chapter::new("My Chapter".to_string(), 0, vec![lesson]).unwrap();

            assert_eq!(chapter.name().as_str(), "My Chapter");
        }

        #[test]
        fn test_index_returns_index() {
            let lesson = create_test_lesson("Test Lesson", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 3, vec![lesson]).unwrap();

            assert_eq!(chapter.index().value(), 3);
        }

        #[test]
        fn test_index_first_chapter() {
            let lesson = create_test_lesson("Test Lesson", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

            assert!(chapter.index().is_first());
        }

        #[test]
        fn test_lessons_returns_all_lessons() {
            let lessons = vec![
                create_test_lesson("Lesson 1", 0),
                create_test_lesson("Lesson 2", 1),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

            assert_eq!(chapter.lessons().len(), 2);
            assert_eq!(chapter.lessons()[0].name().as_str(), "Lesson 1");
            assert_eq!(chapter.lessons()[1].name().as_str(), "Lesson 2");
        }
    }

    mod total_duration {
        use super::*;

        #[test]
        fn test_total_duration_single_lesson() {
            let lesson = Lesson::new(
                "Test".to_string(),
                3600,
                "https://example.com/test.mp4".to_string(),
                0,
            )
            .unwrap();
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

            assert_eq!(chapter.total_duration().total_seconds(), 3600);
        }

        #[test]
        fn test_total_duration_multiple_lessons() {
            let lesson1 =
                Lesson::new("Test 1".to_string(), 1800, "https://example.com/1.mp4".to_string(), 0)
                    .unwrap();
            let lesson2 =
                Lesson::new("Test 2".to_string(), 1200, "https://example.com/2.mp4".to_string(), 1)
                    .unwrap();
            let chapter =
                Chapter::new("Test Chapter".to_string(), 0, vec![lesson1, lesson2]).unwrap();

            assert_eq!(chapter.total_duration().total_seconds(), 3000);
        }

        #[test]
        fn test_total_duration_returns_duration_object() {
            let lesson = Lesson::new(
                "Test".to_string(),
                7200,
                "https://example.com/test.mp4".to_string(),
                0,
            )
            .unwrap();
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

            let duration = chapter.total_duration();
            assert_eq!(duration.hours(), 2);
            assert_eq!(duration.minutes(), 0);
            assert_eq!(duration.seconds(), 0);
        }

        #[test]
        fn test_total_duration_formatted_output() {
            let lesson = Lesson::new(
                "Test".to_string(),
                3661,
                "https://example.com/test.mp4".to_string(),
                0,
            )
            .unwrap();
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

            let duration = chapter.total_duration();
            assert_eq!(duration.hours(), 1);
            assert_eq!(duration.minutes(), 1);
            assert_eq!(duration.seconds(), 1);
        }

        #[test]
        fn test_total_duration_with_varied_durations() {
            let lessons = vec![
                Lesson::new(
                    "Short".to_string(),
                    60,
                    "https://example.com/short.mp4".to_string(),
                    0,
                )
                .unwrap(),
                Lesson::new(
                    "Medium".to_string(),
                    600,
                    "https://example.com/medium.mp4".to_string(),
                    1,
                )
                .unwrap(),
                Lesson::new(
                    "Long".to_string(),
                    3600,
                    "https://example.com/long.mp4".to_string(),
                    2,
                )
                .unwrap(),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

            assert_eq!(chapter.total_duration().total_seconds(), 4260);
        }
    }

    mod lesson_quantity {
        use super::*;

        #[test]
        fn test_lesson_quantity_single_lesson() {
            let lesson = create_test_lesson("Test", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

            assert_eq!(chapter.lesson_quantity(), 1);
        }

        #[test]
        fn test_lesson_quantity_multiple_lessons() {
            let lessons = vec![
                create_test_lesson("Lesson 1", 0),
                create_test_lesson("Lesson 2", 1),
                create_test_lesson("Lesson 3", 2),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

            assert_eq!(chapter.lesson_quantity(), 3);
        }

        #[test]
        fn test_lesson_quantity_matches_lessons_len() {
            let lessons = vec![
                create_test_lesson("Lesson 1", 0),
                create_test_lesson("Lesson 2", 1),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

            assert_eq!(chapter.lesson_quantity(), chapter.lessons().len());
        }
    }

    mod first_lesson {
        use super::*;
        use education_platform_common::Entity;

        #[test]
        fn test_first_lesson_returns_first_lesson() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
                create_test_lesson("Third", 2),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

            let first = chapter.first_lesson().unwrap();
            assert_eq!(first.name().as_str(), "First");
        }

        #[test]
        fn test_first_lesson_single_lesson() {
            let lesson = create_test_lesson("Only One", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

            let first = chapter.first_lesson().unwrap();
            assert_eq!(first.name().as_str(), "Only One");
        }

        #[test]
        fn test_first_lesson_returns_reference() {
            let lesson = create_test_lesson("First", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

            let first = chapter.first_lesson().unwrap();
            assert_eq!(first.id(), chapter.lessons()[0].id());
        }
    }

    mod last_lesson {
        use super::*;
        use education_platform_common::Entity;

        #[test]
        fn test_last_lesson_returns_last_lesson() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
                create_test_lesson("Third", 2),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

            let last = chapter.last_lesson().unwrap();
            assert_eq!(last.name().as_str(), "Third");
        }

        #[test]
        fn test_last_lesson_single_lesson() {
            let lesson = create_test_lesson("Only One", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

            let last = chapter.last_lesson().unwrap();
            assert_eq!(last.name().as_str(), "Only One");
        }

        #[test]
        fn test_last_lesson_returns_reference() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

            let last = chapter.last_lesson().unwrap();
            assert_eq!(last.id(), chapter.lessons()[1].id());
        }

        #[test]
        fn test_first_and_last_same_for_single_lesson() {
            let lesson = create_test_lesson("Only", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

            let first = chapter.first_lesson().unwrap();
            let last = chapter.last_lesson().unwrap();

            assert_eq!(first.id(), last.id());
        }

        #[test]
        fn test_first_and_last_different_for_multiple_lessons() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Last", 1),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

            let first = chapter.first_lesson().unwrap();
            let last = chapter.last_lesson().unwrap();

            assert_ne!(first.id(), last.id());
        }
    }
}
