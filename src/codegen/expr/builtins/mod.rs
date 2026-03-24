mod collections;
mod io;
mod option_result;
mod string;
mod runtime;

use crate::ast::CallExpr;
use crate::error::CompileError;

use super::super::Codegen;

pub fn try_emit(cg: &mut Codegen, call: &CallExpr) -> Result<bool, CompileError> {
    if io::try_emit(cg, call)? {
        return Ok(true);
    }
    if string::try_emit(cg, call)? {
        return Ok(true);
    }
    if collections::try_emit(cg, call)? {
        return Ok(true);
    }
    if option_result::try_emit(cg, call)? {
        return Ok(true);
    }
    if runtime::try_emit(cg, call)? {
        return Ok(true);
    }
    Ok(false)
}

pub(super) fn emit_args(
    cg: &mut Codegen,
    args: &[crate::ast::Expr],
) -> Result<(), CompileError> {
    for arg in args {
        super::emit_expr(cg, arg)?;
    }
    Ok(())
}
