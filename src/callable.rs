use crate::{
    object::Object,
    error::RuntimeError,
};

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
    let string = args.iter()
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