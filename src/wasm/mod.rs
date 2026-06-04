use std::io::Read;

use wat;

struct BinaryReader<'a> {
    remaining_bytes: &'a [u8],
}

impl<'a> BinaryReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { remaining_bytes: bytes }
    }

    pub fn is_done(&self) -> bool {
        self.remaining_bytes.len() > 0
    }

    pub fn read_bytes<const N: usize>(&mut self) -> Result<[u8; N], String> {
        let mut buf = [0u8; N];

        self.remaining_bytes
            .read_exact(&mut buf)
            .map_err(|_| "unexpected end of file while reading raw bytes".to_string())?;
        Ok(buf)
    }

    pub fn read_slice(&mut self, N: usize) -> Result<&'a [u8], String> {
        if self.remaining_bytes.len() < N {
            return Err(format!("not enough bytes remaining to read {N}"));
        }

        let (taken, rest) = self.remaining_bytes.split_at(N);
        self.remaining_bytes = rest;
        Ok(taken)
    }

    pub fn read_le_u32(&mut self) -> Result<u32, String> {
        let bytes = self.read_bytes::<4>()?;
        Ok(u32::from_le_bytes(bytes))
    }

    pub fn read_uleb128_u32(&mut self) -> Result<u32, String> {
        let mut result: u32 = 0;
        let mut shift = 0;

        loop {
            let byte = self.read_byte()?;
            let flag = byte >> 7;
            let data = (byte & 0x7F) as u32;

            if shift > 21 {
                if flag == 1 {
                    return Err("got uleb128 encoding with more than 5 bytes, which is too 
                                many for u32".into());
                }

                let first_half = data & 0xF0;
                if first_half != 0 {
                    return Err("got high bits in locations 0b0xxx0000 of 5th byte 
                                in uleb128 encoding, which will get shifted out for u32".into());
                }
            }

            result |= data << shift;
            shift += 7;

            if flag == 0 { break }
        }

        Ok(result)
    }

    pub fn read_byte(&mut self) -> Result<u8, String> {
        let bytes = self.read_bytes::<1>()?;
        Ok(bytes[0])
    }
}


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
            0x07 => Ok(Self::Code),
            _  => Err(format!("unsupported section code {code} provided"))
        }
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

        ///////////////////////////////////////////
        // Wasm binary header
        ///////////////////////////////////////////
        
        let magic = reader.read_bytes::<4>()?;
        if &magic != b"\0asm" {
            return Err("bad magic in Wasm binary".into());
        }

        let version = reader.read_le_u32()?;
        if version != 1 {
            return Err(format!("bad version {} parsed from binary", version));
        }

        while !reader.is_done() {
            ///////////////////////////////////////////
            // Section header
            ///////////////////////////////////////////
        
            let section_code = SectionCode::from(reader.read_byte()?)?;
            let section_size = reader.read_uleb128_u32()?;
            let section_contents = reader.read_slice(section_size as usize);

            match section_code {
                SectionCode::Type => println!("Got section Type!"),
                SectionCode::Function => println!("Got section Function!"),
                SectionCode::Code => println!("Got section Code!"),
                _ => return Err(format!("unsupported section code {:?}", section_code)),
            }
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

    // #[test]
    // fn decode_empty_module() {
    //     let wasm = wat::parse_str("(module)")
    //         .expect("unit test input WAT invalid");
        
    //     let module = Module::new(&wasm).unwrap();
    //     assert_eq!(module, Module::default());
    // }

    #[test]
    fn test_uleb128_decoding_single_byte() {
        let bytes: &[u8] = &[0x2A];
        let mut reader = BinaryReader::new(bytes); 

        let result = reader.read_uleb128_u32();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_uleb128_decoding_multi_byte() {
        // 0xE5 = 11100101 (MSB = 1, continue)
        // 0x8E = 10001110 (MSB = 1, continue)
        // 0x26 = 00100110 (MSB = 0, stop)
        let mut bytes: &[u8] = &[0xE5, 0x8E, 0x26, 0xAA, 0xBB];
        let mut reader = BinaryReader::new(bytes); 

        let result = reader.read_uleb128_u32();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 624485);
    }

    #[test]
    fn test_uleb128_decoding_too_many_bytes() {
        // 0xE5 = 11100101 (MSB = 1, continue)
        // 0x8E = 10001110 (MSB = 1, continue)
        let bytes: &[u8] = &[0xE5, 0x8E, 0xE5, 0x8E, 0b10010110];
        //                                             ^ fifth byte can't have high continuation
        //                                               bit when decoding to u32
        let mut reader = BinaryReader::new(bytes); 

        let result = reader.read_uleb128_u32();

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("more than 5 bytes"));
    }

    #[test]
    fn test_uleb128_decoding_too_many_bits_in_fifth_byte() {
        // 0xE5 = 11100101 (MSB = 1, continue)
        // 0x8E = 10001110 (MSB = 1, continue)
        let bytes: &[u8] = &[0xE5, 0x8E, 0xE5, 0x8E, 0b01010011];
        //                                              ^^^ these three bits out of scope of u32
        //                                                  in fifth byte of uleb128
        let mut reader = BinaryReader::new(bytes); 

        let result = reader.read_uleb128_u32();

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("got high bits"));
    }
}