pub struct Person {
    pub id: Option<String>,
    pub name: String,
}

impl Person {
    fn new(id: Option<String>, name: String) -> Self {
        Self { id, name }
    }
}
