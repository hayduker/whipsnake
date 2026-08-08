#[derive(Debug)]
pub struct BytecodeError {
    string: String,
}

#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    Return = 0,
}

impl TryFrom<u8> for OpCode {
    type Error = BytecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::Return),
            _ => Err(BytecodeError {
                string: "invalid byte provided".into(),
            }),
        }
    }
}

#[derive(Default)]
pub struct Chunk {
    pub code: Vec<OpCode>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk { code: vec![] }
    }

    pub fn write_chunk(&mut self, byte: u8) -> Result<(), BytecodeError> {
        self.code.push(byte.try_into()?);
        Ok(())
    }
}
