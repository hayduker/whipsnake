use crate::wasm::binary::BinaryReader;
use crate::wasm::{FuncType, Function, FunctionLocal, Instruction, Module, Opcode, SectionCode, ValueType};

pub fn decode_wasm(bytes: &[u8]) -> Result<Module, String> {       
    let mut reader = BinaryReader::new(bytes);
    
    let (magic, version) = decode_module_header(&mut reader)?;

    let mut module = Module::new();
    module.magic = magic;
    module.version = version;

    while !reader.is_done() {    
        let (section_code, section_size) = decode_section_header(&mut reader)?;
        let section_contents = reader.read_slice(section_size as usize)?;

        match section_code {
            SectionCode::Type => {
                let types = decode_type_section(&section_contents)?;
                module.type_section = Some(types);
            },
            SectionCode::Function => {
                let func_indices = decode_function_section(&section_contents)?;
                module.function_section = Some(func_indices);
            },
            SectionCode::Code => {
                let functions = decode_code_section(&section_contents)?;
                module.code_section = Some(functions);
            },
        }
    }

    Ok(module)
}

fn decode_module_header(reader: &mut BinaryReader) -> Result<(String, u32), String> {
    let magic = reader.read_bytes::<4>()?;
    if &magic != b"\0asm" {
        return Err("bad magic in Wasm binary".into());
    }

    let version = reader.read_le_u32()?;
    if version != 1 {
        return Err(format!("bad version {} parsed from binary", version));
    }

    Ok((String::from_utf8(magic.to_vec()).unwrap(), version))
}

fn decode_section_header(reader: &mut BinaryReader) -> Result<(SectionCode, u32), String> {
    let section_code = SectionCode::from(reader.read_byte()?)?;
    let section_size = reader.read_uleb128_u32()?;

    Ok((section_code, section_size))
}

fn decode_type_section(contents: &[u8]) -> Result<Vec<FuncType>, String> {
    let mut reader = BinaryReader::new(contents);

    let num_types = reader.read_uleb128_u32()?;

    let mut func_types = vec![];
    for i in 0..num_types {
        if reader.read_byte()? != 0x60 {
            return Err(format!("expected 'func' marker (0x60) as first byte of func type {}", i));
        }

        let num_params = reader.read_uleb128_u32()?;
        let mut params = vec![];
        for _ in 0..num_params {
            let value_type = ValueType::from(reader.read_byte()?)?;
            params.push(value_type);
        }
    
        let num_results = reader.read_uleb128_u32()?;
        let mut results = vec![];
        for _ in 0..num_results {
            let value_type = ValueType::from(reader.read_byte()?)?;
            results.push(value_type);
        }

        func_types.push(FuncType { params, results })
    }
    
    Ok(func_types)
}

fn decode_function_section(contents: &[u8]) -> Result<Vec<u32>, String> {
    let mut reader = BinaryReader::new(contents);

    let num_functions = reader.read_uleb128_u32()?;
    let mut func_indices = vec![];
    for _ in 0..num_functions {
        func_indices.push(reader.read_uleb128_u32()?);
    }

    Ok(func_indices)
}

fn decode_code_section(contents: &[u8]) -> Result<Vec<Function>, String> {
    let mut reader = BinaryReader::new(contents);

    let num_functions = reader.read_uleb128_u32()?;
    let mut functions = vec![];

    for _ in 0..num_functions {
        let func_body_size = reader.read_uleb128_u32()?;
        let func_body = reader.read_slice(func_body_size as usize)?;

        let function = decode_code_section_func_body(func_body);
        functions.push(function?);
    }

    Ok(functions)
}

fn decode_code_section_func_body(contents: &[u8]) -> Result<Function, String> {
    let mut reader = BinaryReader::new(contents);

    let local_decl_count = reader.read_uleb128_u32()?;
    let mut locals = vec![];
    for _ in 0..local_decl_count {
        locals.push(FunctionLocal {
            type_count: reader.read_uleb128_u32()?,
            value_type: ValueType::from(reader.read_byte()?)?,
        })
    }

    let mut instructions = vec![];
    let opcode = Opcode::from(reader.read_byte()?)?;

    instructions.push(
        match opcode {
            Opcode::End => Instruction::End,
            Opcode::LocalGet => {
                let index = reader.read_uleb128_u32()?;
                Instruction::LocalGet(index)
            },
            Opcode::I32Add => Instruction::I32Add,
        }
    );

    Ok(Function {
        locals,
        code: instructions,
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_empty_module() {
        let wasm = wat::parse_str("(module)")
            .expect("unit test input WAT invalid");
        
        let decoded = decode_wasm(&wasm).unwrap();
        let expected = Module::new();
        assert_eq!(decoded, expected);
    }

    #[test]
    fn decode_empty_single_func_module() {
        let wasm = wat::parse_str("(module (func))")
            .expect("unit test input WAT invalid");
        
        let decoded = decode_wasm(&wasm).unwrap();

        let mut expected = Module::new();
        expected.type_section = Some(vec![
            FuncType {
                params: vec![],
                results: vec![],
            }
        ]);
        expected.function_section = Some(vec![0]);
        expected.code_section = Some(vec![
            Function {
                locals: vec![],
                code: vec![Instruction::End],
            }
        ]);

        assert_eq!(decoded, expected);
    }
}