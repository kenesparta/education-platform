use crate::{Chapter, Course, CourseError};
use education_platform_common::{Duration, Index};

impl Course {
    /// Adds a chapter to the course at the specified position.
    ///
    /// If `index` is `None`, the chapter is appended at the end. If `index` is
    /// `Some`, the chapter is inserted at that position and subsequent chapters
    /// are shifted. All chapters are reindexed sequentially after insertion.
    ///
    /// # Errors
    ///
    /// Returns `CourseError::ChapterError` if reindexing fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Course, Chapter, Lesson};
    /// use education_platform_common::Index;
    ///
    /// let lesson = Lesson::new(
    ///     "Intro".to_string(),
    ///     1800,
    ///     "https://example.com/1.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    /// let chapter = Chapter::new("Chapter 1".to_string(), 0, vec![lesson]).unwrap();
    /// let mut course = Course::new("My Course".to_string(), None, 0, vec![chapter]).unwrap();
    ///
    /// let new_lesson = Lesson::new(
    ///     "New Lesson".to_string(),
    ///     1200,
    ///     "https://example.com/2.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    /// let new_chapter = Chapter::new("Chapter 2".to_string(), 0, vec![new_lesson]).unwrap();
    ///
    /// course.add_chapter(new_chapter, None).unwrap();
    /// assert_eq!(course.chapter_quantity(), 2);
    /// ```
    pub fn add_chapter(
        &mut self,
        chapter: Chapter,
        index: Option<Index>,
    ) -> Result<(), CourseError> {
        let position = index
            .map(|idx| idx.value().min(self.chapters.len()))
            .unwrap_or(self.chapters.len());

        let mut chapters = Vec::with_capacity(self.chapters.len() + 1);
        chapters.extend_from_slice(&self.chapters[..position]);
        chapters.push(chapter);
        chapters.extend_from_slice(&self.chapters[position..]);

        self.chapters = Self::reassign_index_chapters(&chapters)?;
        let (duration, number_of_lessons) =
            Self::calculate_totals(&self.chapters, Duration::default());
        self.duration = duration;
        self.number_of_lessons = number_of_lessons;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Lesson;
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

    fn create_test_chapter(name: &str, index: usize) -> Chapter {
        let lesson = create_test_lesson(&format!("{} Lesson", name), 0);
        Chapter::new(name.to_string(), index, vec![lesson]).unwrap()
    }

    fn create_test_course(name: &str, chapters: Vec<Chapter>) -> Course {
        Course::new(name.to_string(), None, 0, chapters).unwrap()
    }

    mod add_chapter {
        use super::*;

        #[test]
        fn test_add_chapter_at_end_without_index() {
            let initial_chapters = vec![
                create_test_chapter("Chapter 1", 0),
                create_test_chapter("Chapter 2", 1),
            ];
            let mut course = create_test_course("Test Course", initial_chapters);
            let new_chapter = create_test_chapter("Chapter 3", 99);

            course.add_chapter(new_chapter, None).unwrap();

            assert_eq!(course.chapter_quantity(), 3);
            assert_eq!(course.chapters()[2].name().as_str(), "Chapter 3");
            assert_eq!(course.chapters()[2].index().value(), 2);
        }

        #[test]
        fn test_add_chapter_at_beginning() {
            let initial_chapters = vec![
                create_test_chapter("Chapter 2", 0),
                create_test_chapter("Chapter 3", 1),
            ];
            let mut course = create_test_course("Test Course", initial_chapters);
            let new_chapter = create_test_chapter("Chapter 1", 99);

            course
                .add_chapter(new_chapter, Some(Index::new(0)))
                .unwrap();

            assert_eq!(course.chapter_quantity(), 3);
            assert_eq!(course.chapters()[0].name().as_str(), "Chapter 1");
            assert_eq!(course.chapters()[0].index().value(), 0);
            assert_eq!(course.chapters()[1].name().as_str(), "Chapter 2");
            assert_eq!(course.chapters()[1].index().value(), 1);
            assert_eq!(course.chapters()[2].name().as_str(), "Chapter 3");
            assert_eq!(course.chapters()[2].index().value(), 2);
        }

        #[test]
        fn test_add_chapter_in_middle() {
            let initial_chapters = vec![
                create_test_chapter("Chapter 1", 0),
                create_test_chapter("Chapter 3", 1),
            ];
            let mut course = create_test_course("Test Course", initial_chapters);
            let new_chapter = create_test_chapter("Chapter 2", 99);

            course
                .add_chapter(new_chapter, Some(Index::new(1)))
                .unwrap();

            assert_eq!(course.chapter_quantity(), 3);
            assert_eq!(course.chapters()[0].name().as_str(), "Chapter 1");
            assert_eq!(course.chapters()[0].index().value(), 0);
            assert_eq!(course.chapters()[1].name().as_str(), "Chapter 2");
            assert_eq!(course.chapters()[1].index().value(), 1);
            assert_eq!(course.chapters()[2].name().as_str(), "Chapter 3");
            assert_eq!(course.chapters()[2].index().value(), 2);
        }

        #[test]
        fn test_add_chapter_with_index_beyond_length_clamps_to_end() {
            let initial_chapters = vec![
                create_test_chapter("Chapter 1", 0),
                create_test_chapter("Chapter 2", 1),
            ];
            let mut course = create_test_course("Test Course", initial_chapters);
            let new_chapter = create_test_chapter("Chapter 3", 99);

            course
                .add_chapter(new_chapter, Some(Index::new(100)))
                .unwrap();

            assert_eq!(course.chapter_quantity(), 3);
            assert_eq!(course.chapters()[2].name().as_str(), "Chapter 3");
            assert_eq!(course.chapters()[2].index().value(), 2);
        }

        #[test]
        fn test_add_chapter_to_single_chapter_course() {
            let initial_chapters = vec![create_test_chapter("Chapter 1", 0)];
            let mut course = create_test_course("Test Course", initial_chapters);
            let new_chapter = create_test_chapter("Chapter 2", 99);

            course.add_chapter(new_chapter, None).unwrap();

            assert_eq!(course.chapter_quantity(), 2);
            assert_eq!(course.chapters()[0].name().as_str(), "Chapter 1");
            assert_eq!(course.chapters()[1].name().as_str(), "Chapter 2");
        }

        #[test]
        fn test_add_chapter_reassigns_all_indices() {
            let initial_chapters = vec![
                create_test_chapter("Chapter 1", 10),
                create_test_chapter("Chapter 2", 20),
            ];
            let mut course = create_test_course("Test Course", initial_chapters);
            let new_chapter = create_test_chapter("Chapter 3", 99);

            course
                .add_chapter(new_chapter, Some(Index::new(1)))
                .unwrap();

            assert_eq!(course.chapters()[0].index().value(), 0);
            assert_eq!(course.chapters()[1].index().value(), 1);
            assert_eq!(course.chapters()[2].index().value(), 2);
        }

        #[test]
        fn test_add_chapter_preserves_course_id() {
            let initial_chapters = vec![create_test_chapter("Chapter 1", 0)];
            let mut course = create_test_course("Test Course", initial_chapters);
            let original_id = course.id();
            let new_chapter = create_test_chapter("Chapter 2", 99);

            course.add_chapter(new_chapter, None).unwrap();

            assert_eq!(course.id(), original_id);
        }

        #[test]
        fn test_add_chapter_preserves_course_name() {
            let initial_chapters = vec![create_test_chapter("Chapter 1", 0)];
            let mut course = create_test_course("Test Course", initial_chapters);
            let new_chapter = create_test_chapter("Chapter 2", 99);

            course.add_chapter(new_chapter, None).unwrap();

            assert_eq!(course.name().as_str(), "Test Course");
        }

        #[test]
        fn test_add_chapter_preserves_original_chapter_ids() {
            let initial_chapters = vec![
                create_test_chapter("Chapter 1", 0),
                create_test_chapter("Chapter 2", 1),
            ];
            let original_ids: Vec<_> = initial_chapters.iter().map(|c| c.id()).collect();
            let mut course = create_test_course("Test Course", initial_chapters);
            let new_chapter = create_test_chapter("New Chapter", 99);

            course
                .add_chapter(new_chapter, Some(Index::new(1)))
                .unwrap();

            assert_eq!(course.chapters()[0].id(), original_ids[0]);
            assert_eq!(course.chapters()[2].id(), original_ids[1]);
        }

        #[test]
        fn test_add_chapter_updates_total_duration() {
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
            let mut course = create_test_course("Test Course", vec![chapter1]);
            let new_chapter = Chapter::new("Chapter 2".to_string(), 1, vec![lesson2]).unwrap();

            course.add_chapter(new_chapter, None).unwrap();

            assert_eq!(course.duration().total_seconds(), 3000);
        }

        #[test]
        fn test_add_chapter_updates_lesson_count() {
            let lesson1 = create_test_lesson("Lesson 1", 0);
            let lesson2 = create_test_lesson("Lesson 2", 1);
            let lesson3 = create_test_lesson("Lesson 3", 2);
            let chapter1 =
                Chapter::new("Chapter 1".to_string(), 0, vec![lesson1, lesson2]).unwrap();
            let mut course = create_test_course("Test Course", vec![chapter1]);
            let new_chapter = Chapter::new("Chapter 2".to_string(), 1, vec![lesson3]).unwrap();

            course.add_chapter(new_chapter, None).unwrap();

            assert_eq!(course.number_of_lessons(), 3);
        }

        #[test]
        fn test_add_chapter_at_index_zero_to_single_chapter() {
            let initial_chapters = vec![create_test_chapter("Second", 0)];
            let mut course = create_test_course("Test Course", initial_chapters);
            let new_chapter = create_test_chapter("First", 99);

            course
                .add_chapter(new_chapter, Some(Index::new(0)))
                .unwrap();

            assert_eq!(course.chapter_quantity(), 2);
            assert_eq!(course.chapters()[0].name().as_str(), "First");
            assert_eq!(course.chapters()[0].index().value(), 0);
            assert_eq!(course.chapters()[1].name().as_str(), "Second");
            assert_eq!(course.chapters()[1].index().value(), 1);
        }

        #[test]
        fn test_add_chapter_at_exact_length_index() {
            let initial_chapters = vec![
                create_test_chapter("Chapter 1", 0),
                create_test_chapter("Chapter 2", 1),
            ];
            let mut course = create_test_course("Test Course", initial_chapters);
            let new_chapter = create_test_chapter("Chapter 3", 99);

            course
                .add_chapter(new_chapter, Some(Index::new(2)))
                .unwrap();

            assert_eq!(course.chapter_quantity(), 3);
            assert_eq!(course.chapters()[2].name().as_str(), "Chapter 3");
            assert_eq!(course.chapters()[2].index().value(), 2);
        }

        #[test]
        fn test_add_chapter_preserves_chapter_content() {
            let initial_chapters = vec![create_test_chapter("Chapter 1", 0)];
            let mut course = create_test_course("Test Course", initial_chapters);
            let lesson = Lesson::new(
                "Special Lesson".to_string(),
                3600,
                "https://example.com/special.mp4".to_string(),
                0,
            )
            .unwrap();
            let new_chapter =
                Chapter::new("Special Chapter".to_string(), 99, vec![lesson]).unwrap();

            course.add_chapter(new_chapter, None).unwrap();

            assert_eq!(course.chapters()[1].name().as_str(), "Special Chapter");
            assert_eq!(course.chapters()[1].lesson_quantity(), 1);
            assert_eq!(course.chapters()[1].total_duration().total_seconds(), 3600);
        }
    }
}
