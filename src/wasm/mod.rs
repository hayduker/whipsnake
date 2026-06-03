use std::{fmt::Error, io::Read};

use wat;

#[derive(Debug, PartialEq)]
pub struct DecodeError {
    message: String,
}



struct BinaryReader<'a> {
    remaining_bytes: &'a [u8],
}

impl<'a> BinaryReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { remaining_bytes: bytes }
    }

    pub fn len(&self) -> usize {
        self.remaining_bytes.len()
    }

    pub fn read_bytes<const N: usize>(&mut self) -> Result<[u8; N], String> {
        let mut buf = [0u8; N];

        self.remaining_bytes
            .read_exact(&mut buf)
            .map_err(|_| "Unexpected end of file while reading raw bytes".to_string())?;
        Ok(buf)
    }

    pub fn read_le_u32(&mut self) -> Result<u32, String> {
        let bytes = self.read_bytes::<4>()?;
        Ok(u32::from_le_bytes(bytes))
    }

    pub fn read_byte(&mut self) -> Result<u8, String> {
        let bytes = self.read_bytes::<1>()?;
        Ok(bytes[0])
    }
}






#[derive(Debug, PartialEq)]
pub struct Module {
    pub magic: String,
    pub version: u32,
}

impl Default for Module {
    fn default() -> Self {
        Module {
            magic: "\0asm".to_string(),
            version: 1,
        }
    }
}

impl Module {
    fn new(wasm: &Vec<u8>) -> Result<Self, String> {
        Module::decode(wasm)
    }

    fn decode(wasm: &[u8]) -> Result<Module, String> {
        let mut reader = BinaryReader::new(wasm);

        let magic = reader.read_bytes::<4>()?;
        if &magic != b"\0asm" {
            return Err("bad magic in Wasm binary".into());
        }

        let version = reader.read_le_u32()?;
        if version != 1 {
            return Err(format!("bad version {} parsed from binary", version));
        }

        Ok(Module {
            magic: "\0asm".into(),
            version,
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_empty_module() {
        let wasm = wat::parse_str("(module)")
            .expect("unit test input WAT invalid");
        
        let module = Module::new(&wasm).unwrap();
        assert_eq!(module, Module::default());
    }
}