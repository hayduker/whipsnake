use crate::wasm::{FuncType, Function, Module};

#[allow(unused)]
struct InternalFuncInst {
    func_type: FuncType,
    code: Function,
}

#[allow(unused)]
enum FuncInst {
    Internal(InternalFuncInst),
}

#[allow(unused)]
pub struct Store {
    functions: Vec<FuncInst>,
}

impl Store {
    pub fn new(module: Module) -> Result<Self, String> {
        let func_indices = match module.function_section {
            Some(indices) => indices,
            None => vec![],
        };

        let mut functions = vec![];

        if let Some(code_section) = module.code_section {
            let Some(func_types) = module.type_section else {
                return Err("type section expected but not found".to_string());
            };

            for (func_body, type_index) in code_section.iter().zip(func_indices) {
                let Some(func_type) = func_types.get(type_index as usize) else {
                    return Err(format!(
                        "type index {} not found in type section",
                        type_index
                    ));
                };

                let function = FuncInst::Internal(InternalFuncInst {
                    func_type: func_type.clone(),
                    code: Function {
                        locals: func_body.locals.clone(),
                        code: func_body.code.clone(),
                    },
                });

                functions.push(function);
            }
        }

        Ok(Self { functions })
    }
}
