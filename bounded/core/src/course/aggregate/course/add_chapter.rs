use crate::{Chapter, Course, CourseError};
use education_platform_common::Index;

impl Course {
    pub fn add_chapter(
        &self,
        chapter: Chapter,
        index: Option<Index>,
    ) -> Result<Course, CourseError> {
        let position = index
            .map(|idx| idx.value().min(self.chapters.len()))
            .unwrap_or(self.chapters.len());

        let mut chapters = Vec::with_capacity(self.chapters.len() + 1);
        chapters.extend_from_slice(&self.chapters[..position]);
        chapters.push(chapter);
        chapters.extend_from_slice(&self.chapters[position..]);

        let chapters = Self::reassign_index_chapters(&chapters)?;

        Ok(Course {
            chapters,
            ..self.clone()
        })
    }
}
