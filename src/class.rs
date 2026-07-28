use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error::RuntimeError, object::Object, token::SourceLocation};

#[derive(Debug, PartialEq, Clone)]
pub struct PyClass {
    pub name: String,
    pub attrs: HashMap<String, Object>,
}

impl PyClass {
    pub fn new(name: String, attrs: HashMap<String, Object>) -> Self {
        PyClass { name, attrs }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PyInstance {
    pub inner: Rc<RefCell<PyInstanceData>>,
}

impl PyInstance {
    pub fn new(class: Rc<RefCell<PyClass>>) -> Self {
        Self {
            inner: Rc::new(RefCell::new(PyInstanceData {
                class,
                fields: HashMap::default(),
            })),
        }
    }

    pub fn get(&self, name: &str) -> Result<Object, RuntimeError> {
        let inner = self.inner.borrow();
        // TODO: Should we clone here?
        if let Some(value) = inner.fields.get(name) {
            Ok(value.clone())
        } else if let Some(value) = inner.class.borrow().attrs.get(name) {
            match value {
                Object::Function(callable) => Ok(Object::BoundMethod {
                    receiver: self.clone(),
                    function: callable.clone(),
                }),
                _ => Ok(value.clone()),
            }
        } else {
            Err(RuntimeError::AttributeError(
                SourceLocation { line: 0 },
                format!(
                    "'{}' object has no attribute '{}'",
                    inner.class.borrow().name,
                    name
                ),
            ))
        }
    }

    pub fn set(&self, name: &str, value: Object) {
        self.inner
            .borrow_mut()
            .fields
            .insert(name.to_string(), value);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PyInstanceData {
    pub class: Rc<RefCell<PyClass>>,
    fields: HashMap<String, Object>,
}
