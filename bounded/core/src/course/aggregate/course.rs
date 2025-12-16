mod add_chapter;
mod chapter_operations;
mod getters;
mod delete_chapter;
mod move_chapter;

use crate::Chapter;
use education_platform_common::{
    Date, Duration, Entity, Id, SimpleName, SimpleNameConfig, SimpleNameError,
};
use thiserror::Error;

/// Error types for Course validation failures.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CourseError {
    #[error("Chapter name validation failed: {0}")]
    NameError(#[from] SimpleNameError),

    #[error("Course must have at least one chapter")]
    CourseWithEmptyChapters,

    #[error("Chapter with does not exist")]
    ChapterDoesNotExist,
}

/// A course containing multiple chapters.
///
/// `Course` is an aggregate root that groups related chapters together.
/// Each course has a name, date, chapters, and computed totals for duration
/// and number of lessons.
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
/// assert_eq!(course.chapter_quantity(), 1);
/// ```
#[derive(Clone)]
pub struct Course {
    id: Id,
    name: SimpleName,
    date: Date,
    chapters: Vec<Chapter>,
    duration: Duration,
    number_of_lessons: u32,
}

impl Course {
    /// Creates a new `Course` with the provided parameters.
    ///
    /// The course's total duration and number of lessons are automatically
    /// calculated from the provided chapters. If chapters are provided, the
    /// `duration` parameter is ignored and the total is computed from chapter
    /// durations.
    ///
    /// # Arguments
    ///
    /// * `name` - The course name (validated as SimpleName, 3-50 characters)
    /// * `date` - Optional course date; defaults to today if not provided
    /// * `duration` - Base duration in seconds (used only if no chapters provided)
    /// * `chapters` - List of chapters belonging to this course
    ///
    /// # Errors
    ///
    /// Returns `CourseError::NameError` if the name validation fails.
    /// Returns `CourseError::CourseWithEmptyChapters` if no chapters are provided.
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
    /// ```
    pub fn new(
        name: String,
        date: Option<Date>,
        duration: u64,
        chapters: Vec<Chapter>,
    ) -> Result<Self, CourseError> {
        let name = SimpleName::with_config(name, SimpleNameConfig::new(3, 50))?;
        let base_duration = Duration::from_seconds(duration);
        let chapters = Self::order_chapter(chapters)?;
        let (total_duration, number_of_lessons) = Self::calculate_totals(&chapters, base_duration);

        Ok(Self {
            id: Id::default(),
            name,
            date: date.unwrap_or_else(Date::today),
            duration: total_duration,
            chapters,
            number_of_lessons,
        })
    }
}

impl Entity for Course {
    fn id(&self) -> Id {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Lesson;

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

    mod constructors {
        use super::*;

        #[test]
        fn test_new_creates_valid_course() {
            let chapter = create_test_chapter("Chapter One", 0);
            let course = Course::new("Test Course".to_string(), None, 0, vec![chapter]).unwrap();

            assert_eq!(course.name().as_str(), "Test Course");
            assert_eq!(course.chapter_quantity(), 1);
        }

        #[test]
        fn test_new_generates_unique_id() {
            let chapter1 = create_test_chapter("Chapter One", 0);
            let chapter2 = create_test_chapter("Chapter Two", 0);

            let course1 = Course::new("Course 1".to_string(), None, 0, vec![chapter1]).unwrap();
            let course2 = Course::new("Course 2".to_string(), None, 0, vec![chapter2]).unwrap();

            assert_ne!(course1.id(), course2.id());
        }

        #[test]
        fn test_new_with_empty_chapters_returns_error() {
            let result = Course::new("Test Course".to_string(), None, 0, vec![]);

            assert!(result.is_err());
            assert!(matches!(result, Err(CourseError::CourseWithEmptyChapters)));
        }

        #[test]
        fn test_new_with_empty_name_returns_error() {
            let chapter = create_test_chapter("Chapter One", 0);
            let result = Course::new("".to_string(), None, 0, vec![chapter]);

            assert!(result.is_err());
            assert!(matches!(result, Err(CourseError::NameError(_))));
        }

        #[test]
        fn test_new_with_multiple_chapters() {
            let chapters = vec![
                create_test_chapter("Chapter 1", 0),
                create_test_chapter("Chapter 2", 1),
                create_test_chapter("Chapter 3", 2),
            ];

            let course = Course::new("Test Course".to_string(), None, 0, chapters).unwrap();

            assert_eq!(course.chapter_quantity(), 3);
        }

        #[test]
        fn test_new_with_name_at_min_length() {
            let chapter = create_test_chapter("Chapter One", 0);
            let course = Course::new("ABC".to_string(), None, 0, vec![chapter]).unwrap();

            assert_eq!(course.name().as_str(), "ABC");
        }

        #[test]
        fn test_new_with_name_at_max_length() {
            let chapter = create_test_chapter("Chapter One", 0);
            let name = "A".repeat(50);
            let course = Course::new(name.clone(), None, 0, vec![chapter]).unwrap();

            assert_eq!(course.name().as_str(), name);
        }

        #[test]
        fn test_new_with_name_too_short_returns_error() {
            let chapter = create_test_chapter("Chapter One", 0);
            let result = Course::new("AB".to_string(), None, 0, vec![chapter]);

            assert!(result.is_err());
            assert!(matches!(result, Err(CourseError::NameError(_))));
        }

        #[test]
        fn test_new_with_name_too_long_returns_error() {
            let chapter = create_test_chapter("Chapter One", 0);
            let name = "A".repeat(51);
            let result = Course::new(name, None, 0, vec![chapter]);

            assert!(result.is_err());
            assert!(matches!(result, Err(CourseError::NameError(_))));
        }

        #[test]
        fn test_new_with_provided_date() {
            let chapter = create_test_chapter("Chapter One", 0);
            let date = Date::new(2024, 6, 15).unwrap();
            let course =
                Course::new("Test Course".to_string(), Some(date), 0, vec![chapter]).unwrap();

            assert_eq!(course.date().year(), 2024);
            assert_eq!(course.date().month(), 6);
            assert_eq!(course.date().day(), 15);
        }

        #[test]
        fn test_new_calculates_total_duration() {
            let lesson1 = Lesson::new(
                "Lesson 1".to_string(),
                1800,
                "https://example.com/1.mp4".to_string(),
                0,
            )
            .unwrap();
            let lesson2 = Lesson::new(
                "Lesson 2".to_string(),
                1200,
                "https://example.com/2.mp4".to_string(),
                0,
            )
            .unwrap();
            let chapter1 = Chapter::new("Chapter 1".to_string(), 0, vec![lesson1]).unwrap();
            let chapter2 = Chapter::new("Chapter 2".to_string(), 1, vec![lesson2]).unwrap();

            let course =
                Course::new("Test Course".to_string(), None, 0, vec![chapter1, chapter2]).unwrap();

            assert_eq!(course.duration().total_seconds(), 3000);
        }

        #[test]
        fn test_new_calculates_total_lessons() {
            let chapter1 = Chapter::new(
                "Chapter 1".to_string(),
                0,
                vec![
                    create_test_lesson("Lesson 1", 0),
                    create_test_lesson("Lesson 2", 1),
                ],
            )
            .unwrap();
            let chapter2 =
                Chapter::new("Chapter 2".to_string(), 1, vec![create_test_lesson("Lesson 3", 0)])
                    .unwrap();

            let course =
                Course::new("Test Course".to_string(), None, 0, vec![chapter1, chapter2]).unwrap();

            assert_eq!(course.number_of_lessons(), 3);
        }

        #[test]
        fn test_new_orders_chapters_by_index() {
            let chapters = vec![
                create_test_chapter("Third", 10),
                create_test_chapter("First", 2),
                create_test_chapter("Second", 5),
            ];

            let course = Course::new("Test Course".to_string(), None, 0, chapters).unwrap();

            assert_eq!(course.chapters()[0].name().as_str(), "First");
            assert_eq!(course.chapters()[1].name().as_str(), "Second");
            assert_eq!(course.chapters()[2].name().as_str(), "Third");
        }
    }

    mod entity_trait {
        use super::*;

        #[test]
        fn test_id_returns_valid_id() {
            let chapter = create_test_chapter("Chapter One", 0);
            let course = Course::new("Test Course".to_string(), None, 0, vec![chapter]).unwrap();

            let id = course.id();
            assert!(!id.to_string().is_empty());
        }
    }
}
