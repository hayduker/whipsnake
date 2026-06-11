use crate::{error::RuntimeError, object::Object, token::Token, ast::Stmt};

#[derive(Debug, Clone, PartialEq)]
pub enum Callable {
    UserDefined(UserDefinedFn),
    Native(NativeFn),
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserDefinedFn {
    pub name: String,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Arity {
    Exact(usize),
    Minimum(usize),
}

#[derive(Debug, Clone, Copy)]
pub struct NativeFn {
    pub name: &'static str,
    pub arity: Arity,
    pub body: fn(args: Vec<Object>) -> Result<Object, RuntimeError>,
}

impl PartialEq for NativeFn {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
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

pub const PRINT_FUNC: NativeFn = NativeFn {
    name: "print",
    arity: Arity::Minimum(0),
    body: print_impl,
};

pub fn type_impl(args: Vec<Object>) -> Result<Object, RuntimeError> {
    Ok(Object::String(format!("<class '{}'>", &args[0].py_type())))
}

pub const TYPE_FUNC: NativeFn = NativeFn {
    name: "type",
    arity: Arity::Exact(1),
    body: type_impl,
};

pub fn id_impl(args: Vec<Object>) -> Result<Object, RuntimeError> {
    Ok(Object::Int(args[0].identity()))
}

pub const ID_FUNC: NativeFn = NativeFn {
    name: "id",
    arity: Arity::Exact(1),
    body: id_impl,
};
