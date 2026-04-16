use crate::error::CompileError;

use super::value::Value;

pub fn pop(stack: &mut Vec<Value>) -> Result<Value, CompileError> {
    stack
        .pop()
        .ok_or_else(|| CompileError::new_simple("stack underflow"))
}

pub fn pop_int(stack: &mut Vec<Value>) -> Result<i64, CompileError> {
    match pop(stack)? {
        Value::Int(v) => Ok(v),
        _ => Err(CompileError::new_simple("expected integer")),
    }
}

pub fn pop_bool(stack: &mut Vec<Value>) -> Result<bool, CompileError> {
    match pop(stack)? {
        Value::Bool(v) => Ok(v),
        _ => Err(CompileError::new_simple("expected boolean")),
    }
}

pub fn bin_int_checked<F: FnOnce(i64, i64) -> Option<i64>>(
    stack: &mut Vec<Value>,
    op_name: &str,
    f: F,
) -> Result<(), CompileError> {
    let b = pop_int(stack)?;
    let a = pop_int(stack)?;
    let out = f(a, b)
        .ok_or_else(|| CompileError::new_simple(format!("{}: integer overflow", op_name)))?;
    stack.push(Value::Int(out));
    Ok(())
}

pub fn unary_int_checked<F: FnOnce(i64) -> Option<i64>>(
    stack: &mut Vec<Value>,
    op_name: &str,
    f: F,
) -> Result<(), CompileError> {
    let a = pop_int(stack)?;
    let out =
        f(a).ok_or_else(|| CompileError::new_simple(format!("{}: integer overflow", op_name)))?;
    stack.push(Value::Int(out));
    Ok(())
}

pub fn bin_cmp<F: FnOnce(i64, i64) -> bool>(
    stack: &mut Vec<Value>,
    f: F,
) -> Result<(), CompileError> {
    let b = pop_int(stack)?;
    let a = pop_int(stack)?;
    stack.push(Value::Bool(f(a, b)));
    Ok(())
}

pub fn bin_bool<F: FnOnce(bool, bool) -> bool>(
    stack: &mut Vec<Value>,
    f: F,
) -> Result<(), CompileError> {
    let b = pop_bool(stack)?;
    let a = pop_bool(stack)?;
    stack.push(Value::Bool(f(a, b)));
    Ok(())
}
