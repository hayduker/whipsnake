use std::fmt;

use crate::error::RuntimeError;

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
            Object::Function(_) => "function"
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
            Object::Function(callable) => {
                match callable {
                    Callable::Native(native_fn) => {
                        let address = native_fn.body as *const ();
                        write!(f, "<function {} at {:p}>", native_fn.name, address)
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Callable {
    Native(NativeFunction),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NativeFunction {
    pub name: &'static str,
    pub arity: Arity,
    pub body: fn(args: Vec<Object>) -> Result<Object, RuntimeError>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Arity {
    Exact(usize),
    Minimum(usize),
}