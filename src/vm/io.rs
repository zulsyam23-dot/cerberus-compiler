use crate::error::CompileError;

pub fn read_line() -> Result<String, CompileError> {
    let mut input = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    use std::io::BufRead;
    handle
        .read_line(&mut input)
        .map_err(|e| CompileError::new_simple(format!("readln failed: {e}")))?;
    Ok(input.trim_end_matches(&['\r', '\n'][..]).to_string())
}
