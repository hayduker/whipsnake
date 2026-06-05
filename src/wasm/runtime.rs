use crate::wasm::Instruction;
use crate::wasm::value::Value;
use crate::wasm::store::Store;
use crate::wasm::decoder;

struct Frame {
    pc: isize,
    sp: usize,
    instructions: Vec<Instruction>,
    arity: usize,
    locals: Vec<Value>,
}

struct Runtime {
    store: Store,
    stack: Vec<Value>,
    call_stack: Vec<Frame>,
}

impl Runtime {
    fn instantiate(wasm: &[u8]) -> Result<Self, String> {
        let module = decoder::decode_wasm(wasm)?;
        let store = Store::new(module)?;
        Ok(Self {
            store,
            stack: vec![],
            call_stack: vec![],
        })
    }

    fn execute(&mut self) -> Result<(), String> {
        loop {
            let Some(frame) = self.call_stack.last_mut() else {
                break;
            };

            frame.pc += 1;

            let Some(instruction) = frame.instructions.get(frame.pc as usize) else {
                break;
            };

            match instruction {
                Instruction::End => {
                    self.call_stack.pop();

                    let Frame { sp, arity, ..} = frame;
                    self.stack_unwind(sp.clone(), arity.clone());
                },
                Instruction::LocalGet(index) => {
                    let Some(value) = frame.locals.get(*index as usize) else {
                        return Err(panic!("didn't find local index {}", index));
                    };

                    self.stack.push(*value);
                },
                Instruction::I32Add => {
                    let (Some(right), Some(left)) = (self.stack.pop(), self.stack.pop()) else {
                        return Err(panic!("not ennough values on that stack for instruction {:?}", instruction));
                    };

                    self.stack.push(left + right);
                },
            }
        }

        Ok(())
    }

    fn stack_unwind(&mut self, sp: usize, arity: usize) -> Result<(), String> {
        if arity > 0 {
            let Some(value) = self.stack.pop() else {
                return Err("didn't find return value on the stack".to_string());
            };

            self.stack.drain(sp..);
            self.stack.push(value);
        } else {
            self.stack.drain(sp..);
        }
        Ok(())
    }
}