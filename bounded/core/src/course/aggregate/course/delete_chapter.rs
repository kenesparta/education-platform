use crate::{Chapter, Course, CourseError};
use education_platform_common::{Duration, Entity};

impl Course {
    /// Removes a chapter from the course by its identity.
    ///
    /// Creates a new course with the specified chapter removed. Remaining
    /// chapters have their indices reassigned sequentially starting from zero.
    /// The course's total duration and lesson count are recalculated.
    ///
    /// # Arguments
    ///
    /// * `chapter` - Reference to the chapter to remove (matched by ID)
    ///
    /// # Errors
    ///
    /// Returns `CourseError::CourseWithEmptyChapters` if removing the chapter
    /// would leave the course with no chapters.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Course, Chapter, Lesson};
    /// use education_platform_common::Entity;
    ///
    /// let lesson1 = Lesson::new(
    ///     "Lesson 1".to_string(),
    ///     1800,
    ///     "https://example.com/1.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    /// let lesson2 = Lesson::new(
    ///     "Lesson 2".to_string(),
    ///     1200,
    ///     "https://example.com/2.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let chapter1 = Chapter::new("Chapter 1".to_string(), 0, vec![lesson1]).unwrap();
    /// let chapter2 = Chapter::new("Chapter 2".to_string(), 1, vec![lesson2]).unwrap();
    /// let chapter_to_delete = chapter1.clone();
    ///
    /// let course = Course::new(
    ///     "My Course".to_string(),
    ///     None,
    ///     0,
    ///     vec![chapter1, chapter2],
    /// ).unwrap();
    ///
    /// let updated_course = course.delete_chapter(&chapter_to_delete).unwrap();
    /// assert_eq!(updated_course.chapter_quantity(), 1);
    /// assert_eq!(updated_course.chapters()[0].name().as_str(), "Chapter 2");
    /// ```
    pub fn delete_chapter(&self, chapter: &Chapter) -> Result<Course, CourseError> {
        let chapters: Vec<Chapter> = self
            .chapters
            .iter()
            .filter(|c| c.id() != chapter.id())
            .cloned()
            .collect();

        let chapters = Self::reassign_index_chapters(&chapters)?;
        let (duration, number_of_lessons) = Self::calculate_totals(&chapters, Duration::default());

        Ok(Course {
            chapters,
            duration,
            number_of_lessons,
            ..self.clone()
        })
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

    mod delete_chapter {
        use super::*;

        #[test]
        fn test_delete_chapter_from_beginning() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter_to_delete = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            let updated_course = course.delete_chapter(&chapter_to_delete).unwrap();

            assert_eq!(updated_course.chapter_quantity(), 2);
            assert_eq!(updated_course.chapters()[0].name().as_str(), "Chapter 2");
            assert_eq!(updated_course.chapters()[1].name().as_str(), "Chapter 3");
        }

        #[test]
        fn test_delete_chapter_from_middle() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter_to_delete = chapter2.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            let updated_course = course.delete_chapter(&chapter_to_delete).unwrap();

            assert_eq!(updated_course.chapter_quantity(), 2);
            assert_eq!(updated_course.chapters()[0].name().as_str(), "Chapter 1");
            assert_eq!(updated_course.chapters()[1].name().as_str(), "Chapter 3");
        }

        #[test]
        fn test_delete_chapter_from_end() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter_to_delete = chapter3.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            let updated_course = course.delete_chapter(&chapter_to_delete).unwrap();

            assert_eq!(updated_course.chapter_quantity(), 2);
            assert_eq!(updated_course.chapters()[0].name().as_str(), "Chapter 1");
            assert_eq!(updated_course.chapters()[1].name().as_str(), "Chapter 2");
        }

        #[test]
        fn test_delete_last_chapter_returns_error() {
            let chapter = create_test_chapter("Only Chapter", 0);
            let chapter_to_delete = chapter.clone();
            let course = create_test_course("Test Course", vec![chapter]);

            let result = course.delete_chapter(&chapter_to_delete);

            assert!(result.is_err());
            assert!(matches!(result, Err(CourseError::CourseWithEmptyChapters)));
        }

        #[test]
        fn test_delete_chapter_reassigns_indices() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter_to_delete = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            let updated_course = course.delete_chapter(&chapter_to_delete).unwrap();

            assert_eq!(updated_course.chapters()[0].index().value(), 0);
            assert_eq!(updated_course.chapters()[1].index().value(), 1);
        }

        #[test]
        fn test_delete_chapter_preserves_course_id() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter_to_delete = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2]);
            let original_id = course.id();

            let updated_course = course.delete_chapter(&chapter_to_delete).unwrap();

            assert_eq!(updated_course.id(), original_id);
        }

        #[test]
        fn test_delete_chapter_preserves_course_name() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter_to_delete = chapter1.clone();
            let course = create_test_course("My Course", vec![chapter1, chapter2]);

            let updated_course = course.delete_chapter(&chapter_to_delete).unwrap();

            assert_eq!(updated_course.name().as_str(), "My Course");
        }

        #[test]
        fn test_delete_chapter_preserves_course_date() {
            use education_platform_common::Date;

            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter_to_delete = chapter1.clone();
            let date = Date::new(2024, 6, 15).unwrap();
            let course = Course::new(
                "Test Course".to_string(),
                Some(date),
                0,
                vec![chapter1, chapter2],
            )
            .unwrap();

            let updated_course = course.delete_chapter(&chapter_to_delete).unwrap();

            assert_eq!(updated_course.date().year(), 2024);
            assert_eq!(updated_course.date().month(), 6);
            assert_eq!(updated_course.date().day(), 15);
        }

        #[test]
        fn test_delete_chapter_does_not_modify_original_course() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter_to_delete = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2]);
            let original_count = course.chapter_quantity();

            let _ = course.delete_chapter(&chapter_to_delete).unwrap();

            assert_eq!(course.chapter_quantity(), original_count);
            assert_eq!(course.chapter_quantity(), 2);
        }

        #[test]
        fn test_delete_chapter_preserves_remaining_chapter_ids() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter2_id = chapter2.id();
            let chapter3_id = chapter3.id();
            let chapter_to_delete = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            let updated_course = course.delete_chapter(&chapter_to_delete).unwrap();

            assert_eq!(updated_course.chapters()[0].id(), chapter2_id);
            assert_eq!(updated_course.chapters()[1].id(), chapter3_id);
        }

        #[test]
        fn test_delete_chapter_updates_total_duration() {
            let lesson1 = create_test_lesson("Lesson 1", 1800, 0);
            let lesson2 = create_test_lesson("Lesson 2", 1200, 0);
            let lesson3 = create_test_lesson("Lesson 3", 600, 0);
            let chapter1 = Chapter::new("Chapter 1".to_string(), 0, vec![lesson1]).unwrap();
            let chapter2 = Chapter::new("Chapter 2".to_string(), 1, vec![lesson2]).unwrap();
            let chapter3 = Chapter::new("Chapter 3".to_string(), 2, vec![lesson3]).unwrap();
            let chapter_to_delete = chapter2.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            let updated_course = course.delete_chapter(&chapter_to_delete).unwrap();

            assert_eq!(updated_course.duration().total_seconds(), 2400);
        }

        #[test]
        fn test_delete_chapter_updates_lesson_count() {
            let lesson1 = create_test_lesson("Lesson 1", 1800, 0);
            let lesson2 = create_test_lesson("Lesson 2", 1800, 1);
            let lesson3 = create_test_lesson("Lesson 3", 1800, 0);
            let chapter1 =
                Chapter::new("Chapter 1".to_string(), 0, vec![lesson1, lesson2]).unwrap();
            let chapter2 = Chapter::new("Chapter 2".to_string(), 1, vec![lesson3]).unwrap();
            let chapter_to_delete = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2]);

            let updated_course = course.delete_chapter(&chapter_to_delete).unwrap();

            assert_eq!(updated_course.number_of_lessons(), 1);
        }

        #[test]
        fn test_delete_chapter_with_nonexistent_id_removes_nothing() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let nonexistent_chapter = create_test_chapter("Nonexistent", 99);
            let course = create_test_course("Test Course", vec![chapter1, chapter2]);

            let updated_course = course.delete_chapter(&nonexistent_chapter).unwrap();

            assert_eq!(updated_course.chapter_quantity(), 2);
            assert_eq!(updated_course.chapters()[0].name().as_str(), "Chapter 1");
            assert_eq!(updated_course.chapters()[1].name().as_str(), "Chapter 2");
        }

        #[test]
        fn test_delete_chapter_from_two_chapter_course() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter_to_delete = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2]);

            let updated_course = course.delete_chapter(&chapter_to_delete).unwrap();

            assert_eq!(updated_course.chapter_quantity(), 1);
            assert_eq!(updated_course.chapters()[0].name().as_str(), "Chapter 2");
            assert_eq!(updated_course.chapters()[0].index().value(), 0);
        }

        #[test]
        fn test_delete_chapter_preserves_remaining_chapter_content() {
            let lesson1 = Lesson::new(
                "Special Lesson".to_string(),
                3600,
                "https://example.com/special.mp4".to_string(),
                0,
            )
            .unwrap();
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = Chapter::new("Special Chapter".to_string(), 1, vec![lesson1]).unwrap();
            let chapter_to_delete = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2]);

            let updated_course = course.delete_chapter(&chapter_to_delete).unwrap();

            assert_eq!(
                updated_course.chapters()[0].name().as_str(),
                "Special Chapter"
            );
            assert_eq!(updated_course.chapters()[0].lesson_quantity(), 1);
            assert_eq!(
                updated_course.chapters()[0]
                    .total_duration()
                    .total_seconds(),
                3600
            );
        }

        #[test]
        fn test_delete_chapter_multiple_deletions_sequential() {
            let chapter1 = create_test_chapter("Chapter 1", 0);
            let chapter2 = create_test_chapter("Chapter 2", 1);
            let chapter3 = create_test_chapter("Chapter 3", 2);
            let chapter1_to_delete = chapter1.clone();
            let chapter2_to_delete = chapter2.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            let course_after_first = course.delete_chapter(&chapter1_to_delete).unwrap();
            let course_after_second = course_after_first
                .delete_chapter(&chapter2_to_delete)
                .unwrap();

            assert_eq!(course_after_second.chapter_quantity(), 1);
            assert_eq!(
                course_after_second.chapters()[0].name().as_str(),
                "Chapter 3"
            );
            assert_eq!(course_after_second.chapters()[0].index().value(), 0);
        }

        #[test]
        fn test_delete_chapter_recalculates_duration_correctly() {
            let lesson1 = create_test_lesson("Lesson One", 1000, 0);
            let lesson2 = create_test_lesson("Lesson Two", 2000, 0);
            let lesson3 = create_test_lesson("Lesson Three", 3000, 0);
            let chapter1 = Chapter::new("Chapter 1".to_string(), 0, vec![lesson1]).unwrap();
            let chapter2 = Chapter::new("Chapter 2".to_string(), 1, vec![lesson2]).unwrap();
            let chapter3 = Chapter::new("Chapter 3".to_string(), 2, vec![lesson3]).unwrap();
            let course = create_test_course("Test Course", vec![chapter1, chapter2, chapter3]);

            assert_eq!(course.duration().total_seconds(), 6000);

            let chapter2_ref = &course.chapters()[1].clone();
            let updated_course = course.delete_chapter(chapter2_ref).unwrap();

            assert_eq!(updated_course.duration().total_seconds(), 4000);
        }

        #[test]
        fn test_delete_chapter_with_multiple_lessons_updates_count() {
            let lessons1 = vec![
                create_test_lesson("Lesson 1", 1800, 0),
                create_test_lesson("Lesson 2", 1800, 1),
                create_test_lesson("Lesson 3", 1800, 2),
            ];
            let lessons2 = vec![
                create_test_lesson("Lesson 4", 1800, 0),
                create_test_lesson("Lesson 5", 1800, 1),
            ];
            let chapter1 = Chapter::new("Chapter 1".to_string(), 0, lessons1).unwrap();
            let chapter2 = Chapter::new("Chapter 2".to_string(), 1, lessons2).unwrap();
            let chapter_to_delete = chapter1.clone();
            let course = create_test_course("Test Course", vec![chapter1, chapter2]);

            assert_eq!(course.number_of_lessons(), 5);

            let updated_course = course.delete_chapter(&chapter_to_delete).unwrap();

            assert_eq!(updated_course.number_of_lessons(), 2);
        }
    }
}