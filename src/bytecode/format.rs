use std::fs::File;
use std::io::{Read, Write};

use crate::error::CompileError;

use super::Bytecode;

mod decode;
mod encode;
mod util;

use decode::read_function;
use encode::write_function;
use util::{read_str, read_u32, write_str, write_u32};

pub fn write_bytecode(path: &str, bc: &Bytecode) -> Result<(), CompileError> {
    let mut buf = Vec::new();
    buf.extend_from_slice(b"CERB");
    buf.push(2); // version
    write_u32(&mut buf, bc.functions.len() as u32);
    write_u32(&mut buf, bc.entry);
    write_str(&mut buf, &bc.name);
    for func in &bc.functions {
        write_function(&mut buf, func);
    }
    let mut file = File::create(path)
        .map_err(|e| CompileError::new_simple(format!("write bytecode failed: {e}")))?;
    file.write_all(&buf)
        .map_err(|e| CompileError::new_simple(format!("write bytecode failed: {e}")))?;
    Ok(())
}

pub fn read_bytecode(path: &str) -> Result<Bytecode, CompileError> {
    let mut file = File::open(path)
        .map_err(|e| CompileError::new_simple(format!("read bytecode failed: {e}")))?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)
        .map_err(|e| CompileError::new_simple(format!("read bytecode failed: {e}")))?;

    let mut cursor = 0usize;
    if buf.len() < 5 || &buf[0..4] != b"CERB" {
        return Err(CompileError::new_simple("invalid bytecode header"));
    }
    cursor += 4;
    let version = buf[cursor];
    if version != 2 {
        return Err(CompileError::new_simple("unsupported bytecode version"));
    }
    cursor += 1;
    let func_count = read_u32(&buf, &mut cursor)? as usize;
    let entry = read_u32(&buf, &mut cursor)?;
    let name = read_str(&buf, &mut cursor)?;

    let mut functions = Vec::with_capacity(func_count);
    for _ in 0..func_count {
        let func = read_function(&buf, &mut cursor)?;
        functions.push(func);
    }
    Ok(Bytecode {
        name,
        functions,
        entry,
    })
}
