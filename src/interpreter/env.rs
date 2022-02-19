use super::LoxObj;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Env {
    values: HashMap<String, LoxObj>,
    enclosing: Option<Box<Env>>,
}

impl Env {
    pub fn new(enclosing: Box<Env>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn default() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn define(&mut self, name: &str, value: LoxObj) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Option<&LoxObj> {
        self.values.get(name).or_else(|| {
            if let Some(enclosing) = &self.enclosing {
                enclosing.get(name)
            } else {
                None
            }
        })
    }

    pub fn assign(&mut self, name: &str, value: LoxObj) -> Result<(), ()> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);

            Ok(())
        } else if let Some(enclosing) = &mut self.enclosing {
            enclosing.assign(name, value)
        } else {
            Err(())
        }
    }
}
