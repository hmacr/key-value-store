use std::collections::HashMap;

#[derive(Clone)]
pub struct InMemoryStore {
    data_container: HashMap<String, String>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        InMemoryStore {
            data_container: HashMap::new(),
        }
    }

    pub fn get_all_data(&self) -> Vec<(String, String)> {
        let mut all_data = vec![];
        for (key, value) in &self.data_container {
            all_data.push((key.clone(), value.clone()));
        }
        all_data
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
