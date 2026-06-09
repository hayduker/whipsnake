use std::io::Read;

pub struct BinaryReader<'a> {
    remaining_bytes: &'a [u8],
}

impl<'a> BinaryReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            remaining_bytes: bytes,
        }
    }

    pub fn is_done(&self) -> bool {
        self.remaining_bytes.len() <= 0
    }

    pub fn read_bytes<const N: usize>(&mut self) -> Result<[u8; N], String> {
        let mut buf = [0u8; N];

        self.remaining_bytes
            .read_exact(&mut buf)
            .map_err(|_| "unexpected end of file while reading raw bytes".to_string())?;
        Ok(buf)
    }

    pub fn read_slice(&mut self, n: usize) -> Result<&'a [u8], String> {
        if self.remaining_bytes.len() < n {
            return Err(format!("not enough bytes remaining to read {n}"));
        }

        let (taken, rest) = self.remaining_bytes.split_at(n);
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
                                many for u32"
                        .into());
                }

                let first_half = data & 0xF0;
                if first_half != 0 {
                    return Err("got high bits in locations 0b0xxx0000 of 5th byte 
                                in uleb128 encoding, which will get shifted out for u32"
                        .into());
                }
            }

            result |= data << shift;
            shift += 7;

            if flag == 0 {
                break;
            }
        }

        Ok(result)
    }

    pub fn read_byte(&mut self) -> Result<u8, String> {
        let bytes = self.read_bytes::<1>()?;
        Ok(bytes[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let bytes: &[u8] = &[0xE5, 0x8E, 0x26, 0xAA, 0xBB];
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
