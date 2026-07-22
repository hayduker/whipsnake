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
    pub class: PyClass,
}

impl PyInstance {
    pub fn new(class: PyClass) -> Self {
        PyInstance { class }
    }
}
