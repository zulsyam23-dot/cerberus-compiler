use crate::ast::Type;
use crate::error::CompileError;

pub fn expect_int(ty: &Type, ctx: &str) -> Result<(), CompileError> {
    match ty {
        Type::Integer => Ok(()),
        _ => Err(CompileError::new_simple(format!(
            "type error in {}: expected integer",
            ctx
        ))),
    }
}

pub fn expect_bool(ty: &Type, ctx: &str) -> Result<(), CompileError> {
    match ty {
        Type::Boolean => Ok(()),
        _ => Err(CompileError::new_simple(format!(
            "type error in {}: expected boolean",
            ctx
        ))),
    }
}

pub fn expect_printable(ty: &Type, ctx: &str) -> Result<(), CompileError> {
    if matches!(ty, Type::Integer | Type::Boolean | Type::String) {
        Ok(())
    } else {
        Err(CompileError::new_simple(format!(
            "type error in {}: expected integer, boolean, or string",
            ctx
        )))
    }
}
