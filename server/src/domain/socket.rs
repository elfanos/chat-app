use std::collections::HashMap;

#[derive(Debug)]
pub struct Socket<T> {
    pub map: HashMap<String, T>,
}
impl<T> Socket<T> {
    pub fn new(map: HashMap<String, T>) -> Socket<T> {
        Socket { map }
    }
    pub fn add_connection(&mut self, buff: T, key: String) {
        self.map.insert(key, buff);
    }
}
