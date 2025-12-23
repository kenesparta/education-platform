use super::{Lesson, LessonError};
use education_platform_common::{Duration, Index, SimpleName, SimpleNameConfig, Url};

impl Lesson {
    /// Returns a new lesson with an updated name, preserving the original ID.
    ///
    /// # Errors
    ///
    /// Returns `LessonError::NameError` if the name validation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::Lesson;
    /// use education_platform_common::Entity;
    ///
    /// let lesson = Lesson::new(
    ///     "Original Name".to_string(),
    ///     1800,
    ///     "https://example.com/video.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let original_id = lesson.id();
    /// let updated = lesson.with_name("Updated Name".to_string()).unwrap();
    ///
    /// assert_eq!(updated.name().as_str(), "Updated Name");
    /// assert_eq!(updated.id(), original_id);
    /// ```
    pub fn with_name(&self, name: String) -> Result<Self, LessonError> {
        let name = SimpleName::with_config(name, SimpleNameConfig::new(3, 50))?;
        Ok(Self { name, ..self.clone() })
    }

    /// Returns a new lesson with an updated duration, preserving the original ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::Lesson;
    /// use education_platform_common::Entity;
    ///
    /// let lesson = Lesson::new(
    ///     "Introduction".to_string(),
    ///     1800,
    ///     "https://example.com/video.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let original_id = lesson.id();
    /// let updated = lesson.with_duration(3600);
    ///
    /// assert_eq!(updated.duration().total_seconds(), 3600);
    /// assert_eq!(updated.id(), original_id);
    /// ```
    #[inline]
    #[must_use]
    pub fn with_duration(&self, duration_seconds: u64) -> Self {
        Self {
            duration: Duration::from_seconds(duration_seconds),
            ..self.clone()
        }
    }

    /// Returns a new lesson with an updated video URL, preserving the original ID.
    ///
    /// # Errors
    ///
    /// Returns `LessonError::VideoUrlError` if the URL validation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::Lesson;
    /// use education_platform_common::Entity;
    ///
    /// let lesson = Lesson::new(
    ///     "Introduction".to_string(),
    ///     1800,
    ///     "https://example.com/video.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let original_id = lesson.id();
    /// let updated = lesson.with_video_url("https://cdn.example.com/new-video.mp4".to_string()).unwrap();
    ///
    /// assert_eq!(updated.video_url().as_str(), "https://cdn.example.com/new-video.mp4");
    /// assert_eq!(updated.id(), original_id);
    /// ```
    pub fn with_video_url(&self, video_url: String) -> Result<Self, LessonError> {
        let video_url = Url::new(video_url)?;
        Ok(Self {
            video_url,
            ..self.clone()
        })
    }

    /// Returns a new lesson with an updated index, preserving the original ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::Lesson;
    /// use education_platform_common::Entity;
    ///
    /// let lesson = Lesson::new(
    ///     "Introduction".to_string(),
    ///     1800,
    ///     "https://example.com/video.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let original_id = lesson.id();
    /// let updated = lesson.with_index(5);
    ///
    /// assert_eq!(updated.index().value(), 5);
    /// assert_eq!(updated.id(), original_id);
    /// ```
    #[inline]
    #[must_use]
    pub fn with_index(&self, index: usize) -> Self {
        Self {
            index: Index::new(index),
            ..self.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use education_platform_common::Entity;

    fn create_test_lesson(name: &str, duration: u64, index: usize) -> Lesson {
        Lesson::new(
            name.to_string(),
            duration,
            format!("https://example.com/{index}.mp4"),
            index,
        )
        .unwrap()
    }

    mod with_name {
        use super::*;

        #[test]
        fn test_with_name_updates_name() {
            let lesson = create_test_lesson("Original", 1800, 0);

            let updated = lesson.with_name("Updated".to_string()).unwrap();

            assert_eq!(updated.name().as_str(), "Updated");
        }

        #[test]
        fn test_with_name_preserves_id() {
            let lesson = create_test_lesson("Original", 1800, 0);
            let original_id = lesson.id();

            let updated = lesson.with_name("Updated".to_string()).unwrap();

            assert_eq!(updated.id(), original_id);
        }

        #[test]
        fn test_with_name_preserves_duration() {
            let lesson = create_test_lesson("Original", 3600, 0);

            let updated = lesson.with_name("Updated".to_string()).unwrap();

            assert_eq!(updated.duration().total_seconds(), 3600);
        }

        #[test]
        fn test_with_name_invalid_returns_error() {
            let lesson = create_test_lesson("Original", 1800, 0);

            let result = lesson.with_name("AB".to_string());

            assert!(result.is_err());
            assert!(matches!(result, Err(LessonError::NameError(_))));
        }
    }

    mod with_duration {
        use super::*;

        #[test]
        fn test_with_duration_updates_duration() {
            let lesson = create_test_lesson("Lesson", 1800, 0);

            let updated = lesson.with_duration(3600);

            assert_eq!(updated.duration().total_seconds(), 3600);
        }

        #[test]
        fn test_with_duration_preserves_id() {
            let lesson = create_test_lesson("Lesson", 1800, 0);
            let original_id = lesson.id();

            let updated = lesson.with_duration(3600);

            assert_eq!(updated.id(), original_id);
        }

        #[test]
        fn test_with_duration_preserves_name() {
            let lesson = create_test_lesson("My Lesson Name", 1800, 0);

            let updated = lesson.with_duration(3600);

            assert_eq!(updated.name().as_str(), "My Lesson Name");
        }
    }

    mod with_video_url {
        use super::*;

        #[test]
        fn test_with_video_url_updates_url() {
            let lesson = create_test_lesson("Lesson", 1800, 0);

            let updated = lesson
                .with_video_url("https://example.com/new.mp4".to_string())
                .unwrap();

            assert_eq!(updated.video_url().as_str(), "https://example.com/new.mp4");
        }

        #[test]
        fn test_with_video_url_preserves_id() {
            let lesson = create_test_lesson("Lesson", 1800, 0);
            let original_id = lesson.id();

            let updated = lesson
                .with_video_url("https://example.com/new.mp4".to_string())
                .unwrap();

            assert_eq!(updated.id(), original_id);
        }

        #[test]
        fn test_with_video_url_invalid_returns_error() {
            let lesson = create_test_lesson("Lesson", 1800, 0);

            let result = lesson.with_video_url("not-a-url".to_string());

            assert!(result.is_err());
            assert!(matches!(result, Err(LessonError::VideoUrlError(_))));
        }
    }

    mod with_index {
        use super::*;

        #[test]
        fn test_with_index_updates_index() {
            let lesson = create_test_lesson("Lesson", 1800, 0);

            let updated = lesson.with_index(5);

            assert_eq!(updated.index().value(), 5);
        }

        #[test]
        fn test_with_index_preserves_id() {
            let lesson = create_test_lesson("Lesson", 1800, 0);
            let original_id = lesson.id();

            let updated = lesson.with_index(5);

            assert_eq!(updated.id(), original_id);
        }

        #[test]
        fn test_with_index_preserves_name() {
            let lesson = create_test_lesson("My Lesson", 1800, 0);

            let updated = lesson.with_index(5);

            assert_eq!(updated.name().as_str(), "My Lesson");
        }
    }
}
