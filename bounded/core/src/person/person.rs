use education_platform_common::{Id, PersonName};

pub struct Person {
    id: Option<Id>,
    name: PersonName,
}

impl Person {
    fn new(id: Option<Id>, name: PersonName) -> Self {
        Self { id, name }
    }
}
