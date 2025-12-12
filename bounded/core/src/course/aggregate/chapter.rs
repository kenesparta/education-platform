use crate::Class;
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
        let classes = Self::order_classes(classes)?;
        let index = Index::new(index);
        let id = Id::default();

        Ok(Self {
            id,
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

    /// Calculates the total duration of all classes in this chapter.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Chapter, Class};
    ///
    /// let class1 = Class::new(
    ///     "Part 1".to_string(),
    ///     1800,
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
    /// let total = chapter.total_duration();
    /// assert_eq!(total.total_seconds(), 3000);
    /// ```
    #[must_use]
    pub fn total_duration(&self) -> Duration {
        self.classes
            .iter()
            .fold(Duration::default(), |acc, class| acc.add(&class.duration()))
    }

    /// Returns the number of classes in this chapter.
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
    /// assert_eq!(chapter.class_quantity(), 2);
    /// ```
    #[inline]
    #[must_use]
    pub fn class_quantity(&self) -> usize {
        self.classes.len()
    }

    /// Returns a reference to the first class in this chapter.
    ///
    /// # Errors
    ///
    /// Returns `ChapterError::ChapterWithEmptyClasses` if the chapter has no classes.
    /// Note: This error should not occur with a properly constructed `Chapter`,
    /// as the constructor validates that at least one class is provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Chapter, Class};
    ///
    /// let class = Class::new(
    ///     "First Lesson".to_string(),
    ///     1800,
    ///     "https://example.com/first.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let chapter = Chapter::new(
    ///     "Getting Started".to_string(),
    ///     0,
    ///     vec![class],
    /// ).unwrap();
    ///
    /// let first = chapter.first_class().unwrap();
    /// assert_eq!(first.name().as_str(), "First Lesson");
    /// ```
    #[inline]
    pub fn first_class(&self) -> Result<&Class, ChapterError> {
        self.classes
            .first()
            .ok_or(ChapterError::ChapterWithEmptyClasses)
    }

    /// Returns a reference to the last class in this chapter.
    ///
    /// # Errors
    ///
    /// Returns `ChapterError::ChapterWithEmptyClasses` if the chapter has no classes.
    /// Note: This error should not occur with a properly constructed `Chapter`,
    /// as the constructor validates that at least one class is provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Chapter, Class};
    ///
    /// let class1 = Class::new(
    ///     "First Lesson".to_string(),
    ///     1800,
    ///     "https://example.com/first.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let class2 = Class::new(
    ///     "Last Lesson".to_string(),
    ///     1200,
    ///     "https://example.com/last.mp4".to_string(),
    ///     1,
    /// ).unwrap();
    ///
    /// let chapter = Chapter::new(
    ///     "Getting Started".to_string(),
    ///     0,
    ///     vec![class1, class2],
    /// ).unwrap();
    ///
    /// let last = chapter.last_class().unwrap();
    /// assert_eq!(last.name().as_str(), "Last Lesson");
    /// ```
    #[inline]
    pub fn last_class(&self) -> Result<&Class, ChapterError> {
        self.classes
            .last()
            .ok_or(ChapterError::ChapterWithEmptyClasses)
    }

    /// Reassigns indices to classes based on their position in the vector.
    ///
    /// Each class will have its index set to match its position in the vector
    /// (0-based indexing).
    ///
    /// # Errors
    ///
    /// Returns `ChapterError::ChapterWithEmptyClasses` if the classes vector is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Chapter, Class};
    ///
    /// let classes = vec![
    ///     Class::new("First".to_string(), 600, "https://example.com/1.mp4".to_string(), 5).unwrap(),
    ///     Class::new("Second".to_string(), 600, "https://example.com/2.mp4".to_string(), 10).unwrap(),
    ///     Class::new("Third".to_string(), 600, "https://example.com/3.mp4".to_string(), 2).unwrap(),
    /// ];
    ///
    /// let reindexed = Chapter::reassign_index_classes(&classes).unwrap();
    ///
    /// assert_eq!(reindexed[0].index().value(), 0);
    /// assert_eq!(reindexed[1].index().value(), 1);
    /// assert_eq!(reindexed[2].index().value(), 2);
    /// ```
    pub fn reassign_index_classes(classes: &[Class]) -> Result<Vec<Class>, ChapterError> {
        if classes.is_empty() {
            return Err(ChapterError::ChapterWithEmptyClasses);
        }

        Ok(classes
            .iter()
            .enumerate()
            .map(|(index, class)| {
                let mut cloned = class.clone();
                cloned.update_index(index);
                cloned
            })
            .collect())
    }

    /// Orders classes by their index and reassigns sequential indices.
    ///
    /// Takes a collection of classes, sorts them by their current index value,
    /// then reassigns indices sequentially starting from 0. This ensures classes
    /// are both ordered and have contiguous indices.
    ///
    /// # Errors
    ///
    /// Returns `ChapterError::ChapterWithEmptyClasses` if the classes vector is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Chapter, Class};
    ///
    /// let classes = vec![
    ///     Class::new("Third".to_string(), 600, "https://example.com/3.mp4".to_string(), 10).unwrap(),
    ///     Class::new("First".to_string(), 600, "https://example.com/1.mp4".to_string(), 2).unwrap(),
    ///     Class::new("Second".to_string(), 600, "https://example.com/2.mp4".to_string(), 5).unwrap(),
    /// ];
    ///
    /// let ordered = Chapter::order_classes(classes).unwrap();
    ///
    /// assert_eq!(ordered[0].name().as_str(), "First");
    /// assert_eq!(ordered[0].index().value(), 0);
    /// assert_eq!(ordered[1].name().as_str(), "Second");
    /// assert_eq!(ordered[1].index().value(), 1);
    /// assert_eq!(ordered[2].name().as_str(), "Third");
    /// assert_eq!(ordered[2].index().value(), 2);
    /// ```
    pub fn order_classes(mut classes: Vec<Class>) -> Result<Vec<Class>, ChapterError> {
        if classes.is_empty() {
            return Err(ChapterError::ChapterWithEmptyClasses);
        }

        classes.sort_by_key(|class| class.index().value());
        Self::reassign_index_classes(&classes)
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

    mod total_duration {
        use super::*;

        fn create_class_with_duration(name: &str, duration_seconds: u64, index: usize) -> Class {
            Class::new(
                name.to_string(),
                duration_seconds,
                format!("https://example.com/{}.mp4", index),
                index,
            )
            .unwrap()
        }

        #[test]
        fn test_total_duration_single_class() {
            let class = create_class_with_duration("Single Class", 1800, 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![class]).unwrap();

            let total = chapter.total_duration();
            assert_eq!(total.total_seconds(), 1800);
        }

        #[test]
        fn test_total_duration_multiple_classes() {
            let classes = vec![
                create_class_with_duration("Class 1", 1800, 0),
                create_class_with_duration("Class 2", 1200, 1),
                create_class_with_duration("Class 3", 600, 2),
            ];

            let chapter = Chapter::new("Test Chapter".to_string(), 0, classes).unwrap();

            let total = chapter.total_duration();
            assert_eq!(total.total_seconds(), 3600);
        }

        #[test]
        fn test_total_duration_returns_duration_object() {
            let classes = vec![
                create_class_with_duration("Class 1", 3665, 0),
                create_class_with_duration("Class 2", 1800, 1),
            ];

            let chapter = Chapter::new("Test Chapter".to_string(), 0, classes).unwrap();

            let total = chapter.total_duration();
            assert_eq!(total.total_seconds(), 5465);
            assert_eq!(total.hours(), 1);
            assert_eq!(total.minutes(), 31);
            assert_eq!(total.seconds(), 5);
        }

        #[test]
        fn test_total_duration_with_varied_durations() {
            let classes = vec![
                create_class_with_duration("Intro", 300, 0),
                create_class_with_duration("Main Content", 7200, 1),
                create_class_with_duration("Summary", 180, 2),
            ];

            let chapter = Chapter::new("Test Chapter".to_string(), 0, classes).unwrap();

            let total = chapter.total_duration();
            assert_eq!(total.total_seconds(), 7680);
            assert_eq!(total.format_hours(), "02h 08m");
        }

        #[test]
        fn test_total_duration_formatted_output() {
            let classes = vec![
                create_class_with_duration("Part 1", 1800, 0),
                create_class_with_duration("Part 2", 1800, 1),
            ];

            let chapter = Chapter::new("Test Chapter".to_string(), 0, classes).unwrap();

            let total = chapter.total_duration();
            assert_eq!(total.format_hours(), "01h");
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

        #[test]
        fn test_chapter_total_duration_real_world() {
            let classes = vec![
                Class::new(
                    "Welcome".to_string(),
                    301,
                    "https://example.com/welcome.mp4".to_string(),
                    0,
                )
                .unwrap(),
                Class::new(
                    "Course Overview".to_string(),
                    600,
                    "https://example.com/overview.mp4".to_string(),
                    1,
                )
                .unwrap(),
                Class::new(
                    "Getting Started".to_string(),
                    1800,
                    "https://example.com/start.mp4".to_string(),
                    2,
                )
                .unwrap(),
            ];

            let chapter = Chapter::new("Introduction".to_string(), 0, classes).unwrap();

            let total = chapter.total_duration();
            assert_eq!(total.total_seconds(), 2701);
            assert_eq!(total.format_hours(), "45m 01s");
        }
    }

    mod class_quantity {
        use super::*;

        #[test]
        fn test_class_quantity_single_class() {
            let class = create_test_class("Test Class", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![class]).unwrap();

            assert_eq!(chapter.class_quantity(), 1);
        }

        #[test]
        fn test_class_quantity_multiple_classes() {
            let classes = vec![
                create_test_class("Class 1", 0),
                create_test_class("Class 2", 1),
                create_test_class("Class 3", 2),
            ];

            let chapter = Chapter::new("Test Chapter".to_string(), 0, classes).unwrap();

            assert_eq!(chapter.class_quantity(), 3);
        }

        #[test]
        fn test_class_quantity_matches_classes_len() {
            let classes = vec![
                create_test_class("Class A", 0),
                create_test_class("Class B", 1),
            ];

            let chapter = Chapter::new("Test Chapter".to_string(), 0, classes).unwrap();

            assert_eq!(chapter.class_quantity(), chapter.classes().len());
        }
    }

    mod first_class {
        use super::*;

        #[test]
        fn test_first_class_returns_first_class() {
            let classes = vec![
                create_test_class("First Class", 0),
                create_test_class("Second Class", 1),
                create_test_class("Third Class", 2),
            ];

            let chapter = Chapter::new("Test Chapter".to_string(), 0, classes).unwrap();

            let first = chapter.first_class().unwrap();
            assert_eq!(first.name().as_str(), "First Class");
        }

        #[test]
        fn test_first_class_single_class() {
            let class = create_test_class("Only Class", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![class]).unwrap();

            let first = chapter.first_class().unwrap();
            assert_eq!(first.name().as_str(), "Only Class");
        }

        #[test]
        fn test_first_class_returns_reference() {
            let class = create_test_class("Test Class", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![class]).unwrap();

            let first = chapter.first_class().unwrap();

            assert_eq!(first.index().value(), chapter.classes()[0].index().value());
        }
    }

    mod last_class {
        use super::*;

        #[test]
        fn test_last_class_returns_last_class() {
            let classes = vec![
                create_test_class("First Class", 0),
                create_test_class("Second Class", 1),
                create_test_class("Third Class", 2),
            ];

            let chapter = Chapter::new("Test Chapter".to_string(), 0, classes).unwrap();

            let last = chapter.last_class().unwrap();
            assert_eq!(last.name().as_str(), "Third Class");
        }

        #[test]
        fn test_last_class_single_class() {
            let class = create_test_class("Only Class", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![class]).unwrap();

            let last = chapter.last_class().unwrap();
            assert_eq!(last.name().as_str(), "Only Class");
        }

        #[test]
        fn test_last_class_returns_reference() {
            let classes = vec![create_test_class("First", 0), create_test_class("Last", 1)];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, classes).unwrap();

            let last = chapter.last_class().unwrap();

            assert_eq!(last.index().value(), chapter.classes()[1].index().value());
        }

        #[test]
        fn test_first_and_last_same_for_single_class() {
            let class = create_test_class("Single Class", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![class]).unwrap();

            let first = chapter.first_class().unwrap();
            let last = chapter.last_class().unwrap();

            assert_eq!(first.name().as_str(), last.name().as_str());
        }

        #[test]
        fn test_first_and_last_different_for_multiple_classes() {
            let classes = vec![create_test_class("First", 0), create_test_class("Last", 1)];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, classes).unwrap();

            let first = chapter.first_class().unwrap();
            let last = chapter.last_class().unwrap();

            assert_ne!(first.name().as_str(), last.name().as_str());
        }
    }

    mod reassign_index_classes {
        use super::*;

        #[test]
        fn test_reassign_index_classes_with_unordered_indices() {
            let classes = vec![
                create_test_class("First", 5),
                create_test_class("Second", 10),
                create_test_class("Third", 2),
            ];

            let reindexed = Chapter::reassign_index_classes(&classes).unwrap();

            assert_eq!(reindexed[0].index().value(), 0);
            assert_eq!(reindexed[1].index().value(), 1);
            assert_eq!(reindexed[2].index().value(), 2);
        }

        #[test]
        fn test_reassign_index_classes_preserves_class_data() {
            let classes = vec![
                create_test_class("First Class", 99),
                create_test_class("Second Class", 50),
            ];

            let original_names: Vec<_> = classes
                .iter()
                .map(|c| c.name().as_str().to_string())
                .collect();

            let reindexed = Chapter::reassign_index_classes(&classes).unwrap();

            assert_eq!(reindexed[0].name().as_str(), original_names[0]);
            assert_eq!(reindexed[1].name().as_str(), original_names[1]);
        }

        #[test]
        fn test_reassign_index_classes_single_class() {
            let classes = vec![create_test_class("Only Class", 100)];

            let reindexed = Chapter::reassign_index_classes(&classes).unwrap();

            assert_eq!(reindexed.len(), 1);
            assert_eq!(reindexed[0].index().value(), 0);
            assert_eq!(reindexed[0].name().as_str(), "Only Class");
        }

        #[test]
        fn test_reassign_index_classes_empty_returns_error() {
            let classes: Vec<Class> = vec![];

            let result = Chapter::reassign_index_classes(&classes);

            assert!(result.is_err());
            assert!(matches!(result, Err(ChapterError::ChapterWithEmptyClasses)));
        }

        #[test]
        fn test_reassign_index_classes_already_correct_indices() {
            let classes = vec![
                create_test_class("First", 0),
                create_test_class("Second", 1),
                create_test_class("Third", 2),
            ];

            let reindexed = Chapter::reassign_index_classes(&classes).unwrap();

            assert_eq!(reindexed[0].index().value(), 0);
            assert_eq!(reindexed[1].index().value(), 1);
            assert_eq!(reindexed[2].index().value(), 2);
        }

        #[test]
        fn test_reassign_index_classes_with_duplicate_indices() {
            let classes = vec![
                create_test_class("First", 0),
                create_test_class("Second", 0),
                create_test_class("Third", 0),
            ];

            let reindexed = Chapter::reassign_index_classes(&classes).unwrap();

            assert_eq!(reindexed[0].index().value(), 0);
            assert_eq!(reindexed[1].index().value(), 1);
            assert_eq!(reindexed[2].index().value(), 2);
        }

        #[test]
        fn test_reassign_index_classes_preserves_id() {
            let classes = vec![
                create_test_class("First", 5),
                create_test_class("Second", 10),
            ];

            let original_ids: Vec<_> = classes.iter().map(|c| c.id()).collect();

            let reindexed = Chapter::reassign_index_classes(&classes).unwrap();

            assert_eq!(reindexed[0].id(), original_ids[0]);
            assert_eq!(reindexed[1].id(), original_ids[1]);
        }

        #[test]
        fn test_reassign_index_classes_large_collection() {
            let classes: Vec<Class> = (0..100)
                .map(|i| create_test_class(&format!("Class {}", i), 100 - i))
                .collect();

            let reindexed = Chapter::reassign_index_classes(&classes).unwrap();

            assert_eq!(reindexed.len(), 100);
            for (i, class) in reindexed.iter().enumerate() {
                assert_eq!(class.index().value(), i);
            }
        }

        #[test]
        fn test_reassign_index_classes_does_not_modify_original() {
            let classes = vec![
                create_test_class("First", 5),
                create_test_class("Second", 10),
            ];

            let _ = Chapter::reassign_index_classes(&classes).unwrap();

            assert_eq!(classes[0].index().value(), 5);
            assert_eq!(classes[1].index().value(), 10);
        }
    }

    mod order_classes {
        use super::*;

        #[test]
        fn test_order_classes_sorts_by_index() {
            let classes = vec![
                create_test_class("Third", 10),
                create_test_class("First", 2),
                create_test_class("Second", 5),
            ];

            let ordered = Chapter::order_classes(classes).unwrap();

            assert_eq!(ordered[0].name().as_str(), "First");
            assert_eq!(ordered[1].name().as_str(), "Second");
            assert_eq!(ordered[2].name().as_str(), "Third");
        }

        #[test]
        fn test_order_classes_reassigns_indices() {
            let classes = vec![
                create_test_class("Third", 100),
                create_test_class("First", 5),
                create_test_class("Second", 50),
            ];

            let ordered = Chapter::order_classes(classes).unwrap();

            assert_eq!(ordered[0].index().value(), 0);
            assert_eq!(ordered[1].index().value(), 1);
            assert_eq!(ordered[2].index().value(), 2);
        }

        #[test]
        fn test_order_classes_empty_returns_error() {
            let classes: Vec<Class> = vec![];

            let result = Chapter::order_classes(classes);

            assert!(result.is_err());
            assert!(matches!(result, Err(ChapterError::ChapterWithEmptyClasses)));
        }

        #[test]
        fn test_order_classes_single_class() {
            let classes = vec![create_test_class("Only Class", 99)];

            let ordered = Chapter::order_classes(classes).unwrap();

            assert_eq!(ordered.len(), 1);
            assert_eq!(ordered[0].name().as_str(), "Only Class");
            assert_eq!(ordered[0].index().value(), 0);
        }

        #[test]
        fn test_order_classes_already_ordered() {
            let classes = vec![
                create_test_class("First", 0),
                create_test_class("Second", 1),
                create_test_class("Third", 2),
            ];

            let ordered = Chapter::order_classes(classes).unwrap();

            assert_eq!(ordered[0].name().as_str(), "First");
            assert_eq!(ordered[1].name().as_str(), "Second");
            assert_eq!(ordered[2].name().as_str(), "Third");
            assert_eq!(ordered[0].index().value(), 0);
            assert_eq!(ordered[1].index().value(), 1);
            assert_eq!(ordered[2].index().value(), 2);
        }

        #[test]
        fn test_order_classes_with_duplicate_indices() {
            let classes = vec![
                create_test_class("Class A", 5),
                create_test_class("Class B", 5),
                create_test_class("Class C", 5),
            ];

            let ordered = Chapter::order_classes(classes).unwrap();

            assert_eq!(ordered.len(), 3);
            assert_eq!(ordered[0].index().value(), 0);
            assert_eq!(ordered[1].index().value(), 1);
            assert_eq!(ordered[2].index().value(), 2);
        }

        #[test]
        fn test_order_classes_preserves_id() {
            let classes = vec![
                create_test_class("Second", 10),
                create_test_class("First", 5),
            ];

            let original_ids: Vec<_> = classes.iter().map(|c| c.id()).collect();

            let ordered = Chapter::order_classes(classes).unwrap();

            assert_eq!(ordered[0].id(), original_ids[1]);
            assert_eq!(ordered[1].id(), original_ids[0]);
        }

        #[test]
        fn test_order_classes_large_collection() {
            let classes: Vec<Class> = (0..50)
                .map(|i| create_test_class(&format!("Class {}", i), 50 - i))
                .collect();

            let ordered = Chapter::order_classes(classes).unwrap();

            assert_eq!(ordered.len(), 50);
            for (i, class) in ordered.iter().enumerate() {
                assert_eq!(class.index().value(), i);
            }
            assert_eq!(ordered[0].name().as_str(), "Class 49");
            assert_eq!(ordered[49].name().as_str(), "Class 0");
        }

        #[test]
        fn test_order_classes_reverse_order() {
            let classes = vec![
                create_test_class("Last", 3),
                create_test_class("Third", 2),
                create_test_class("Second", 1),
                create_test_class("First", 0),
            ];

            let ordered = Chapter::order_classes(classes).unwrap();

            assert_eq!(ordered[0].name().as_str(), "First");
            assert_eq!(ordered[1].name().as_str(), "Second");
            assert_eq!(ordered[2].name().as_str(), "Third");
            assert_eq!(ordered[3].name().as_str(), "Last");
        }
    }
}
