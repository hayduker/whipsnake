use std::fmt;

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

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Float(num) => write!(f, "{}", num),
            Object::String(text) => write!(f, "'{}'", text),
            Object::Bool(true) => write!(f, "True"),
            Object::Bool(false) => write!(f, "False"),
            Object::None => write!(f, "None"),
        }
    }
}