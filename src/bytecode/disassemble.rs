use crate::bytecode::chunk::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{offset:04} ");

    match chunk.code.get(offset) {
        Some(OpCode::Return) => simple_instruction("Return", offset),
        Some(op) => {
            println!("Unknown opcode {op:?}");
            offset + 1
        }
        None => {
            println!("Offset {offset} out of bounds.");
            offset
        }
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}
