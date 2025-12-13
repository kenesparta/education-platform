use education_platform_common::{Date, Entity, Id, SimpleName};
use crate::Chapter;

pub struct Course {
    id: Id,
    name: SimpleName,
    date: Date,
    chapters: Vec<Chapter>,
    number_of_lessons: u32,
}

impl Course {}

impl Entity for Course {
    fn id(&self) -> Id {
        self.id
    }
}