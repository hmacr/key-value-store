use std::collections::HashMap;

pub struct InMemoryStore {
    data_container: HashMap<String, String>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        InMemoryStore {
            data_container: HashMap::new(),
        }
    }

    pub fn get_data(&self, key: &String) -> Option<String> {
        self.data_container.get(key).map(String::clone)
    }

    pub fn put_data(&mut self, key: &String, value: &String) {
        self.data_container.insert(key.clone(), value.clone());
    }

    pub fn remove_data(&mut self, key: &String) -> Option<String> {
        self.data_container.remove(key)
    }
}
