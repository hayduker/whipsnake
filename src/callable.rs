use crate::{error::RuntimeError, object::Object};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Callable {
    Native(NativeFunction),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Arity {
    Exact(usize),
    Minimum(usize),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NativeFunction {
    pub name: &'static str,
    pub arity: Arity,
    pub body: fn(args: Vec<Object>) -> Result<Object, RuntimeError>,
}

pub fn print_impl(args: Vec<Object>) -> Result<Object, RuntimeError> {
    let string = args
        .iter()
        .map(|arg| arg.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{string}");
    Ok(Object::None)
}

pub const PRINT_FUNC: NativeFunction = NativeFunction {
    name: "print",
    arity: Arity::Minimum(0),
    body: print_impl,
};

pub fn type_impl(args: Vec<Object>) -> Result<Object, RuntimeError> {
    Ok(Object::String(format!("<class '{}'>", &args[0].py_type())))
}

pub const TYPE_FUNC: NativeFunction = NativeFunction {
    name: "type",
    arity: Arity::Exact(1),
    body: type_impl,
};

pub fn id_impl(args: Vec<Object>) -> Result<Object, RuntimeError> {
    Ok(Object::Int(args[0].identity()))
}

pub const ID_FUNC: NativeFunction = NativeFunction {
    name: "id",
    arity: Arity::Exact(1),
    body: id_impl,
};
