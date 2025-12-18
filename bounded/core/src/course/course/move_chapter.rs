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
    /// let course = Course::new("Test Course".to_string(), None, 0,
    ///     vec![chapter1, chapter2, chapter3]).unwrap();
    ///
    /// let updated_course = course.move_chapter(&chapter_to_move, Index::new(2)).unwrap();
    /// assert_eq!(updated_course.chapters()[2].id(), chapter_to_move.id());
    /// ```
    pub fn move_chapter(&self, chapter: &Chapter, to_index: Index) -> Result<Self, CourseError> {
        self.delete_chapter(chapter)?
            .add_chapter(chapter.clone(), Some(to_index))
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
    /// let course = Course::new("Test Course".to_string(), None, 0,
    ///     vec![chapter1, chapter2]).unwrap();
    ///
    /// let updated_course = course.move_chapter_up(&chapter_to_move).unwrap();
    /// assert_eq!(updated_course.chapters()[0].id(), chapter_to_move.id());
    /// ```
    pub fn move_chapter_up(&self, chapter: &Chapter) -> Result<Self, CourseError> {
        let current_position = self
            .chapters
            .iter()
            .position(|c| c.id() == chapter.id())
            .ok_or(CourseError::ChapterDoesNotExist)?;

        if current_position == 0 {
            return Ok(self.clone());
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
    /// let course = Course::new("Test Course".to_string(), None, 0,
    ///     vec![chapter1, chapter2]).unwrap();
    ///
    /// let updated_course = course.move_chapter_down(&chapter_to_move).unwrap();
    /// assert_eq!(updated_course.chapters()[1].id(), chapter_to_move.id());
    /// ```
    pub fn move_chapter_down(&self, chapter: &Chapter) -> Result<Self, CourseError> {
        let current_position = self
            .chapters
            .iter()
            .position(|c| c.id() == chapter.id())
            .ok_or(CourseError::ChapterDoesNotExist)?;

        if current_position >= self.chapters.len() - 1 {
            return Ok(self.clone());
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
            let course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            let updated_course = course
                .move_chapter(&chapter_to_move, Index::new(2))
                .unwrap();

            assert_eq!(updated_course.chapters()[0].name().as_str(), "Chapter 2");
            assert_eq!(updated_course.chapters()[1].name().as_str(), "Chapter 3");
            assert_eq!(updated_course.chapters()[2].name().as_str(), "Chapter 1");
        }

        #[test]
        fn test_move_chapter_from_end_to_beginning() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter_to_move = chapter3.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            let updated_course = course
                .move_chapter(&chapter_to_move, Index::new(0))
                .unwrap();

            assert_eq!(updated_course.chapters()[0].name().as_str(), "Chapter 3");
            assert_eq!(updated_course.chapters()[1].name().as_str(), "Chapter 1");
            assert_eq!(updated_course.chapters()[2].name().as_str(), "Chapter 2");
        }

        #[test]
        fn test_move_chapter_from_middle_to_different_position() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter4 = create_test_chapter("Chapter 4", 3);
            let chapter_to_move = chapter2.clone();
            let course =
                create_test_course("Test Course", vec![chapter1, chapter2, chapter3, chapter4]);

            let updated_course = course
                .move_chapter(&chapter_to_move, Index::new(2))
                .unwrap();

            assert_eq!(updated_course.chapters()[0].name().as_str(), "Chapter 1");
            assert_eq!(updated_course.chapters()[1].name().as_str(), "Chapter 3");
            assert_eq!(updated_course.chapters()[2].name().as_str(), "Chapter 2");
            assert_eq!(updated_course.chapters()[3].name().as_str(), "Chapter 4");
        }

        #[test]
        fn test_move_chapter_reassigns_indices() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter_to_move = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            let updated_course = course
                .move_chapter(&chapter_to_move, Index::new(2))
                .unwrap();

            assert_eq!(updated_course.chapters()[0].index().value(), 0);
            assert_eq!(updated_course.chapters()[1].index().value(), 1);
            assert_eq!(updated_course.chapters()[2].index().value(), 2);
        }

        #[test]
        fn test_move_chapter_preserves_course_id() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter_to_move = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2]);
            let original_id = course.id();

            let updated_course = course
                .move_chapter(&chapter_to_move, Index::new(1))
                .unwrap();

            assert_eq!(updated_course.id(), original_id);
        }

        #[test]
        fn test_move_chapter_preserves_course_name() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter_to_move = chapter1.clone();
            let course = create_test_course("My Course", vec![chapter1, chapter2]);

            let updated_course = course
                .move_chapter(&chapter_to_move, Index::new(1))
                .unwrap();

            assert_eq!(updated_course.name().as_str(), "My Course");
        }

        #[test]
        fn test_move_chapter_preserves_course_date() {
            use education_platform_common::Date;

            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter_to_move = chapter1.clone();
            let date = Date::new(2024, 6, 15).unwrap();
            let course =
                Course::new("Test Course".to_string(), Some(date), 0, vec![chapter1, chapter2])
                    .unwrap();

            let updated_course = course
                .move_chapter(&chapter_to_move, Index::new(1))
                .unwrap();

            assert_eq!(updated_course.date().year(), 2024);
            assert_eq!(updated_course.date().month(), 6);
            assert_eq!(updated_course.date().day(), 15);
        }

        #[test]
        fn test_move_chapter_preserves_chapter_ids() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter1_id = chapter1.id();
            let chapter2_id = chapter2.id();
            let chapter3_id = chapter3.id();
            let chapter_to_move = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            let updated_course = course
                .move_chapter(&chapter_to_move, Index::new(2))
                .unwrap();

            assert_eq!(updated_course.chapters()[0].id(), chapter2_id);
            assert_eq!(updated_course.chapters()[1].id(), chapter3_id);
            assert_eq!(updated_course.chapters()[2].id(), chapter1_id);
        }

        #[test]
        fn test_move_chapter_preserves_total_duration() {
            let lesson1 = create_test_lesson("Lesson 1", 1800, 0);
            let lesson2 = create_test_lesson("Lesson 2", 1200, 0);
            let lesson3 = create_test_lesson("Lesson 3", 600, 0);
            let chapter1 = Chapter::new("Chapter 1".to_string(), 0, vec![lesson1]).unwrap();
            let chapter2 = Chapter::new("Chapter 2".to_string(), 1, vec![lesson2]).unwrap();
            let chapter3 = Chapter::new("Chapter 3".to_string(), 2, vec![lesson3]).unwrap();
            let chapter_to_move = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);
            let original_duration = course.duration().total_seconds();

            let updated_course = course
                .move_chapter(&chapter_to_move, Index::new(2))
                .unwrap();

            assert_eq!(updated_course.duration().total_seconds(), original_duration);
            assert_eq!(updated_course.duration().total_seconds(), 3600);
        }

        #[test]
        fn test_move_chapter_preserves_lesson_count() {
            let lesson1 = create_test_lesson("Lesson 1", 1800, 0);
            let lesson2 = create_test_lesson("Lesson 2", 1800, 1);
            let lesson3 = create_test_lesson("Lesson 3", 1800, 0);
            let chapter1 =
                Chapter::new("Chapter 1".to_string(), 0, vec![lesson1, lesson2]).unwrap();
            let chapter2 = Chapter::new("Chapter 2".to_string(), 1, vec![lesson3]).unwrap();
            let chapter_to_move = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2]);
            let original_lesson_count = course.number_of_lessons();

            let updated_course = course
                .move_chapter(&chapter_to_move, Index::new(1))
                .unwrap();

            assert_eq!(updated_course.number_of_lessons(), original_lesson_count);
            assert_eq!(updated_course.number_of_lessons(), 3);
        }

        #[test]
        fn test_move_chapter_does_not_modify_original_course() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter_to_move = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2]);

            let _ = course
                .move_chapter(&chapter_to_move, Index::new(1))
                .unwrap();

            assert_eq!(course.chapters()[0].name().as_str(), "Chapter 1");
            assert_eq!(course.chapters()[1].name().as_str(), "Chapter 2");
        }

        #[test]
        fn test_move_chapter_preserves_chapter_content() {
            let lesson = Lesson::new(
                "Special Lesson".to_string(),
                3600,
                "https://example.com/special.mp4".to_string(),
                0,
            )
            .unwrap();
            let chapter1 = Chapter::new("Special Chapter".to_string(), 0, vec![lesson]).unwrap();
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter_to_move = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2]);

            let updated_course = course
                .move_chapter(&chapter_to_move, Index::new(1))
                .unwrap();

            assert_eq!(updated_course.chapters()[1].name().as_str(), "Special Chapter");
            assert_eq!(updated_course.chapters()[1].lesson_quantity(), 1);
            assert_eq!(
                updated_course.chapters()[1]
                    .total_duration()
                    .total_seconds(),
                3600
            );
        }

        #[test]
        fn test_move_chapter_to_same_position() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter_to_move = chapter2.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            let updated_course = course
                .move_chapter(&chapter_to_move, Index::new(1))
                .unwrap();

            assert_eq!(updated_course.chapters()[0].name().as_str(), "Chapter 1");
            assert_eq!(updated_course.chapters()[1].name().as_str(), "Chapter 2");
            assert_eq!(updated_course.chapters()[2].name().as_str(), "Chapter 3");
        }

        #[test]
        fn test_move_chapter_in_two_chapter_course() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter_to_move = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2]);

            let updated_course = course
                .move_chapter(&chapter_to_move, Index::new(1))
                .unwrap();

            assert_eq!(updated_course.chapters()[0].name().as_str(), "Chapter 2");
            assert_eq!(updated_course.chapters()[1].name().as_str(), "Chapter 1");
        }

        #[test]
        fn test_move_chapter_multiple_times() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter_to_move = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            let course1 = course
                .move_chapter(&chapter_to_move, Index::new(1))
                .unwrap();
            let course2 = course1
                .move_chapter(&chapter_to_move, Index::new(2))
                .unwrap();

            assert_eq!(course2.chapters()[0].name().as_str(), "Chapter 2");
            assert_eq!(course2.chapters()[1].name().as_str(), "Chapter 3");
            assert_eq!(course2.chapters()[2].name().as_str(), "Chapter 1");
        }

        #[test]
        fn test_move_chapter_with_large_index_clamps_to_end() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter_to_move = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            let updated_course = course
                .move_chapter(&chapter_to_move, Index::new(999))
                .unwrap();

            assert_eq!(updated_course.chapters()[0].name().as_str(), "Chapter 2");
            assert_eq!(updated_course.chapters()[1].name().as_str(), "Chapter 3");
            assert_eq!(updated_course.chapters()[2].name().as_str(), "Chapter 1");
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
            let course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            let updated_course = course.move_chapter_up(&chapter_to_move).unwrap();

            assert_eq!(updated_course.chapters()[0].name().as_str(), "Chapter 2");
            assert_eq!(updated_course.chapters()[1].name().as_str(), "Chapter 1");
            assert_eq!(updated_course.chapters()[2].name().as_str(), "Chapter 3");
        }

        #[test]
        fn test_move_chapter_up_from_last_position() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter_to_move = chapter3.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            let updated_course = course.move_chapter_up(&chapter_to_move).unwrap();

            assert_eq!(updated_course.chapters()[0].name().as_str(), "Chapter 1");
            assert_eq!(updated_course.chapters()[1].name().as_str(), "Chapter 3");
            assert_eq!(updated_course.chapters()[2].name().as_str(), "Chapter 2");
        }

        #[test]
        fn test_move_chapter_up_from_first_position_returns_unchanged() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter_to_move = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2]);

            let updated_course = course.move_chapter_up(&chapter_to_move).unwrap();

            assert_eq!(updated_course.chapters()[0].name().as_str(), "Chapter 1");
            assert_eq!(updated_course.chapters()[1].name().as_str(), "Chapter 2");
        }

        #[test]
        fn test_move_chapter_up_with_nonexistent_chapter_returns_error() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let nonexistent_chapter = create_test_chapter("Nonexistent", 99);
            let course = create_test_course("Test Course", vec![chapter1, chapter2]);

            let result = course.move_chapter_up(&nonexistent_chapter);

            assert!(result.is_err());
            assert!(matches!(result, Err(CourseError::ChapterDoesNotExist)));
        }

        #[test]
        fn test_move_chapter_up_preserves_course_id() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter_to_move = chapter2.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2]);
            let original_id = course.id();

            let updated_course = course.move_chapter_up(&chapter_to_move).unwrap();

            assert_eq!(updated_course.id(), original_id);
        }

        #[test]
        fn test_move_chapter_up_preserves_chapter_ids() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter1_id = chapter1.id();
            let chapter2_id = chapter2.id();
            let chapter_to_move = chapter2.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2]);

            let updated_course = course.move_chapter_up(&chapter_to_move).unwrap();

            assert_eq!(updated_course.chapters()[0].id(), chapter2_id);
            assert_eq!(updated_course.chapters()[1].id(), chapter1_id);
        }

        #[test]
        fn test_move_chapter_up_reassigns_indices() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter_to_move = chapter3.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            let updated_course = course.move_chapter_up(&chapter_to_move).unwrap();

            assert_eq!(updated_course.chapters()[0].index().value(), 0);
            assert_eq!(updated_course.chapters()[1].index().value(), 1);
            assert_eq!(updated_course.chapters()[2].index().value(), 2);
        }

        #[test]
        fn test_move_chapter_up_multiple_times() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter_to_move = chapter3.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            let course1 = course.move_chapter_up(&chapter_to_move).unwrap();
            let course2 = course1.move_chapter_up(&chapter_to_move).unwrap();

            assert_eq!(course2.chapters()[0].name().as_str(), "Chapter 3");
            assert_eq!(course2.chapters()[1].name().as_str(), "Chapter 1");
            assert_eq!(course2.chapters()[2].name().as_str(), "Chapter 2");
        }

        #[test]
        fn test_move_chapter_up_does_not_modify_original_course() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter_to_move = chapter2.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2]);

            let _ = course.move_chapter_up(&chapter_to_move).unwrap();

            assert_eq!(course.chapters()[0].name().as_str(), "Chapter 1");
            assert_eq!(course.chapters()[1].name().as_str(), "Chapter 2");
        }

        #[test]
        fn test_move_chapter_up_preserves_total_duration() {
            let lesson1 = create_test_lesson("Lesson 1", 1800, 0);
            let lesson2 = create_test_lesson("Lesson 2", 1200, 0);
            let chapter1 = Chapter::new("Chapter 1".to_string(), 0, vec![lesson1]).unwrap();
            let chapter2 = Chapter::new("Chapter 2".to_string(), 1, vec![lesson2]).unwrap();
            let chapter_to_move = chapter2.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2]);
            let original_duration = course.duration().total_seconds();

            let updated_course = course.move_chapter_up(&chapter_to_move).unwrap();

            assert_eq!(updated_course.duration().total_seconds(), original_duration);
            assert_eq!(updated_course.duration().total_seconds(), 3000);
        }

        #[test]
        fn test_move_chapter_up_in_two_chapter_course() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter_to_move = chapter2.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2]);

            let updated_course = course.move_chapter_up(&chapter_to_move).unwrap();

            assert_eq!(updated_course.chapters()[0].name().as_str(), "Chapter 2");
            assert_eq!(updated_course.chapters()[1].name().as_str(), "Chapter 1");
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
            let course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            let updated_course = course.move_chapter_down(&chapter_to_move).unwrap();

            assert_eq!(updated_course.chapters()[0].name().as_str(), "Chapter 1");
            assert_eq!(updated_course.chapters()[1].name().as_str(), "Chapter 3");
            assert_eq!(updated_course.chapters()[2].name().as_str(), "Chapter 2");
        }

        #[test]
        fn test_move_chapter_down_from_first_position() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter_to_move = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            let updated_course = course.move_chapter_down(&chapter_to_move).unwrap();

            assert_eq!(updated_course.chapters()[0].name().as_str(), "Chapter 2");
            assert_eq!(updated_course.chapters()[1].name().as_str(), "Chapter 1");
            assert_eq!(updated_course.chapters()[2].name().as_str(), "Chapter 3");
        }

        #[test]
        fn test_move_chapter_down_from_last_position_returns_unchanged() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter_to_move = chapter2.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2]);

            let updated_course = course.move_chapter_down(&chapter_to_move).unwrap();

            assert_eq!(updated_course.chapters()[0].name().as_str(), "Chapter 1");
            assert_eq!(updated_course.chapters()[1].name().as_str(), "Chapter 2");
        }

        #[test]
        fn test_move_chapter_down_with_nonexistent_chapter_returns_error() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let nonexistent_chapter = create_test_chapter("Nonexistent", 99);
            let course = create_test_course("Test Course", vec![chapter1, chapter2]);

            let result = course.move_chapter_down(&nonexistent_chapter);

            assert!(result.is_err());
            assert!(matches!(result, Err(CourseError::ChapterDoesNotExist)));
        }

        #[test]
        fn test_move_chapter_down_preserves_course_id() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter_to_move = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2]);
            let original_id = course.id();

            let updated_course = course.move_chapter_down(&chapter_to_move).unwrap();

            assert_eq!(updated_course.id(), original_id);
        }

        #[test]
        fn test_move_chapter_down_preserves_chapter_ids() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter1_id = chapter1.id();
            let chapter2_id = chapter2.id();
            let chapter_to_move = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2]);

            let updated_course = course.move_chapter_down(&chapter_to_move).unwrap();

            assert_eq!(updated_course.chapters()[0].id(), chapter2_id);
            assert_eq!(updated_course.chapters()[1].id(), chapter1_id);
        }

        #[test]
        fn test_move_chapter_down_reassigns_indices() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter_to_move = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            let updated_course = course.move_chapter_down(&chapter_to_move).unwrap();

            assert_eq!(updated_course.chapters()[0].index().value(), 0);
            assert_eq!(updated_course.chapters()[1].index().value(), 1);
            assert_eq!(updated_course.chapters()[2].index().value(), 2);
        }

        #[test]
        fn test_move_chapter_down_multiple_times() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter_to_move = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            let course1 = course.move_chapter_down(&chapter_to_move).unwrap();
            let course2 = course1.move_chapter_down(&chapter_to_move).unwrap();

            assert_eq!(course2.chapters()[0].name().as_str(), "Chapter 2");
            assert_eq!(course2.chapters()[1].name().as_str(), "Chapter 3");
            assert_eq!(course2.chapters()[2].name().as_str(), "Chapter 1");
        }

        #[test]
        fn test_move_chapter_down_does_not_modify_original_course() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter_to_move = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2]);

            let _ = course.move_chapter_down(&chapter_to_move).unwrap();

            assert_eq!(course.chapters()[0].name().as_str(), "Chapter 1");
            assert_eq!(course.chapters()[1].name().as_str(), "Chapter 2");
        }

        #[test]
        fn test_move_chapter_down_preserves_total_duration() {
            let lesson1 = create_test_lesson("Lesson 1", 1800, 0);
            let lesson2 = create_test_lesson("Lesson 2", 1200, 0);
            let chapter1 = Chapter::new("Chapter 1".to_string(), 0, vec![lesson1]).unwrap();
            let chapter2 = Chapter::new("Chapter 2".to_string(), 1, vec![lesson2]).unwrap();
            let chapter_to_move = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2]);
            let original_duration = course.duration().total_seconds();

            let updated_course = course.move_chapter_down(&chapter_to_move).unwrap();

            assert_eq!(updated_course.duration().total_seconds(), original_duration);
            assert_eq!(updated_course.duration().total_seconds(), 3000);
        }

        #[test]
        fn test_move_chapter_down_in_two_chapter_course() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter_to_move = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2]);

            let updated_course = course.move_chapter_down(&chapter_to_move).unwrap();

            assert_eq!(updated_course.chapters()[0].name().as_str(), "Chapter 2");
            assert_eq!(updated_course.chapters()[1].name().as_str(), "Chapter 1");
        }

        #[test]
        fn test_move_chapter_down_from_second_to_last_to_last() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter4 = create_test_chapter("Chapter 4", 3);
            let chapter_to_move = chapter3.clone();
            let course =
                create_test_course("Test Course", vec![chapter1, chapter2, chapter3, chapter4]);

            let updated_course = course.move_chapter_down(&chapter_to_move).unwrap();

            assert_eq!(updated_course.chapters()[0].name().as_str(), "Chapter 1");
            assert_eq!(updated_course.chapters()[1].name().as_str(), "Chapter 2");
            assert_eq!(updated_course.chapters()[2].name().as_str(), "Chapter 4");
            assert_eq!(updated_course.chapters()[3].name().as_str(), "Chapter 3");
        }
    }
}
