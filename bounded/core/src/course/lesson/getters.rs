use super::Lesson;
use education_platform_common::{Duration, Index, SimpleName, Url};

impl Lesson {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_lesson(name: &str, duration: u64, index: usize) -> Lesson {
        Lesson::new(
            name.to_string(),
            duration,
            format!("https://example.com/{index}.mp4"),
            index,
        )
        .unwrap()
    }

    mod getters {
        use super::*;

        #[test]
        fn test_name_returns_simple_name() {
            let lesson = create_test_lesson("Test Lesson", 1200, 0);

            assert_eq!(lesson.name().as_str(), "Test Lesson");
        }

        #[test]
        fn test_duration_returns_duration() {
            let lesson = create_test_lesson("Test Lesson", 7265, 0);

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
            let lesson = create_test_lesson("Test Lesson", 1200, 10);

            assert_eq!(lesson.index().value(), 10);
            assert!(!lesson.index().is_first());
        }

        #[test]
        fn test_index_first_lesson() {
            let lesson = create_test_lesson("First Lesson", 1200, 0);

            assert!(lesson.index().is_first());
        }

        #[test]
        fn test_update_index() {
            let mut lesson = create_test_lesson("Lesson", 1800, 0);

            lesson.update_index(5);

            assert_eq!(lesson.index().value(), 5);
        }
    }
}
