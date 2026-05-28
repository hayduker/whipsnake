#[derive(Debug, PartialEq)]
pub enum Object {
    Float(f64),
    String(String),
    Bool(bool),
    None,
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::None => false,
            Object::Bool(b) => *b,
            Object::Float(f) => *f != 0.0,
            Object::String(s) => !s.is_empty(),
        }
    }
}