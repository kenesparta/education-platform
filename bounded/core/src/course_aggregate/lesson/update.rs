use super::{Lesson, LessonError};
use education_platform_common::{Duration, Index, SimpleName, SimpleNameConfig, Url};

impl Lesson {
    /// Updates the lesson name in place.
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
    /// let mut lesson = Lesson::new(
    ///     "Original Name".to_string(),
    ///     1800,
    ///     "https://example.com/video.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let original_id = lesson.id();
    /// lesson.update_name("Updated Name".to_string()).unwrap();
    ///
    /// assert_eq!(lesson.name().as_str(), "Updated Name");
    /// assert_eq!(lesson.id(), original_id);
    /// ```
    pub fn update_name(&mut self, name: String) -> Result<(), LessonError> {
        self.name = SimpleName::with_config(name, SimpleNameConfig::new(3, 50))?;
        Ok(())
    }

    /// Updates the lesson duration in place.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::Lesson;
    /// use education_platform_common::Entity;
    ///
    /// let mut lesson = Lesson::new(
    ///     "Introduction".to_string(),
    ///     1800,
    ///     "https://example.com/video.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let original_id = lesson.id();
    /// lesson.update_duration(3600);
    ///
    /// assert_eq!(lesson.duration().total_seconds(), 3600);
    /// assert_eq!(lesson.id(), original_id);
    /// ```
    #[inline]
    pub fn update_duration(&mut self, duration_seconds: u64) {
        self.duration = Duration::from_seconds(duration_seconds);
    }

    /// Updates the lesson video URL in place.
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
    /// let mut lesson = Lesson::new(
    ///     "Introduction".to_string(),
    ///     1800,
    ///     "https://example.com/video.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let original_id = lesson.id();
    /// lesson.update_video_url("https://cdn.example.com/new-video.mp4".to_string()).unwrap();
    ///
    /// assert_eq!(lesson.video_url().as_str(), "https://cdn.example.com/new-video.mp4");
    /// assert_eq!(lesson.id(), original_id);
    /// ```
    pub fn update_video_url(&mut self, video_url: String) -> Result<(), LessonError> {
        self.video_url = Url::new(video_url)?;
        Ok(())
    }

    /// Updates the lesson index in place.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::Lesson;
    /// use education_platform_common::Entity;
    ///
    /// let mut lesson = Lesson::new(
    ///     "Introduction".to_string(),
    ///     1800,
    ///     "https://example.com/video.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// let original_id = lesson.id();
    /// lesson.update_index(5);
    ///
    /// assert_eq!(lesson.index().value(), 5);
    /// assert_eq!(lesson.id(), original_id);
    /// ```
    #[inline]
    pub fn update_index(&mut self, index: usize) {
        self.index = Index::new(index);
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

    mod update_name {
        use super::*;

        #[test]
        fn test_update_name_modifies_name() {
            let mut lesson = create_test_lesson("Original", 1800, 0);

            lesson.update_name("Updated".to_string()).unwrap();

            assert_eq!(lesson.name().as_str(), "Updated");
        }

        #[test]
        fn test_update_name_preserves_id() {
            let mut lesson = create_test_lesson("Original", 1800, 0);
            let original_id = lesson.id();

            lesson.update_name("Updated".to_string()).unwrap();

            assert_eq!(lesson.id(), original_id);
        }

        #[test]
        fn test_update_name_preserves_duration() {
            let mut lesson = create_test_lesson("Original", 3600, 0);

            lesson.update_name("Updated".to_string()).unwrap();

            assert_eq!(lesson.duration().total_seconds(), 3600);
        }

        #[test]
        fn test_update_name_invalid_returns_error() {
            let mut lesson = create_test_lesson("Original", 1800, 0);

            let result = lesson.update_name("AB".to_string());

            assert!(result.is_err());
            assert!(matches!(result, Err(LessonError::NameError(_))));
        }

        #[test]
        fn test_update_name_invalid_preserves_original() {
            let mut lesson = create_test_lesson("Original", 1800, 0);

            let _ = lesson.update_name("AB".to_string());

            assert_eq!(lesson.name().as_str(), "Original");
        }
    }

    mod update_duration {
        use super::*;

        #[test]
        fn test_update_duration_modifies_duration() {
            let mut lesson = create_test_lesson("Lesson", 1800, 0);

            lesson.update_duration(3600);

            assert_eq!(lesson.duration().total_seconds(), 3600);
        }

        #[test]
        fn test_update_duration_preserves_id() {
            let mut lesson = create_test_lesson("Lesson", 1800, 0);
            let original_id = lesson.id();

            lesson.update_duration(3600);

            assert_eq!(lesson.id(), original_id);
        }

        #[test]
        fn test_update_duration_preserves_name() {
            let mut lesson = create_test_lesson("My Lesson Name", 1800, 0);

            lesson.update_duration(3600);

            assert_eq!(lesson.name().as_str(), "My Lesson Name");
        }
    }

    mod update_video_url {
        use super::*;

        #[test]
        fn test_update_video_url_modifies_url() {
            let mut lesson = create_test_lesson("Lesson", 1800, 0);

            lesson
                .update_video_url("https://example.com/new.mp4".to_string())
                .unwrap();

            assert_eq!(lesson.video_url().as_str(), "https://example.com/new.mp4");
        }

        #[test]
        fn test_update_video_url_preserves_id() {
            let mut lesson = create_test_lesson("Lesson", 1800, 0);
            let original_id = lesson.id();

            lesson
                .update_video_url("https://example.com/new.mp4".to_string())
                .unwrap();

            assert_eq!(lesson.id(), original_id);
        }

        #[test]
        fn test_update_video_url_invalid_returns_error() {
            let mut lesson = create_test_lesson("Lesson", 1800, 0);

            let result = lesson.update_video_url("not-a-url".to_string());

            assert!(result.is_err());
            assert!(matches!(result, Err(LessonError::VideoUrlError(_))));
        }

        #[test]
        fn test_update_video_url_invalid_preserves_original() {
            let mut lesson = create_test_lesson("Lesson", 1800, 0);
            let original_url = lesson.video_url().as_str().to_string();

            let _ = lesson.update_video_url("not-a-url".to_string());

            assert_eq!(lesson.video_url().as_str(), original_url);
        }
    }

    mod update_index {
        use super::*;

        #[test]
        fn test_update_index_modifies_index() {
            let mut lesson = create_test_lesson("Lesson", 1800, 0);

            lesson.update_index(5);

            assert_eq!(lesson.index().value(), 5);
        }

        #[test]
        fn test_update_index_preserves_id() {
            let mut lesson = create_test_lesson("Lesson", 1800, 0);
            let original_id = lesson.id();

            lesson.update_index(5);

            assert_eq!(lesson.id(), original_id);
        }

        #[test]
        fn test_update_index_preserves_name() {
            let mut lesson = create_test_lesson("My Lesson", 1800, 0);

            lesson.update_index(5);

            assert_eq!(lesson.name().as_str(), "My Lesson");
        }
    }
}
