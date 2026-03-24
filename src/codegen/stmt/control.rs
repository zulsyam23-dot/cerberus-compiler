use crate::ast::{IfStmt, WhileStmt};
use crate::bytecode::Instr;
use crate::error::CompileError;

use super::super::expr::emit_expr;
use super::super::Codegen;
use super::emit_stmt;

pub(super) fn emit_if(cg: &mut Codegen, s: &IfStmt) -> Result<(), CompileError> {
    emit_expr(cg, &s.cond)?;
    let jmp_false_pos = cg.code.len();
    cg.code.push(Instr::JumpIfFalse(0));
    emit_stmt(cg, &s.then_branch)?;
    if let Some(else_branch) = &s.else_branch {
        let jmp_end_pos = cg.code.len();
        cg.code.push(Instr::Jump(0));
        let after_then = cg.code.len() as u32;
        patch_jump(cg, jmp_false_pos, after_then);
        emit_stmt(cg, else_branch)?;
        let after_else = cg.code.len() as u32;
        patch_jump(cg, jmp_end_pos, after_else);
    } else {
        let after_then = cg.code.len() as u32;
        patch_jump(cg, jmp_false_pos, after_then);
    }
    Ok(())
}

pub(super) fn emit_while(cg: &mut Codegen, s: &WhileStmt) -> Result<(), CompileError> {
    let loop_start = cg.code.len() as u32;
    emit_expr(cg, &s.cond)?;
    let jmp_false_pos = cg.code.len();
    cg.code.push(Instr::JumpIfFalse(0));
    emit_stmt(cg, &s.body)?;
    cg.code.push(Instr::Jump(loop_start));
    let after_loop = cg.code.len() as u32;
    patch_jump(cg, jmp_false_pos, after_loop);
    Ok(())
}

fn patch_jump(cg: &mut Codegen, pos: usize, target: u32) {
    match cg.code[pos] {
        Instr::Jump(_) => cg.code[pos] = Instr::Jump(target),
        Instr::JumpIfFalse(_) => cg.code[pos] = Instr::JumpIfFalse(target),
        _ => {}
    }
}
