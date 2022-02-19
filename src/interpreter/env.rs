use super::LoxObj;
use std::collections::HashMap;

pub struct Env {
    values: HashMap<String, LoxObj>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: LoxObj) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Option<&LoxObj> {
        self.values.get(name)
    }

    pub fn assign(&mut self, name: &str, value: LoxObj) -> Result<(), ()> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);

            Ok(())
        } else {
            Err(())
        }
    }
}
