use std::fmt;

use crate::callable::Callable;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    None,
    Function(Callable),
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::None => false,
            Object::Bool(b) => *b,
            Object::Int(i) => *i != 0,
            Object::Float(f) => *f != 0.0,
            Object::String(s) => !s.is_empty(),
            Object::Function(_) => true,
        }
    }

    pub fn py_type(&self) -> &str {
        match self {
            Object::None => "NoneType",
            Object::Bool(_) => "bool",
            Object::Int(_) => "int",
            Object::Float(_) => "float",
            Object::String(_) => "str",
            Object::Function(callable) => match callable {
                Callable::Native(_) => "builtin_function_or_method",
            },
        }
    }

    pub fn identity(&self) -> i64 {
        match self {
            Object::Bool(b) => {
                if *b {
                    140735165470832
                } else {
                    140735165470864
                }
            }
            Object::Int(n) => {
                if *n >= -5 && *n <= 256 {
                    (1000000 + n) as i64
                } else {
                    self as *const Object as usize as i64
                }
            }
            Object::None => 140735165431120,
            Object::Function(callable) => match callable {
                Callable::Native(native) => {
                    native.name.as_ptr() as usize as i64
                }
            },
            _ => self as *const Object as usize as i64,
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
            Object::Function(callable) => match callable {
                Callable::Native(native_fn) => {
                    let address = native_fn.body as *const ();
                    write!(f, "<function {} at {:p}>", native_fn.name, address)
                }
            },
        }
    }
}
