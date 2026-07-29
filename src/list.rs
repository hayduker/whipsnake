use std::collections::HashMap;

use crate::{
    callable::{Arity, Callable, NativeFn},
    class::{NativeAlloc, NativePayload, PyClass},
    error::RuntimeError,
    object::Object,
    token::SourceLocation,
};

pub const LIST_NAME: &str = "list";

#[derive(Debug, PartialEq, Clone, Default)]
pub struct ListPayload {
    items: Vec<Object>,
}

impl ListPayload {
    pub fn new() -> Self {
        ListPayload { items: vec![] }
    }
}

pub fn create_list_class() -> Object {
    let mut attrs = HashMap::new();

    attrs.insert(
        APPEND_FN.name.to_string(),
        Object::Function(Callable::Native(APPEND_FN)),
    );

    attrs.insert(
        LEN_FN.name.to_string(),
        Object::Function(Callable::Native(LEN_FN)),
    );

    attrs.insert(
        STR_FN.name.to_string(),
        Object::Function(Callable::Native(STR_FN)),
    );

    let class_rc = PyClass::new(
        LIST_NAME.to_string(),
        vec![],
        attrs,
        Some(NativeAlloc::List),
    );

    let mro = PyClass::compute_mro(&class_rc);
    class_rc.borrow_mut().mro = mro;

    Object::Class(class_rc)
}

fn append_impl(args: Vec<Object>) -> Result<Object, RuntimeError> {
    let receiver = &args[0];
    let item = &args[1];

    #[allow(clippy::collapsible_if)]
    if let Object::Instance(instance) = receiver {
        if let Some(NativePayload::List(ref mut list)) = instance.inner.borrow_mut().payload {
            list.items.push(item.clone());
            return Ok(Object::None);
        } else {
            println!("payload is {:?}", instance.inner.borrow_mut().payload);
        }
    } else {
        println!("receiver is {:?}", receiver);
    }

    Err(RuntimeError::TypeError(
        SourceLocation { line: 0 },
        format!("'{}' type has no append method", receiver.py_type()),
    ))
}

const APPEND_FN: NativeFn = NativeFn {
    name: "append",
    arity: Arity::Exact(2),
    body: append_impl,
};

fn len_impl(args: Vec<Object>) -> Result<Object, RuntimeError> {
    let receiver = &args[0];

    #[allow(clippy::collapsible_if)]
    if let Object::Instance(instance) = receiver {
        if let Some(NativePayload::List(ref mut list)) = instance.inner.borrow_mut().payload {
            let length = list.items.len();
            return Ok(Object::Int(length as i64));
        }
    }

    Err(RuntimeError::TypeError(
        SourceLocation { line: 0 },
        format!("'{}' type has no __len__ method", receiver.py_type()),
    ))
}

const LEN_FN: NativeFn = NativeFn {
    name: "__len__",
    arity: Arity::Exact(1),
    body: len_impl,
};

fn str_impl(args: Vec<Object>) -> Result<Object, RuntimeError> {
    let receiver = &args[0];

    #[allow(clippy::collapsible_if)]
    if let Object::Instance(instance) = receiver {
        if let Some(NativePayload::List(ref mut list)) = instance.inner.borrow_mut().payload {
            let mut string = String::from("[");
            for item in &list.items {
                string += format!("{}, ", item).as_str();
            }
            string += "]";

            return Ok(Object::String(string));
        }
    }

    Err(RuntimeError::TypeError(
        SourceLocation { line: 0 },
        format!("'{}' type has no __str__ method", receiver.py_type()),
    ))
}

const STR_FN: NativeFn = NativeFn {
    name: "__str__",
    arity: Arity::Exact(1),
    body: str_impl,
};
