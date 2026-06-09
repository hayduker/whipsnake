use crate::error::RuntimeError;
use crate::evaluator::Evaluator;
use crate::object::Object;

// pub enum Callable {
//     NativeFn(Object),
// }

pub trait Callable {
    fn call(&self, evaluator: &Evaluator, arguments: Vec<Object>) -> Result<Object, RuntimeError>;
    fn arity(&self)-> usize;
}




// pub struct NativeFn;

// impl Callable for NativeFn {
//     fn call(evaluator: Evaluator, arguments: Vec<Object>) -> Object {

//     }
// }



// struct Function<'a> {
//     definition: Stmt<'a>,
// }

// impl<'a> Function<'a> {
//     fn new(definition: Stmt) -> Result<Self, String> {
//         match definition {
//             Stmt::Function
//             Function {
//                 definition:
//             }
//         }
//     }
// }

// impl<'a> Callable for Function<'a> {
//     fn call(&self, evaluator: Evaluator, arguments: Vec<Object>) -> Object {

//     }

//     fn arity(&self) -> usize {
//         self.definition.len()
//     }
// }