use crate::{Course, CourseEnded, CourseProgress, CourseProgressError, Lesson, LessonProgress};
use education_platform_common::{DomainEventDispatcher, Entity, Id};
use std::sync::Arc;

/// Service for creating and synchronizing course progress.
///
/// This service handles the creation of new course progress records
/// and synchronization of existing progress when course content changes.
///
/// # Examples
///
/// ```
/// use education_platform_core::{Course, Chapter, Lesson, CreateCourseProgress};
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
/// let service = CreateCourseProgress::new(course);
/// let progress = service.new_progress("user@example.com".to_string()).unwrap();
///
/// assert_eq!(progress.course_name().as_str(), "Rust Programming");
/// assert_eq!(progress.lesson_progress().len(), 1);
/// ```
pub struct CreateCourseProgress {
    course: Course,
    event_dispatcher: Arc<DomainEventDispatcher<CourseEnded>>,
}

impl CreateCourseProgress {
    /// Creates a new service instance for the given course.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Course, Chapter, Lesson, CreateCourseProgress};
    ///
    /// let lesson = Lesson::new(
    ///     "Intro".to_string(),
    ///     1800,
    ///     "https://example.com/intro.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let chapter = Chapter::new(
    ///     "Chapter 1".to_string(),
    ///     0,
    ///     vec![lesson],
    /// ).unwrap();
    ///
    /// let course = Course::new(
    ///     "My Course".to_string(),
    ///     None,
    ///     0,
    ///     vec![chapter],
    /// ).unwrap();
    ///
    /// let service = CreateCourseProgress::new(course);
    /// ```
    #[must_use]
    pub fn new(course: Course) -> Self {
        Self {
            course,
            event_dispatcher: Arc::new(DomainEventDispatcher::new()),
        }
    }

    /// Creates a new service instance with a custom event dispatcher.
    #[must_use]
    pub fn with_dispatcher(
        course: Course,
        event_dispatcher: Arc<DomainEventDispatcher<CourseEnded>>,
    ) -> Self {
        Self {
            course,
            event_dispatcher,
        }
    }

    /// Creates a new course progress for a user starting the course.
    ///
    /// This creates fresh lesson progress records for all lessons in the course,
    /// with no start or end dates (user hasn't begun any lessons yet).
    ///
    /// # Errors
    ///
    /// Returns `CourseProgressError` if:
    /// - The email validation fails
    /// - The course has no lessons
    /// - Lesson progress creation fails
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{Course, Chapter, Lesson, CreateCourseProgress};
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
    ///     "Rust Course".to_string(),
    ///     None,
    ///     0,
    ///     vec![chapter],
    /// ).unwrap();
    ///
    /// let service = CreateCourseProgress::new(course);
    /// let progress = service.new_progress("student@example.com".to_string()).unwrap();
    ///
    /// assert!(!progress.selected_lesson().has_started());
    /// ```
    pub fn new_progress(&self, email: String) -> Result<CourseProgress, CourseProgressError> {
        let lessons = self.create_lesson_progress_list()?;

        CourseProgress::builder()
            .course_name(self.course.name().as_str())
            .user_email(email)
            .lessons(lessons)
            .event_dispatcher(Arc::clone(&self.event_dispatcher))
            .build()
    }

    /// Synchronizes existing progress with the current course structure.
    ///
    /// This method preserves the user's progress when course content changes:
    /// - Existing lesson progress is preserved for lessons that still exist
    /// - New lessons added to the course get fresh progress records
    /// - The user's email and selected lesson are preserved when possible
    ///
    /// # Errors
    ///
    /// Returns `CourseProgressError` if:
    /// - The course has no lessons
    /// - Lesson progress creation fails
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::{
    ///     Course, Chapter, Lesson, CreateCourseProgress, CourseEnded, LessonProgress,
    /// };
    /// use education_platform_common::DomainEventDispatcher;
    /// use std::sync::Arc;
    ///
    /// // Create initial course and progress
    /// let lesson = Lesson::new(
    ///     "Intro".to_string(),
    ///     1800,
    ///     "https://example.com/intro.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let chapter = Chapter::new(
    ///     "Chapter 1".to_string(),
    ///     0,
    ///     vec![lesson],
    /// ).unwrap();
    ///
    /// let course = Course::new(
    ///     "My Course".to_string(),
    ///     None,
    ///     0,
    ///     vec![chapter],
    /// ).unwrap();
    ///
    /// let service = CreateCourseProgress::new(course.clone());
    /// let mut progress = service.new_progress("user@example.com".to_string()).unwrap();
    ///
    /// // User starts the lesson
    /// progress.start_selected_lesson();
    ///
    /// // Course is updated (same structure for this example)
    /// let synced = service.sync_with(&progress).unwrap();
    ///
    /// // Progress is preserved
    /// assert!(synced.selected_lesson().has_started());
    /// ```
    pub fn sync_with(
        &self,
        current_progress: &CourseProgress,
    ) -> Result<CourseProgress, CourseProgressError> {
        let lessons = self.sync_lesson_progress(current_progress)?;
        let selected_lesson_id = self.find_selected_lesson_id(current_progress, &lessons);

        let mut builder = CourseProgress::builder()
            .course_name(self.course.name().as_str())
            .user_email(current_progress.user_email().address())
            .lessons(lessons)
            .event_dispatcher(Arc::clone(&self.event_dispatcher));

        if let Some(creation_date) = current_progress.creation_date() {
            builder = builder.creation_date(creation_date);
        }

        if let Some(end_date) = current_progress.end_date() {
            builder = builder.end_date(end_date);
        }

        if let Some(id) = selected_lesson_id {
            builder = builder.selected_lesson_id(id);
        }

        builder.build()
    }

    fn create_lesson_progress_list(&self) -> Result<Vec<LessonProgress>, CourseProgressError> {
        let course_lessons = self.course.lessons()?;

        course_lessons
            .iter()
            .map(|lesson| self.lesson_to_progress(lesson))
            .collect()
    }

    fn sync_lesson_progress(
        &self,
        current_progress: &CourseProgress,
    ) -> Result<Vec<LessonProgress>, CourseProgressError> {
        let course_lessons = self.course.lessons()?;

        course_lessons
            .iter()
            .map(|lesson| {
                self.find_existing_progress(lesson, current_progress)
                    .map_or_else(|| self.lesson_to_progress(lesson), Ok)
            })
            .collect()
    }

    fn lesson_to_progress(&self, lesson: &Lesson) -> Result<LessonProgress, CourseProgressError> {
        LessonProgress::new(
            lesson.name().as_str().to_string(),
            lesson.duration().total_seconds(),
            None,
            None,
        )
        .map_err(CourseProgressError::from)
    }

    fn find_existing_progress(
        &self,
        lesson: &Lesson,
        current_progress: &CourseProgress,
    ) -> Option<LessonProgress> {
        current_progress
            .lesson_progress()
            .iter()
            .find(|lp| lp.lesson_name().as_str() == lesson.name().as_str())
            .cloned()
    }

    fn find_selected_lesson_id(
        &self,
        current_progress: &CourseProgress,
        new_lessons: &[LessonProgress],
    ) -> Option<Id> {
        let current_selected_name = current_progress.selected_lesson().lesson_name().as_str();

        new_lessons
            .iter()
            .find(|lp| lp.lesson_name().as_str() == current_selected_name)
            .map(|lp| lp.id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Chapter;

    fn create_test_lesson(name: &str, duration: u64, index: usize) -> Lesson {
        Lesson::new(
            name.to_string(),
            duration,
            format!("https://example.com/{index}.mp4"),
            index,
        )
        .unwrap()
    }

    fn create_test_chapter(name: &str, index: usize, lessons: Vec<Lesson>) -> Chapter {
        Chapter::new(name.to_string(), index, lessons).unwrap()
    }

    fn create_test_course(name: &str, chapters: Vec<Chapter>) -> Course {
        Course::new(name.to_string(), None, 0, chapters).unwrap()
    }

    mod new_progress {
        use super::*;

        #[test]
        fn test_creates_progress_with_all_lessons() {
            let lessons = vec![
                create_test_lesson("Lesson 1", 1800, 0),
                create_test_lesson("Lesson 2", 2400, 1),
            ];
            let chapter = create_test_chapter("Chapter 1", 0, lessons);
            let course = create_test_course("Test Course", vec![chapter]);

            let service = CreateCourseProgress::new(course);
            let progress = service
                .new_progress("user@example.com".to_string())
                .unwrap();

            assert_eq!(progress.lesson_progress().len(), 2);
            assert_eq!(progress.lesson_progress()[0].lesson_name().as_str(), "Lesson 1");
            assert_eq!(progress.lesson_progress()[1].lesson_name().as_str(), "Lesson 2");
        }

        #[test]
        fn test_lessons_have_correct_duration() {
            let lesson = create_test_lesson("Intro", 3600, 0);
            let chapter = create_test_chapter("Chapter 1", 0, vec![lesson]);
            let course = create_test_course("Test Course", vec![chapter]);

            let service = CreateCourseProgress::new(course);
            let progress = service
                .new_progress("user@example.com".to_string())
                .unwrap();

            assert_eq!(progress.lesson_progress()[0].duration().total_seconds(), 3600);
        }

        #[test]
        fn test_lessons_are_not_started() {
            let lesson = create_test_lesson("Intro", 1800, 0);
            let chapter = create_test_chapter("Chapter 1", 0, vec![lesson]);
            let course = create_test_course("Test Course", vec![chapter]);

            let service = CreateCourseProgress::new(course);
            let progress = service
                .new_progress("user@example.com".to_string())
                .unwrap();

            assert!(!progress.lesson_progress()[0].has_started());
            assert!(!progress.lesson_progress()[0].has_ended());
        }

        #[test]
        fn test_uses_course_name() {
            let lesson = create_test_lesson("Intro", 1800, 0);
            let chapter = create_test_chapter("Chapter 1", 0, vec![lesson]);
            let course = create_test_course("Rust Programming", vec![chapter]);

            let service = CreateCourseProgress::new(course);
            let progress = service
                .new_progress("user@example.com".to_string())
                .unwrap();

            assert_eq!(progress.course_name().as_str(), "Rust Programming");
        }

        #[test]
        fn test_uses_provided_email() {
            let lesson = create_test_lesson("Intro", 1800, 0);
            let chapter = create_test_chapter("Chapter 1", 0, vec![lesson]);
            let course = create_test_course("Test Course", vec![chapter]);

            let service = CreateCourseProgress::new(course);
            let progress = service
                .new_progress("student@university.edu".to_string())
                .unwrap();

            assert_eq!(progress.user_email().address(), "student@university.edu");
        }

        #[test]
        fn test_selects_first_lesson_by_default() {
            let lessons = vec![
                create_test_lesson("First Lesson", 1800, 0),
                create_test_lesson("Second Lesson", 2400, 1),
            ];
            let chapter = create_test_chapter("Chapter 1", 0, lessons);
            let course = create_test_course("Test Course", vec![chapter]);

            let service = CreateCourseProgress::new(course);
            let progress = service
                .new_progress("user@example.com".to_string())
                .unwrap();

            assert_eq!(progress.selected_lesson().lesson_name().as_str(), "First Lesson");
        }

        #[test]
        fn test_invalid_email_returns_error() {
            let lesson = create_test_lesson("Intro", 1800, 0);
            let chapter = create_test_chapter("Chapter 1", 0, vec![lesson]);
            let course = create_test_course("Test Course", vec![chapter]);

            let service = CreateCourseProgress::new(course);
            let result = service.new_progress("invalid-email".to_string());

            assert!(result.is_err());
        }

        #[test]
        fn test_handles_multiple_chapters() {
            let lessons1 = vec![create_test_lesson("Ch1 Lesson", 1800, 0)];
            let lessons2 = vec![create_test_lesson("Ch2 Lesson", 2400, 0)];
            let chapters = vec![
                create_test_chapter("Chapter 1", 0, lessons1),
                create_test_chapter("Chapter 2", 1, lessons2),
            ];
            let course = create_test_course("Multi-Chapter Course", chapters);

            let service = CreateCourseProgress::new(course);
            let progress = service
                .new_progress("user@example.com".to_string())
                .unwrap();

            assert_eq!(progress.lesson_progress().len(), 2);
        }
    }

    mod sync_with {
        use super::*;

        #[test]
        fn test_preserves_user_email() {
            let lesson = create_test_lesson("Intro", 1800, 0);
            let chapter = create_test_chapter("Chapter 1", 0, vec![lesson]);
            let course = create_test_course("Test Course", vec![chapter]);

            let service = CreateCourseProgress::new(course);
            let original = service
                .new_progress("original@example.com".to_string())
                .unwrap();

            let synced = service.sync_with(&original).unwrap();

            assert_eq!(synced.user_email().address(), "original@example.com");
        }

        #[test]
        fn test_preserves_started_lesson_progress() {
            let lesson = create_test_lesson("Intro", 1800, 0);
            let chapter = create_test_chapter("Chapter 1", 0, vec![lesson]);
            let course = create_test_course("Test Course", vec![chapter]);

            let service = CreateCourseProgress::new(course);
            let mut progress = service
                .new_progress("user@example.com".to_string())
                .unwrap();
            progress.start_selected_lesson();

            let synced = service.sync_with(&progress).unwrap();

            assert!(synced.selected_lesson().has_started());
        }

        #[test]
        fn test_preserves_completed_lesson_progress() {
            let lesson = create_test_lesson("Intro", 1800, 0);
            let chapter = create_test_chapter("Chapter 1", 0, vec![lesson]);
            let course = create_test_course("Test Course", vec![chapter]);

            let service = CreateCourseProgress::new(course);
            let mut progress = service
                .new_progress("user@example.com".to_string())
                .unwrap();
            progress.start_selected_lesson();
            progress.end_selected_lesson().unwrap();

            let synced = service.sync_with(&progress).unwrap();

            assert!(synced.selected_lesson().is_completed());
        }

        #[test]
        fn test_preserves_selected_lesson() {
            let lessons = vec![
                create_test_lesson("Lesson 1", 1800, 0),
                create_test_lesson("Lesson 2", 2400, 1),
            ];
            let chapter = create_test_chapter("Chapter 1", 0, lessons);
            let course = create_test_course("Test Course", vec![chapter]);

            let service = CreateCourseProgress::new(course);
            let mut progress = service
                .new_progress("user@example.com".to_string())
                .unwrap();
            progress.select_next_lesson();

            let synced = service.sync_with(&progress).unwrap();

            assert_eq!(synced.selected_lesson().lesson_name().as_str(), "Lesson 2");
        }
    }

    mod observer_pattern {
        use super::*;
        use std::sync::Mutex;

        #[test]
        fn test_event_dispatcher_receives_course_ended_event() {
            let lesson = create_test_lesson("Intro", 1800, 0);
            let chapter = create_test_chapter("Chapter 1", 0, vec![lesson]);
            let course = create_test_course("Test Course", vec![chapter]);

            let dispatcher = Arc::new(DomainEventDispatcher::<CourseEnded>::new());
            let received_events = Arc::new(Mutex::new(Vec::new()));

            let events_clone = Arc::clone(&received_events);
            dispatcher.subscribe(move |event: &CourseEnded| {
                events_clone.lock().unwrap().push(event.clone());
            });

            let service = CreateCourseProgress::with_dispatcher(course, Arc::clone(&dispatcher));
            let mut progress = service
                .new_progress("student@example.com".to_string())
                .unwrap();

            progress.start_selected_lesson();
            progress.end_selected_lesson().unwrap();
            progress.publish_ended();

            let events = received_events.lock().unwrap();
            assert_eq!(events.len(), 1);
            assert_eq!(events[0].user_email().address(), "student@example.com");
            assert_eq!(events[0].course_id(), progress.id());
        }

        #[test]
        fn test_multiple_observers_receive_event() {
            let lesson = create_test_lesson("Intro", 1800, 0);
            let chapter = create_test_chapter("Chapter 1", 0, vec![lesson]);
            let course = create_test_course("Test Course", vec![chapter]);

            let dispatcher = Arc::new(DomainEventDispatcher::<CourseEnded>::new());

            let observer1_events = Arc::new(Mutex::new(Vec::new()));
            let observer2_events = Arc::new(Mutex::new(Vec::new()));

            let obs1_clone = Arc::clone(&observer1_events);
            dispatcher.subscribe(move |event: &CourseEnded| {
                obs1_clone.lock().unwrap().push(event.clone());
            });

            let obs2_clone = Arc::clone(&observer2_events);
            dispatcher.subscribe(move |event: &CourseEnded| {
                obs2_clone.lock().unwrap().push(event.clone());
            });

            let service = CreateCourseProgress::with_dispatcher(course, Arc::clone(&dispatcher));
            let mut progress = service
                .new_progress("user@example.com".to_string())
                .unwrap();

            progress.start_selected_lesson();
            progress.end_selected_lesson().unwrap();
            progress.publish_ended();

            assert_eq!(observer1_events.lock().unwrap().len(), 1);
            assert_eq!(observer2_events.lock().unwrap().len(), 1);
        }

        #[test]
        fn test_synced_progress_uses_same_dispatcher() {
            let lesson = create_test_lesson("Intro", 1800, 0);
            let chapter = create_test_chapter("Chapter 1", 0, vec![lesson]);
            let course = create_test_course("Test Course", vec![chapter]);

            let dispatcher = Arc::new(DomainEventDispatcher::<CourseEnded>::new());
            let received_events = Arc::new(Mutex::new(Vec::new()));

            let events_clone = Arc::clone(&received_events);
            dispatcher.subscribe(move |event: &CourseEnded| {
                events_clone.lock().unwrap().push(event.clone());
            });

            let service = CreateCourseProgress::with_dispatcher(course, Arc::clone(&dispatcher));
            let mut progress = service
                .new_progress("user@example.com".to_string())
                .unwrap();

            progress.start_selected_lesson();
            let mut synced = service.sync_with(&progress).unwrap();

            synced.end_selected_lesson().unwrap();
            synced.publish_ended();

            let events = received_events.lock().unwrap();
            assert_eq!(events.len(), 1);
        }

        #[test]
        fn test_no_event_published_without_explicit_call() {
            let lesson = create_test_lesson("Intro", 1800, 0);
            let chapter = create_test_chapter("Chapter 1", 0, vec![lesson]);
            let course = create_test_course("Test Course", vec![chapter]);

            let dispatcher = Arc::new(DomainEventDispatcher::<CourseEnded>::new());
            let received_events = Arc::new(Mutex::new(Vec::new()));

            let events_clone = Arc::clone(&received_events);
            dispatcher.subscribe(move |event: &CourseEnded| {
                events_clone.lock().unwrap().push(event.clone());
            });

            let service = CreateCourseProgress::with_dispatcher(course, Arc::clone(&dispatcher));
            let mut progress = service
                .new_progress("user@example.com".to_string())
                .unwrap();

            progress.start_selected_lesson();
            progress.end_selected_lesson().unwrap();

            // Event is NOT published automatically - must be explicit
            let events = received_events.lock().unwrap();
            assert_eq!(events.len(), 0);
        }
    }
}
