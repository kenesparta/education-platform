use crate::Class;
use education_platform_common::{
    Entity, Id, Index, SimpleName, SimpleNameConfig, SimpleNameError,
};
use thiserror::Error;

/// Error types for Chapter validation failures.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ChapterError {
    #[error("Chapter name validation failed: {0}")]
    NameError(#[from] SimpleNameError),

    #[error("Chapter must have at least one class")]
    ChapterWithEmptyClasses,
}

/// A chapter within a course, containing multiple classes.
///
/// `Chapter` is an entity that groups related classes together within a course.
/// Each chapter has a name, position index, and a collection of classes.
///
/// # Examples
///
/// ```
/// use education_platform_core::{Chapter, Class};
///
/// let class = Class::new(
///     "Introduction".to_string(),
///     1800,
///     "https://example.com/intro.mp4".to_string(),
///     0,
/// ).unwrap();
///
/// let chapter = Chapter::new(
///     "Getting Started".to_string(),
///     0,
///     vec![class],
/// ).unwrap();
///
/// assert_eq!(chapter.name().as_str(), "Getting Started");
/// assert!(chapter.index().is_first());
/// assert_eq!(chapter.classes().len(), 1);
/// ```
pub struct Chapter {
    id: Id,
    name: SimpleName,
    index: Index,
    classes: Vec<Class>,
}

impl Chapter {
    /// Creates a new `Chapter` with the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `name` - The chapter name (3-50 characters)
    /// * `index` - Position of this chapter within the course (zero-based)
    /// * `classes` - Collection of classes belonging to this chapter (must not be empty)
    ///
    /// # Errors
    ///
    /// Returns `ChapterError::NameError` if the name validation fails.
    /// Returns `ChapterError::ChapterWithEmptyClasses` if no classes are provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Chapter, Class};
    ///
    /// let class = Class::new(
    ///     "Lesson One".to_string(),
    ///     3600,
    ///     "https://example.com/lesson1.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let chapter = Chapter::new(
    ///     "Chapter 1: Basics".to_string(),
    ///     0,
    ///     vec![class],
    /// ).unwrap();
    ///
    /// assert_eq!(chapter.name().as_str(), "Chapter 1: Basics");
    /// ```
    pub fn new(name: String, index: usize, classes: Vec<Class>) -> Result<Self, ChapterError> {
        let name = SimpleName::with_config(name, SimpleNameConfig::new(3, 50))?;
        let index = Index::new(index);

        if classes.is_empty() {
            return Err(ChapterError::ChapterWithEmptyClasses);
        }

        Ok(Self {
            id: Id::new(),
            name,
            index,
            classes,
        })
    }

    /// Returns the chapter name.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Chapter, Class};
    ///
    /// let class = Class::new(
    ///     "Intro".to_string(),
    ///     600,
    ///     "https://example.com/intro.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let chapter = Chapter::new(
    ///     "Fundamentals".to_string(),
    ///     0,
    ///     vec![class],
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
    /// use education_platform_core::{Chapter, Class};
    ///
    /// let class = Class::new(
    ///     "Lesson".to_string(),
    ///     1200,
    ///     "https://example.com/lesson.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let chapter = Chapter::new(
    ///     "Advanced Topics".to_string(),
    ///     5,
    ///     vec![class],
    /// ).unwrap();
    ///
    /// assert_eq!(chapter.index().value(), 5);
    /// ```
    #[inline]
    #[must_use]
    pub const fn index(&self) -> Index {
        self.index
    }

    /// Returns a reference to the classes in this chapter.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Chapter, Class};
    ///
    /// let class1 = Class::new(
    ///     "Part 1".to_string(),
    ///     900,
    ///     "https://example.com/part1.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let class2 = Class::new(
    ///     "Part 2".to_string(),
    ///     1200,
    ///     "https://example.com/part2.mp4".to_string(),
    ///     1,
    /// ).unwrap();
    ///
    /// let chapter = Chapter::new(
    ///     "Introduction".to_string(),
    ///     0,
    ///     vec![class1, class2],
    /// ).unwrap();
    ///
    /// assert_eq!(chapter.classes().len(), 2);
    /// ```
    #[inline]
    #[must_use]
    pub fn classes(&self) -> &[Class] {
        &self.classes
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

    fn create_test_class(name: &str, index: usize) -> Class {
        Class::new(
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
            let class = create_test_class("Test Class", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![class]);

            assert!(chapter.is_ok());
            let chapter = chapter.unwrap();
            assert_eq!(chapter.name().as_str(), "Test Chapter");
            assert_eq!(chapter.index().value(), 0);
            assert_eq!(chapter.classes().len(), 1);
        }

        #[test]
        fn test_new_with_multiple_classes() {
            let classes = vec![
                create_test_class("Class 1", 0),
                create_test_class("Class 2", 1),
                create_test_class("Class 3", 2),
            ];

            let chapter = Chapter::new("Multi-Class Chapter".to_string(), 0, classes).unwrap();

            assert_eq!(chapter.classes().len(), 3);
        }

        #[test]
        fn test_new_with_different_index() {
            let class = create_test_class("Test Class", 0);
            let chapter = Chapter::new("Chapter 5".to_string(), 4, vec![class]).unwrap();

            assert_eq!(chapter.index().value(), 4);
            assert!(!chapter.index().is_first());
        }

        #[test]
        fn test_new_generates_unique_id() {
            let class1 = create_test_class("Class 1", 0);
            let class2 = create_test_class("Class 2", 0);

            let chapter1 = Chapter::new("Chapter 1".to_string(), 0, vec![class1]).unwrap();
            let chapter2 = Chapter::new("Chapter 2".to_string(), 1, vec![class2]).unwrap();

            assert_ne!(chapter1.id(), chapter2.id());
        }

        #[test]
        fn test_new_with_empty_classes_returns_error() {
            let result = Chapter::new("Valid Name".to_string(), 0, vec![]);

            assert!(result.is_err());
            assert!(matches!(result, Err(ChapterError::ChapterWithEmptyClasses)));
        }

        #[test]
        fn test_new_with_empty_name_returns_error() {
            let class = create_test_class("Test Class", 0);
            let result = Chapter::new("".to_string(), 0, vec![class]);

            assert!(result.is_err());
            assert!(matches!(result, Err(ChapterError::NameError(_))));
        }

        #[test]
        fn test_new_with_name_too_short_returns_error() {
            let class = create_test_class("Test Class", 0);
            let result = Chapter::new("AB".to_string(), 0, vec![class]);

            assert!(result.is_err());
            assert!(matches!(result, Err(ChapterError::NameError(_))));
        }

        #[test]
        fn test_new_with_name_at_min_length() {
            let class = create_test_class("Test Class", 0);
            let result = Chapter::new("ABC".to_string(), 0, vec![class]);

            assert!(result.is_ok());
            assert_eq!(result.unwrap().name().as_str(), "ABC");
        }

        #[test]
        fn test_new_with_name_too_long_returns_error() {
            let class = create_test_class("Test Class", 0);
            let long_name = "A".repeat(51);
            let result = Chapter::new(long_name, 0, vec![class]);

            assert!(result.is_err());
            assert!(matches!(result, Err(ChapterError::NameError(_))));
        }

        #[test]
        fn test_new_with_name_at_max_length() {
            let class = create_test_class("Test Class", 0);
            let max_name = "A".repeat(50);
            let result = Chapter::new(max_name.clone(), 0, vec![class]);

            assert!(result.is_ok());
            assert_eq!(result.unwrap().name().as_str(), max_name);
        }
    }

    mod getters {
        use super::*;

        #[test]
        fn test_name_returns_simple_name() {
            let class = create_test_class("Test Class", 0);
            let chapter = Chapter::new("My Chapter".to_string(), 0, vec![class]).unwrap();

            assert_eq!(chapter.name().as_str(), "My Chapter");
        }

        #[test]
        fn test_index_returns_index() {
            let class = create_test_class("Test Class", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 7, vec![class]).unwrap();

            assert_eq!(chapter.index().value(), 7);
        }

        #[test]
        fn test_index_first_chapter() {
            let class = create_test_class("Test Class", 0);
            let chapter = Chapter::new("First Chapter".to_string(), 0, vec![class]).unwrap();

            assert!(chapter.index().is_first());
        }

        #[test]
        fn test_classes_returns_all_classes() {
            let classes = vec![
                create_test_class("Class A", 0),
                create_test_class("Class B", 1),
            ];

            let chapter = Chapter::new("Test Chapter".to_string(), 0, classes).unwrap();

            assert_eq!(chapter.classes().len(), 2);
            assert_eq!(chapter.classes()[0].name().as_str(), "Class A");
            assert_eq!(chapter.classes()[1].name().as_str(), "Class B");
        }
    }

    mod entity_trait {
        use super::*;

        #[test]
        fn test_id_returns_valid_id() {
            let class = create_test_class("Test Class", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![class]).unwrap();

            let id = chapter.id();
            assert_eq!(id.as_bytes().len(), 16);
        }
    }

    mod real_world_examples {
        use super::*;

        #[test]
        fn test_introduction_chapter() {
            let classes = vec![
                create_test_class("Welcome to the Course", 0),
                create_test_class("Course Overview", 1),
                create_test_class("Setting Up Your Environment", 2),
            ];

            let chapter = Chapter::new("Introduction".to_string(), 0, classes).unwrap();

            assert_eq!(chapter.name().as_str(), "Introduction");
            assert!(chapter.index().is_first());
            assert_eq!(chapter.classes().len(), 3);
        }

        #[test]
        fn test_advanced_chapter() {
            let classes = vec![
                create_test_class("Advanced Patterns", 0),
                create_test_class("Performance Optimization", 1),
            ];

            let chapter = Chapter::new("Advanced Topics".to_string(), 5, classes).unwrap();

            assert_eq!(chapter.index().value(), 5);
            assert_eq!(chapter.classes().len(), 2);
        }

        #[test]
        fn test_chapter_with_special_characters_in_name() {
            let class = create_test_class("Test Class", 0);
            let chapter =
                Chapter::new("Chapter 1: Getting Started!".to_string(), 0, vec![class]).unwrap();

            assert_eq!(chapter.name().as_str(), "Chapter 1: Getting Started!");
        }
    }
}
