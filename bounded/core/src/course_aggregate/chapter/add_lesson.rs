use super::{Chapter, ChapterError, Index, Lesson};

impl Chapter {
    /// Adds a lesson to this chapter at the specified position.
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
    /// let mut chapter = Chapter::new("My Chapter".to_string(), 0, vec![lesson1]).unwrap();
    ///
    /// let new_lesson = Lesson::new(
    ///     "Second".to_string(),
    ///     600,
    ///     "https://example.com/2.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// chapter.add_lesson(new_lesson, None).unwrap();
    /// assert_eq!(chapter.lessons().len(), 2);
    /// assert_eq!(chapter.lessons()[1].name().as_str(), "Second");
    /// ```
    pub fn add_lesson(&mut self, lesson: Lesson, index: Option<Index>) -> Result<(), ChapterError> {
        let position = index
            .map(|idx| idx.value().min(self.lessons.len()))
            .unwrap_or(self.lessons.len());

        let mut lessons = Vec::with_capacity(self.lessons.len() + 1);
        lessons.extend_from_slice(&self.lessons[..position]);
        lessons.push(lesson);
        lessons.extend_from_slice(&self.lessons[position..]);

        self.lessons = Self::reassign_index_lessons(&lessons)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn test_add_lesson_appends_at_end_when_index_is_none() {
        let lesson = create_test_lesson("First", 0);
        let mut chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

        let new_lesson = create_test_lesson("Second", 0);
        chapter.add_lesson(new_lesson, None).unwrap();

        assert_eq!(chapter.lessons().len(), 2);
        assert_eq!(chapter.lessons()[0].name().as_str(), "First");
        assert_eq!(chapter.lessons()[1].name().as_str(), "Second");
    }

    #[test]
    fn test_add_lesson_inserts_at_beginning_with_index_zero() {
        let lesson = create_test_lesson("Second", 0);
        let mut chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

        let new_lesson = create_test_lesson("First", 0);
        chapter.add_lesson(new_lesson, Some(Index::new(0))).unwrap();

        assert_eq!(chapter.lessons().len(), 2);
        assert_eq!(chapter.lessons()[0].name().as_str(), "First");
        assert_eq!(chapter.lessons()[1].name().as_str(), "Second");
    }

    #[test]
    fn test_add_lesson_inserts_at_middle() {
        let lessons = vec![
            create_test_lesson("First", 0),
            create_test_lesson("Third", 1),
        ];
        let mut chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

        let new_lesson = create_test_lesson("Second", 0);
        chapter.add_lesson(new_lesson, Some(Index::new(1))).unwrap();

        assert_eq!(chapter.lessons().len(), 3);
        assert_eq!(chapter.lessons()[0].name().as_str(), "First");
        assert_eq!(chapter.lessons()[1].name().as_str(), "Second");
        assert_eq!(chapter.lessons()[2].name().as_str(), "Third");
    }

    #[test]
    fn test_add_lesson_reassigns_indices_sequentially() {
        let lessons = vec![
            create_test_lesson("First", 0),
            create_test_lesson("Second", 1),
        ];
        let mut chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

        let new_lesson = create_test_lesson("New", 99);
        chapter.add_lesson(new_lesson, Some(Index::new(1))).unwrap();

        assert_eq!(chapter.lessons()[0].index().value(), 0);
        assert_eq!(chapter.lessons()[1].index().value(), 1);
        assert_eq!(chapter.lessons()[2].index().value(), 2);
    }

    #[test]
    fn test_add_lesson_preserves_chapter_id() {
        let lesson = create_test_lesson("First", 0);
        let mut chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();
        let original_id = chapter.id();

        let new_lesson = create_test_lesson("Second", 0);
        chapter.add_lesson(new_lesson, None).unwrap();

        assert_eq!(chapter.id(), original_id);
    }

    #[test]
    fn test_add_lesson_preserves_chapter_name() {
        let lesson = create_test_lesson("First", 0);
        let mut chapter = Chapter::new("My Chapter".to_string(), 0, vec![lesson]).unwrap();

        let new_lesson = create_test_lesson("Second", 0);
        chapter.add_lesson(new_lesson, None).unwrap();

        assert_eq!(chapter.name().as_str(), "My Chapter");
    }

    #[test]
    fn test_add_lesson_preserves_chapter_index() {
        let lesson = create_test_lesson("First", 0);
        let mut chapter = Chapter::new("Test Chapter".to_string(), 5, vec![lesson]).unwrap();

        let new_lesson = create_test_lesson("Second", 0);
        chapter.add_lesson(new_lesson, None).unwrap();

        assert_eq!(chapter.index().value(), 5);
    }

    #[test]
    fn test_add_lesson_with_index_beyond_length_appends_at_end() {
        let lesson = create_test_lesson("First", 0);
        let mut chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

        let new_lesson = create_test_lesson("Second", 0);
        chapter
            .add_lesson(new_lesson, Some(Index::new(100)))
            .unwrap();

        assert_eq!(chapter.lessons().len(), 2);
        assert_eq!(chapter.lessons()[1].name().as_str(), "Second");
        assert_eq!(chapter.lessons()[1].index().value(), 1);
    }

    #[test]
    fn test_add_lesson_preserves_existing_lesson_ids() {
        let lessons = vec![
            create_test_lesson("First", 0),
            create_test_lesson("Second", 1),
        ];
        let mut chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();
        let original_ids: Vec<_> = chapter.lessons().iter().map(|c| c.id()).collect();

        let new_lesson = create_test_lesson("New", 0);
        chapter.add_lesson(new_lesson, Some(Index::new(1))).unwrap();

        assert_eq!(chapter.lessons()[0].id(), original_ids[0]);
        assert_eq!(chapter.lessons()[2].id(), original_ids[1]);
    }

    #[test]
    fn test_add_lesson_multiple_times() {
        let lesson = create_test_lesson("First", 0);
        let mut chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

        chapter
            .add_lesson(create_test_lesson("Second", 0), None)
            .unwrap();
        chapter
            .add_lesson(create_test_lesson("Third", 0), None)
            .unwrap();
        chapter
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
        let mut chapter = Chapter::new("Test Chapter".to_string(), 0, lessons).unwrap();

        let new_lesson = create_test_lesson("Third", 0);
        chapter.add_lesson(new_lesson, Some(Index::new(2))).unwrap();

        assert_eq!(chapter.lessons().len(), 3);
        assert_eq!(chapter.lessons()[2].name().as_str(), "Third");
        assert_eq!(chapter.lessons()[2].index().value(), 2);
    }

    #[test]
    fn test_add_lesson_updates_total_duration() {
        let lesson =
            Lesson::new("First".to_string(), 1800, "https://example.com/1.mp4".to_string(), 0)
                .unwrap();
        let mut chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

        let new_lesson =
            Lesson::new("Second".to_string(), 1200, "https://example.com/2.mp4".to_string(), 0)
                .unwrap();
        chapter.add_lesson(new_lesson, None).unwrap();

        assert_eq!(chapter.total_duration().total_seconds(), 3000);
    }

    #[test]
    fn test_add_lesson_updates_lesson_quantity() {
        let lesson = create_test_lesson("First", 0);
        let mut chapter = Chapter::new("Test Chapter".to_string(), 0, vec![lesson]).unwrap();

        assert_eq!(chapter.lesson_quantity(), 1);

        let new_lesson = create_test_lesson("Second", 0);
        chapter.add_lesson(new_lesson, None).unwrap();

        assert_eq!(chapter.lesson_quantity(), 2);
    }
}
