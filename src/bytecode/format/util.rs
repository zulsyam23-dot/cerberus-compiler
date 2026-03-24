use crate::error::CompileError;

pub(super) fn write_u32(buf: &mut Vec<u8>, v: u32) {
    buf.extend_from_slice(&v.to_le_bytes());
}

pub(super) fn write_i64(buf: &mut Vec<u8>, v: i64) {
    buf.extend_from_slice(&v.to_le_bytes());
}

pub(super) fn write_str(buf: &mut Vec<u8>, s: &str) {
    let bytes = s.as_bytes();
    write_u32(buf, bytes.len() as u32);
    buf.extend_from_slice(bytes);
}

pub(super) fn read_u8(buf: &[u8], cursor: &mut usize) -> Result<u8, CompileError> {
    if *cursor >= buf.len() {
        return Err(CompileError::new_simple("unexpected end of bytecode"));
    }
    let v = buf[*cursor];
    *cursor += 1;
    Ok(v)
}

pub(super) fn read_u32(buf: &[u8], cursor: &mut usize) -> Result<u32, CompileError> {
    if *cursor + 4 > buf.len() {
        return Err(CompileError::new_simple("unexpected end of bytecode"));
    }
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&buf[*cursor..*cursor + 4]);
    *cursor += 4;
    Ok(u32::from_le_bytes(bytes))
}

pub(super) fn read_i64(buf: &[u8], cursor: &mut usize) -> Result<i64, CompileError> {
    if *cursor + 8 > buf.len() {
        return Err(CompileError::new_simple("unexpected end of bytecode"));
    }
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&buf[*cursor..*cursor + 8]);
    *cursor += 8;
    Ok(i64::from_le_bytes(bytes))
}

pub(super) fn read_str(buf: &[u8], cursor: &mut usize) -> Result<String, CompileError> {
    let len = read_u32(buf, cursor)? as usize;
    if *cursor + len > buf.len() {
        return Err(CompileError::new_simple("unexpected end of bytecode"));
    }
    let s = std::str::from_utf8(&buf[*cursor..*cursor + len])
        .map_err(|_| CompileError::new_simple("invalid bytecode string"))?;
    *cursor += len;
    Ok(s.to_string())
}
