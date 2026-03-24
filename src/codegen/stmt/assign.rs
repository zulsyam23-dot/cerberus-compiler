use crate::ast::{Assign, AssignIndex};
use crate::bytecode::Instr;
use crate::error::CompileError;

use super::super::expr::emit_expr;
use super::super::Codegen;

pub(super) fn emit_assign(cg: &mut Codegen, a: &Assign) -> Result<(), CompileError> {
    let idx = cg.symbols.get(&a.name)?;
    emit_expr(cg, &a.expr)?;
    cg.code.push(Instr::Store(idx));
    Ok(())
}

pub(super) fn emit_assign_index(cg: &mut Codegen, a: &AssignIndex) -> Result<(), CompileError> {
    let idx = cg.symbols.get(&a.name)?;
    cg.code.push(Instr::Load(idx));
    emit_expr(cg, &a.index)?;
    emit_expr(cg, &a.expr)?;
    cg.code.push(Instr::StoreIndex);
    cg.code.push(Instr::Store(idx));
    Ok(())
}
