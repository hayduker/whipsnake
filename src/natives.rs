use crate::{
    error::RuntimeError,
    object::Object,
};

pub fn print_impl(args: Vec<Object>) -> Result<Object, RuntimeError> {
    let string = args.iter()
        .map(|arg| arg.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{string}");
    Ok(Object::None)
}