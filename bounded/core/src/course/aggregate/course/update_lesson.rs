use crate::{Chapter, Course, CourseError, Lesson};
use education_platform_common::{Duration, Entity};

impl Course {
    /// Updates an existing lesson in the course by replacing it with a new version.
    ///
    /// This method searches for a lesson with the same ID as the provided lesson
    /// across all chapters and replaces it. The course's total duration and lesson
    /// count are recalculated after the update.
    ///
    /// # Arguments
    ///
    /// * `lesson` - The updated lesson to replace the existing one (matched by ID)
    ///
    /// # Errors
    ///
    /// Returns `CourseError::LessonDoesNotExist` if no lesson with the given ID is found.
    /// Returns `CourseError::ChapterError` if chapter recreation fails during the update.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Course, Chapter, Lesson};
    /// use education_platform_common::Entity;
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
    /// let course = Course::new(
    ///     "Rust Programming".to_string(),
    ///     None,
    ///     0,
    ///     vec![chapter],
    /// ).unwrap();
    ///
    /// let original_lesson = &course.chapters()[0].lessons()[0];
    /// let updated_lesson = original_lesson.with_name("Introduction Updated".to_string()).unwrap();
    ///
    /// let updated_course = course.update_lesson(updated_lesson).unwrap();
    /// assert_eq!(updated_course.chapters()[0].lessons()[0].name().as_str(), "Introduction Updated");
    /// ```
    pub fn update_lesson(&self, lesson: Lesson) -> Result<Self, CourseError> {
        let chapter_with_lesson = self
            .chapters
            .iter()
            .find(|chapter| chapter.lessons().iter().any(|l| l.id() == lesson.id()))
            .ok_or(CourseError::LessonDoesNotExist)?;

        let chapters: Result<Vec<Chapter>, CourseError> = self
            .chapters
            .iter()
            .map(|chapter| {
                if chapter.index() != chapter_with_lesson.index() {
                    return Ok(chapter.clone());
                }

                let updated_lessons: Vec<Lesson> = chapter
                    .lessons()
                    .iter()
                    .map(|existing_lesson| {
                        if existing_lesson.id() == lesson.id() {
                            lesson.clone()
                        } else {
                            existing_lesson.clone()
                        }
                    })
                    .collect();

                Chapter::new(chapter.name().to_string(), chapter.index().value(), updated_lessons)
                    .map_err(CourseError::from)
            })
            .collect();

        let chapters = chapters?;
        let (duration, number_of_lessons) = Self::calculate_totals(&chapters, Duration::default());

        Ok(Self {
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

    fn create_test_lesson(name: &str, duration: u64, index: usize) -> Lesson {
        Lesson::new(
            name.to_string(),
            duration,
            format!("https://example.com/{}.mp4", index),
            index,
        )
        .unwrap()
    }

    fn create_test_chapter(name: &str, index: usize, lessons: Vec<Lesson>) -> Chapter {
        Chapter::new(name.to_string(), index, lessons).unwrap()
    }

    mod update_lesson {
        use super::*;

        #[test]
        fn test_update_lesson_updates_name() {
            let lesson = create_test_lesson("Original", 1800, 0);
            let chapter = create_test_chapter("Chapter One", 0, vec![lesson]);
            let course = Course::new("Test Course".to_string(), None, 0, vec![chapter]).unwrap();

            let original_lesson = &course.chapters()[0].lessons()[0];
            let updated_lesson = original_lesson
                .with_name("Updated Name".to_string())
                .unwrap();

            let updated_course = course.update_lesson(updated_lesson).unwrap();

            assert_eq!(
                updated_course.chapters()[0].lessons()[0].name().as_str(),
                "Updated Name"
            );
        }

        #[test]
        fn test_update_lesson_updates_duration() {
            let lesson = create_test_lesson("Lesson", 1800, 0);
            let chapter = create_test_chapter("Chapter One", 0, vec![lesson]);
            let course = Course::new("Test Course".to_string(), None, 0, vec![chapter]).unwrap();

            let original_lesson = &course.chapters()[0].lessons()[0];
            let updated_lesson = original_lesson.with_duration(3600);

            let updated_course = course.update_lesson(updated_lesson).unwrap();

            assert_eq!(
                updated_course.chapters()[0].lessons()[0]
                    .duration()
                    .total_seconds(),
                3600
            );
        }

        #[test]
        fn test_update_lesson_recalculates_course_duration() {
            let lesson1 = create_test_lesson("Lesson 1", 1800, 0);
            let lesson2 = create_test_lesson("Lesson 2", 1200, 1);
            let chapter = create_test_chapter("Chapter One", 0, vec![lesson1, lesson2]);
            let course = Course::new("Test Course".to_string(), None, 0, vec![chapter]).unwrap();

            assert_eq!(course.duration().total_seconds(), 3000);

            let original_lesson = &course.chapters()[0].lessons()[0];
            let updated_lesson = original_lesson.with_duration(3600);

            let updated_course = course.update_lesson(updated_lesson).unwrap();

            assert_eq!(updated_course.duration().total_seconds(), 4800);
        }

        #[test]
        fn test_update_lesson_in_second_chapter() {
            let lesson1 = create_test_lesson("Lesson 1", 1800, 0);
            let lesson2 = create_test_lesson("Lesson 2", 1200, 0);
            let chapter1 = create_test_chapter("Chapter One", 0, vec![lesson1]);
            let chapter2 = create_test_chapter("Chapter Two", 1, vec![lesson2]);
            let course =
                Course::new("Test Course".to_string(), None, 0, vec![chapter1, chapter2]).unwrap();

            let original_lesson = &course.chapters()[1].lessons()[0];
            let updated_lesson = original_lesson
                .with_name("Updated Lesson 2".to_string())
                .unwrap();

            let updated_course = course.update_lesson(updated_lesson).unwrap();

            assert_eq!(
                updated_course.chapters()[1].lessons()[0].name().as_str(),
                "Updated Lesson 2"
            );
        }

        #[test]
        fn test_update_lesson_preserves_course_id() {
            let lesson = create_test_lesson("Lesson", 1800, 0);
            let chapter = create_test_chapter("Chapter One", 0, vec![lesson]);
            let course = Course::new("Test Course".to_string(), None, 0, vec![chapter]).unwrap();
            let original_id = course.id();

            let original_lesson = &course.chapters()[0].lessons()[0];
            let updated_lesson = original_lesson.with_name("Updated".to_string()).unwrap();

            let updated_course = course.update_lesson(updated_lesson).unwrap();

            assert_eq!(updated_course.id(), original_id);
        }

        #[test]
        fn test_update_lesson_preserves_course_name() {
            let lesson = create_test_lesson("Lesson", 1800, 0);
            let chapter = create_test_chapter("Chapter One", 0, vec![lesson]);
            let course = Course::new("My Course Name".to_string(), None, 0, vec![chapter]).unwrap();

            let original_lesson = &course.chapters()[0].lessons()[0];
            let updated_lesson = original_lesson.with_name("Updated".to_string()).unwrap();

            let updated_course = course.update_lesson(updated_lesson).unwrap();

            assert_eq!(updated_course.name().as_str(), "My Course Name");
        }

        #[test]
        fn test_update_lesson_preserves_other_lessons() {
            let lesson1 = create_test_lesson("Lesson 1", 1800, 0);
            let lesson2 = create_test_lesson("Lesson 2", 1200, 1);
            let lesson3 = create_test_lesson("Lesson 3", 900, 2);
            let chapter = create_test_chapter("Chapter One", 0, vec![lesson1, lesson2, lesson3]);
            let course = Course::new("Test Course".to_string(), None, 0, vec![chapter]).unwrap();

            let original_lesson = &course.chapters()[0].lessons()[1];
            let updated_lesson = original_lesson
                .with_name("Updated Lesson 2".to_string())
                .unwrap();

            let updated_course = course.update_lesson(updated_lesson).unwrap();

            assert_eq!(updated_course.chapters()[0].lessons()[0].name().as_str(), "Lesson 1");
            assert_eq!(
                updated_course.chapters()[0].lessons()[1].name().as_str(),
                "Updated Lesson 2"
            );
            assert_eq!(updated_course.chapters()[0].lessons()[2].name().as_str(), "Lesson 3");
        }

        #[test]
        fn test_update_lesson_preserves_lesson_id() {
            let lesson = create_test_lesson("Lesson", 1800, 0);
            let chapter = create_test_chapter("Chapter One", 0, vec![lesson]);
            let course = Course::new("Test Course".to_string(), None, 0, vec![chapter]).unwrap();

            let original_lesson = &course.chapters()[0].lessons()[0];
            let original_lesson_id = original_lesson.id();
            let updated_lesson = original_lesson.with_name("Updated".to_string()).unwrap();

            let updated_course = course.update_lesson(updated_lesson).unwrap();

            assert_eq!(updated_course.chapters()[0].lessons()[0].id(), original_lesson_id);
        }

        #[test]
        fn test_update_lesson_nonexistent_returns_error() {
            let lesson = create_test_lesson("Lesson", 1800, 0);
            let chapter = create_test_chapter("Chapter One", 0, vec![lesson]);
            let course = Course::new("Test Course".to_string(), None, 0, vec![chapter]).unwrap();

            let nonexistent_lesson = create_test_lesson("Nonexistent", 1800, 99);

            let result = course.update_lesson(nonexistent_lesson);

            assert!(result.is_err());
            assert!(matches!(result, Err(CourseError::LessonDoesNotExist)));
        }

        #[test]
        fn test_update_lesson_does_not_modify_original_course() {
            let lesson = create_test_lesson("Original", 1800, 0);
            let chapter = create_test_chapter("Chapter One", 0, vec![lesson]);
            let course = Course::new("Test Course".to_string(), None, 0, vec![chapter]).unwrap();

            let original_lesson = &course.chapters()[0].lessons()[0];
            let updated_lesson = original_lesson.with_name("Updated".to_string()).unwrap();

            let _ = course.update_lesson(updated_lesson).unwrap();

            assert_eq!(course.chapters()[0].lessons()[0].name().as_str(), "Original");
        }

        #[test]
        fn test_update_lesson_preserves_chapter_structure() {
            let lesson1 = create_test_lesson("Lesson 1", 1800, 0);
            let lesson2 = create_test_lesson("Lesson 2", 1200, 0);
            let chapter1 = create_test_chapter("Chapter One", 0, vec![lesson1]);
            let chapter2 = create_test_chapter("Chapter Two", 1, vec![lesson2]);
            let course =
                Course::new("Test Course".to_string(), None, 0, vec![chapter1, chapter2]).unwrap();

            let original_lesson = &course.chapters()[0].lessons()[0];
            let updated_lesson = original_lesson.with_name("Updated".to_string()).unwrap();

            let updated_course = course.update_lesson(updated_lesson).unwrap();

            assert_eq!(updated_course.chapter_quantity(), 2);
            assert_eq!(updated_course.chapters()[0].name().as_str(), "Chapter One");
            assert_eq!(updated_course.chapters()[1].name().as_str(), "Chapter Two");
        }

        #[test]
        fn test_update_lesson_preserves_number_of_lessons() {
            let lesson1 = create_test_lesson("Lesson 1", 1800, 0);
            let lesson2 = create_test_lesson("Lesson 2", 1200, 1);
            let chapter = create_test_chapter("Chapter One", 0, vec![lesson1, lesson2]);
            let course = Course::new("Test Course".to_string(), None, 0, vec![chapter]).unwrap();

            let original_lesson = &course.chapters()[0].lessons()[0];
            let updated_lesson = original_lesson.with_name("Updated".to_string()).unwrap();

            let updated_course = course.update_lesson(updated_lesson).unwrap();

            assert_eq!(updated_course.number_of_lessons(), 2);
        }

        #[test]
        fn test_update_lesson_with_multiple_chapters_and_lessons() {
            let lessons1 = vec![
                create_test_lesson("Lesson 1", 1800, 0),
                create_test_lesson("Lesson 2", 1200, 1),
            ];
            let lessons2 = vec![
                create_test_lesson("Lesson 3", 900, 0),
                create_test_lesson("Lesson 4", 600, 1),
            ];
            let chapter1 = create_test_chapter("Chapter One", 0, lessons1);
            let chapter2 = create_test_chapter("Chapter Two", 1, lessons2);
            let course =
                Course::new("Test Course".to_string(), None, 0, vec![chapter1, chapter2]).unwrap();

            let original_lesson = &course.chapters()[1].lessons()[1];
            let updated_lesson = original_lesson
                .with_name("Updated Lesson 4".to_string())
                .unwrap();

            let updated_course = course.update_lesson(updated_lesson).unwrap();

            assert_eq!(
                updated_course.chapters()[1].lessons()[1].name().as_str(),
                "Updated Lesson 4"
            );
            assert_eq!(updated_course.number_of_lessons(), 4);
        }
    }
}
