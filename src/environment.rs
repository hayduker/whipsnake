use std::collections::HashMap;

use crate::object::Object;

pub struct Environment<'a> {
    values: HashMap<String, Object>,
    enclosing: Option<&'a Environment<'a>>,
}

impl<'a> Environment<'a> {
    pub fn new_global() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_local(enclosing: &'a Environment) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Object> {
        self.values.get(name).or_else(|| self.enclosing?.get(name))
    }
}