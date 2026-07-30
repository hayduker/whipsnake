use crate::{
    ast::Stmt,
    environment::Environment,
    error::RuntimeError,
    evaluator::Evaluator,
    object::Object,
    token::{SourceLocation, Token, TokenKind::LeftParen},
};

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

type NativeFnBody = fn(&mut Evaluator, args: Vec<Object>) -> Result<Object, RuntimeError>;

#[derive(Debug, Clone, Copy)]
pub struct NativeFn {
    pub name: &'static str,
    pub arity: Arity,
    pub body: NativeFnBody,
}

impl PartialEq for NativeFn {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

pub fn len_impl(evaluator: &mut Evaluator, args: Vec<Object>) -> Result<Object, RuntimeError> {
    let object = args[0].clone();
    let len_fn = match object.get_attr("__len__") {
        Ok(f) => f,
        Err(_) => {
            return Err(RuntimeError::TypeError(
                SourceLocation { line: 0 },
                format!("object of type '{}' has no len()", object.py_type()),
            ));
        }
    };

    // TODO: This is hacky af. The artificial token doesn't carry actual line info with it,
    // and using a new empty environment won't work for user-defined native functions that
    // make use of their surrounding environment. I could pass in the actual paren and
    // environment from the evaluator but I don't want to muck up all the native function
    // signatures that wouldn't even be using it just for this edge case. Also Evaluator::call
    // is now the only public method. All this makes me think this could be designed better.

    let artificial_token = Token::new(LeftParen, "(", 0);
    let length_value = evaluator.call(
        len_fn,
        &artificial_token,
        vec![],
        &Environment::new_global(),
    )?;

    Ok(length_value)
}

pub const LEN_FUNC: NativeFn = NativeFn {
    name: "len",
    arity: Arity::Exact(1),
    body: len_impl,
};

pub fn print_impl(_evaluator: &mut Evaluator, args: Vec<Object>) -> Result<Object, RuntimeError> {
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

pub fn type_impl(_evaluator: &mut Evaluator, args: Vec<Object>) -> Result<Object, RuntimeError> {
    Ok(Object::String(format!("<class '{}'>", &args[0].py_type())))
}

pub const TYPE_FUNC: NativeFn = NativeFn {
    name: "type",
    arity: Arity::Exact(1),
    body: type_impl,
};

pub fn id_impl(_evaluator: &mut Evaluator, args: Vec<Object>) -> Result<Object, RuntimeError> {
    Ok(Object::Int(args[0].identity()))
}

pub const ID_FUNC: NativeFn = NativeFn {
    name: "id",
    arity: Arity::Exact(1),
    body: id_impl,
};
