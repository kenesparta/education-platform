use super::{Chapter, ChapterError, Lesson};
use education_platform_common::Entity;

impl Chapter {
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

        let lessons = Self::reassign_index_lessons(&lessons)?;

        Ok(Chapter {
            lessons,
            ..self.clone()
        })
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

    #[test]
    fn test_remove_lesson_removes_by_id() {
        let lessons = vec![
            create_test_lesson("First", 0),
            create_test_lesson("Second", 1),
        ];
        let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
        let lesson_to_remove = &chapter.lessons()[0].clone();

        let updated = chapter.delete_lesson(lesson_to_remove).unwrap();

        assert_eq!(updated.lessons().len(), 1);
        assert_eq!(updated.lessons()[0].name().as_str(), "Second");
    }

    #[test]
    fn test_remove_lesson_from_beginning() {
        let lessons = vec![
            create_test_lesson("First", 0),
            create_test_lesson("Second", 1),
            create_test_lesson("Third", 2),
        ];
        let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
        let first = &chapter.lessons()[0].clone();

        let updated = chapter.delete_lesson(first).unwrap();

        assert_eq!(updated.lessons().len(), 2);
        assert_eq!(updated.lessons()[0].name().as_str(), "Second");
        assert_eq!(updated.lessons()[1].name().as_str(), "Third");
    }

    #[test]
    fn test_remove_lesson_from_end() {
        let lessons = vec![
            create_test_lesson("First", 0),
            create_test_lesson("Second", 1),
            create_test_lesson("Third", 2),
        ];
        let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
        let last = &chapter.lessons()[2].clone();

        let updated = chapter.delete_lesson(last).unwrap();

        assert_eq!(updated.lessons().len(), 2);
        assert_eq!(updated.lessons()[0].name().as_str(), "First");
        assert_eq!(updated.lessons()[1].name().as_str(), "Second");
    }

    #[test]
    fn test_remove_lesson_reassigns_indices() {
        let lessons = vec![
            create_test_lesson("First", 0),
            create_test_lesson("Second", 1),
            create_test_lesson("Third", 2),
        ];
        let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
        let middle = &chapter.lessons()[1].clone();

        let updated = chapter.delete_lesson(middle).unwrap();

        assert_eq!(updated.lessons()[0].index().value(), 0);
        assert_eq!(updated.lessons()[1].index().value(), 1);
    }

    #[test]
    fn test_remove_lesson_returns_error_when_last_lesson() {
        let lesson = create_test_lesson("Only", 0);
        let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();
        let only = &chapter.lessons()[0].clone();

        let result = chapter.delete_lesson(only);

        assert!(result.is_err());
        assert!(matches!(result, Err(ChapterError::ChapterWithEmptyLessons)));
    }

    #[test]
    fn test_remove_lesson_nonexistent_lesson_returns_same_lessons() {
        let lessons = vec![
            create_test_lesson("First", 0),
            create_test_lesson("Second", 1),
        ];
        let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
        let nonexistent = create_test_lesson("Nonexistent", 99);

        let updated = chapter.delete_lesson(&nonexistent).unwrap();

        assert_eq!(updated.lessons().len(), 2);
    }

    #[test]
    fn test_remove_lesson_preserves_chapter_id() {
        let lessons = vec![
            create_test_lesson("First", 0),
            create_test_lesson("Second", 1),
        ];
        let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
        let original_id = chapter.id();
        let first = &chapter.lessons()[0].clone();

        let updated = chapter.delete_lesson(first).unwrap();

        assert_eq!(updated.id(), original_id);
    }

    #[test]
    fn test_remove_lesson_preserves_chapter_name() {
        let lessons = vec![
            create_test_lesson("First", 0),
            create_test_lesson("Second", 1),
        ];
        let chapter = Chapter::new("My Chapter".to_string(), 0, lessons).unwrap();
        let first = &chapter.lessons()[0].clone();

        let updated = chapter.delete_lesson(first).unwrap();

        assert_eq!(updated.name().as_str(), "My Chapter");
    }

    #[test]
    fn test_remove_lesson_preserves_chapter_index() {
        let lessons = vec![
            create_test_lesson("First", 0),
            create_test_lesson("Second", 1),
        ];
        let chapter = Chapter::new("Test Chapter".to_string(), 5, lessons).unwrap();
        let first = &chapter.lessons()[0].clone();

        let updated = chapter.delete_lesson(first).unwrap();

        assert_eq!(updated.index().value(), 5);
    }

    #[test]
    fn test_remove_lesson_does_not_modify_original() {
        let lessons = vec![
            create_test_lesson("First", 0),
            create_test_lesson("Second", 1),
        ];
        let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
        let first = &chapter.lessons()[0].clone();

        let _ = chapter.delete_lesson(first).unwrap();

        assert_eq!(chapter.lessons().len(), 2);
    }

    #[test]
    fn test_remove_lesson_preserves_remaining_lesson_ids() {
        let lessons = vec![
            create_test_lesson("First", 0),
            create_test_lesson("Second", 1),
            create_test_lesson("Third", 2),
        ];
        let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
        let original_second_id = chapter.lessons()[1].id();
        let original_third_id = chapter.lessons()[2].id();
        let first = &chapter.lessons()[0].clone();

        let updated = chapter.delete_lesson(first).unwrap();

        assert_eq!(updated.lessons()[0].id(), original_second_id);
        assert_eq!(updated.lessons()[1].id(), original_third_id);
    }

    #[test]
    fn test_remove_lesson_multiple_times() {
        let lessons = vec![
            create_test_lesson("First", 0),
            create_test_lesson("Second", 1),
            create_test_lesson("Third", 2),
        ];
        let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

        let first = &chapter.lessons()[0].clone();
        let chapter = chapter.delete_lesson(first).unwrap();

        let first = &chapter.lessons()[0].clone();
        let chapter = chapter.delete_lesson(first).unwrap();

        assert_eq!(chapter.lessons().len(), 1);
        assert_eq!(chapter.lessons()[0].name().as_str(), "Third");
    }

    #[test]
    fn test_remove_lesson_updates_total_duration() {
        let lesson1 =
            Lesson::new("First".to_string(), 1800, "https://example.com/1.mp4".to_string(), 0)
                .unwrap();
        let lesson2 =
            Lesson::new("Second".to_string(), 1200, "https://example.com/2.mp4".to_string(), 1)
                .unwrap();
        let chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson1, lesson2]).unwrap();

        let first = &chapter.lessons()[0].clone();
        let updated = chapter.delete_lesson(first).unwrap();

        assert_eq!(updated.total_duration().total_seconds(), 1200);
    }

    #[test]
    fn test_remove_lesson_updates_lesson_quantity() {
        let lessons = vec![
            create_test_lesson("First", 0),
            create_test_lesson("Second", 1),
        ];
        let chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

        let first = &chapter.lessons()[0].clone();
        let updated = chapter.delete_lesson(first).unwrap();

        assert_eq!(updated.lesson_quantity(), 1);
    }
}
