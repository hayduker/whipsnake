use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    error::RuntimeError,
    object::Object,
    token::{SourceLocation, Token},
};

#[derive(Debug, PartialEq, Clone)]
pub struct PyClass {
    pub name: String,
}

impl PyClass {
    pub fn new(name: String) -> Self {
        PyClass { name }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PyInstance {
    pub inner: Rc<RefCell<PyInstanceData>>,
}

impl PyInstance {
    pub fn new(class: PyClass) -> Self {
        Self {
            inner: Rc::new(RefCell::new(PyInstanceData {
                class,
                fields: HashMap::default(),
            })),
        }
    }

    pub fn get(&self, name: &Token) -> Result<Object, RuntimeError> {
        let inner = self.inner.borrow();

        // TODO: Should we clone here?
        inner.fields.get(&name.lexeme).cloned().ok_or_else(|| {
            RuntimeError::AttributeError(
                SourceLocation { line: name.line },
                format!(
                    "'{}' object has no attribute '{}'",
                    inner.class.name, name.lexeme
                ),
            )
        })
    }

    pub fn set(&mut self, name: &Token, value: Object) {
        self.inner
            .borrow_mut()
            .fields
            .insert(name.lexeme.clone(), value);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PyInstanceData {
    pub class: PyClass,
    fields: HashMap<String, Object>,
}
