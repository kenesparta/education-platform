use super::{Chapter, ChapterError, Index, Lesson};
use education_platform_common::Entity;

impl Chapter {
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
            return Ok(self.clone());
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
            return Ok(self.clone());
        }

        self.move_lesson(lesson, Index::new(current_position + 1))
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

    mod move_lesson {
        use super::*;

        #[test]
        fn test_move_lesson_to_beginning() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
                create_test_lesson("Third", 2),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
            let third = &chapter.lessons()[2].clone();

            let updated = chapter.move_lesson(third, Index::new(0)).unwrap();

            assert_eq!(updated.lessons()[0].name().as_str(), "Third");
            assert_eq!(updated.lessons()[1].name().as_str(), "First");
            assert_eq!(updated.lessons()[2].name().as_str(), "Second");
        }

        #[test]
        fn test_move_lesson_to_end() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
                create_test_lesson("Third", 2),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
            let first = &chapter.lessons()[0].clone();

            let updated = chapter.move_lesson(first, Index::new(2)).unwrap();

            assert_eq!(updated.lessons()[0].name().as_str(), "Second");
            assert_eq!(updated.lessons()[1].name().as_str(), "Third");
            assert_eq!(updated.lessons()[2].name().as_str(), "First");
        }

        #[test]
        fn test_move_lesson_to_middle() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
                create_test_lesson("Third", 2),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
            let first = &chapter.lessons()[0].clone();

            let updated = chapter.move_lesson(first, Index::new(1)).unwrap();

            assert_eq!(updated.lessons()[0].name().as_str(), "Second");
            assert_eq!(updated.lessons()[1].name().as_str(), "First");
            assert_eq!(updated.lessons()[2].name().as_str(), "Third");
        }

        #[test]
        fn test_move_lesson_to_same_position() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
            let first = &chapter.lessons()[0].clone();

            let updated = chapter.move_lesson(first, Index::new(0)).unwrap();

            assert_eq!(updated.lessons()[0].name().as_str(), "First");
            assert_eq!(updated.lessons()[1].name().as_str(), "Second");
        }

        #[test]
        fn test_move_lesson_reassigns_indices() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
                create_test_lesson("Third", 2),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
            let third = &chapter.lessons()[2].clone();

            let updated = chapter.move_lesson(third, Index::new(0)).unwrap();

            assert_eq!(updated.lessons()[0].index().value(), 0);
            assert_eq!(updated.lessons()[1].index().value(), 1);
            assert_eq!(updated.lessons()[2].index().value(), 2);
        }

        #[test]
        fn test_move_lesson_preserves_lesson_ids() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
            let original_ids: Vec<_> = chapter.lessons().iter().map(|l| l.id()).collect();
            let first = &chapter.lessons()[0].clone();

            let updated = chapter.move_lesson(first, Index::new(1)).unwrap();

            assert!(updated.lessons().iter().any(|l| l.id() == original_ids[0]));
            assert!(updated.lessons().iter().any(|l| l.id() == original_ids[1]));
        }

        #[test]
        fn test_move_lesson_preserves_chapter_id() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
            let original_id = chapter.id();
            let first = &chapter.lessons()[0].clone();

            let updated = chapter.move_lesson(first, Index::new(1)).unwrap();

            assert_eq!(updated.id(), original_id);
        }

        #[test]
        fn test_move_lesson_preserves_chapter_name() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
            ];
            let chapter = Chapter::new("My Chapter".to_string(), 0, lessons).unwrap();
            let first = &chapter.lessons()[0].clone();

            let updated = chapter.move_lesson(first, Index::new(1)).unwrap();

            assert_eq!(updated.name().as_str(), "My Chapter");
        }

        #[test]
        fn test_move_lesson_preserves_chapter_index() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 5, lessons).unwrap();
            let first = &chapter.lessons()[0].clone();

            let updated = chapter.move_lesson(first, Index::new(1)).unwrap();

            assert_eq!(updated.index().value(), 5);
        }

        #[test]
        fn test_move_lesson_does_not_modify_original() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
            let first = &chapter.lessons()[0].clone();

            let _ = chapter.move_lesson(first, Index::new(1)).unwrap();

            assert_eq!(chapter.lessons()[0].name().as_str(), "First");
            assert_eq!(chapter.lessons()[1].name().as_str(), "Second");
        }

        #[test]
        fn test_move_lesson_preserves_lesson_count() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
                create_test_lesson("Third", 2),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
            let first = &chapter.lessons()[0].clone();

            let updated = chapter.move_lesson(first, Index::new(2)).unwrap();

            assert_eq!(updated.lessons().len(), 3);
        }

        #[test]
        fn test_move_lesson_preserves_total_duration() {
            let lessons = vec![
                Lesson::new("First".to_string(), 1800, "https://example.com/1.mp4".to_string(), 0)
                    .unwrap(),
                Lesson::new("Second".to_string(), 1200, "https://example.com/2.mp4".to_string(), 1)
                    .unwrap(),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
            let original_duration = chapter.total_duration().total_seconds();
            let first = &chapter.lessons()[0].clone();

            let updated = chapter.move_lesson(first, Index::new(1)).unwrap();

            assert_eq!(updated.total_duration().total_seconds(), original_duration);
        }

        #[test]
        fn test_move_lesson_single_lesson_returns_error() {
            let lesson = create_test_lesson("Only", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();
            let only = &chapter.lessons()[0].clone();

            let result = chapter.move_lesson(only, Index::new(0));

            assert!(result.is_err());
        }

        #[test]
        fn test_move_lesson_with_index_beyond_length() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
            let first = &chapter.lessons()[0].clone();

            let updated = chapter.move_lesson(first, Index::new(100)).unwrap();

            assert_eq!(updated.lessons()[0].name().as_str(), "Second");
            assert_eq!(updated.lessons()[1].name().as_str(), "First");
        }

        #[test]
        fn test_move_lesson_multiple_times() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
                create_test_lesson("Third", 2),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

            let first = &chapter.lessons()[0].clone();
            let chapter = chapter.move_lesson(first, Index::new(2)).unwrap();

            let second = &chapter.lessons()[0].clone();
            let chapter = chapter.move_lesson(second, Index::new(2)).unwrap();

            assert_eq!(chapter.lessons()[0].name().as_str(), "Third");
            assert_eq!(chapter.lessons()[1].name().as_str(), "First");
            assert_eq!(chapter.lessons()[2].name().as_str(), "Second");
        }
    }

    mod move_lesson_up {
        use super::*;

        #[test]
        fn test_move_lesson_up_from_second_to_first() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
            let second = &chapter.lessons()[1].clone();

            let updated = chapter.move_lesson_up(second).unwrap();

            assert_eq!(updated.lessons()[0].name().as_str(), "Second");
            assert_eq!(updated.lessons()[1].name().as_str(), "First");
        }

        #[test]
        fn test_move_lesson_up_already_at_first_position_returns_unchanged() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
            let first = &chapter.lessons()[0].clone();

            let updated = chapter.move_lesson_up(first).unwrap();

            assert_eq!(updated.lessons()[0].name().as_str(), "First");
            assert_eq!(updated.lessons()[1].name().as_str(), "Second");
        }

        #[test]
        fn test_move_lesson_up_from_third_to_second() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
                create_test_lesson("Third", 2),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
            let third = &chapter.lessons()[2].clone();

            let updated = chapter.move_lesson_up(third).unwrap();

            assert_eq!(updated.lessons()[0].name().as_str(), "First");
            assert_eq!(updated.lessons()[1].name().as_str(), "Third");
            assert_eq!(updated.lessons()[2].name().as_str(), "Second");
        }

        #[test]
        fn test_move_lesson_up_reassigns_indices() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
            let second = &chapter.lessons()[1].clone();

            let updated = chapter.move_lesson_up(second).unwrap();

            assert_eq!(updated.lessons()[0].index().value(), 0);
            assert_eq!(updated.lessons()[1].index().value(), 1);
        }

        #[test]
        fn test_move_lesson_up_preserves_lesson_ids() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
            let original_ids: Vec<_> = chapter.lessons().iter().map(|l| l.id()).collect();
            let second = &chapter.lessons()[1].clone();

            let updated = chapter.move_lesson_up(second).unwrap();

            assert_eq!(updated.lessons()[0].id(), original_ids[1]);
            assert_eq!(updated.lessons()[1].id(), original_ids[0]);
        }

        #[test]
        fn test_move_lesson_up_preserves_chapter_id() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
            let original_id = chapter.id();
            let second = &chapter.lessons()[1].clone();

            let updated = chapter.move_lesson_up(second).unwrap();

            assert_eq!(updated.id(), original_id);
        }

        #[test]
        fn test_move_lesson_up_single_lesson_returns_unchanged() {
            let lesson = create_test_lesson("Only", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();
            let only = &chapter.lessons()[0].clone();

            let result = chapter.move_lesson_up(only);

            assert!(result.is_ok());
            let updated = result.unwrap();
            assert_eq!(updated.lessons()[0].name().as_str(), "Only");
        }

        #[test]
        fn test_move_lesson_up_nonexistent_lesson_returns_error() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
            let nonexistent = create_test_lesson("Nonexistent", 99);

            let result = chapter.move_lesson_up(&nonexistent);

            assert!(result.is_err());
            assert!(matches!(result, Err(ChapterError::LessonDoesNotExist)));
        }

        #[test]
        fn test_move_lesson_up_multiple_times() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
                create_test_lesson("Third", 2),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

            let third = &chapter.lessons()[2].clone();
            let chapter = chapter.move_lesson_up(third).unwrap();
            let third = &chapter.lessons()[1].clone();
            let chapter = chapter.move_lesson_up(third).unwrap();

            assert_eq!(chapter.lessons()[0].name().as_str(), "Third");
            assert_eq!(chapter.lessons()[1].name().as_str(), "First");
            assert_eq!(chapter.lessons()[2].name().as_str(), "Second");
        }
    }

    mod move_lesson_down {
        use super::*;

        #[test]
        fn test_move_lesson_down_from_first_to_second() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
            let first = &chapter.lessons()[0].clone();

            let updated = chapter.move_lesson_down(first).unwrap();

            assert_eq!(updated.lessons()[0].name().as_str(), "Second");
            assert_eq!(updated.lessons()[1].name().as_str(), "First");
        }

        #[test]
        fn test_move_lesson_down_already_at_last_position_returns_unchanged() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
            let second = &chapter.lessons()[1].clone();

            let updated = chapter.move_lesson_down(second).unwrap();

            assert_eq!(updated.lessons()[0].name().as_str(), "First");
            assert_eq!(updated.lessons()[1].name().as_str(), "Second");
        }

        #[test]
        fn test_move_lesson_down_from_middle() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
                create_test_lesson("Third", 2),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
            let second = &chapter.lessons()[1].clone();

            let updated = chapter.move_lesson_down(second).unwrap();

            assert_eq!(updated.lessons()[0].name().as_str(), "First");
            assert_eq!(updated.lessons()[1].name().as_str(), "Third");
            assert_eq!(updated.lessons()[2].name().as_str(), "Second");
        }

        #[test]
        fn test_move_lesson_down_reassigns_indices() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
            let first = &chapter.lessons()[0].clone();

            let updated = chapter.move_lesson_down(first).unwrap();

            assert_eq!(updated.lessons()[0].index().value(), 0);
            assert_eq!(updated.lessons()[1].index().value(), 1);
        }

        #[test]
        fn test_move_lesson_down_preserves_lesson_ids() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
            let original_ids: Vec<_> = chapter.lessons().iter().map(|l| l.id()).collect();
            let first = &chapter.lessons()[0].clone();

            let updated = chapter.move_lesson_down(first).unwrap();

            assert_eq!(updated.lessons()[0].id(), original_ids[1]);
            assert_eq!(updated.lessons()[1].id(), original_ids[0]);
        }

        #[test]
        fn test_move_lesson_down_preserves_chapter_id() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
            let original_id = chapter.id();
            let first = &chapter.lessons()[0].clone();

            let updated = chapter.move_lesson_down(first).unwrap();

            assert_eq!(updated.id(), original_id);
        }

        #[test]
        fn test_move_lesson_down_single_lesson_returns_unchanged() {
            let lesson = create_test_lesson("Only", 0);
            let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();
            let only = &chapter.lessons()[0].clone();

            let result = chapter.move_lesson_down(only);

            assert!(result.is_ok());
            let updated = result.unwrap();
            assert_eq!(updated.lessons()[0].name().as_str(), "Only");
        }

        #[test]
        fn test_move_lesson_down_nonexistent_lesson_returns_error() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
            let nonexistent = create_test_lesson("Nonexistent", 99);

            let result = chapter.move_lesson_down(&nonexistent);

            assert!(result.is_err());
            assert!(matches!(result, Err(ChapterError::LessonDoesNotExist)));
        }

        #[test]
        fn test_move_lesson_down_multiple_times() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
                create_test_lesson("Third", 2),
            ];
            let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

            let first = &chapter.lessons()[0].clone();
            let chapter = chapter.move_lesson_down(first).unwrap();
            let first = &chapter.lessons()[1].clone();
            let chapter = chapter.move_lesson_down(first).unwrap();

            assert_eq!(chapter.lessons()[0].name().as_str(), "Second");
            assert_eq!(chapter.lessons()[1].name().as_str(), "Third");
            assert_eq!(chapter.lessons()[2].name().as_str(), "First");
        }
    }
}
