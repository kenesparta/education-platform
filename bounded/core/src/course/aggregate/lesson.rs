use education_platform_common::{
    Duration, Entity, Id, Index, IndexError, SimpleName, SimpleNameConfig, SimpleNameError, Url,
    UrlError,
};
use thiserror::Error;

/// Error types for Lesson validation failures.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LessonError {
    #[error("Lesson name validation failed: {0}")]
    NameError(#[from] SimpleNameError),

    #[error("Lesson video URL validation failed: {0}")]
    VideoUrlError(#[from] UrlError),

    #[error("Lesson index error: {0}")]
    IndexError(#[from] IndexError),

    #[error("Duration must be different from zero")]
    DurationIsZero,
}

/// A lesson within a course, representing a single video or learning unit.
///
/// `Lesson` is an entity that belongs to a `Course` aggregate. It contains
/// information about a single lesson including its name, duration, video URL,
/// and position within the course.
///
/// # Examples
///
/// ```
/// use education_platform_core::Lesson;
///
/// let lesson = Lesson::new(
///     "Introduction to Rust".to_string(),
///     3600,
///     "https://example.com/videos/intro.mp4".to_string(),
///     0,
/// ).unwrap();
///
/// assert_eq!(lesson.name().as_str(), "Introduction to Rust");
/// assert_eq!(lesson.duration().total_seconds(), 3600);
/// assert!(lesson.video_url().is_secure());
/// assert!(lesson.index().is_first());
/// ```
#[derive(Clone)]
pub struct Lesson {
    id: Id,
    name: SimpleName,
    duration: Duration,
    video_url: Url,
    index: Index,
}

impl Lesson {
    /// Creates a new `Lesson` with the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `name` - The lesson name (will be validated as a SimpleName)
    /// * `duration_seconds` - Duration of the lesson in seconds
    /// * `video_url` - URL to the lesson video (must be valid HTTP/HTTPS)
    /// * `index` - Position of this lesson within the course (zero-based)
    ///
    /// # Errors
    ///
    /// Returns `LessonError::NameError` if the name validation fails.
    /// Returns `LessonError::VideoUrlError` if the URL validation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::Lesson;
    ///
    /// let lesson = Lesson::new(
    ///     "Getting Started with Rust".to_string(),
    ///     1800,
    ///     "https://example.com/videos/lesson1.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// assert_eq!(lesson.name().as_str(), "Getting Started with Rust");
    /// ```
    pub fn new(
        name: String,
        duration_seconds: u64,
        video_url: String,
        index: usize,
    ) -> Result<Self, LessonError> {
        let duration = Duration::from_seconds(duration_seconds);
        if duration.is_zero() {
            return Err(LessonError::DurationIsZero);
        }

        let name = SimpleName::with_config(name, SimpleNameConfig::new(3, 50))?;
        let video_url = Url::new(video_url)?;
        let index = Index::new(index);
        let id = Id::default();

        Ok(Self {
            id,
            name,
            duration,
            video_url,
            index,
        })
    }

    /// Returns the lesson name.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::Lesson;
    ///
    /// let lesson = Lesson::new(
    ///     "Rust Basics".to_string(),
    ///     1200,
    ///     "https://example.com/video.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// assert_eq!(lesson.name().as_str(), "Rust Basics");
    /// ```
    #[inline]
    #[must_use]
    pub const fn name(&self) -> &SimpleName {
        &self.name
    }

    /// Returns the lesson duration.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::Lesson;
    ///
    /// let lesson = Lesson::new(
    ///     "Introduction".to_string(),
    ///     3665,
    ///     "https://example.com/video.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// assert_eq!(lesson.duration().total_seconds(), 3665);
    /// assert_eq!(lesson.duration().hours(), 1);
    /// ```
    #[inline]
    #[must_use]
    pub const fn duration(&self) -> Duration {
        self.duration
    }

    /// Returns the video URL.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::Lesson;
    ///
    /// let lesson = Lesson::new(
    ///     "Introduction".to_string(),
    ///     1800,
    ///     "https://example.com/video.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// assert_eq!(lesson.video_url().as_str(), "https://example.com/video.mp4");
    /// ```
    #[inline]
    #[must_use]
    pub const fn video_url(&self) -> &Url {
        &self.video_url
    }

    /// Returns the lesson index (position within the course).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::Lesson;
    ///
    /// let lesson = Lesson::new(
    ///     "Introduction".to_string(),
    ///     1800,
    ///     "https://example.com/video.mp4".to_string(),
    ///     5,
    /// ).unwrap();
    ///
    /// assert_eq!(lesson.index().value(), 5);
    /// ```
    #[inline]
    #[must_use]
    pub const fn index(&self) -> Index {
        self.index
    }

    /// Sets the lesson index (position within the course).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_core::Lesson;
    ///
    /// let mut lesson = Lesson::new(
    ///     "Introduction".to_string(),
    ///     1800,
    ///     "https://example.com/video.mp4".to_string(),
    ///     0,
    /// ).unwrap();
    ///
    /// lesson.update_index(5);
    /// assert_eq!(lesson.index().value(), 5);
    /// ```
    #[inline]
    pub fn update_index(&mut self, index: usize) {
        self.index = Index::new(index);
    }
}

impl Entity for Lesson {
    fn id(&self) -> Id {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod constructors {
        use super::*;

        #[test]
        fn test_new_creates_valid_lesson() {
            let lesson = Lesson::new(
                "Introduction to Rust".to_string(),
                3600,
                "https://example.com/video.mp4".to_string(),
                0,
            );

            assert!(lesson.is_ok());
            let lesson = lesson.unwrap();
            assert_eq!(lesson.name().as_str(), "Introduction to Rust");
            assert_eq!(lesson.duration().total_seconds(), 3600);
            assert_eq!(lesson.video_url().as_str(), "https://example.com/video.mp4");
            assert_eq!(lesson.index().value(), 0);
        }

        #[test]
        fn test_new_with_different_index() {
            let lesson = Lesson::new(
                "Lesson 5".to_string(),
                1800,
                "https://example.com/lesson5.mp4".to_string(),
                4,
            )
            .unwrap();

            assert_eq!(lesson.index().value(), 4);
        }

        #[test]
        fn test_new_generates_unique_id() {
            let lesson1 = Lesson::new(
                "Lesson 1".to_string(),
                600,
                "https://example.com/c1.mp4".to_string(),
                0,
            )
            .unwrap();

            let lesson2 = Lesson::new(
                "Lesson 2".to_string(),
                600,
                "https://example.com/c2.mp4".to_string(),
                1,
            )
            .unwrap();

            assert_ne!(lesson1.id(), lesson2.id());
        }

        #[test]
        fn test_new_with_empty_name_returns_error() {
            let result =
                Lesson::new("".to_string(), 3600, "https://example.com/video.mp4".to_string(), 0);

            assert!(result.is_err());
            assert!(matches!(result, Err(LessonError::NameError(_))));
        }

        #[test]
        fn test_new_with_invalid_url_returns_error() {
            let result =
                Lesson::new("Valid Name".to_string(), 3600, "not-a-valid-url".to_string(), 0);

            assert!(result.is_err());
            assert!(matches!(result, Err(LessonError::VideoUrlError(_))));
        }

        #[test]
        fn test_new_with_invalid_url_scheme_returns_error() {
            let result = Lesson::new(
                "Valid Name".to_string(),
                3600,
                "ftp://example.com/video.mp4".to_string(),
                0,
            );

            assert!(result.is_err());
            assert!(matches!(result, Err(LessonError::VideoUrlError(_))));
        }

        #[test]
        fn test_new_with_zero_duration_returns_error() {
            let result = Lesson::new(
                "Valid Name".to_string(),
                0,
                "https://example.com/video.mp4".to_string(),
                0,
            );

            assert!(result.is_err());
            assert!(matches!(result, Err(LessonError::DurationIsZero)));
        }

        #[test]
        fn test_new_with_name_too_short_returns_error() {
            let result =
                Lesson::new("AB".to_string(), 3600, "https://example.com/video.mp4".to_string(), 0);

            assert!(result.is_err());
            assert!(matches!(result, Err(LessonError::NameError(_))));
        }

        #[test]
        fn test_new_with_name_at_min_length() {
            let result = Lesson::new(
                "ABC".to_string(),
                3600,
                "https://example.com/video.mp4".to_string(),
                0,
            );

            assert!(result.is_ok());
            assert_eq!(result.unwrap().name().as_str(), "ABC");
        }

        #[test]
        fn test_new_with_name_too_long_returns_error() {
            let long_name = "A".repeat(51);
            let result =
                Lesson::new(long_name, 3600, "https://example.com/video.mp4".to_string(), 0);

            assert!(result.is_err());
            assert!(matches!(result, Err(LessonError::NameError(_))));
        }

        #[test]
        fn test_new_with_name_at_max_length() {
            let max_name = "A".repeat(50);
            let result =
                Lesson::new(max_name.clone(), 3600, "https://example.com/video.mp4".to_string(), 0);

            assert!(result.is_ok());
            assert_eq!(result.unwrap().name().as_str(), max_name);
        }
    }

    mod getters {
        use super::*;

        #[test]
        fn test_name_returns_simple_name() {
            let lesson = Lesson::new(
                "Test Lesson".to_string(),
                1200,
                "https://example.com/test.mp4".to_string(),
                0,
            )
            .unwrap();

            assert_eq!(lesson.name().as_str(), "Test Lesson");
        }

        #[test]
        fn test_duration_returns_duration() {
            let lesson = Lesson::new(
                "Test Lesson".to_string(),
                7265,
                "https://example.com/test.mp4".to_string(),
                0,
            )
            .unwrap();

            assert_eq!(lesson.duration().total_seconds(), 7265);
            assert_eq!(lesson.duration().hours(), 2);
            assert_eq!(lesson.duration().minutes(), 1);
            assert_eq!(lesson.duration().seconds(), 5);
        }

        #[test]
        fn test_video_url_returns_url() {
            let lesson = Lesson::new(
                "Test Lesson".to_string(),
                1200,
                "https://cdn.example.com/videos/lesson.mp4".to_string(),
                0,
            )
            .unwrap();

            assert_eq!(
                lesson.video_url().as_str(),
                "https://cdn.example.com/videos/lesson.mp4"
            );
            assert!(lesson.video_url().is_secure());
        }

        #[test]
        fn test_index_returns_index() {
            let lesson = Lesson::new(
                "Test Lesson".to_string(),
                1200,
                "https://example.com/test.mp4".to_string(),
                10,
            )
            .unwrap();

            assert_eq!(lesson.index().value(), 10);
            assert!(!lesson.index().is_first());
        }

        #[test]
        fn test_index_first_lesson() {
            let lesson = Lesson::new(
                "First Lesson".to_string(),
                1200,
                "https://example.com/first.mp4".to_string(),
                0,
            )
            .unwrap();

            assert!(lesson.index().is_first());
        }
    }

    mod entity_trait {
        use super::*;

        #[test]
        fn test_id_returns_valid_id() {
            let lesson = Lesson::new(
                "Test Lesson".to_string(),
                1200,
                "https://example.com/test.mp4".to_string(),
                0,
            )
            .unwrap();

            let id = lesson.id();
            assert_eq!(id.as_bytes().len(), 16);
        }
    }

    mod real_world_examples {
        use super::*;

        #[test]
        fn test_course_introduction_lesson() {
            let lesson = Lesson::new(
                "Introduction to the Course".to_string(),
                300,
                "https://edu.example.com/courses/rust/intro.mp4".to_string(),
                0,
            )
            .unwrap();

            assert_eq!(lesson.name().as_str(), "Introduction to the Course");
            assert_eq!(lesson.duration().format_hours(), "05m 00s");
            assert!(lesson.index().is_first());
        }

        #[test]
        fn test_main_lesson() {
            let lesson = Lesson::new(
                "Chapter 1: Getting Started with Rust".to_string(),
                3665,
                "https://cdn.example.com/rust-course/chapter1.mp4".to_string(),
                1,
            )
            .unwrap();

            assert_eq!(lesson.duration().format_hours(), "01h 01m 05s");
            assert_eq!(lesson.index().value(), 1);
        }

        #[test]
        fn test_final_lesson() {
            let lesson = Lesson::new(
                "Conclusion & Next Steps".to_string(),
                600,
                "https://videos.example.com/conclusion.mp4".to_string(),
                24,
            )
            .unwrap();

            assert_eq!(lesson.index().value(), 24);
            assert_eq!(lesson.duration().format_hours(), "10m 00s");
        }
    }
}
