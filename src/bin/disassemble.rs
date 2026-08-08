use whipsnake::bytecode::{
    chunk::{BytecodeError, Chunk},
    disassemble::disassemble_chunk,
};

fn main() -> Result<(), BytecodeError> {
    let mut chunk = Chunk::new();
    chunk.write_chunk(0)?;

    disassemble_chunk(&chunk, "test chunk");

    Ok(())
}
