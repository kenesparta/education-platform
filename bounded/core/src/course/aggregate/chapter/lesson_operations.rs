use super::{Chapter, ChapterError, Lesson};

impl Chapter {
    /// Reassigns indices to lessons based on their position in the vector.
    ///
    /// Each lesson will have its index set to match its position in the vector
    /// (0-based indexing).
    ///
    /// # Errors
    ///
    /// Returns `ChapterError::ChapterWithEmptyLessons` if the lessons vector is empty.
    pub(super) fn reassign_index_lessons(lessons: &[Lesson]) -> Result<Vec<Lesson>, ChapterError> {
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
    pub(super) fn order_lessons(mut lessons: Vec<Lesson>) -> Result<Vec<Lesson>, ChapterError> {
        if lessons.is_empty() {
            return Err(ChapterError::ChapterWithEmptyLessons);
        }

        lessons.sort_by_key(|lesson| lesson.index().value());
        Self::reassign_index_lessons(&lessons)
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

    mod reassign_index_lessons {
        use super::*;

        #[test]
        fn test_reassign_index_lessons_with_unordered_indices() {
            let lessons = vec![
                create_test_lesson("First", 5),
                create_test_lesson("Second", 10),
                create_test_lesson("Third", 15),
            ];

            let reassigned = Chapter::reassign_index_lessons(&lessons).unwrap();

            assert_eq!(reassigned[0].index().value(), 0);
            assert_eq!(reassigned[1].index().value(), 1);
            assert_eq!(reassigned[2].index().value(), 2);
        }

        #[test]
        fn test_reassign_index_lessons_empty_returns_error() {
            let lessons: Vec<Lesson> = vec![];

            let result = Chapter::reassign_index_lessons(&lessons);

            assert!(result.is_err());
            assert!(matches!(result, Err(ChapterError::ChapterWithEmptyLessons)));
        }

        #[test]
        fn test_reassign_index_lessons_single_lesson() {
            let lessons = vec![create_test_lesson("Only", 99)];

            let reassigned = Chapter::reassign_index_lessons(&lessons).unwrap();

            assert_eq!(reassigned.len(), 1);
            assert_eq!(reassigned[0].index().value(), 0);
        }

        #[test]
        fn test_reassign_index_lessons_preserves_id() {
            let lessons = vec![
                create_test_lesson("First", 10),
                create_test_lesson("Second", 20),
            ];
            let original_ids: Vec<_> = lessons.iter().map(|l| l.id()).collect();

            let reassigned = Chapter::reassign_index_lessons(&lessons).unwrap();

            assert_eq!(reassigned[0].id(), original_ids[0]);
            assert_eq!(reassigned[1].id(), original_ids[1]);
        }

        #[test]
        fn test_reassign_index_lessons_preserves_lesson_data() {
            let lessons = vec![create_test_lesson("My Lesson", 50)];

            let reassigned = Chapter::reassign_index_lessons(&lessons).unwrap();

            assert_eq!(reassigned[0].name().as_str(), "My Lesson");
        }

        #[test]
        fn test_reassign_index_lessons_already_correct_indices() {
            let lessons = vec![
                create_test_lesson("First", 0),
                create_test_lesson("Second", 1),
                create_test_lesson("Third", 2),
            ];

            let reassigned = Chapter::reassign_index_lessons(&lessons).unwrap();

            assert_eq!(reassigned[0].index().value(), 0);
            assert_eq!(reassigned[1].index().value(), 1);
            assert_eq!(reassigned[2].index().value(), 2);
        }

        #[test]
        fn test_reassign_index_lessons_with_duplicate_indices() {
            let lessons = vec![
                create_test_lesson("First", 5),
                create_test_lesson("Second", 5),
                create_test_lesson("Third", 5),
            ];

            let reassigned = Chapter::reassign_index_lessons(&lessons).unwrap();

            assert_eq!(reassigned[0].index().value(), 0);
            assert_eq!(reassigned[1].index().value(), 1);
            assert_eq!(reassigned[2].index().value(), 2);
        }

        #[test]
        fn test_reassign_index_lessons_does_not_modify_original() {
            let lessons = vec![
                create_test_lesson("First", 10),
                create_test_lesson("Second", 20),
            ];

            let _ = Chapter::reassign_index_lessons(&lessons).unwrap();

            assert_eq!(lessons[0].index().value(), 10);
            assert_eq!(lessons[1].index().value(), 20);
        }

        #[test]
        fn test_reassign_index_lessons_large_collection() {
            let lessons: Vec<Lesson> = (0..100)
                .map(|i| create_test_lesson(&format!("Lesson {}", i), 100 - i))
                .collect();

            let reassigned = Chapter::reassign_index_lessons(&lessons).unwrap();

            for (i, lesson) in reassigned.iter().enumerate() {
                assert_eq!(lesson.index().value(), i);
            }
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
}
