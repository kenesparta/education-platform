use crate::{Chapter, Course, CourseError};
use education_platform_common::{Entity, Index};

impl Course {
    /// Moves a chapter to a specific index position in the course.
    ///
    /// Creates a new course with the chapter relocated to the specified index.
    /// The chapter is first removed from its current position, then inserted at
    /// the target index. All chapter indices are reassigned sequentially.
    ///
    /// # Arguments
    ///
    /// * `chapter` - Reference to the chapter to move (matched by ID)
    /// * `to_index` - Target position for the chapter (zero-based)
    ///
    /// # Errors
    ///
    /// Returns `CourseError::ChapterDoesNotExist` if the chapter is not found
    /// in the course.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Course, Chapter, Lesson};
    /// use education_platform_common::{Entity, Index};
    ///
    /// let lesson1 = Lesson::new("Lesson 1".to_string(), 1800,
    ///     "https://example.com/1.mp4".to_string(), 0).unwrap();
    /// let lesson2 = Lesson::new("Lesson 2".to_string(), 1800,
    ///     "https://example.com/2.mp4".to_string(), 0).unwrap();
    /// let lesson3 = Lesson::new("Lesson 3".to_string(), 1800,
    ///     "https://example.com/3.mp4".to_string(), 0).unwrap();
    ///
    /// let chapter1 = Chapter::new("Chapter 1".to_string(), 0, vec![lesson1]).unwrap();
    /// let chapter2 = Chapter::new("Chapter 2".to_string(), 1, vec![lesson2]).unwrap();
    /// let chapter3 = Chapter::new("Chapter 3".to_string(), 2, vec![lesson3]).unwrap();
    /// let chapter_to_move = chapter1.clone();
    ///
    /// let mut course = Course::new("Test Course".to_string(), None, 0,
    ///     vec![chapter1, chapter2, chapter3]).unwrap();
    ///
    /// course.move_chapter(&chapter_to_move, Index::new(2)).unwrap();
    /// assert_eq!(course.chapters()[2].id(), chapter_to_move.id());
    /// ```
    pub fn move_chapter(&mut self, chapter: &Chapter, to_index: Index) -> Result<(), CourseError> {
        self.delete_chapter(chapter)?;
        self.add_chapter(chapter.clone(), Some(to_index))
    }

    /// Moves a chapter one position up (towards the beginning) in the course.
    ///
    /// Creates a new course with the chapter moved to the previous position.
    /// If the chapter is already at the first position, returns a clone of the
    /// original course unchanged.
    ///
    /// # Arguments
    ///
    /// * `chapter` - Reference to the chapter to move up (matched by ID)
    ///
    /// # Errors
    ///
    /// Returns `CourseError::ChapterDoesNotExist` if the chapter is not found
    /// in the course.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Course, Chapter, Lesson};
    /// use education_platform_common::Entity;
    ///
    /// let lesson1 = Lesson::new("Lesson 1".to_string(), 1800,
    ///     "https://example.com/1.mp4".to_string(), 0).unwrap();
    /// let lesson2 = Lesson::new("Lesson 2".to_string(), 1800,
    ///     "https://example.com/2.mp4".to_string(), 0).unwrap();
    ///
    /// let chapter1 = Chapter::new("Chapter 1".to_string(), 0, vec![lesson1]).unwrap();
    /// let chapter2 = Chapter::new("Chapter 2".to_string(), 1, vec![lesson2]).unwrap();
    /// let chapter_to_move = chapter2.clone();
    ///
    /// let mut course = Course::new("Test Course".to_string(), None, 0,
    ///     vec![chapter1, chapter2]).unwrap();
    ///
    /// course.move_chapter_up(&chapter_to_move).unwrap();
    /// assert_eq!(course.chapters()[0].id(), chapter_to_move.id());
    /// ```
    pub fn move_chapter_up(&mut self, chapter: &Chapter) -> Result<(), CourseError> {
        let current_position = self
            .chapters
            .iter()
            .position(|c| c.id() == chapter.id())
            .ok_or(CourseError::ChapterDoesNotExist)?;

        if current_position == 0 {
            return Ok(());
        }

        self.move_chapter(chapter, Index::new(current_position - 1))
    }

    /// Moves a chapter one position down (towards the end) in the course.
    ///
    /// Creates a new course with the chapter moved to the next position.
    /// If the chapter is already at the last position, returns a clone of the
    /// original course unchanged.
    ///
    /// # Arguments
    ///
    /// * `chapter` - Reference to the chapter to move down (matched by ID)
    ///
    /// # Errors
    ///
    /// Returns `CourseError::ChapterDoesNotExist` if the chapter is not found
    /// in the course.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Course, Chapter, Lesson};
    /// use education_platform_common::Entity;
    ///
    /// let lesson1 = Lesson::new("Lesson 1".to_string(), 1800,
    ///     "https://example.com/1.mp4".to_string(), 0).unwrap();
    /// let lesson2 = Lesson::new("Lesson 2".to_string(), 1800,
    ///     "https://example.com/2.mp4".to_string(), 0).unwrap();
    ///
    /// let chapter1 = Chapter::new("Chapter 1".to_string(), 0, vec![lesson1]).unwrap();
    /// let chapter2 = Chapter::new("Chapter 2".to_string(), 1, vec![lesson2]).unwrap();
    /// let chapter_to_move = chapter1.clone();
    ///
    /// let mut course = Course::new("Test Course".to_string(), None, 0,
    ///     vec![chapter1, chapter2]).unwrap();
    ///
    /// course.move_chapter_down(&chapter_to_move).unwrap();
    /// assert_eq!(course.chapters()[1].id(), chapter_to_move.id());
    /// ```
    pub fn move_chapter_down(&mut self, chapter: &Chapter) -> Result<(), CourseError> {
        let current_position = self
            .chapters
            .iter()
            .position(|c| c.id() == chapter.id())
            .ok_or(CourseError::ChapterDoesNotExist)?;

        if current_position >= self.chapters.len() - 1 {
            return Ok(());
        }

        self.move_chapter(chapter, Index::new(current_position + 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Lesson;

    fn create_test_lesson(name: &str, duration: u64, index: usize) -> Lesson {
        Lesson::new(
            name.to_string(),
            duration,
            format!("https://example.com/{}.mp4", index),
            index,
        )
        .unwrap()
    }

    fn create_test_chapter(name: &str, index: usize) -> Chapter {
        let lesson = create_test_lesson(&format!("{} Lesson", name), 1800, 0);
        Chapter::new(name.to_string(), index, vec![lesson]).unwrap()
    }

    fn create_test_course(name: &str, chapters: Vec<Chapter>) -> Course {
        Course::new(name.to_string(), None, 0, chapters).unwrap()
    }

    mod move_chapter {
        use super::*;

        #[test]
        fn test_move_chapter_from_beginning_to_end() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter_to_move = chapter1.clone();
            let mut course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            course
                .move_chapter(&chapter_to_move, Index::new(2))
                .unwrap();

            assert_eq!(course.chapters()[0].name().as_str(), "Chapter 2");
            assert_eq!(course.chapters()[1].name().as_str(), "Chapter 3");
            assert_eq!(course.chapters()[2].name().as_str(), "Chapter 1");
        }

        #[test]
        fn test_move_chapter_from_end_to_beginning() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter_to_move = chapter3.clone();
            let mut course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            course
                .move_chapter(&chapter_to_move, Index::new(0))
                .unwrap();

            assert_eq!(course.chapters()[0].name().as_str(), "Chapter 3");
            assert_eq!(course.chapters()[1].name().as_str(), "Chapter 1");
            assert_eq!(course.chapters()[2].name().as_str(), "Chapter 2");
        }

        #[test]
        fn test_move_chapter_reassigns_indices() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter_to_move = chapter1.clone();
            let mut course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            course
                .move_chapter(&chapter_to_move, Index::new(2))
                .unwrap();

            assert_eq!(course.chapters()[0].index().value(), 0);
            assert_eq!(course.chapters()[1].index().value(), 1);
            assert_eq!(course.chapters()[2].index().value(), 2);
        }

        #[test]
        fn test_move_chapter_preserves_course_id() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter_to_move = chapter1.clone();
            let mut course = create_test_course("Test Course", vec![chapter1, chapter2]);
            let original_id = course.id();

            course
                .move_chapter(&chapter_to_move, Index::new(1))
                .unwrap();

            assert_eq!(course.id(), original_id);
        }

        #[test]
        fn test_move_chapter_preserves_course_name() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter_to_move = chapter1.clone();
            let mut course = create_test_course("My Course", vec![chapter1, chapter2]);

            course
                .move_chapter(&chapter_to_move, Index::new(1))
                .unwrap();

            assert_eq!(course.name().as_str(), "My Course");
        }

        #[test]
        fn test_move_chapter_multiple_times() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter_to_move = chapter1.clone();
            let mut course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            course
                .move_chapter(&chapter_to_move, Index::new(1))
                .unwrap();
            course
                .move_chapter(&chapter_to_move, Index::new(2))
                .unwrap();

            assert_eq!(course.chapters()[0].name().as_str(), "Chapter 2");
            assert_eq!(course.chapters()[1].name().as_str(), "Chapter 3");
            assert_eq!(course.chapters()[2].name().as_str(), "Chapter 1");
        }
    }

    mod move_chapter_up {
        use super::*;

        #[test]
        fn test_move_chapter_up_from_middle() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter_to_move = chapter2.clone();
            let mut course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            course.move_chapter_up(&chapter_to_move).unwrap();

            assert_eq!(course.chapters()[0].name().as_str(), "Chapter 2");
            assert_eq!(course.chapters()[1].name().as_str(), "Chapter 1");
            assert_eq!(course.chapters()[2].name().as_str(), "Chapter 3");
        }

        #[test]
        fn test_move_chapter_up_from_first_position_stays() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter_to_move = chapter1.clone();
            let mut course = create_test_course("Test Course", vec![chapter1, chapter2]);

            course.move_chapter_up(&chapter_to_move).unwrap();

            assert_eq!(course.chapters()[0].name().as_str(), "Chapter 1");
            assert_eq!(course.chapters()[1].name().as_str(), "Chapter 2");
        }

        #[test]
        fn test_move_chapter_up_with_nonexistent_chapter_returns_error() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let nonexistent_chapter = create_test_chapter("Nonexistent", 99);
            let mut course = create_test_course("Test Course", vec![chapter1, chapter2]);

            let result = course.move_chapter_up(&nonexistent_chapter);

            assert!(result.is_err());
            assert!(matches!(result, Err(CourseError::ChapterDoesNotExist)));
        }

        #[test]
        fn test_move_chapter_up_preserves_course_id() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter_to_move = chapter2.clone();
            let mut course = create_test_course("Test Course", vec![chapter1, chapter2]);
            let original_id = course.id();

            course.move_chapter_up(&chapter_to_move).unwrap();

            assert_eq!(course.id(), original_id);
        }

        #[test]
        fn test_move_chapter_up_preserves_chapter_ids() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter1_id = chapter1.id();
            let chapter2_id = chapter2.id();
            let chapter_to_move = chapter2.clone();
            let mut course = create_test_course("Test Course", vec![chapter1, chapter2]);

            course.move_chapter_up(&chapter_to_move).unwrap();

            assert_eq!(course.chapters()[0].id(), chapter2_id);
            assert_eq!(course.chapters()[1].id(), chapter1_id);
        }

        #[test]
        fn test_move_chapter_up_multiple_times() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter_to_move = chapter3.clone();
            let mut course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            course.move_chapter_up(&chapter_to_move).unwrap();
            course.move_chapter_up(&chapter_to_move).unwrap();

            assert_eq!(course.chapters()[0].name().as_str(), "Chapter 3");
            assert_eq!(course.chapters()[1].name().as_str(), "Chapter 1");
            assert_eq!(course.chapters()[2].name().as_str(), "Chapter 2");
        }
    }

    mod move_chapter_down {
        use super::*;

        #[test]
        fn test_move_chapter_down_from_middle() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter_to_move = chapter2.clone();
            let mut course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            course.move_chapter_down(&chapter_to_move).unwrap();

            assert_eq!(course.chapters()[0].name().as_str(), "Chapter 1");
            assert_eq!(course.chapters()[1].name().as_str(), "Chapter 3");
            assert_eq!(course.chapters()[2].name().as_str(), "Chapter 2");
        }

        #[test]
        fn test_move_chapter_down_from_first_position() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter_to_move = chapter1.clone();
            let mut course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            course.move_chapter_down(&chapter_to_move).unwrap();

            assert_eq!(course.chapters()[0].name().as_str(), "Chapter 2");
            assert_eq!(course.chapters()[1].name().as_str(), "Chapter 1");
            assert_eq!(course.chapters()[2].name().as_str(), "Chapter 3");
        }

        #[test]
        fn test_move_chapter_down_from_last_position_stays() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter_to_move = chapter2.clone();
            let mut course = create_test_course("Test Course", vec![chapter1, chapter2]);

            course.move_chapter_down(&chapter_to_move).unwrap();

            assert_eq!(course.chapters()[0].name().as_str(), "Chapter 1");
            assert_eq!(course.chapters()[1].name().as_str(), "Chapter 2");
        }

        #[test]
        fn test_move_chapter_down_with_nonexistent_chapter_returns_error() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let nonexistent_chapter = create_test_chapter("Nonexistent", 99);
            let mut course = create_test_course("Test Course", vec![chapter1, chapter2]);

            let result = course.move_chapter_down(&nonexistent_chapter);

            assert!(result.is_err());
            assert!(matches!(result, Err(CourseError::ChapterDoesNotExist)));
        }

        #[test]
        fn test_move_chapter_down_preserves_course_id() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter_to_move = chapter1.clone();
            let mut course = create_test_course("Test Course", vec![chapter1, chapter2]);
            let original_id = course.id();

            course.move_chapter_down(&chapter_to_move).unwrap();

            assert_eq!(course.id(), original_id);
        }

        #[test]
        fn test_move_chapter_down_multiple_times() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter_to_move = chapter1.clone();
            let mut course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            course.move_chapter_down(&chapter_to_move).unwrap();
            course.move_chapter_down(&chapter_to_move).unwrap();

            assert_eq!(course.chapters()[0].name().as_str(), "Chapter 2");
            assert_eq!(course.chapters()[1].name().as_str(), "Chapter 3");
            assert_eq!(course.chapters()[2].name().as_str(), "Chapter 1");
        }
    }
}
