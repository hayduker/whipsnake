// use wat;

pub mod binary;
pub mod decoder;



#[derive(Debug)]
enum SectionCode {
    Type = 0x01,
    Function = 0x03,
    Code = 0x0A,
}

impl SectionCode {
    fn from(code: u8) -> Result<Self, String> {
        match code {
            0x01 => Ok(Self::Type),
            0x03 => Ok(Self::Function),
            0x0A => Ok(Self::Code),
            _  => Err(format!("unsupported section code {code} provided"))
        }
    }
}



#[derive(Debug, PartialEq)]
struct FuncType {
    params: Vec<ValueType>,
    results: Vec<ValueType>,
}

#[derive(Debug, PartialEq)]
enum ValueType {
    I32 = 0x7F,
    I64 = 0x7E,
}

impl ValueType {
    fn from(value: u8) -> Result<Self, String> {
        match value {
            0x7F => Ok(Self::I32),
            0x7E => Ok(Self::I64),
            _ => Err(format!("unsupported value type {value} provided"))
        }
    }
}



#[derive(Debug, PartialEq)]
struct Function {
    locals: Vec<FunctionLocal>,
    code: Vec<Instruction>,
}

#[derive(Debug, PartialEq)]
struct FunctionLocal {
    type_count: u32,
    value_type: ValueType,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    End,
    LocalGet(u32),
    I32Add,
}

#[derive(Debug, PartialEq)]
enum Opcode {
    End = 0x0B,
    LocalGet = 0x20,
    I32Add = 0x6A,
}

impl Opcode {
    fn from(value: u8) -> Result<Self, String> {
        match value {
            0x0B => Ok(Self::End),
            0x20 => Ok(Self::LocalGet),
            0x6A => Ok(Self::I32Add),
            _ => Err(format!("unsupported instruction code {value} provided"))
        }
    }
}


///////////////////////////////////////////////////////
// Module definition
// Outcome of decoding process. Stores instructions
// and functions for execution process.
///////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub struct Module {
    magic: String,
    version: u32,
    type_section: Option<Vec<FuncType>>,
    function_section: Option<Vec<u32>>,
    code_section: Option<Vec<Function>>,
}

impl Module {
    fn new() -> Self {
        Module {
            magic: "\0asm".to_string(),
            version: 1,
            type_section: None,
            function_section: None,
            code_section: None,
        }
    }
}