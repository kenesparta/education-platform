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

    /// Adds a lesson to this chapter and returns a new `Chapter` instance.
    ///
    /// If `index` is `None`, the lesson is appended at the end. If `index` is
    /// `Some`, the lesson is inserted at that position and subsequent lessons
    /// are shifted. After insertion, all lessons are reindexed sequentially.
    ///
    /// # Errors
    ///
    /// Returns `ChapterError::ChapterWithEmptyLessons` if reindexing fails
    /// (should not occur in practice).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Chapter, Lesson};
    /// use education_platform_common::Index;
    ///
    /// let lesson1 = Lesson::new(
    ///     "First".to_string(),
    ///     600,
    ///     "https://example.com/1.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let chapter = Chapter::new("My Chapter".to_string(), 0, vec![lesson1]).unwrap();
    ///
    /// let new_lesson = Lesson::new(
    ///     "Second".to_string(),
    ///     600,
    ///     "https://example.com/2.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let updated = chapter.add_lesson(new_lesson, None).unwrap();
    /// assert_eq!(updated.lessons().len(), 2);
    /// assert_eq!(updated.lessons()[1].name().as_str(), "Second");
    /// ```
    pub fn add_lesson(&self, lesson: Lesson, index: Option<Index>) -> Result<Chapter, ChapterError> {
        let mut lessons = self.lessons.clone();

        match index {
            Some(idx) => {
                let position = idx.value().min(lessons.len());
                lessons.insert(position, lesson);
            }
            None => {
                lessons.push(lesson);
            }
        }

        Ok(Chapter {
            id: self.id,
            name: self.name.clone(),
            index: self.index,
            lessons: Self::reassign_index_lessons(&lessons)?,
        })
    }

    /// Removes a lesson from this chapter and returns a new `Chapter` instance.
    ///
    /// The lesson is identified by its ID. After removal, all remaining lessons
    /// are reindexed sequentially starting from 0.
    ///
    /// # Errors
    ///
    /// Returns `ChapterError::ChapterWithEmptyLessons` if removing the lesson
    /// would result in an empty chapter (a chapter must have at least one lesson).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Chapter, Lesson};
    ///
    /// let lesson1 = Lesson::new(
    ///     "First".to_string(),
    ///     600,
    ///     "https://example.com/1.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let lesson2 = Lesson::new(
    ///     "Second".to_string(),
    ///     600,
    ///     "https://example.com/2.mp4".to_string(),
    ///     1,
    /// ).unwrap();
    ///
    /// let chapter = Chapter::new(
    ///     "My Chapter".to_string(),
    ///     0,
    ///     vec![lesson1.clone(), lesson2],
    /// ).unwrap();
    ///
    /// let updated = chapter.delete_lesson(&lesson1).unwrap();
    /// assert_eq!(updated.lessons().len(), 1);
    /// assert_eq!(updated.lessons()[0].name().as_str(), "Second");
    /// ```
    pub fn delete_lesson(&self, lesson: &Lesson) -> Result<Chapter, ChapterError> {
        let lessons: Vec<Lesson> = self
            .lessons
            .iter()
            .filter(|c| c.id() != lesson.id())
            .cloned()
            .collect();

        Ok(Chapter {
            id: self.id,
            name: self.name.clone(),
            index: self.index,
            lessons: Self::reassign_index_lessons(&lessons)?,
        })
    }

    /// Moves a lesson to a new position within this chapter.
    ///
    /// The lesson is identified by its ID. If found, it is removed from its
    /// current position and inserted at the specified index. After moving,
    /// all lessons are reindexed sequentially starting from 0.
    ///
    /// If the lesson is not found in the chapter, the original chapter is
    /// returned unchanged.
    ///
    /// # Errors
    ///
    /// Returns `ChapterError::ChapterWithEmptyLessons` if the chapter has only
    /// one lesson (cannot temporarily remove it during the move operation).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Chapter, Lesson};
    /// use education_platform_common::Index;
    ///
    /// let lesson1 = Lesson::new(
    ///     "First".to_string(),
    ///     600,
    ///     "https://example.com/1.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let lesson2 = Lesson::new(
    ///     "Second".to_string(),
    ///     600,
    ///     "https://example.com/2.mp4".to_string(),
    ///     1,
    /// ).unwrap();
    ///
    /// let chapter = Chapter::new(
    ///     "My Chapter".to_string(),
    ///     0,
    ///     vec![lesson1.clone(), lesson2],
    /// ).unwrap();
    ///
    /// let updated = chapter.move_lesson(&lesson1, Index::new(1)).unwrap();
    /// assert_eq!(updated.lessons()[0].name().as_str(), "Second");
    /// assert_eq!(updated.lessons()[1].name().as_str(), "First");
    /// ```
    pub fn move_lesson(&self, lesson: &Lesson, to_index: Index) -> Result<Chapter, ChapterError> {
        self.delete_lesson(lesson)?
            .add_lesson(lesson.clone(), Some(to_index))
    }

    /// Moves a lesson one position up (toward index 0) within this chapter.
    ///
    /// If the lesson is already at index 0 (first position), it cannot be moved
    /// up further, so the original chapter is returned unchanged.
    ///
    /// # Errors
    ///
    /// Returns `ChapterError::ChapterWithEmptyLessons` if the chapter has only
    /// one lesson (cannot perform move operation on single-lesson chapters).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Chapter, Lesson};
    ///
    /// let lesson1 = Lesson::new(
    ///     "First".to_string(),
    ///     600,
    ///     "https://example.com/1.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let lesson2 = Lesson::new(
    ///     "Second".to_string(),
    ///     600,
    ///     "https://example.com/2.mp4".to_string(),
    ///     1,
    /// ).unwrap();
    ///
    /// let chapter = Chapter::new(
    ///     "My Chapter".to_string(),
    ///     0,
    ///     vec![lesson1, lesson2.clone()],
    /// ).unwrap();
    ///
    /// let updated = chapter.move_lesson_up(&lesson2).unwrap();
    /// assert_eq!(updated.lessons()[0].name().as_str(), "Second");
    /// assert_eq!(updated.lessons()[1].name().as_str(), "First");
    /// ```
    pub fn move_lesson_up(&self, lesson: &Lesson) -> Result<Chapter, ChapterError> {
        let current_position = self
            .lessons
            .iter()
            .position(|c| c.id() == lesson.id())
            .ok_or(ChapterError::LessonDoesNotExist)?;

        if current_position == 0 {
            return Ok(Chapter {
                id: self.id,
                name: self.name.clone(),
                index: self.index,
                lessons: self.lessons.clone(),
            });
        }

        self.move_lesson(lesson, Index::new(current_position - 1))
    }

    /// Moves a lesson one position down (toward the last index) within this chapter.
    ///
    /// If the lesson is already at the last position, it cannot be moved
    /// down further, so the original chapter is returned unchanged.
    ///
    /// # Errors
    ///
    /// Returns `ChapterError::LessonDoesNotExist` if the lesson is not found in
    /// the chapter.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Chapter, Lesson};
    ///
    /// let lesson1 = Lesson::new(
    ///     "First".to_string(),
    ///     600,
    ///     "https://example.com/1.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let lesson2 = Lesson::new(
    ///     "Second".to_string(),
    ///     600,
    ///     "https://example.com/2.mp4".to_string(),
    ///     1,
    /// ).unwrap();
    ///
    /// let chapter = Chapter::new(
    ///     "My Chapter".to_string(),
    ///     0,
    ///     vec![lesson1.clone(), lesson2],
    /// ).unwrap();
    ///
    /// let updated = chapter.move_lesson_down(&lesson1).unwrap();
    /// assert_eq!(updated.lessons()[0].name().as_str(), "Second");
    /// assert_eq!(updated.lessons()[1].name().as_str(), "First");
    /// ```
    pub fn move_lesson_down(&self, lesson: &Lesson) -> Result<Chapter, ChapterError> {
        let current_position = self
            .lessons
            .iter()
            .position(|c| c.id() == lesson.id())
            .ok_or(ChapterError::LessonDoesNotExist)?;

        if current_position >= self.lessons.len() - 1 {
            return Ok(Chapter {
                id: self.id,
                name: self.name.clone(),
                index: self.index,
                lessons: self.lessons.clone(),
            });
        }

        self.move_lesson(lesson, Index::new(current_position + 1))
    }

    /// Reassigns indices to lessons based on their position in the vector.
    ///
    /// Each lesson will have its index set to match its position in the vector
    /// (0-based indexing).
    ///
    /// # Errors
    ///
    /// Returns `ChapterError::ChapterWithEmptyLessons` if the lessons vector is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Chapter, Lesson};
    ///
    /// let lessons = vec![
    ///     Lesson::new("First".to_string(), 600, "https://example.com/1.mp4".to_string(), 5).unwrap(),
    ///     Lesson::new("Second".to_string(), 600, "https://example.com/2.mp4".to_string(), 10).unwrap(),
    ///     Lesson::new("Third".to_string(), 600, "https://example.com/3.mp4".to_string(), 2).unwrap(),
    /// ];
    ///
    /// let reindexed = Chapter::reassign_index_lessons(&lessons).unwrap();
    ///
    /// assert_eq!(reindexed[0].index().value(), 0);
    /// assert_eq!(reindexed[1].index().value(), 1);
    /// assert_eq!(reindexed[2].index().value(), 2);
    /// ```
    pub fn reassign_index_lessons(lessons: &[Lesson]) -> Result<Vec<Lesson>, ChapterError> {
        if lessons.is_empty() {
            return Err(ChapterError::ChapterWithEmptyLessons);
        }

        Ok(lessons
            .iter()
            .enumerate()
            .map(|(index, lesson)| {
                let mut cloned = lesson.clone();
                cloned.update_index(index);
                cloned
            })
            .collect())
    }

    /// Orders lessons by their index and reassigns sequential indices.
    ///
    /// Takes a collection of lessons, sorts them by their current index value,
    /// then reassigns indices sequentially starting from 0. This ensures lessons
    /// are both ordered and have contiguous indices.
    ///
    /// # Errors
    ///
    /// Returns `ChapterError::ChapterWithEmptyLessons` if the lessons vector is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Chapter, Lesson};
    ///
    /// let lessons = vec![
    ///     Lesson::new("Third".to_string(), 600, "https://example.com/3.mp4".to_string(), 10).unwrap(),
    ///     Lesson::new("First".to_string(), 600, "https://example.com/1.mp4".to_string(), 2).unwrap(),
    ///     Lesson::new("Second".to_string(), 600, "https://example.com/2.mp4".to_string(), 5).unwrap(),
    /// ];
    ///
    /// let ordered = Chapter::order_lessons(lessons).unwrap();
    ///
    /// assert_eq!(ordered[0].name().as_str(), "First");
    /// assert_eq!(ordered[0].index().value(), 0);
    /// assert_eq!(ordered[1].name().as_str(), "Second");
    /// assert_eq!(ordered[1].index().value(), 1);
    /// assert_eq!(ordered[2].name().as_str(), "Third");
    /// assert_eq!(ordered[2].index().value(), 2);
    /// ```
    pub fn order_lessons(mut lessons: Vec<Lesson>) -> Result<Vec<Lesson>, ChapterError> {
        if lessons.is_empty() {
            return Err(ChapterError::ChapterWithEmptyLessons);
        }

        lessons.sort_by_key(|lesson| lesson.index().value());
        Self::reassign_index_lessons(&lessons)
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

    fn create_test_lesson(name: &str, index: usize) -> Lesson {
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
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]);

            assert!(chapter.is_ok());
            let chapter = chapter.unwrap();
            assert_eq!(chapter.name().as_str(), "Test Chapter");
            assert_eq!(chapter.index().value(), 0);
            assert_eq!(chapter.lessons().len(), 1);
        }

        #[test]
        fn test_new_with_multiple_lessons() {
            let lessons = vec![
                create_test_lesson("Lesson 1", 0),
                create_test_lesson("Lesson 2", 1),
                create_test_lesson("Lesson 3", 2),
            ];

            let chapter = Chapter::new("Multi-Lesson Chapter".to_string(), 0, lessons).unwrap();

            assert_eq!(chapter.lessons().len(), 3);
        }

        #[test]
        fn test_new_with_different_index() {
            let lesson = create_test_lesson("Test Lesson", 0);
            let chapter = Chapter::new("Chapter 5".to_string(), 4, vec![lesson]).unwrap();

            assert_eq!(chapter.index().value(), 4);
            assert!(!chapter.index().is_first());
        }

        #[test]
        fn test_new_generates_unique_id() {
            let lesson1 = create_test_lesson("Lesson 1", 0);
            let lesson2 = create_test_lesson("Lesson 2", 0);

            let chapter1 = Chapter::new("Chapter 1".to_string(), 0, vec![lesson1]).unwrap();
            let chapter2 = Chapter::new("Chapter 2".to_string(), 1, vec![lesson2]).unwrap();

            assert_ne!(chapter1.id(), chapter2.id());
        }

        #[test]
        fn test_new_with_empty_lessons_returns_error() {
            let result = Chapter::new("Valid Name".to_string(), 0, vec![]);

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
        fn test_new_with_name_too_short_returns_error() {
            let lesson = create_test_lesson("Test Lesson", 0);
            let result = Chapter::new("AB".to_string(), 0, vec![lesson]);

            assert!(result.is_err());
            assert!(matches!(result, Err(ChapterError::NameError(_))));
        }

        #[test]
        fn test_new_with_name_at_min_length() {
            let lesson = create_test_lesson("Test Lesson", 0);
            let result = Chapter::new("ABC".to_string(), 0, vec![lesson]);

            assert!(result.is_ok());
            assert_eq!(result.unwrap().name().as_str(), "ABC");
        }

        #[test]
        fn test_new_with_name_too_long_returns_error() {
            let lesson = create_test_lesson("Test Lesson", 0);
            let long_name = "A".repeat(51);
            let result = Chapter::new(long_name, 0, vec![lesson]);

            assert!(result.is_err());
            assert!(matches!(result, Err(ChapterError::NameError(_))));
        }

        #[test]
        fn test_new_with_name_at_max_length() {
            let lesson = create_test_lesson("Test Lesson", 0);
            let max_name = "A".repeat(50);
            let result = Chapter::new(max_name.clone(), 0, vec![lesson]);

            assert!(result.is_ok());
            assert_eq!(result.unwrap().name().as_str(), max_name);
        }
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
            let chapter = Chapter::new("Test Chapter".to_string(), 7, vec![lesson]).unwrap();

            assert_eq!(chapter.index().value(), 7);
        }

        #[test]
        fn test_index_first_chapter() {
            let lesson = create_test_lesson("Test Lesson", 0);
            let chapter = Chapter::new("First Chapter".to_string(), 0, vec![lesson]).unwrap();

            assert!(chapter.index().is_first());
        }

        #[test]
        fn test_lessons_returns_all_lessons() {
            let lessons = vec![
                create_test_lesson("Lesson A", 0),
                create_test_lesson("Lesson B", 1),
            ];

            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

            assert_eq!(chapter.lessons().len(), 2);
            assert_eq!(chapter.lessons()[0].name().as_str(), "Lesson A");
            assert_eq!(chapter.lessons()[1].name().as_str(), "Lesson B");
        }
    }

    mod entity_trait {
        use super::*;

        #[test]
        fn test_id_returns_valid_id() {
            let lesson = create_test_lesson("Test Lesson", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

            let id = chapter.id();
            assert_eq!(id.as_bytes().len(), 16);
        }
    }

    mod total_duration {
        use super::*;

        fn create_lesson_with_duration(name: &str, duration_seconds: u64, index: usize) -> Lesson {
            Lesson::new(
                name.to_string(),
                duration_seconds,
                format!("https://example.com/{}.mp4", index),
                index,
            )
            .unwrap()
        }

        #[test]
        fn test_total_duration_single_lesson() {
            let lesson = create_lesson_with_duration("Single Lesson", 1800, 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

            let total = chapter.total_duration();
            assert_eq!(total.total_seconds(), 1800);
        }

        #[test]
        fn test_total_duration_multiple_lessons() {
            let lessons = vec![
                create_lesson_with_duration("Lesson 1", 1800, 0),
                create_lesson_with_duration("Lesson 2", 1200, 1),
                create_lesson_with_duration("Lesson 3", 600, 2),
            ];

            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

            let total = chapter.total_duration();
            assert_eq!(total.total_seconds(), 3600);
        }

        #[test]
        fn test_total_duration_returns_duration_object() {
            let lessons = vec![
                create_lesson_with_duration("Lesson 1", 3665, 0),
                create_lesson_with_duration("Lesson 2", 1800, 1),
            ];

            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

            let total = chapter.total_duration();
            assert_eq!(total.total_seconds(), 5465);
            assert_eq!(total.hours(), 1);
            assert_eq!(total.minutes(), 31);
            assert_eq!(total.seconds(), 5);
        }

        #[test]
        fn test_total_duration_with_varied_durations() {
            let lessons = vec![
                create_lesson_with_duration("Intro", 300, 0),
                create_lesson_with_duration("Main Content", 7200, 1),
                create_lesson_with_duration("Summary", 180, 2),
            ];

            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

            let total = chapter.total_duration();
            assert_eq!(total.total_seconds(), 7680);
            assert_eq!(total.format_hours(), "02h 08m");
        }

        #[test]
        fn test_total_duration_formatted_output() {
            let lessons = vec![
                create_lesson_with_duration("Part 1", 1800, 0),
                create_lesson_with_duration("Part 2", 1800, 1),
            ];

            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

            let total = chapter.total_duration();
            assert_eq!(total.format_hours(), "01h");
        }
    }

    mod real_world_examples {
        use super::*;

        #[test]
        fn test_introduction_chapter() {
            let lessons = vec![
                create_test_lesson("Welcome to the Course", 0),
                create_test_lesson("Course Overview", 1),
                create_test_lesson("Setting Up Your Environment", 2),
            ];

            let chapter = Chapter::new("Introduction".to_string(), 0, lessons).unwrap();

            assert_eq!(chapter.name().as_str(), "Introduction");
            assert!(chapter.index().is_first());
            assert_eq!(chapter.lessons().len(), 3);
        }

        #[test]
        fn test_advanced_chapter() {
            let lessons = vec![
                create_test_lesson("Advanced Patterns", 0),
                create_test_lesson("Performance Optimization", 1),
            ];

            let chapter = Chapter::new("Advanced Topics".to_string(), 5, lessons).unwrap();

            assert_eq!(chapter.index().value(), 5);
            assert_eq!(chapter.lessons().len(), 2);
        }

        #[test]
        fn test_chapter_with_special_characters_in_name() {
            let lesson = create_test_lesson("Test Lesson", 0);
            let chapter =
                Chapter::new("Chapter 1: Getting Started!".to_string(), 0, vec![lesson]).unwrap();

            assert_eq!(chapter.name().as_str(), "Chapter 1: Getting Started!");
        }

        #[test]
        fn test_chapter_total_duration_real_world() {
            let lessons = vec![
                Lesson::new(
                    "Welcome".to_string(),
                    301,
                    "https://example.com/welcome.mp4".to_string(),
                    0,
                )
                .unwrap(),
                Lesson::new(
                    "Course Overview".to_string(),
                    600,
                    "https://example.com/overview.mp4".to_string(),
                    1,
                )
                .unwrap(),
                Lesson::new(
                    "Getting Started".to_string(),
                    1800,
                    "https://example.com/start.mp4".to_string(),
                    2,
                )
                .unwrap(),
            ];

            let chapter = Chapter::new("Introduction".to_string(), 0, lessons).unwrap();

            let total = chapter.total_duration();
            assert_eq!(total.total_seconds(), 2701);
            assert_eq!(total.format_hours(), "45m 01s");
        }
    }

    mod lesson_quantity {
        use super::*;

        #[test]
        fn test_lesson_quantity_single_lesson() {
            let lesson = create_test_lesson("Test Lesson", 0);
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
                create_test_lesson("Lesson A", 0),
                create_test_lesson("Lesson B", 1),
            ];

            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

            assert_eq!(chapter.lesson_quantity(), chapter.lessons().len());
        }
    }

    mod first_lesson {
        use super::*;

        #[test]
        fn test_first_lesson_returns_first_lesson() {
            let lessons = vec![
                create_test_lesson("First Lesson", 0),
                create_test_lesson("Second Lesson", 1),
                create_test_lesson("Third Lesson", 2),
            ];

            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

            let first = chapter.first_lesson().unwrap();
            assert_eq!(first.name().as_str(), "First Lesson");
        }

        #[test]
        fn test_first_lesson_single_lesson() {
            let lesson = create_test_lesson("Only Lesson", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

            let first = chapter.first_lesson().unwrap();
            assert_eq!(first.name().as_str(), "Only Lesson");
        }

        #[test]
        fn test_first_lesson_returns_reference() {
            let lesson = create_test_lesson("Test Lesson", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

            let first = chapter.first_lesson().unwrap();

            assert_eq!(first.index().value(), chapter.lessons()[0].index().value());
        }
    }

    mod last_lesson {
        use super::*;

        #[test]
        fn test_last_lesson_returns_last_lesson() {
            let lessons = vec![
                create_test_lesson("First Lesson", 0),
                create_test_lesson("Second Lesson", 1),
                create_test_lesson("Third Lesson", 2),
            ];

            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

            let last = chapter.last_lesson().unwrap();
            assert_eq!(last.name().as_str(), "Third Lesson");
        }

        #[test]
        fn test_last_lesson_single_lesson() {
            let lesson = create_test_lesson("Only Lesson", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

            let last = chapter.last_lesson().unwrap();
            assert_eq!(last.name().as_str(), "Only Lesson");
        }

        #[test]
        fn test_last_lesson_returns_reference() {
            let lessons = vec![create_test_lesson("First", 0), create_test_lesson("Last", 1)];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

            let last = chapter.last_lesson().unwrap();

            assert_eq!(last.index().value(), chapter.lessons()[1].index().value());
        }

        #[test]
        fn test_first_and_last_same_for_single_lesson() {
            let lesson = create_test_lesson("Single Lesson", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

            let first = chapter.first_lesson().unwrap();
            let last = chapter.last_lesson().unwrap();

            assert_eq!(first.name().as_str(), last.name().as_str());
        }

        #[test]
        fn test_first_and_last_different_for_multiple_lessons() {
            let lessons = vec![create_test_lesson("First", 0), create_test_lesson("Last", 1)];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

            let first = chapter.first_lesson().unwrap();
            let last = chapter.last_lesson().unwrap();

            assert_ne!(first.name().as_str(), last.name().as_str());
        }
    }

    mod reassign_index_lessons {
        use super::*;

        #[test]
        fn test_reassign_index_lessons_with_unordered_indices() {
            let lessons = vec![
                create_test_lesson("First", 5),
                create_test_lesson("Second", 10),
                create_test_lesson("Third", 2),
            ];

            let reindexed = Chapter::reassign_index_lessons(&lessons).unwrap();

            assert_eq!(reindexed[0].index().value(), 0);
            assert_eq!(reindexed[1].index().value(), 1);
            assert_eq!(reindexed[2].index().value(), 2);
        }

        #[test]
        fn test_reassign_index_lessons_preserves_lesson_data() {
            let lessons = vec![
                create_test_lesson("First Lesson", 99),
                create_test_lesson("Second Lesson", 50),
            ];

            let original_names: Vec<_> = lessons
                .iter()
                .map(|c| c.name().as_str().to_string())
                .collect();

            let reindexed = Chapter::reassign_index_lessons(&lessons).unwrap();

            assert_eq!(reindexed[0].name().as_str(), original_names[0]);
            assert_eq!(reindexed[1].name().as_str(), original_names[1]);
        }

        #[test]
        fn test_reassign_index_lessons_single_lesson() {
            let lessons = vec![create_test_lesson("Only Lesson", 100)];

            let reindexed = Chapter::reassign_index_lessons(&lessons).unwrap();

            assert_eq!(reindexed.len(), 1);
            assert_eq!(reindexed[0].index().value(), 0);
            assert_eq!(reindexed[0].name().as_str(), "Only Lesson");
        }

        #[test]
        fn test_reassign_index_lessons_empty_returns_error() {
            let lessons: Vec<Lesson> = vec![];

            let result = Chapter::reassign_index_lessons(&lessons);

            assert!(result.is_err());
            assert!(matches!(result, Err(ChapterError::ChapterWithEmptyLessons)));
        }

        #[test]
        fn test_reassign_index_lessons_already_correct_indices() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
                create_test_lesson("Third", 2),
            ];

            let reindexed = Chapter::reassign_index_lessons(&lessons).unwrap();

            assert_eq!(reindexed[0].index().value(), 0);
            assert_eq!(reindexed[1].index().value(), 1);
            assert_eq!(reindexed[2].index().value(), 2);
        }

        #[test]
        fn test_reassign_index_lessons_with_duplicate_indices() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 0),
                create_test_lesson("Third", 0),
            ];

            let reindexed = Chapter::reassign_index_lessons(&lessons).unwrap();

            assert_eq!(reindexed[0].index().value(), 0);
            assert_eq!(reindexed[1].index().value(), 1);
            assert_eq!(reindexed[2].index().value(), 2);
        }

        #[test]
        fn test_reassign_index_lessons_preserves_id() {
            let lessons = vec![
                create_test_lesson("First", 5),
                create_test_lesson("Second", 10),
            ];

            let original_ids: Vec<_> = lessons.iter().map(|c| c.id()).collect();

            let reindexed = Chapter::reassign_index_lessons(&lessons).unwrap();

            assert_eq!(reindexed[0].id(), original_ids[0]);
            assert_eq!(reindexed[1].id(), original_ids[1]);
        }

        #[test]
        fn test_reassign_index_lessons_large_collection() {
            let lessons: Vec<Lesson> = (0..100)
                .map(|i| create_test_lesson(&format!("Lesson {}", i), 100 - i))
                .collect();

            let reindexed = Chapter::reassign_index_lessons(&lessons).unwrap();

            assert_eq!(reindexed.len(), 100);
            for (i, lesson) in reindexed.iter().enumerate() {
                assert_eq!(lesson.index().value(), i);
            }
        }

        #[test]
        fn test_reassign_index_lessons_does_not_modify_original() {
            let lessons = vec![
                create_test_lesson("First", 5),
                create_test_lesson("Second", 10),
            ];

            let _ = Chapter::reassign_index_lessons(&lessons).unwrap();

            assert_eq!(lessons[0].index().value(), 5);
            assert_eq!(lessons[1].index().value(), 10);
        }
    }

    mod order_lessons {
        use super::*;

        #[test]
        fn test_order_lessons_sorts_by_index() {
            let lessons = vec![
                create_test_lesson("Third", 10),
                create_test_lesson("First", 2),
                create_test_lesson("Second", 5),
            ];

            let ordered = Chapter::order_lessons(lessons).unwrap();

            assert_eq!(ordered[0].name().as_str(), "First");
            assert_eq!(ordered[1].name().as_str(), "Second");
            assert_eq!(ordered[2].name().as_str(), "Third");
        }

        #[test]
        fn test_order_lessons_reassigns_indices() {
            let lessons = vec![
                create_test_lesson("Third", 100),
                create_test_lesson("First", 5),
                create_test_lesson("Second", 50),
            ];

            let ordered = Chapter::order_lessons(lessons).unwrap();

            assert_eq!(ordered[0].index().value(), 0);
            assert_eq!(ordered[1].index().value(), 1);
            assert_eq!(ordered[2].index().value(), 2);
        }

        #[test]
        fn test_order_lessons_empty_returns_error() {
            let lessons: Vec<Lesson> = vec![];

            let result = Chapter::order_lessons(lessons);

            assert!(result.is_err());
            assert!(matches!(result, Err(ChapterError::ChapterWithEmptyLessons)));
        }

        #[test]
        fn test_order_lessons_single_lesson() {
            let lessons = vec![create_test_lesson("Only Lesson", 99)];

            let ordered = Chapter::order_lessons(lessons).unwrap();

            assert_eq!(ordered.len(), 1);
            assert_eq!(ordered[0].name().as_str(), "Only Lesson");
            assert_eq!(ordered[0].index().value(), 0);
        }

        #[test]
        fn test_order_lessons_already_ordered() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
                create_test_lesson("Third", 2),
            ];

            let ordered = Chapter::order_lessons(lessons).unwrap();

            assert_eq!(ordered[0].name().as_str(), "First");
            assert_eq!(ordered[1].name().as_str(), "Second");
            assert_eq!(ordered[2].name().as_str(), "Third");
            assert_eq!(ordered[0].index().value(), 0);
            assert_eq!(ordered[1].index().value(), 1);
            assert_eq!(ordered[2].index().value(), 2);
        }

        #[test]
        fn test_order_lessons_with_duplicate_indices() {
            let lessons = vec![
                create_test_lesson("Lesson A", 5),
                create_test_lesson("Lesson B", 5),
                create_test_lesson("Lesson C", 5),
            ];

            let ordered = Chapter::order_lessons(lessons).unwrap();

            assert_eq!(ordered.len(), 3);
            assert_eq!(ordered[0].index().value(), 0);
            assert_eq!(ordered[1].index().value(), 1);
            assert_eq!(ordered[2].index().value(), 2);
        }

        #[test]
        fn test_order_lessons_preserves_id() {
            let lessons = vec![
                create_test_lesson("Second", 10),
                create_test_lesson("First", 5),
            ];

            let original_ids: Vec<_> = lessons.iter().map(|c| c.id()).collect();

            let ordered = Chapter::order_lessons(lessons).unwrap();

            assert_eq!(ordered[0].id(), original_ids[1]);
            assert_eq!(ordered[1].id(), original_ids[0]);
        }

        #[test]
        fn test_order_lessons_large_collection() {
            let lessons: Vec<Lesson> = (0..50)
                .map(|i| create_test_lesson(&format!("Lesson {}", i), 50 - i))
                .collect();

            let ordered = Chapter::order_lessons(lessons).unwrap();

            assert_eq!(ordered.len(), 50);
            for (i, lesson) in ordered.iter().enumerate() {
                assert_eq!(lesson.index().value(), i);
            }
            assert_eq!(ordered[0].name().as_str(), "Lesson 49");
            assert_eq!(ordered[49].name().as_str(), "Lesson 0");
        }

        #[test]
        fn test_order_lessons_reverse_order() {
            let lessons = vec![
                create_test_lesson("Last", 3),
                create_test_lesson("Third", 2),
                create_test_lesson("Second", 1),
                create_test_lesson("First", 0),
            ];

            let ordered = Chapter::order_lessons(lessons).unwrap();

            assert_eq!(ordered[0].name().as_str(), "First");
            assert_eq!(ordered[1].name().as_str(), "Second");
            assert_eq!(ordered[2].name().as_str(), "Third");
            assert_eq!(ordered[3].name().as_str(), "Last");
        }
    }

    mod add_lesson {
        use super::*;

        #[test]
        fn test_add_lesson_appends_at_end_when_index_is_none() {
            let lesson = create_test_lesson("First", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

            let new_lesson = create_test_lesson("Second", 0);
            let updated = chapter.add_lesson(new_lesson, None).unwrap();

            assert_eq!(updated.lessons().len(), 2);
            assert_eq!(updated.lessons()[0].name().as_str(), "First");
            assert_eq!(updated.lessons()[1].name().as_str(), "Second");
        }

        #[test]
        fn test_add_lesson_inserts_at_beginning_with_index_zero() {
            let lesson = create_test_lesson("Second", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

            let new_lesson = create_test_lesson("First", 0);
            let updated = chapter.add_lesson(new_lesson, Some(Index::new(0))).unwrap();

            assert_eq!(updated.lessons().len(), 2);
            assert_eq!(updated.lessons()[0].name().as_str(), "First");
            assert_eq!(updated.lessons()[1].name().as_str(), "Second");
        }

        #[test]
        fn test_add_lesson_inserts_at_middle() {
            let lessons = vec![create_test_lesson("First", 0), create_test_lesson("Third", 1)];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

            let new_lesson = create_test_lesson("Second", 0);
            let updated = chapter.add_lesson(new_lesson, Some(Index::new(1))).unwrap();

            assert_eq!(updated.lessons().len(), 3);
            assert_eq!(updated.lessons()[0].name().as_str(), "First");
            assert_eq!(updated.lessons()[1].name().as_str(), "Second");
            assert_eq!(updated.lessons()[2].name().as_str(), "Third");
        }

        #[test]
        fn test_add_lesson_reassigns_indices_sequentially() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

            let new_lesson = create_test_lesson("New", 99);
            let updated = chapter.add_lesson(new_lesson, Some(Index::new(1))).unwrap();

            assert_eq!(updated.lessons()[0].index().value(), 0);
            assert_eq!(updated.lessons()[1].index().value(), 1);
            assert_eq!(updated.lessons()[2].index().value(), 2);
        }

        #[test]
        fn test_add_lesson_preserves_chapter_id() {
            let lesson = create_test_lesson("First", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();
            let original_id = chapter.id();

            let new_lesson = create_test_lesson("Second", 0);
            let updated = chapter.add_lesson(new_lesson, None).unwrap();

            assert_eq!(updated.id(), original_id);
        }

        #[test]
        fn test_add_lesson_preserves_chapter_name() {
            let lesson = create_test_lesson("First", 0);
            let chapter = Chapter::new("My Chapter".to_string(), 0, vec![lesson]).unwrap();

            let new_lesson = create_test_lesson("Second", 0);
            let updated = chapter.add_lesson(new_lesson, None).unwrap();

            assert_eq!(updated.name().as_str(), "My Chapter");
        }

        #[test]
        fn test_add_lesson_preserves_chapter_index() {
            let lesson = create_test_lesson("First", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 5, vec![lesson]).unwrap();

            let new_lesson = create_test_lesson("Second", 0);
            let updated = chapter.add_lesson(new_lesson, None).unwrap();

            assert_eq!(updated.index().value(), 5);
        }

        #[test]
        fn test_add_lesson_does_not_modify_original() {
            let lesson = create_test_lesson("First", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

            let new_lesson = create_test_lesson("Second", 0);
            let _ = chapter.add_lesson(new_lesson, None).unwrap();

            assert_eq!(chapter.lessons().len(), 1);
            assert_eq!(chapter.lessons()[0].name().as_str(), "First");
        }

        #[test]
        fn test_add_lesson_with_index_beyond_length_appends_at_end() {
            let lesson = create_test_lesson("First", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

            let new_lesson = create_test_lesson("Second", 0);
            let updated = chapter.add_lesson(new_lesson, Some(Index::new(100))).unwrap();

            assert_eq!(updated.lessons().len(), 2);
            assert_eq!(updated.lessons()[1].name().as_str(), "Second");
            assert_eq!(updated.lessons()[1].index().value(), 1);
        }

        #[test]
        fn test_add_lesson_preserves_existing_lesson_ids() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
            let original_ids: Vec<_> = chapter.lessons().iter().map(|c| c.id()).collect();

            let new_lesson = create_test_lesson("New", 0);
            let updated = chapter.add_lesson(new_lesson, Some(Index::new(1))).unwrap();

            assert_eq!(updated.lessons()[0].id(), original_ids[0]);
            assert_eq!(updated.lessons()[2].id(), original_ids[1]);
        }

        #[test]
        fn test_add_lesson_multiple_times() {
            let lesson = create_test_lesson("First", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

            let chapter = chapter
                .add_lesson(create_test_lesson("Second", 0), None)
                .unwrap();
            let chapter = chapter
                .add_lesson(create_test_lesson("Third", 0), None)
                .unwrap();
            let chapter = chapter
                .add_lesson(create_test_lesson("Fourth", 0), None)
                .unwrap();

            assert_eq!(chapter.lessons().len(), 4);
            assert_eq!(chapter.lessons()[0].name().as_str(), "First");
            assert_eq!(chapter.lessons()[1].name().as_str(), "Second");
            assert_eq!(chapter.lessons()[2].name().as_str(), "Third");
            assert_eq!(chapter.lessons()[3].name().as_str(), "Fourth");
        }

        #[test]
        fn test_add_lesson_at_last_position() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

            let new_lesson = create_test_lesson("Third", 0);
            let updated = chapter.add_lesson(new_lesson, Some(Index::new(2))).unwrap();

            assert_eq!(updated.lessons().len(), 3);
            assert_eq!(updated.lessons()[2].name().as_str(), "Third");
            assert_eq!(updated.lessons()[2].index().value(), 2);
        }

        #[test]
        fn test_add_lesson_updates_total_duration() {
            let lesson = Lesson::new(
                "First".to_string(),
                1800,
                "https://example.com/1.mp4".to_string(),
                0,
            )
            .unwrap();
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

            let new_lesson = Lesson::new(
                "Second".to_string(),
                1200,
                "https://example.com/2.mp4".to_string(),
                0,
            )
            .unwrap();
            let updated = chapter.add_lesson(new_lesson, None).unwrap();

            assert_eq!(updated.total_duration().total_seconds(), 3000);
        }

        #[test]
        fn test_add_lesson_updates_lesson_quantity() {
            let lesson = create_test_lesson("First", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

            assert_eq!(chapter.lesson_quantity(), 1);

            let new_lesson = create_test_lesson("Second", 0);
            let updated = chapter.add_lesson(new_lesson, None).unwrap();

            assert_eq!(updated.lesson_quantity(), 2);
        }
    }

    mod remove_lesson {
        use super::*;

        #[test]
        fn test_remove_lesson_removes_by_id() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);
            let lesson1_id = lesson1.id();

            let chapter =
                Chapter::new("Test Chapter".to_string(), 0, vec![lesson1.clone(), lesson2]).unwrap();

            let updated = chapter.delete_lesson(&lesson1).unwrap();

            assert_eq!(updated.lessons().len(), 1);
            assert_ne!(updated.lessons()[0].id(), lesson1_id);
            assert_eq!(updated.lessons()[0].name().as_str(), "Second");
        }

        #[test]
        fn test_remove_lesson_reassigns_indices() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);
            let lesson3 = create_test_lesson("Third", 2);

            let chapter = Chapter::new(
                "Test Chapter".to_string(),
                0,
                vec![lesson1, lesson2.clone(), lesson3],
            )
            .unwrap();

            let updated = chapter.delete_lesson(&lesson2).unwrap();

            assert_eq!(updated.lessons().len(), 2);
            assert_eq!(updated.lessons()[0].index().value(), 0);
            assert_eq!(updated.lessons()[1].index().value(), 1);
        }

        #[test]
        fn test_remove_lesson_from_beginning() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);
            let lesson3 = create_test_lesson("Third", 2);

            let chapter = Chapter::new(
                "Test Chapter".to_string(),
                0,
                vec![lesson1.clone(), lesson2, lesson3],
            )
            .unwrap();

            let updated = chapter.delete_lesson(&lesson1).unwrap();

            assert_eq!(updated.lessons().len(), 2);
            assert_eq!(updated.lessons()[0].name().as_str(), "Second");
            assert_eq!(updated.lessons()[0].index().value(), 0);
            assert_eq!(updated.lessons()[1].name().as_str(), "Third");
            assert_eq!(updated.lessons()[1].index().value(), 1);
        }

        #[test]
        fn test_remove_lesson_from_end() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);
            let lesson3 = create_test_lesson("Third", 2);

            let chapter = Chapter::new(
                "Test Chapter".to_string(),
                0,
                vec![lesson1, lesson2, lesson3.clone()],
            )
            .unwrap();

            let updated = chapter.delete_lesson(&lesson3).unwrap();

            assert_eq!(updated.lessons().len(), 2);
            assert_eq!(updated.lessons()[0].name().as_str(), "First");
            assert_eq!(updated.lessons()[1].name().as_str(), "Second");
        }

        #[test]
        fn test_remove_lesson_returns_error_when_last_lesson() {
            let lesson = create_test_lesson("Only Lesson", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson.clone()]).unwrap();

            let result = chapter.delete_lesson(&lesson);

            assert!(result.is_err());
            assert!(matches!(result, Err(ChapterError::ChapterWithEmptyLessons)));
        }

        #[test]
        fn test_remove_lesson_preserves_chapter_id() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);

            let chapter =
                Chapter::new("Test Chapter".to_string(), 0, vec![lesson1.clone(), lesson2]).unwrap();
            let original_id = chapter.id();

            let updated = chapter.delete_lesson(&lesson1).unwrap();

            assert_eq!(updated.id(), original_id);
        }

        #[test]
        fn test_remove_lesson_preserves_chapter_name() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);

            let chapter =
                Chapter::new("My Chapter".to_string(), 0, vec![lesson1.clone(), lesson2]).unwrap();

            let updated = chapter.delete_lesson(&lesson1).unwrap();

            assert_eq!(updated.name().as_str(), "My Chapter");
        }

        #[test]
        fn test_remove_lesson_preserves_chapter_index() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);

            let chapter =
                Chapter::new("Test Chapter".to_string(), 5, vec![lesson1.clone(), lesson2]).unwrap();

            let updated = chapter.delete_lesson(&lesson1).unwrap();

            assert_eq!(updated.index().value(), 5);
        }

        #[test]
        fn test_remove_lesson_does_not_modify_original() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);

            let chapter =
                Chapter::new("Test Chapter".to_string(), 0, vec![lesson1.clone(), lesson2]).unwrap();

            let _ = chapter.delete_lesson(&lesson1).unwrap();

            assert_eq!(chapter.lessons().len(), 2);
        }

        #[test]
        fn test_remove_lesson_nonexistent_lesson_returns_same_lessons() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);
            let nonexistent = create_test_lesson("Nonexistent", 99);

            let chapter =
                Chapter::new("Test Chapter".to_string(), 0, vec![lesson1, lesson2]).unwrap();

            let updated = chapter.delete_lesson(&nonexistent).unwrap();

            assert_eq!(updated.lessons().len(), 2);
        }

        #[test]
        fn test_remove_lesson_preserves_remaining_lesson_ids() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);
            let lesson3 = create_test_lesson("Third", 2);

            let chapter = Chapter::new(
                "Test Chapter".to_string(),
                0,
                vec![lesson1, lesson2.clone(), lesson3],
            )
            .unwrap();

            let original_ids: Vec<_> = chapter
                .lessons()
                .iter()
                .filter(|c| c.id() != lesson2.id())
                .map(|c| c.id())
                .collect();

            let updated = chapter.delete_lesson(&lesson2).unwrap();

            assert_eq!(updated.lessons()[0].id(), original_ids[0]);
            assert_eq!(updated.lessons()[1].id(), original_ids[1]);
        }

        #[test]
        fn test_remove_lesson_updates_total_duration() {
            let lesson1 = Lesson::new(
                "First".to_string(),
                1800,
                "https://example.com/1.mp4".to_string(),
                0,
            )
            .unwrap();
            let lesson2 = Lesson::new(
                "Second".to_string(),
                1200,
                "https://example.com/2.mp4".to_string(),
                1,
            )
            .unwrap();

            let chapter =
                Chapter::new("Test Chapter".to_string(), 0, vec![lesson1.clone(), lesson2]).unwrap();

            assert_eq!(chapter.total_duration().total_seconds(), 3000);

            let updated = chapter.delete_lesson(&lesson1).unwrap();

            assert_eq!(updated.total_duration().total_seconds(), 1200);
        }

        #[test]
        fn test_remove_lesson_updates_lesson_quantity() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);
            let lesson3 = create_test_lesson("Third", 2);

            let chapter = Chapter::new(
                "Test Chapter".to_string(),
                0,
                vec![lesson1.clone(), lesson2, lesson3],
            )
            .unwrap();

            assert_eq!(chapter.lesson_quantity(), 3);

            let updated = chapter.delete_lesson(&lesson1).unwrap();

            assert_eq!(updated.lesson_quantity(), 2);
        }

        #[test]
        fn test_remove_lesson_multiple_times() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);
            let lesson3 = create_test_lesson("Third", 2);

            let chapter = Chapter::new(
                "Test Chapter".to_string(),
                0,
                vec![lesson1.clone(), lesson2.clone(), lesson3],
            )
            .unwrap();

            let chapter = chapter.delete_lesson(&lesson1).unwrap();
            assert_eq!(chapter.lessons().len(), 2);

            let chapter = chapter.delete_lesson(&lesson2).unwrap();
            assert_eq!(chapter.lessons().len(), 1);
            assert_eq!(chapter.lessons()[0].name().as_str(), "Third");
            assert_eq!(chapter.lessons()[0].index().value(), 0);
        }
    }

    mod move_lesson {
        use super::*;

        #[test]
        fn test_move_lesson_to_end() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);
            let lesson3 = create_test_lesson("Third", 2);

            let chapter = Chapter::new(
                "Test Chapter".to_string(),
                0,
                vec![lesson1.clone(), lesson2, lesson3],
            )
            .unwrap();

            let updated = chapter.move_lesson(&lesson1, Index::new(2)).unwrap();

            assert_eq!(updated.lessons()[0].name().as_str(), "Second");
            assert_eq!(updated.lessons()[1].name().as_str(), "Third");
            assert_eq!(updated.lessons()[2].name().as_str(), "First");
        }

        #[test]
        fn test_move_lesson_to_beginning() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);
            let lesson3 = create_test_lesson("Third", 2);

            let chapter = Chapter::new(
                "Test Chapter".to_string(),
                0,
                vec![lesson1, lesson2, lesson3.clone()],
            )
            .unwrap();

            let updated = chapter.move_lesson(&lesson3, Index::new(0)).unwrap();

            assert_eq!(updated.lessons()[0].name().as_str(), "Third");
            assert_eq!(updated.lessons()[1].name().as_str(), "First");
            assert_eq!(updated.lessons()[2].name().as_str(), "Second");
        }

        #[test]
        fn test_move_lesson_to_middle() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);
            let lesson3 = create_test_lesson("Third", 2);

            let chapter = Chapter::new(
                "Test Chapter".to_string(),
                0,
                vec![lesson1.clone(), lesson2, lesson3],
            )
            .unwrap();

            let updated = chapter.move_lesson(&lesson1, Index::new(1)).unwrap();

            assert_eq!(updated.lessons()[0].name().as_str(), "Second");
            assert_eq!(updated.lessons()[1].name().as_str(), "First");
            assert_eq!(updated.lessons()[2].name().as_str(), "Third");
        }

        #[test]
        fn test_move_lesson_reassigns_indices() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);
            let lesson3 = create_test_lesson("Third", 2);

            let chapter = Chapter::new(
                "Test Chapter".to_string(),
                0,
                vec![lesson1.clone(), lesson2, lesson3],
            )
            .unwrap();

            let updated = chapter.move_lesson(&lesson1, Index::new(2)).unwrap();

            assert_eq!(updated.lessons()[0].index().value(), 0);
            assert_eq!(updated.lessons()[1].index().value(), 1);
            assert_eq!(updated.lessons()[2].index().value(), 2);
        }

        #[test]
        fn test_move_lesson_preserves_chapter_id() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);

            let chapter =
                Chapter::new("Test Chapter".to_string(), 0, vec![lesson1.clone(), lesson2]).unwrap();
            let original_id = chapter.id();

            let updated = chapter.move_lesson(&lesson1, Index::new(1)).unwrap();

            assert_eq!(updated.id(), original_id);
        }

        #[test]
        fn test_move_lesson_preserves_chapter_name() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);

            let chapter =
                Chapter::new("My Chapter".to_string(), 0, vec![lesson1.clone(), lesson2]).unwrap();

            let updated = chapter.move_lesson(&lesson1, Index::new(1)).unwrap();

            assert_eq!(updated.name().as_str(), "My Chapter");
        }

        #[test]
        fn test_move_lesson_preserves_chapter_index() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);

            let chapter =
                Chapter::new("Test Chapter".to_string(), 5, vec![lesson1.clone(), lesson2]).unwrap();

            let updated = chapter.move_lesson(&lesson1, Index::new(1)).unwrap();

            assert_eq!(updated.index().value(), 5);
        }

        #[test]
        fn test_move_lesson_does_not_modify_original() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);

            let chapter =
                Chapter::new("Test Chapter".to_string(), 0, vec![lesson1.clone(), lesson2]).unwrap();

            let _ = chapter.move_lesson(&lesson1, Index::new(1)).unwrap();

            assert_eq!(chapter.lessons()[0].name().as_str(), "First");
            assert_eq!(chapter.lessons()[1].name().as_str(), "Second");
        }

        #[test]
        fn test_move_lesson_preserves_lesson_count() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);
            let lesson3 = create_test_lesson("Third", 2);

            let chapter = Chapter::new(
                "Test Chapter".to_string(),
                0,
                vec![lesson1.clone(), lesson2, lesson3],
            )
            .unwrap();

            let updated = chapter.move_lesson(&lesson1, Index::new(2)).unwrap();

            assert_eq!(updated.lessons().len(), 3);
        }

        #[test]
        fn test_move_lesson_preserves_lesson_ids() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);
            let lesson3 = create_test_lesson("Third", 2);

            let chapter = Chapter::new(
                "Test Chapter".to_string(),
                0,
                vec![lesson1.clone(), lesson2.clone(), lesson3.clone()],
            )
            .unwrap();

            let updated = chapter.move_lesson(&lesson1, Index::new(2)).unwrap();

            assert_eq!(updated.lessons()[0].id(), lesson2.id());
            assert_eq!(updated.lessons()[1].id(), lesson3.id());
            assert_eq!(updated.lessons()[2].id(), lesson1.id());
        }

        #[test]
        fn test_move_lesson_with_index_beyond_length() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);

            let chapter =
                Chapter::new("Test Chapter".to_string(), 0, vec![lesson1.clone(), lesson2]).unwrap();

            let updated = chapter.move_lesson(&lesson1, Index::new(100)).unwrap();

            assert_eq!(updated.lessons()[0].name().as_str(), "Second");
            assert_eq!(updated.lessons()[1].name().as_str(), "First");
        }

        #[test]
        fn test_move_lesson_single_lesson_returns_error() {
            let lesson = create_test_lesson("Only Lesson", 0);

            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson.clone()]).unwrap();

            let result = chapter.move_lesson(&lesson, Index::new(0));

            assert!(result.is_err());
            assert!(matches!(result, Err(ChapterError::ChapterWithEmptyLessons)));
        }

        #[test]
        fn test_move_lesson_preserves_total_duration() {
            let lesson1 = Lesson::new(
                "First".to_string(),
                1800,
                "https://example.com/1.mp4".to_string(),
                0,
            )
            .unwrap();
            let lesson2 = Lesson::new(
                "Second".to_string(),
                1200,
                "https://example.com/2.mp4".to_string(),
                1,
            )
            .unwrap();

            let chapter =
                Chapter::new("Test Chapter".to_string(), 0, vec![lesson1.clone(), lesson2]).unwrap();

            let updated = chapter.move_lesson(&lesson1, Index::new(1)).unwrap();

            assert_eq!(updated.total_duration().total_seconds(), 3000);
        }

        #[test]
        fn test_move_lesson_multiple_times() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);
            let lesson3 = create_test_lesson("Third", 2);

            let chapter = Chapter::new(
                "Test Chapter".to_string(),
                0,
                vec![lesson1.clone(), lesson2.clone(), lesson3.clone()],
            )
            .unwrap();

            let chapter = chapter.move_lesson(&lesson1, Index::new(2)).unwrap();
            assert_eq!(chapter.lessons()[0].name().as_str(), "Second");
            assert_eq!(chapter.lessons()[1].name().as_str(), "Third");
            assert_eq!(chapter.lessons()[2].name().as_str(), "First");

            let chapter = chapter.move_lesson(&lesson3, Index::new(0)).unwrap();
            assert_eq!(chapter.lessons()[0].name().as_str(), "Third");
            assert_eq!(chapter.lessons()[1].name().as_str(), "Second");
            assert_eq!(chapter.lessons()[2].name().as_str(), "First");
        }

        #[test]
        fn test_move_lesson_to_same_position() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);
            let lesson3 = create_test_lesson("Third", 2);

            let chapter = Chapter::new(
                "Test Chapter".to_string(),
                0,
                vec![lesson1, lesson2.clone(), lesson3],
            )
            .unwrap();

            let updated = chapter.move_lesson(&lesson2, Index::new(1)).unwrap();

            assert_eq!(updated.lessons()[0].name().as_str(), "First");
            assert_eq!(updated.lessons()[1].name().as_str(), "Second");
            assert_eq!(updated.lessons()[2].name().as_str(), "Third");
        }
    }

    mod move_lesson_up {
        use super::*;

        #[test]
        fn test_move_lesson_up_from_second_to_first() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);

            let chapter =
                Chapter::new("Test Chapter".to_string(), 0, vec![lesson1, lesson2.clone()]).unwrap();

            let updated = chapter.move_lesson_up(&lesson2).unwrap();

            assert_eq!(updated.lessons()[0].name().as_str(), "Second");
            assert_eq!(updated.lessons()[1].name().as_str(), "First");
        }

        #[test]
        fn test_move_lesson_up_from_third_to_second() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);
            let lesson3 = create_test_lesson("Third", 2);

            let chapter = Chapter::new(
                "Test Chapter".to_string(),
                0,
                vec![lesson1, lesson2, lesson3.clone()],
            )
            .unwrap();

            let updated = chapter.move_lesson_up(&lesson3).unwrap();

            assert_eq!(updated.lessons()[0].name().as_str(), "First");
            assert_eq!(updated.lessons()[1].name().as_str(), "Third");
            assert_eq!(updated.lessons()[2].name().as_str(), "Second");
        }

        #[test]
        fn test_move_lesson_up_already_at_first_position_returns_unchanged() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);

            let chapter =
                Chapter::new("Test Chapter".to_string(), 0, vec![lesson1.clone(), lesson2]).unwrap();

            let updated = chapter.move_lesson_up(&lesson1).unwrap();

            assert_eq!(updated.lessons()[0].name().as_str(), "First");
            assert_eq!(updated.lessons()[1].name().as_str(), "Second");
        }

        #[test]
        fn test_move_lesson_up_nonexistent_lesson_returns_error() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);
            let nonexistent = create_test_lesson("Nonexistent", 99);

            let chapter =
                Chapter::new("Test Chapter".to_string(), 0, vec![lesson1, lesson2]).unwrap();

            let result = chapter.move_lesson_up(&nonexistent);

            assert!(result.is_err());
            assert!(matches!(result, Err(ChapterError::LessonDoesNotExist)));
        }

        #[test]
        fn test_move_lesson_up_single_lesson_returns_error() {
            let lesson = create_test_lesson("Only Lesson", 0);

            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson.clone()]).unwrap();

            let result = chapter.move_lesson_up(&lesson);

            assert!(result.is_ok());
        }

        #[test]
        fn test_move_lesson_up_reassigns_indices() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);
            let lesson3 = create_test_lesson("Third", 2);

            let chapter = Chapter::new(
                "Test Chapter".to_string(),
                0,
                vec![lesson1, lesson2, lesson3.clone()],
            )
            .unwrap();

            let updated = chapter.move_lesson_up(&lesson3).unwrap();

            assert_eq!(updated.lessons()[0].index().value(), 0);
            assert_eq!(updated.lessons()[1].index().value(), 1);
            assert_eq!(updated.lessons()[2].index().value(), 2);
        }

        #[test]
        fn test_move_lesson_up_preserves_chapter_id() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);

            let chapter =
                Chapter::new("Test Chapter".to_string(), 0, vec![lesson1, lesson2.clone()]).unwrap();
            let original_id = chapter.id();

            let updated = chapter.move_lesson_up(&lesson2).unwrap();

            assert_eq!(updated.id(), original_id);
        }

        #[test]
        fn test_move_lesson_up_preserves_chapter_name() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);

            let chapter =
                Chapter::new("My Chapter".to_string(), 0, vec![lesson1, lesson2.clone()]).unwrap();

            let updated = chapter.move_lesson_up(&lesson2).unwrap();

            assert_eq!(updated.name().as_str(), "My Chapter");
        }

        #[test]
        fn test_move_lesson_up_preserves_chapter_index() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);

            let chapter =
                Chapter::new("Test Chapter".to_string(), 5, vec![lesson1, lesson2.clone()]).unwrap();

            let updated = chapter.move_lesson_up(&lesson2).unwrap();

            assert_eq!(updated.index().value(), 5);
        }

        #[test]
        fn test_move_lesson_up_does_not_modify_original() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);

            let chapter =
                Chapter::new("Test Chapter".to_string(), 0, vec![lesson1, lesson2.clone()]).unwrap();

            let _ = chapter.move_lesson_up(&lesson2).unwrap();

            assert_eq!(chapter.lessons()[0].name().as_str(), "First");
            assert_eq!(chapter.lessons()[1].name().as_str(), "Second");
        }

        #[test]
        fn test_move_lesson_up_preserves_lesson_count() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);
            let lesson3 = create_test_lesson("Third", 2);

            let chapter = Chapter::new(
                "Test Chapter".to_string(),
                0,
                vec![lesson1, lesson2, lesson3.clone()],
            )
            .unwrap();

            let updated = chapter.move_lesson_up(&lesson3).unwrap();

            assert_eq!(updated.lessons().len(), 3);
        }

        #[test]
        fn test_move_lesson_up_preserves_lesson_ids() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);

            let chapter = Chapter::new(
                "Test Chapter".to_string(),
                0,
                vec![lesson1.clone(), lesson2.clone()],
            )
            .unwrap();

            let updated = chapter.move_lesson_up(&lesson2).unwrap();

            assert_eq!(updated.lessons()[0].id(), lesson2.id());
            assert_eq!(updated.lessons()[1].id(), lesson1.id());
        }

        #[test]
        fn test_move_lesson_up_multiple_times() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);
            let lesson3 = create_test_lesson("Third", 2);

            let chapter = Chapter::new(
                "Test Chapter".to_string(),
                0,
                vec![lesson1.clone(), lesson2.clone(), lesson3.clone()],
            )
            .unwrap();

            let chapter = chapter.move_lesson_up(&lesson3).unwrap();
            assert_eq!(chapter.lessons()[0].name().as_str(), "First");
            assert_eq!(chapter.lessons()[1].name().as_str(), "Third");
            assert_eq!(chapter.lessons()[2].name().as_str(), "Second");

            let chapter = chapter.move_lesson_up(&lesson3).unwrap();
            assert_eq!(chapter.lessons()[0].name().as_str(), "Third");
            assert_eq!(chapter.lessons()[1].name().as_str(), "First");
            assert_eq!(chapter.lessons()[2].name().as_str(), "Second");

            let chapter = chapter.move_lesson_up(&lesson3).unwrap();
            assert_eq!(chapter.lessons()[0].name().as_str(), "Third");
            assert_eq!(chapter.lessons()[1].name().as_str(), "First");
            assert_eq!(chapter.lessons()[2].name().as_str(), "Second");
        }

        #[test]
        fn test_move_lesson_up_preserves_total_duration() {
            let lesson1 = Lesson::new(
                "First".to_string(),
                1800,
                "https://example.com/1.mp4".to_string(),
                0,
            )
            .unwrap();
            let lesson2 = Lesson::new(
                "Second".to_string(),
                1200,
                "https://example.com/2.mp4".to_string(),
                1,
            )
            .unwrap();

            let chapter =
                Chapter::new("Test Chapter".to_string(), 0, vec![lesson1, lesson2.clone()]).unwrap();

            let updated = chapter.move_lesson_up(&lesson2).unwrap();

            assert_eq!(updated.total_duration().total_seconds(), 3000);
        }
    }

    mod move_lesson_down {
        use super::*;

        #[test]
        fn test_move_lesson_down_from_first_to_second() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);

            let chapter =
                Chapter::new("Test Chapter".to_string(), 0, vec![lesson1.clone(), lesson2]).unwrap();

            let updated = chapter.move_lesson_down(&lesson1).unwrap();

            assert_eq!(updated.lessons()[0].name().as_str(), "Second");
            assert_eq!(updated.lessons()[1].name().as_str(), "First");
        }

        #[test]
        fn test_move_lesson_down_from_first_to_second_in_three_lessons() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);
            let lesson3 = create_test_lesson("Third", 2);

            let chapter = Chapter::new(
                "Test Chapter".to_string(),
                0,
                vec![lesson1.clone(), lesson2, lesson3],
            )
            .unwrap();

            let updated = chapter.move_lesson_down(&lesson1).unwrap();

            assert_eq!(updated.lessons()[0].name().as_str(), "Second");
            assert_eq!(updated.lessons()[1].name().as_str(), "First");
            assert_eq!(updated.lessons()[2].name().as_str(), "Third");
        }

        #[test]
        fn test_move_lesson_down_already_at_last_position_returns_unchanged() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);

            let chapter =
                Chapter::new("Test Chapter".to_string(), 0, vec![lesson1, lesson2.clone()]).unwrap();

            let updated = chapter.move_lesson_down(&lesson2).unwrap();

            assert_eq!(updated.lessons()[0].name().as_str(), "First");
            assert_eq!(updated.lessons()[1].name().as_str(), "Second");
        }

        #[test]
        fn test_move_lesson_down_nonexistent_lesson_returns_error() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);
            let nonexistent = create_test_lesson("Nonexistent", 99);

            let chapter =
                Chapter::new("Test Chapter".to_string(), 0, vec![lesson1, lesson2]).unwrap();

            let result = chapter.move_lesson_down(&nonexistent);

            assert!(result.is_err());
            assert!(matches!(result, Err(ChapterError::LessonDoesNotExist)));
        }

        #[test]
        fn test_move_lesson_down_single_lesson_returns_unchanged() {
            let lesson = create_test_lesson("Only Lesson", 0);

            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson.clone()]).unwrap();

            let updated = chapter.move_lesson_down(&lesson).unwrap();

            assert_eq!(updated.lessons().len(), 1);
            assert_eq!(updated.lessons()[0].name().as_str(), "Only Lesson");
        }

        #[test]
        fn test_move_lesson_down_reassigns_indices() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);
            let lesson3 = create_test_lesson("Third", 2);

            let chapter = Chapter::new(
                "Test Chapter".to_string(),
                0,
                vec![lesson1.clone(), lesson2, lesson3],
            )
            .unwrap();

            let updated = chapter.move_lesson_down(&lesson1).unwrap();

            assert_eq!(updated.lessons()[0].index().value(), 0);
            assert_eq!(updated.lessons()[1].index().value(), 1);
            assert_eq!(updated.lessons()[2].index().value(), 2);
        }

        #[test]
        fn test_move_lesson_down_preserves_chapter_id() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);

            let chapter =
                Chapter::new("Test Chapter".to_string(), 0, vec![lesson1.clone(), lesson2]).unwrap();
            let original_id = chapter.id();

            let updated = chapter.move_lesson_down(&lesson1).unwrap();

            assert_eq!(updated.id(), original_id);
        }

        #[test]
        fn test_move_lesson_down_preserves_chapter_name() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);

            let chapter =
                Chapter::new("My Chapter".to_string(), 0, vec![lesson1.clone(), lesson2]).unwrap();

            let updated = chapter.move_lesson_down(&lesson1).unwrap();

            assert_eq!(updated.name().as_str(), "My Chapter");
        }

        #[test]
        fn test_move_lesson_down_preserves_chapter_index() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);

            let chapter =
                Chapter::new("Test Chapter".to_string(), 5, vec![lesson1.clone(), lesson2]).unwrap();

            let updated = chapter.move_lesson_down(&lesson1).unwrap();

            assert_eq!(updated.index().value(), 5);
        }

        #[test]
        fn test_move_lesson_down_does_not_modify_original() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);

            let chapter =
                Chapter::new("Test Chapter".to_string(), 0, vec![lesson1.clone(), lesson2]).unwrap();

            let _ = chapter.move_lesson_down(&lesson1).unwrap();

            assert_eq!(chapter.lessons()[0].name().as_str(), "First");
            assert_eq!(chapter.lessons()[1].name().as_str(), "Second");
        }

        #[test]
        fn test_move_lesson_down_preserves_lesson_count() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);
            let lesson3 = create_test_lesson("Third", 2);

            let chapter = Chapter::new(
                "Test Chapter".to_string(),
                0,
                vec![lesson1.clone(), lesson2, lesson3],
            )
            .unwrap();

            let updated = chapter.move_lesson_down(&lesson1).unwrap();

            assert_eq!(updated.lessons().len(), 3);
        }

        #[test]
        fn test_move_lesson_down_preserves_lesson_ids() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);

            let chapter = Chapter::new(
                "Test Chapter".to_string(),
                0,
                vec![lesson1.clone(), lesson2.clone()],
            )
            .unwrap();

            let updated = chapter.move_lesson_down(&lesson1).unwrap();

            assert_eq!(updated.lessons()[0].id(), lesson2.id());
            assert_eq!(updated.lessons()[1].id(), lesson1.id());
        }

        #[test]
        fn test_move_lesson_down_multiple_times() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);
            let lesson3 = create_test_lesson("Third", 2);

            let chapter = Chapter::new(
                "Test Chapter".to_string(),
                0,
                vec![lesson1.clone(), lesson2.clone(), lesson3.clone()],
            )
            .unwrap();

            let chapter = chapter.move_lesson_down(&lesson1).unwrap();
            assert_eq!(chapter.lessons()[0].name().as_str(), "Second");
            assert_eq!(chapter.lessons()[1].name().as_str(), "First");
            assert_eq!(chapter.lessons()[2].name().as_str(), "Third");

            let chapter = chapter.move_lesson_down(&lesson1).unwrap();
            assert_eq!(chapter.lessons()[0].name().as_str(), "Second");
            assert_eq!(chapter.lessons()[1].name().as_str(), "Third");
            assert_eq!(chapter.lessons()[2].name().as_str(), "First");

            // Already at last position, should remain unchanged
            let chapter = chapter.move_lesson_down(&lesson1).unwrap();
            assert_eq!(chapter.lessons()[0].name().as_str(), "Second");
            assert_eq!(chapter.lessons()[1].name().as_str(), "Third");
            assert_eq!(chapter.lessons()[2].name().as_str(), "First");
        }

        #[test]
        fn test_move_lesson_down_preserves_total_duration() {
            let lesson1 = Lesson::new(
                "First".to_string(),
                1800,
                "https://example.com/1.mp4".to_string(),
                0,
            )
            .unwrap();
            let lesson2 = Lesson::new(
                "Second".to_string(),
                1200,
                "https://example.com/2.mp4".to_string(),
                1,
            )
            .unwrap();

            let chapter =
                Chapter::new("Test Chapter".to_string(), 0, vec![lesson1.clone(), lesson2]).unwrap();

            let updated = chapter.move_lesson_down(&lesson1).unwrap();

            assert_eq!(updated.total_duration().total_seconds(), 3000);
        }

        #[test]
        fn test_move_lesson_down_from_middle() {
            let lesson1 = create_test_lesson("First", 0);
            let lesson2 = create_test_lesson("Second", 1);
            let lesson3 = create_test_lesson("Third", 2);

            let chapter = Chapter::new(
                "Test Chapter".to_string(),
                0,
                vec![lesson1, lesson2.clone(), lesson3],
            )
            .unwrap();

            let updated = chapter.move_lesson_down(&lesson2).unwrap();

            assert_eq!(updated.lessons()[0].name().as_str(), "First");
            assert_eq!(updated.lessons()[1].name().as_str(), "Third");
            assert_eq!(updated.lessons()[2].name().as_str(), "Second");
        }
    }
}
