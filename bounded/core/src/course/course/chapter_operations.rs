use super::{Chapter, Course, CourseError, Duration};

impl Course {
    /// Calculates total duration and lesson count from chapters.
    ///
    /// If chapters are empty, returns the base duration and zero lessons.
    #[inline]
    pub(super) fn calculate_totals(
        chapters: &[Chapter],
        base_duration: Duration,
    ) -> (Duration, u32) {
        if chapters.is_empty() {
            return (base_duration, 0);
        }

        let duration = chapters
            .iter()
            .map(|chapter| chapter.total_duration())
            .fold(Duration::default(), |acc, d| acc.add(&d));

        let lessons = chapters
            .iter()
            .map(|chapter| chapter.lesson_quantity() as u32)
            .sum();

        (duration, lessons)
    }

    /// Reassigns sequential indices to all chapters starting from zero.
    ///
    /// Creates a new vector of chapters with indices reassigned to match
    /// their position in the collection (0, 1, 2, ...).
    ///
    /// # Errors
    ///
    /// Returns `CourseError::CourseWithEmptyChapters` if the chapters slice is empty.
    pub(super) fn reassign_index_chapters(
        chapters: &[Chapter],
    ) -> Result<Vec<Chapter>, CourseError> {
        if chapters.is_empty() {
            return Err(CourseError::CourseWithEmptyChapters);
        }

        Ok(chapters
            .iter()
            .enumerate()
            .map(|(idx, chapter)| {
                let mut cloned = chapter.clone();
                cloned.update_index(idx);
                cloned
            })
            .collect())
    }

    /// Sorts chapters by their index and reassigns sequential indices.
    ///
    /// Takes ownership of the chapter vector, sorts them by their current
    /// index values, then reassigns indices sequentially starting from zero.
    ///
    /// # Errors
    ///
    /// Returns `CourseError::CourseWithEmptyChapters` if the chapter vector is empty.
    pub(super) fn order_chapter(mut chapters: Vec<Chapter>) -> Result<Vec<Chapter>, CourseError> {
        if chapters.is_empty() {
            return Err(CourseError::CourseWithEmptyChapters);
        }

        chapters.sort_by_key(|chapter| chapter.index().value());
        Self::reassign_index_chapters(&chapters)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Lesson;
    use education_platform_common::Entity;

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

    mod calculate_totals {
        use super::*;

        #[test]
        fn test_calculate_totals_empty_chapters_returns_base_duration() {
            let base = Duration::from_seconds(3600);
            let chapters: Vec<Chapter> = vec![];

            let (duration, lessons) = Course::calculate_totals(&chapters, base);

            assert_eq!(duration.total_seconds(), 3600);
            assert_eq!(lessons, 0);
        }

        #[test]
        fn test_calculate_totals_single_chapter() {
            let lesson = create_test_lesson("Test", 1800, 0);
            let chapter = Chapter::new("Chapter One".to_string(), 0, vec![lesson]).unwrap();
            let base = Duration::from_seconds(0);

            let (duration, lessons) = Course::calculate_totals(&[chapter], base);

            assert_eq!(duration.total_seconds(), 1800);
            assert_eq!(lessons, 1);
        }

        #[test]
        fn test_calculate_totals_multiple_chapters() {
            let lesson1 = create_test_lesson("Lesson 1", 1800, 0);
            let lesson2 = create_test_lesson("Lesson 2", 1200, 0);
            let lesson3 = create_test_lesson("Lesson 3", 600, 1);
            let chapter1 = Chapter::new("Chapter One".to_string(), 0, vec![lesson1]).unwrap();
            let chapter2 =
                Chapter::new("Chapter Two".to_string(), 1, vec![lesson2, lesson3]).unwrap();
            let base = Duration::from_seconds(0);

            let (duration, lessons) = Course::calculate_totals(&[chapter1, chapter2], base);

            assert_eq!(duration.total_seconds(), 3600);
            assert_eq!(lessons, 3);
        }

        #[test]
        fn test_calculate_totals_ignores_base_duration_when_chapters_present() {
            let lesson = create_test_lesson("Test", 1800, 0);
            let chapter = Chapter::new("Chapter One".to_string(), 0, vec![lesson]).unwrap();
            let base = Duration::from_seconds(9999);

            let (duration, _) = Course::calculate_totals(&[chapter], base);

            assert_eq!(duration.total_seconds(), 1800);
        }
    }

    mod reassign_index_chapters {
        use super::*;

        #[test]
        fn test_reassign_index_chapters_empty_returns_error() {
            let chapters: Vec<Chapter> = vec![];

            let result = Course::reassign_index_chapters(&chapters);

            assert!(result.is_err());
            assert!(matches!(result, Err(CourseError::CourseWithEmptyChapters)));
        }

        #[test]
        fn test_reassign_index_chapters_single_chapter() {
            let chapter = create_test_chapter("Chapter", 99);

            let reassigned = Course::reassign_index_chapters(&[chapter]).unwrap();

            assert_eq!(reassigned.len(), 1);
            assert_eq!(reassigned[0].index().value(), 0);
        }

        #[test]
        fn test_reassign_index_chapters_multiple_chapters() {
            let chapters = vec![
                create_test_chapter("First", 10),
                create_test_chapter("Second", 20),
                create_test_chapter("Third", 30),
            ];

            let reassigned = Course::reassign_index_chapters(&chapters).unwrap();

            assert_eq!(reassigned[0].index().value(), 0);
            assert_eq!(reassigned[1].index().value(), 1);
            assert_eq!(reassigned[2].index().value(), 2);
        }

        #[test]
        fn test_reassign_index_chapters_preserves_id() {
            let chapters = vec![
                create_test_chapter("First", 10),
                create_test_chapter("Second", 20),
            ];
            let original_ids: Vec<_> = chapters.iter().map(|c| c.id()).collect();

            let reassigned = Course::reassign_index_chapters(&chapters).unwrap();

            assert_eq!(reassigned[0].id(), original_ids[0]);
            assert_eq!(reassigned[1].id(), original_ids[1]);
        }

        #[test]
        fn test_reassign_index_chapters_preserves_name() {
            let chapters = vec![create_test_chapter("My Chapter", 50)];

            let reassigned = Course::reassign_index_chapters(&chapters).unwrap();

            assert_eq!(reassigned[0].name().as_str(), "My Chapter");
        }

        #[test]
        fn test_reassign_index_chapters_does_not_modify_original() {
            let chapters = vec![
                create_test_chapter("First", 10),
                create_test_chapter("Second", 20),
            ];

            let _ = Course::reassign_index_chapters(&chapters).unwrap();

            assert_eq!(chapters[0].index().value(), 10);
            assert_eq!(chapters[1].index().value(), 20);
        }

        #[test]
        fn test_reassign_index_chapters_already_correct_indices() {
            let chapters = vec![
                create_test_chapter("First", 0),
                create_test_chapter("Second", 1),
                create_test_chapter("Third", 2),
            ];

            let reassigned = Course::reassign_index_chapters(&chapters).unwrap();

            assert_eq!(reassigned[0].index().value(), 0);
            assert_eq!(reassigned[1].index().value(), 1);
            assert_eq!(reassigned[2].index().value(), 2);
        }

        #[test]
        fn test_reassign_index_chapters_with_duplicate_indices() {
            let chapters = vec![
                create_test_chapter("First", 5),
                create_test_chapter("Second", 5),
                create_test_chapter("Third", 5),
            ];

            let reassigned = Course::reassign_index_chapters(&chapters).unwrap();

            assert_eq!(reassigned[0].index().value(), 0);
            assert_eq!(reassigned[1].index().value(), 1);
            assert_eq!(reassigned[2].index().value(), 2);
        }
    }

    mod order_chapter {
        use super::*;

        #[test]
        fn test_order_chapter_empty_returns_error() {
            let chapters: Vec<Chapter> = vec![];

            let result = Course::order_chapter(chapters);

            assert!(result.is_err());
            assert!(matches!(result, Err(CourseError::CourseWithEmptyChapters)));
        }

        #[test]
        fn test_order_chapter_sorts_by_index() {
            let chapters = vec![
                create_test_chapter("Third", 10),
                create_test_chapter("First", 2),
                create_test_chapter("Second", 5),
            ];

            let ordered = Course::order_chapter(chapters).unwrap();

            assert_eq!(ordered[0].name().as_str(), "First");
            assert_eq!(ordered[1].name().as_str(), "Second");
            assert_eq!(ordered[2].name().as_str(), "Third");
        }

        #[test]
        fn test_order_chapter_reassigns_indices() {
            let chapters = vec![
                create_test_chapter("Third", 100),
                create_test_chapter("First", 5),
                create_test_chapter("Second", 50),
            ];

            let ordered = Course::order_chapter(chapters).unwrap();

            assert_eq!(ordered[0].index().value(), 0);
            assert_eq!(ordered[1].index().value(), 1);
            assert_eq!(ordered[2].index().value(), 2);
        }

        #[test]
        fn test_order_chapter_single_chapter() {
            let chapters = vec![create_test_chapter("Only Chapter", 99)];

            let ordered = Course::order_chapter(chapters).unwrap();

            assert_eq!(ordered.len(), 1);
            assert_eq!(ordered[0].name().as_str(), "Only Chapter");
            assert_eq!(ordered[0].index().value(), 0);
        }

        #[test]
        fn test_order_chapter_already_ordered() {
            let chapters = vec![
                create_test_chapter("First", 0),
                create_test_chapter("Second", 1),
                create_test_chapter("Third", 2),
            ];

            let ordered = Course::order_chapter(chapters).unwrap();

            assert_eq!(ordered[0].name().as_str(), "First");
            assert_eq!(ordered[1].name().as_str(), "Second");
            assert_eq!(ordered[2].name().as_str(), "Third");
        }

        #[test]
        fn test_order_chapter_preserves_id() {
            let chapters = vec![
                create_test_chapter("Second", 10),
                create_test_chapter("First", 5),
            ];
            let original_ids: Vec<_> = chapters.iter().map(|c| c.id()).collect();

            let ordered = Course::order_chapter(chapters).unwrap();

            assert_eq!(ordered[0].id(), original_ids[1]);
            assert_eq!(ordered[1].id(), original_ids[0]);
        }

        #[test]
        fn test_order_chapter_with_duplicate_indices() {
            let chapters = vec![
                create_test_chapter("Chapter A", 5),
                create_test_chapter("Chapter B", 5),
                create_test_chapter("Chapter C", 5),
            ];

            let ordered = Course::order_chapter(chapters).unwrap();

            assert_eq!(ordered.len(), 3);
            assert_eq!(ordered[0].index().value(), 0);
            assert_eq!(ordered[1].index().value(), 1);
            assert_eq!(ordered[2].index().value(), 2);
        }

        #[test]
        fn test_order_chapter_reverse_order() {
            let chapters = vec![
                create_test_chapter("Last", 3),
                create_test_chapter("Third", 2),
                create_test_chapter("Second", 1),
                create_test_chapter("First", 0),
            ];

            let ordered = Course::order_chapter(chapters).unwrap();

            assert_eq!(ordered[0].name().as_str(), "First");
            assert_eq!(ordered[1].name().as_str(), "Second");
            assert_eq!(ordered[2].name().as_str(), "Third");
            assert_eq!(ordered[3].name().as_str(), "Last");
        }
    }
}
