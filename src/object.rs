//! The `object` module defines the `Object` enum, which represents all Python runtime values
//! in Whipsake. This includes integers, floats, strings, booleans, None, and callable functions.
//! It also provides methods for type introspection and truthiness checking.

use std::fmt;

use crate::callable::Callable;

/// Represents all possible runtime values in the Whipsake interpreter.
///
/// This enum encapsulates various data types such as integers, floating-point numbers,
/// strings, booleans, the `None` type, and callable functions (both user-defined and native).
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
    /// Determines the truthiness of an `Object` according to Python's rules.
    ///
    /// In Python, `None`, `False`, zero (0 or 0.0), and empty strings are considered falsy.
    /// All other values are considered truthy.
    ///
    /// # Returns
    ///
    /// `true` if the object is truthy, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use whipsnake::object::Object;
    ///
    /// assert_eq!(Object::Int(0).is_truthy(), false);
    /// assert_eq!(Object::Float(0.0).is_truthy(), false);
    /// assert_eq!(Object::String("".to_string()).is_truthy(), false);
    /// assert_eq!(Object::Bool(false).is_truthy(), false);
    /// assert_eq!(Object::None.is_truthy(), false);
    ///
    /// assert_eq!(Object::Int(1).is_truthy(), true);
    /// assert_eq!(Object::Float(1.0).is_truthy(), true);
    /// assert_eq!(Object::String("hello".to_string()).is_truthy(), true);
    /// assert_eq!(Object::Bool(true).is_truthy(), true);
    /// ```
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

    /// Returns a string representation of the Python type of the `Object`.
    ///
    /// # Returns
    ///
    /// A string slice representing the Python type (e.g., "int", "str", "function").
    ///
    /// # Examples
    ///
    /// ```
    /// use whipsnake::object::Object;
    ///
    /// assert_eq!(Object::Int(10).py_type(), "int");
    /// assert_eq!(Object::String("test".to_string()).py_type(), "str");
    /// assert_eq!(Object::None.py_type(), "NoneType");
    /// ```
    pub fn py_type(&self) -> &str {
        match self {
            Object::None => "NoneType",
            Object::Bool(_) => "bool",
            Object::Int(_) => "int",
            Object::Float(_) => "float",
            Object::String(_) => "str",
            Object::Function(callable) => match callable {
                Callable::UserDefined(_) => "function",
                Callable::Native(_) => "builtin_function_or_method",
            },
        }
    }

    /// Returns a pseudo-identity value for the object.
    ///
    /// This method attempts to mimic Python's `id()` function, which returns a unique
    /// integer for an object. For some common immutable types (e.g., small integers,
    /// `True`, `False`, `None`), it returns a consistent hardcoded value. For other objects,
    /// it uses the memory address of the object as its identity.
    ///
    /// # Returns
    ///
    /// An `i64` representing the identity of the object.
    ///
    /// # Examples
    ///
    /// ```
    /// use whipsnake::object::Object;
    ///
    /// let i1 = Object::Int(10);
    /// let i2 = Object::Int(10);
    /// let i3 = Object::Int(1000); // Larger int, likely different identity
    ///
    /// // Small integers have consistent identities
    /// assert_eq!(Object::Int(5).identity(), Object::Int(5).identity());
    ///
    /// // Different large integers or mutable objects might have different identities
    /// // The exact values of these will vary based on memory allocation
    /// // Note: This doctest might fail if Object::String allocates at the same address in some rare cases
    /// assert_ne!(Object::String("hello".to_string()).identity(), Object::String("hello".to_string()).identity());
    /// ```
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
                    1000000 + n
                } else {
                    self as *const Object as usize as i64
                }
            }
            Object::None => 140735165431120,
            Object::Function(callable) => match callable {
                Callable::UserDefined(user_defined) => user_defined.body.as_ptr() as usize as i64,
                Callable::Native(native) => native.name.as_ptr() as usize as i64,
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
                Callable::UserDefined(user_fn) => {
                    let address = user_fn.body.as_ptr();
                    write!(f, "<function {} at {:p}>", user_fn.name, address)
                }
                Callable::Native(native_fn) => {
                    write!(f, "<built-in function or method {}>", native_fn.name)
                }
            },
        }
    }
}
