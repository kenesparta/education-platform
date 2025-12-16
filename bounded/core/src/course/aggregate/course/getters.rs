use super::{Chapter, Course, Date, Duration, SimpleName};
use crate::CourseError;

impl Course {
    /// Returns the course name.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Course, Chapter, Lesson};
    ///
    /// let lesson = Lesson::new(
    ///     "Introduction".to_string(),
    ///     1800,
    ///     "https://example.com/intro.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let chapter = Chapter::new(
    ///     "Getting Started".to_string(),
    ///     0,
    ///     vec![lesson],
    /// ).unwrap();
    ///
    /// let course = Course::new(
    ///     "Rust Programming".to_string(),
    ///     None,
    ///     0,
    ///     vec![chapter],
    /// ).unwrap();
    ///
    /// assert_eq!(course.name().as_str(), "Rust Programming");
    /// ```
    #[inline]
    #[must_use]
    pub fn name(&self) -> &SimpleName {
        &self.name
    }

    /// Returns the course date.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Course, Chapter, Lesson};
    /// use education_platform_common::Date;
    ///
    /// let lesson = Lesson::new(
    ///     "Introduction".to_string(),
    ///     1800,
    ///     "https://example.com/intro.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let chapter = Chapter::new(
    ///     "Getting Started".to_string(),
    ///     0,
    ///     vec![lesson],
    /// ).unwrap();
    ///
    /// let date = Date::new(2024, 1, 15).unwrap();
    /// let course = Course::new(
    ///     "Rust Programming".to_string(),
    ///     Some(date),
    ///     0,
    ///     vec![chapter],
    /// ).unwrap();
    ///
    /// assert_eq!(course.date().year(), 2024);
    /// ```
    #[inline]
    #[must_use]
    pub fn date(&self) -> &Date {
        &self.date
    }

    /// Returns a slice of all chapters in this course.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Course, Chapter, Lesson};
    ///
    /// let lesson = Lesson::new(
    ///     "Introduction".to_string(),
    ///     1800,
    ///     "https://example.com/intro.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let chapter = Chapter::new(
    ///     "Getting Started".to_string(),
    ///     0,
    ///     vec![lesson],
    /// ).unwrap();
    ///
    /// let course = Course::new(
    ///     "Rust Programming".to_string(),
    ///     None,
    ///     0,
    ///     vec![chapter],
    /// ).unwrap();
    ///
    /// assert_eq!(course.chapters().len(), 1);
    /// ```
    #[inline]
    #[must_use]
    pub fn chapters(&self) -> &[Chapter] {
        &self.chapters
    }

    /// Returns the total duration of the course.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Course, Chapter, Lesson};
    ///
    /// let lesson = Lesson::new(
    ///     "Introduction".to_string(),
    ///     1800,
    ///     "https://example.com/intro.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let chapter = Chapter::new(
    ///     "Getting Started".to_string(),
    ///     0,
    ///     vec![lesson],
    /// ).unwrap();
    ///
    /// let course = Course::new(
    ///     "Rust Programming".to_string(),
    ///     None,
    ///     0,
    ///     vec![chapter],
    /// ).unwrap();
    ///
    /// assert_eq!(course.duration().total_seconds(), 1800);
    /// ```
    #[inline]
    #[must_use]
    pub fn duration(&self) -> &Duration {
        &self.duration
    }

    /// Returns the total number of lessons across all chapters.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Course, Chapter, Lesson};
    ///
    /// let lesson1 = Lesson::new(
    ///     "Introduction".to_string(),
    ///     1800,
    ///     "https://example.com/intro.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let lesson2 = Lesson::new(
    ///     "Basics".to_string(),
    ///     1200,
    ///     "https://example.com/basics.mp4".to_string(),
    ///     1,
    /// ).unwrap();
    ///
    /// let chapter = Chapter::new(
    ///     "Getting Started".to_string(),
    ///     0,
    ///     vec![lesson1, lesson2],
    /// ).unwrap();
    ///
    /// let course = Course::new(
    ///     "Rust Programming".to_string(),
    ///     None,
    ///     0,
    ///     vec![chapter],
    /// ).unwrap();
    ///
    /// assert_eq!(course.number_of_lessons(), 2);
    /// ```
    #[inline]
    #[must_use]
    pub fn number_of_lessons(&self) -> u32 {
        self.number_of_lessons
    }

    /// Returns the number of chapters in this course.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Course, Chapter, Lesson};
    ///
    /// let lesson = Lesson::new(
    ///     "Introduction".to_string(),
    ///     1800,
    ///     "https://example.com/intro.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let chapter = Chapter::new(
    ///     "Getting Started".to_string(),
    ///     0,
    ///     vec![lesson],
    /// ).unwrap();
    ///
    /// let course = Course::new(
    ///     "Rust Programming".to_string(),
    ///     None,
    ///     0,
    ///     vec![chapter],
    /// ).unwrap();
    ///
    /// assert_eq!(course.chapter_quantity(), 1);
    /// ```
    #[inline]
    #[must_use]
    pub fn chapter_quantity(&self) -> usize {
        self.chapters.len()
    }

    /// Returns a reference to the first chapter in this course.
    ///
    /// # Errors
    ///
    /// Returns `CourseError::CourseWithEmptyChapters` if the course has no chapters.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Course, Chapter, Lesson};
    ///
    /// let lesson1 = Lesson::new(
    ///     "Introduction".to_string(),
    ///     1800,
    ///     "https://example.com/intro.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let lesson2 = Lesson::new(
    ///     "Advanced".to_string(),
    ///     1800,
    ///     "https://example.com/advanced.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let chapter1 = Chapter::new(
    ///     "Getting Started".to_string(),
    ///     0,
    ///     vec![lesson1],
    /// ).unwrap();
    ///
    /// let chapter2 = Chapter::new(
    ///     "Advanced Topics".to_string(),
    ///     1,
    ///     vec![lesson2],
    /// ).unwrap();
    ///
    /// let course = Course::new(
    ///     "Rust Programming".to_string(),
    ///     None,
    ///     0,
    ///     vec![chapter1, chapter2],
    /// ).unwrap();
    ///
    /// assert_eq!(course.first_chapter().unwrap().name().as_str(), "Getting Started");
    /// ```
    #[inline]
    pub fn first_chapter(&self) -> Result<&Chapter, CourseError> {
        self.chapters
            .first()
            .ok_or(CourseError::CourseWithEmptyChapters)
    }

    /// Returns a reference to the last chapter in this course.
    ///
    /// # Errors
    ///
    /// Returns `CourseError::CourseWithEmptyChapters` if the course has no chapters.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Course, Chapter, Lesson};
    ///
    /// let lesson1 = Lesson::new(
    ///     "Introduction".to_string(),
    ///     1800,
    ///     "https://example.com/intro.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let lesson2 = Lesson::new(
    ///     "Advanced".to_string(),
    ///     1800,
    ///     "https://example.com/advanced.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let chapter1 = Chapter::new(
    ///     "Getting Started".to_string(),
    ///     0,
    ///     vec![lesson1],
    /// ).unwrap();
    ///
    /// let chapter2 = Chapter::new(
    ///     "Advanced Topics".to_string(),
    ///     1,
    ///     vec![lesson2],
    /// ).unwrap();
    ///
    /// let course = Course::new(
    ///     "Rust Programming".to_string(),
    ///     None,
    ///     0,
    ///     vec![chapter1, chapter2],
    /// ).unwrap();
    ///
    /// assert_eq!(course.last_chapter().unwrap().name().as_str(), "Advanced Topics");
    /// ```
    #[inline]
    pub fn last_chapter(&self) -> Result<&Chapter, CourseError> {
        self.chapters
            .last()
            .ok_or(CourseError::CourseWithEmptyChapters)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Lesson;
    use education_platform_common::Entity;

    fn create_test_lesson(name: &str, index: usize) -> Lesson {
        Lesson::new(
            name.to_string(),
            1800,
            format!("https://example.com/{}.mp4", index),
            index,
        )
        .unwrap()
    }

    fn create_test_chapter(name: &str, index: usize) -> Chapter {
        let lesson = create_test_lesson(&format!("{} Lesson", name), 0);
        Chapter::new(name.to_string(), index, vec![lesson]).unwrap()
    }

    mod name {
        use super::*;

        #[test]
        fn test_name_returns_simple_name() {
            let chapter = create_test_chapter("Chapter One", 0);
            let course =
                Course::new("Rust Programming".to_string(), None, 0, vec![chapter]).unwrap();

            assert_eq!(course.name().as_str(), "Rust Programming");
        }
    }

    mod date {
        use super::*;

        #[test]
        fn test_date_returns_provided_date() {
            let chapter = create_test_chapter("Chapter One", 0);
            let date = Date::new(2024, 6, 15).unwrap();
            let course =
                Course::new("Rust Programming".to_string(), Some(date), 0, vec![chapter]).unwrap();

            assert_eq!(course.date().year(), 2024);
            assert_eq!(course.date().month(), 6);
            assert_eq!(course.date().day(), 15);
        }

        #[test]
        fn test_date_defaults_to_today_when_none() {
            let chapter = create_test_chapter("Chapter One", 0);
            let today = Date::today();
            let course =
                Course::new("Rust Programming".to_string(), None, 0, vec![chapter]).unwrap();

            assert_eq!(course.date().year(), today.year());
            assert_eq!(course.date().month(), today.month());
            assert_eq!(course.date().day(), today.day());
        }
    }

    mod chapters {
        use super::*;

        #[test]
        fn test_chapters_returns_all_chapters() {
            let chapters = vec![
                create_test_chapter("Chapter One", 0),
                create_test_chapter("Chapter Two", 1),
            ];
            let course = Course::new("Rust Programming".to_string(), None, 0, chapters).unwrap();

            assert_eq!(course.chapters().len(), 2);
        }

        #[test]
        fn test_chapters_preserves_order() {
            let chapters = vec![
                create_test_chapter("First", 0),
                create_test_chapter("Second", 1),
                create_test_chapter("Third", 2),
            ];
            let course = Course::new("Rust Programming".to_string(), None, 0, chapters).unwrap();

            assert_eq!(course.chapters()[0].name().as_str(), "First");
            assert_eq!(course.chapters()[1].name().as_str(), "Second");
            assert_eq!(course.chapters()[2].name().as_str(), "Third");
        }
    }

    mod duration {
        use super::*;

        #[test]
        fn test_duration_returns_total_duration() {
            let lesson = Lesson::new(
                "Test Lesson".to_string(),
                3600,
                "https://example.com/test.mp4".to_string(),
                0,
            )
            .unwrap();
            let chapter = Chapter::new("Chapter One".to_string(), 0, vec![lesson]).unwrap();
            let course =
                Course::new("Rust Programming".to_string(), None, 0, vec![chapter]).unwrap();

            assert_eq!(course.duration().total_seconds(), 3600);
        }

        #[test]
        fn test_duration_sums_multiple_chapters() {
            let lesson1 = Lesson::new(
                "Lesson One".to_string(),
                1800,
                "https://example.com/1.mp4".to_string(),
                0,
            )
            .unwrap();
            let lesson2 = Lesson::new(
                "Lesson Two".to_string(),
                1200,
                "https://example.com/2.mp4".to_string(),
                0,
            )
            .unwrap();
            let chapter1 = Chapter::new("Chapter One".to_string(), 0, vec![lesson1]).unwrap();
            let chapter2 = Chapter::new("Chapter Two".to_string(), 1, vec![lesson2]).unwrap();
            let course =
                Course::new("Rust Programming".to_string(), None, 0, vec![chapter1, chapter2])
                    .unwrap();

            assert_eq!(course.duration().total_seconds(), 3000);
        }
    }

    mod number_of_lessons {
        use super::*;

        #[test]
        fn test_number_of_lessons_single_chapter() {
            let lessons = vec![
                create_test_lesson("Lesson 1", 0),
                create_test_lesson("Lesson 2", 1),
            ];
            let chapter = Chapter::new("Chapter One".to_string(), 0, lessons).unwrap();
            let course =
                Course::new("Rust Programming".to_string(), None, 0, vec![chapter]).unwrap();

            assert_eq!(course.number_of_lessons(), 2);
        }

        #[test]
        fn test_number_of_lessons_multiple_chapters() {
            let chapter1 = Chapter::new(
                "Chapter One".to_string(),
                0,
                vec![
                    create_test_lesson("Lesson 1", 0),
                    create_test_lesson("Lesson 2", 1),
                ],
            )
            .unwrap();
            let chapter2 =
                Chapter::new("Chapter Two".to_string(), 1, vec![create_test_lesson("Lesson 3", 0)])
                    .unwrap();
            let course =
                Course::new("Rust Programming".to_string(), None, 0, vec![chapter1, chapter2])
                    .unwrap();

            assert_eq!(course.number_of_lessons(), 3);
        }
    }

    mod chapter_quantity {
        use super::*;

        #[test]
        fn test_chapter_quantity_single_chapter() {
            let chapter = create_test_chapter("Chapter One", 0);
            let course =
                Course::new("Rust Programming".to_string(), None, 0, vec![chapter]).unwrap();

            assert_eq!(course.chapter_quantity(), 1);
        }

        #[test]
        fn test_chapter_quantity_multiple_chapters() {
            let chapters = vec![
                create_test_chapter("Chapter One", 0),
                create_test_chapter("Chapter Two", 1),
                create_test_chapter("Chapter Three", 2),
            ];
            let course = Course::new("Rust Programming".to_string(), None, 0, chapters).unwrap();

            assert_eq!(course.chapter_quantity(), 3);
        }
    }

    mod first_chapter {
        use super::*;

        #[test]
        fn test_first_chapter_returns_first() {
            let chapters = vec![
                create_test_chapter("First Chapter", 0),
                create_test_chapter("Second Chapter", 1),
            ];
            let course = Course::new("Rust Programming".to_string(), None, 0, chapters).unwrap();

            assert_eq!(course.first_chapter().unwrap().name().as_str(), "First Chapter");
        }

        #[test]
        fn test_first_chapter_single_chapter() {
            let chapter = create_test_chapter("Only Chapter", 0);
            let course =
                Course::new("Rust Programming".to_string(), None, 0, vec![chapter]).unwrap();

            assert_eq!(course.first_chapter().unwrap().name().as_str(), "Only Chapter");
        }

        #[test]
        fn test_first_chapter_returns_reference() {
            let chapter = create_test_chapter("Chapter One", 0);
            let course =
                Course::new("Rust Programming".to_string(), None, 0, vec![chapter]).unwrap();

            let first = course.first_chapter().unwrap();
            assert_eq!(first.id(), course.chapters()[0].id());
        }
    }

    mod last_chapter {
        use super::*;

        #[test]
        fn test_last_chapter_returns_last() {
            let chapters = vec![
                create_test_chapter("First Chapter", 0),
                create_test_chapter("Last Chapter", 1),
            ];
            let course = Course::new("Rust Programming".to_string(), None, 0, chapters).unwrap();

            assert_eq!(course.last_chapter().unwrap().name().as_str(), "Last Chapter");
        }

        #[test]
        fn test_last_chapter_single_chapter() {
            let chapter = create_test_chapter("Only Chapter", 0);
            let course =
                Course::new("Rust Programming".to_string(), None, 0, vec![chapter]).unwrap();

            assert_eq!(course.last_chapter().unwrap().name().as_str(), "Only Chapter");
        }

        #[test]
        fn test_first_and_last_same_for_single_chapter() {
            let chapter = create_test_chapter("Only Chapter", 0);
            let course =
                Course::new("Rust Programming".to_string(), None, 0, vec![chapter]).unwrap();

            assert_eq!(
                course.first_chapter().unwrap().id(),
                course.last_chapter().unwrap().id()
            );
        }

        #[test]
        fn test_first_and_last_different_for_multiple_chapters() {
            let chapters = vec![
                create_test_chapter("First", 0),
                create_test_chapter("Last", 1),
            ];
            let course = Course::new("Rust Programming".to_string(), None, 0, chapters).unwrap();

            assert_ne!(
                course.first_chapter().unwrap().id(),
                course.last_chapter().unwrap().id()
            );
        }
    }
}
