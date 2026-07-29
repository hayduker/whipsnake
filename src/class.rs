use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    fmt,
    rc::Rc,
};

use crate::{error::RuntimeError, object::Object, token::SourceLocation};

#[derive(PartialEq, Clone)]
pub struct PyClass {
    pub name: String,
    pub supers: Vec<Rc<RefCell<PyClass>>>,
    pub mro: Vec<Rc<RefCell<PyClass>>>,
    pub attrs: HashMap<String, Object>,
}

impl PyClass {
    pub fn new(
        name: String,
        supers: Vec<Rc<RefCell<PyClass>>>,
        attrs: HashMap<String, Object>,
    ) -> Rc<RefCell<Self>> {
        let class_rc = Rc::new(RefCell::new(PyClass {
            name,
            supers,
            mro: vec![],
            attrs,
        }));

        let mro = Self::compute_mro(&class_rc);
        class_rc.borrow_mut().mro = mro;

        class_rc
    }

    pub fn compute_mro(class_rc: &Rc<RefCell<PyClass>>) -> Vec<Rc<RefCell<PyClass>>> {
        let mut added = HashSet::new();
        let mut mro = vec![];
        let mut queue = VecDeque::new();

        mro.push(class_rc.clone());
        added.insert(class_rc.borrow().name.clone());

        for superclass in &class_rc.borrow().supers {
            queue.push_back(superclass.clone());
        }

        while let Some(class) = queue.pop_front() {
            if !added.contains(&class.borrow().name) {
                mro.push(class.clone());
                added.insert(class.borrow().name.clone());
            }

            for superclass in &class.borrow().mro {
                if !added.contains(&superclass.borrow().name) {
                    queue.push_back(superclass.clone());
                }
            }
        }

        let pretty: Vec<String> = mro.iter().map(|c| c.borrow().name.clone()).collect();
        println!("computed mro: {:?}", pretty);

        mro
    }
}

impl fmt::Debug for PyClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let super_names: Vec<String> = self
            .supers
            .iter()
            .map(|s| s.borrow().name.clone())
            .collect();
        let mro_names: Vec<String> = self.mro.iter().map(|m| m.borrow().name.clone()).collect();

        f.debug_struct("PyClass")
            .field("name", &self.name)
            .field("supers", &super_names)
            .field("mro", &mro_names)
            .field("attrs", &self.attrs.keys().collect::<Vec<_>>())
            .finish()
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

        if let Some(value) = inner.fields.get(name) {
            println!("got {} in object", name);
            return Ok(value.clone());
        } else {
            for class in &inner.class.borrow().mro {
                if let Some(value) = class.borrow().attrs.get(name) {
                    match value {
                        Object::Function(callable) => {
                            println!(
                                "got {} in class {} and its a function",
                                name,
                                class.borrow().name
                            );

                            return Ok(Object::BoundMethod {
                                receiver: self.clone(),
                                function: callable.clone(),
                            });
                        }
                        _ => {
                            println!(
                                "got {} in class {} and it aint no function",
                                name,
                                class.borrow().name
                            );

                            return Ok(value.clone());
                        }
                    }
                }
            }
        }

        Err(RuntimeError::AttributeError(
            SourceLocation { line: 0 },
            format!(
                "'{}' object has no attribute '{}'",
                inner.class.borrow().name,
                name
            ),
        ))
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
