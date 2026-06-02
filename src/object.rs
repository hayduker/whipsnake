use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Int(i64),
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
            Object::Int(i) => *i != 0,
            Object::Float(f) => *f != 0.0,
            Object::String(s) => !s.is_empty(),
        }
    }

    pub fn is_iterable(&self) -> bool {
        match self {
            Object::String(_) => true,
            _ => false,
        }
    }

    pub fn py_type(&self) -> &str {
        match self {
            Object::None => "NoneType",
            Object::Bool(_) => "bool",
            Object::Int(_) => "int",
            Object::Float(_) => "float",
            Object::String(_) => "str",
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Int(num) => write!(f, "{}", num),
            Object::Float(num) => write!(f, "{}", num),
            Object::String(text) => write!(f, "'{}'", text),
            Object::Bool(true) => write!(f, "True"),
            Object::Bool(false) => write!(f, "False"),
            Object::None => write!(f, "None"),
        }
    }
}
