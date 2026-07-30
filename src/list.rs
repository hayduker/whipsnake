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
    pub items: Vec<Object>,
}

impl ListPayload {
    pub fn new() -> Self {
        ListPayload { items: vec![] }
    }
}

pub fn create_list_class() -> Object {
    let mut attrs = HashMap::new();

    attrs.insert(
        APPEND_FN_NAME.to_string(),
        Object::Function(Callable::Native(APPEND_FN)),
    );

    attrs.insert(
        CLEAR_FN_NAME.to_string(),
        Object::Function(Callable::Native(CLEAR_FN)),
    );

    attrs.insert(
        GETITEM_FN_NAME.to_string(),
        Object::Function(Callable::Native(GETITEM_FN)),
    );

    attrs.insert(
        LEN_FN_NAME.to_string(),
        Object::Function(Callable::Native(LEN_FN)),
    );

    attrs.insert(
        SETITEM_FN_NAME.to_string(),
        Object::Function(Callable::Native(SETITEM_FN)),
    );

    attrs.insert(
        STR_FN_NAME.to_string(),
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

fn with_self_list<F, R>(args: &[Object], f: F) -> Result<R, RuntimeError>
where
    F: FnOnce(&mut ListPayload) -> R,
{
    let receiver = &args[0];

    #[allow(clippy::collapsible_if)]
    if let Object::Instance(instance) = receiver {
        if let Some(NativePayload::List(ref mut list)) = instance.inner.borrow_mut().payload {
            return Ok(f(list));
        }
    }

    Err(RuntimeError::TypeError(
        SourceLocation { line: 0 },
        format!("failed to extract self for '{}' type", receiver.py_type()),
    ))
}

// =======================================================
// append
// =======================================================

const APPEND_FN_NAME: &str = "append";

pub fn append_impl(args: Vec<Object>) -> Result<Object, RuntimeError> {
    let item = args[1].clone();

    with_self_list(&args, |list| {
        list.items.push(item);
    })?;

    Ok(Object::None)
}

const APPEND_FN: NativeFn = NativeFn {
    name: APPEND_FN_NAME,
    arity: Arity::Exact(2),
    body: append_impl,
};

// =======================================================
// append
// =======================================================

const CLEAR_FN_NAME: &str = "clear";

fn clear_impl(args: Vec<Object>) -> Result<Object, RuntimeError> {
    with_self_list(&args, |list| {
        list.items.clear();
    })?;

    Ok(Object::None)
}

const CLEAR_FN: NativeFn = NativeFn {
    name: CLEAR_FN_NAME,
    arity: Arity::Exact(1),
    body: clear_impl,
};

// =======================================================
// __getitem__
// =======================================================

const GETITEM_FN_NAME: &str = "__getitem__";

fn getitem_impl(args: Vec<Object>) -> Result<Object, RuntimeError> {
    let mut index = match args[1].clone() {
        Object::Int(i) => i,
        object => {
            return Err(RuntimeError::TypeError(
                SourceLocation { line: 0 },
                format!(
                    "list indices must be integers or slices, not {}",
                    object.py_type()
                ),
            ));
        }
    };

    with_self_list(&args, |list| {
        let len = list.items.len() as i64;

        // Handle negative indexing
        if index < 0 {
            index += len;
        }

        if index < 0 || index >= len {
            return Err(RuntimeError::IndexError(
                SourceLocation { line: 0 },
                "list index out of range".to_string(),
            ));
        }

        Ok(list.items[index as usize].clone())
    })?
}

const GETITEM_FN: NativeFn = NativeFn {
    name: GETITEM_FN_NAME,
    arity: Arity::Exact(2),
    body: getitem_impl,
};

// =======================================================
// __len__
// =======================================================

const LEN_FN_NAME: &str = "__len__";

fn len_impl(args: Vec<Object>) -> Result<Object, RuntimeError> {
    let length = with_self_list(&args, |list| list.items.len())?;
    Ok(Object::Int(length as i64))
}

const LEN_FN: NativeFn = NativeFn {
    name: LEN_FN_NAME,
    arity: Arity::Exact(1),
    body: len_impl,
};

// =======================================================
// __setitem__
// =======================================================

const SETITEM_FN_NAME: &str = "__setitem__";

fn setitem_impl(args: Vec<Object>) -> Result<Object, RuntimeError> {
    let mut index = match args[1] {
        Object::Int(i) => i,
        ref object => {
            return Err(RuntimeError::TypeError(
                SourceLocation { line: 0 },
                format!(
                    "list indices must be integers or slices, not {}",
                    object.py_type()
                ),
            ));
        }
    };

    // Clone the item so we have an owned Object to put into the Vec
    let item = args[2].clone();

    with_self_list(&args, |list| {
        let len = list.items.len() as i64;

        // Handle negative indexing
        if index < 0 {
            index += len;
        }

        if index < 0 || index >= len {
            return Err(RuntimeError::IndexError(
                SourceLocation { line: 0 },
                "list assignment index out of range".to_string(),
            ));
        }

        // Now item is an owned Object, so assignment succeeds!
        list.items[index as usize] = item;
        Ok(())
    })??; // Note the double '??' (or return Ok(()) inside)

    Ok(Object::None)
}

const SETITEM_FN: NativeFn = NativeFn {
    name: SETITEM_FN_NAME,
    arity: Arity::Exact(3),
    body: setitem_impl,
};

// =======================================================
// __str__
// =======================================================

const STR_FN_NAME: &str = "__str__";

fn str_impl(args: Vec<Object>) -> Result<Object, RuntimeError> {
    let string = with_self_list(&args, |list| {
        let mut s = String::from("[");
        for item in &list.items {
            s += format!("{}, ", item).as_str();
        }
        s + "]"
    })?;

    Ok(Object::String(string))
}

const STR_FN: NativeFn = NativeFn {
    name: STR_FN_NAME,
    arity: Arity::Exact(1),
    body: str_impl,
};
