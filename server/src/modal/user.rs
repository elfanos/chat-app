pub struct User {
    pub name: String,
}

impl User {
    pub fn create(name: String) -> User {
        User { name }
    }
}
