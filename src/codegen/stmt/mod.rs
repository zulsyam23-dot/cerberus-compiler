mod assign;
mod block;
mod call;
mod control;
mod builtins;

use crate::ast::{Block, Stmt};
use crate::error::CompileError;

use super::Codegen;

pub fn emit_block(cg: &mut Codegen, block: &Block) -> Result<(), CompileError> {
    block::emit_block(cg, block)
}

pub(super) fn emit_stmt(cg: &mut Codegen, stmt: &Stmt) -> Result<(), CompileError> {
    match stmt {
        Stmt::Assign(a) => assign::emit_assign(cg, a),
        Stmt::AssignIndex(a) => assign::emit_assign_index(cg, a),
        Stmt::Writeln(expr) => call::emit_writeln(cg, expr),
        Stmt::Readln(names) => call::emit_readln(cg, names),
        Stmt::Call(call) => call::emit_call(cg, call),
        Stmt::Return(expr) => {
            if let Some(e) = expr {
                super::expr::emit_expr(cg, e)?;
                cg.code.push(crate::bytecode::Instr::RetVal);
            } else {
                cg.code.push(crate::bytecode::Instr::Ret);
            }
            Ok(())
        }
        Stmt::If(s) => control::emit_if(cg, s),
        Stmt::While(s) => control::emit_while(cg, s),
        Stmt::Compound(stmts) => {
            for s in stmts {
                emit_stmt(cg, s)?;
            }
            Ok(())
        }
        Stmt::Empty => Ok(()),
    }
}
