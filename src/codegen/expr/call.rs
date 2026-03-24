use crate::ast::CallExpr;
use crate::bytecode::Instr;
use crate::error::CompileError;

use super::builtins;
use super::super::Codegen;

pub fn emit_call_expr(cg: &mut Codegen, call: &CallExpr) -> Result<(), CompileError> {
    if builtins::try_emit(cg, call)? {
        return Ok(());
    }
    for arg in &call.args {
        super::emit_expr(cg, arg)?;
    }
    let idx = cg.register_function(&call.name, call.args.len() as u32)?;
    cg.code.push(Instr::Call(idx as u32));
    Ok(())
}
