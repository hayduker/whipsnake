use crate::{ast::Stmt, error::RuntimeError, object::Object, token::Token};

// TODO: Technically class constructors should be represented here, but
// when I tried it that way the evaluator balked in its call() method
// because the Object being called wasn't a Function variant, but rather
// a Class variant.
//
// For now I've just matched on Object::Class in call(), but this means
// constructors don't get registered here, which feels a little weird.
// One idea would be to register a new user-defined function with the
// same name as the class whenever we evaluate a class definition. Then
// we could promote an Object::Class in a call() setting to an
// Object::Function. This feels a little weird though.
//
// As I'm implementing classes, I am starting to feel like the model
// I've used in the interpreter isn't exactly meshing with Python's
// representation of things. For example, I have an Object::Instance
// variant but really, as they always say, everything in Python is an
// instance (or "object" in the Python parlance) so it feel a bit weird
// that objects of user-defined classes are treated differently than
// objects of built-in classes, like int and str and so on. I suspect
// this will make things more complex down the road when implementing
// namespaces and dynamic attributes and so on. So I'll want to revisit
// this issue.
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
