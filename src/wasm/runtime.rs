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
}