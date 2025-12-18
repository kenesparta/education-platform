mod add_lesson;
mod delete_lesson;
mod getters;
mod lesson_operations;
mod move_lesson;

use crate::Lesson;
use education_platform_common::{
    Duration, Entity, Id, Index, SimpleName, SimpleNameConfig, SimpleNameError,
};
use thiserror::Error;

/// Error types for Chapter validation failures.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ChapterError {
    #[error("Chapter name validation failed: {0}")]
    NameError(#[from] SimpleNameError),

    #[error("Chapter must have at least one lesson")]
    ChapterWithEmptyLessons,

    #[error("Lesson does not exist")]
    LessonDoesNotExist,
}

/// A chapter within a course, containing multiple lessons.
///
/// `Chapter` is an entity that groups related lessons together within a course.
/// Each chapter has a name, position index, and a collection of lessons.
///
/// # Examples
///
/// ```
/// use education_platform_core::{Chapter, Lesson};
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
/// assert_eq!(chapter.name().as_str(), "Getting Started");
/// assert!(chapter.index().is_first());
/// assert_eq!(chapter.lessons().len(), 1);
/// ```
#[derive(Clone)]
pub struct Chapter {
    id: Id,
    name: SimpleName,
    index: Index,
    lessons: Vec<Lesson>,
}

impl Chapter {
    /// Creates a new `Chapter` with the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `name` - The chapter name (3-50 characters)
    /// * `index` - Position of this chapter within the course (zero-based)
    /// * `lessons` - Collection of lessons belonging to this chapter (must not be empty)
    ///
    /// # Errors
    ///
    /// Returns `ChapterError::NameError` if the name validation fails.
    /// Returns `ChapterError::ChapterWithEmptyLessons` if no lessons are provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Chapter, Lesson};
    ///
    /// let lesson = Lesson::new(
    ///     "Lesson One".to_string(),
    ///     3600,
    ///     "https://example.com/lesson1.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let chapter = Chapter::new(
    ///     "Chapter 1: Basics".to_string(),
    ///     0,
    ///     vec![lesson],
    /// ).unwrap();
    ///
    /// assert_eq!(chapter.name().as_str(), "Chapter 1: Basics");
    /// ```
    pub fn new(name: String, index: usize, lessons: Vec<Lesson>) -> Result<Self, ChapterError> {
        let name = SimpleName::with_config(name, SimpleNameConfig::new(3, 50))?;
        let lessons = Self::order_lessons(lessons)?;
        let index = Index::new(index);
        let id = Id::default();

        Ok(Self {
            id,
            name,
            index,
            lessons,
        })
    }

    /// Updates the position index of this chapter within the course.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Chapter, Lesson};
    ///
    /// let lesson = Lesson::new(
    ///     "Introduction".to_string(),
    ///     1800,
    ///     "https://example.com/intro.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let mut chapter = Chapter::new(
    ///     "Getting Started".to_string(),
    ///     0,
    ///     vec![lesson],
    /// ).unwrap();
    ///
    /// assert_eq!(chapter.index().value(), 0);
    ///
    /// chapter.update_index(2);
    /// assert_eq!(chapter.index().value(), 2);
    /// ```
    #[inline]
    pub fn update_index(&mut self, index: usize) {
        self.index = Index::new(index);
    }
}

impl Entity for Chapter {
    fn id(&self) -> Id {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn create_test_lesson(name: &str, index: usize) -> Lesson {
        Lesson::new(
            name.to_string(),
            1800,
            format!("https://example.com/{}.mp4", index),
            index,
        )
        .unwrap()
    }

    mod constructors {
        use super::*;

        #[test]
        fn test_new_creates_valid_chapter() {
            let lesson = create_test_lesson("Test Lesson", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

            assert_eq!(chapter.name().as_str(), "Test Chapter");
            assert_eq!(chapter.index().value(), 0);
            assert_eq!(chapter.lessons().len(), 1);
        }

        #[test]
        fn test_new_generates_unique_id() {
            let lesson1 = create_test_lesson("Test Lesson", 0);
            let lesson2 = create_test_lesson("Test Lesson", 0);

            let chapter1 = Chapter::new("Chapter 1".to_string(), 0, vec![lesson1]).unwrap();
            let chapter2 = Chapter::new("Chapter 2".to_string(), 0, vec![lesson2]).unwrap();

            assert_ne!(chapter1.id(), chapter2.id());
        }

        #[test]
        fn test_new_with_different_index() {
            let lesson = create_test_lesson("Test Lesson", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 5, vec![lesson]).unwrap();

            assert_eq!(chapter.index().value(), 5);
        }

        #[test]
        fn test_new_with_empty_lessons_returns_error() {
            let result = Chapter::new("Test Chapter".to_string(), 0, vec![]);

            assert!(result.is_err());
            assert!(matches!(result, Err(ChapterError::ChapterWithEmptyLessons)));
        }

        #[test]
        fn test_new_with_empty_name_returns_error() {
            let lesson = create_test_lesson("Test Lesson", 0);
            let result = Chapter::new("".to_string(), 0, vec![lesson]);

            assert!(result.is_err());
            assert!(matches!(result, Err(ChapterError::NameError(_))));
        }

        #[test]
        fn test_new_with_multiple_lessons() {
            let lessons = vec![
                create_test_lesson("Lesson 1", 0),
                create_test_lesson("Lesson 2", 1),
                create_test_lesson("Lesson 3", 2),
            ];

            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

            assert_eq!(chapter.lessons().len(), 3);
        }

        #[test]
        fn test_new_with_name_at_min_length() {
            let lesson = create_test_lesson("Test Lesson", 0);
            let chapter = Chapter::new("ABC".to_string(), 0, vec![lesson]).unwrap();

            assert_eq!(chapter.name().as_str(), "ABC");
        }

        #[test]
        fn test_new_with_name_at_max_length() {
            let lesson = create_test_lesson("Test Lesson", 0);
            let name = "A".repeat(50);
            let chapter = Chapter::new(name.clone(), 0, vec![lesson]).unwrap();

            assert_eq!(chapter.name().as_str(), name);
        }

        #[test]
        fn test_new_with_name_too_short_returns_error() {
            let lesson = create_test_lesson("Test Lesson", 0);
            let result = Chapter::new("AB".to_string(), 0, vec![lesson]);

            assert!(result.is_err());
            assert!(matches!(result, Err(ChapterError::NameError(_))));
        }

        #[test]
        fn test_new_with_name_too_long_returns_error() {
            let lesson = create_test_lesson("Test Lesson", 0);
            let name = "A".repeat(51);
            let result = Chapter::new(name, 0, vec![lesson]);

            assert!(result.is_err());
            assert!(matches!(result, Err(ChapterError::NameError(_))));
        }
    }

    mod entity_trait {
        use super::*;

        #[test]
        fn test_id_returns_valid_id() {
            let lesson = create_test_lesson("Test Lesson", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

            let id = chapter.id();
            assert!(!id.to_string().is_empty());
        }
    }

    mod update_index {
        use super::*;

        #[test]
        fn test_update_index_changes_index() {
            let lesson = create_test_lesson("Test Lesson", 0);
            let mut chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

            chapter.update_index(5);

            assert_eq!(chapter.index().value(), 5);
        }

        #[test]
        fn test_update_index_to_zero() {
            let lesson = create_test_lesson("Test Lesson", 0);
            let mut chapter = Chapter::new("Test Chapter".to_string(), 3, vec![lesson]).unwrap();

            chapter.update_index(0);

            assert_eq!(chapter.index().value(), 0);
            assert!(chapter.index().is_first());
        }

        #[test]
        fn test_update_index_to_large_value() {
            let lesson = create_test_lesson("Test Lesson", 0);
            let mut chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

            chapter.update_index(1000);

            assert_eq!(chapter.index().value(), 1000);
        }

        #[test]
        fn test_update_index_preserves_id() {
            let lesson = create_test_lesson("Test Lesson", 0);
            let mut chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();
            let original_id = chapter.id();

            chapter.update_index(10);

            assert_eq!(chapter.id(), original_id);
        }

        #[test]
        fn test_update_index_preserves_name() {
            let lesson = create_test_lesson("Test Lesson", 0);
            let mut chapter = Chapter::new("My Chapter".to_string(), 0, vec![lesson]).unwrap();

            chapter.update_index(5);

            assert_eq!(chapter.name().as_str(), "My Chapter");
        }

        #[test]
        fn test_update_index_preserves_lessons() {
            let lessons = vec![
                create_test_lesson("Lesson 1", 0),
                create_test_lesson("Lesson 2", 1),
            ];
            let mut chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

            chapter.update_index(5);

            assert_eq!(chapter.lessons().len(), 2);
            assert_eq!(chapter.lessons()[0].name().as_str(), "Lesson 1");
            assert_eq!(chapter.lessons()[1].name().as_str(), "Lesson 2");
        }

        #[test]
        fn test_update_index_multiple_times() {
            let lesson = create_test_lesson("Test Lesson", 0);
            let mut chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

            chapter.update_index(1);
            assert_eq!(chapter.index().value(), 1);

            chapter.update_index(5);
            assert_eq!(chapter.index().value(), 5);

            chapter.update_index(0);
            assert_eq!(chapter.index().value(), 0);
        }

        #[test]
        fn test_update_index_same_value() {
            let lesson = create_test_lesson("Test Lesson", 0);
            let mut chapter = Chapter::new("Test Chapter".to_string(), 3, vec![lesson]).unwrap();

            chapter.update_index(3);

            assert_eq!(chapter.index().value(), 3);
        }
    }
}
