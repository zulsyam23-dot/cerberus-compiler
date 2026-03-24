mod call;
mod builtins;
mod compare;

use crate::ast::{BinaryOp, Expr, UnaryOp};
use crate::bytecode::Instr;
use crate::error::CompileError;

use super::Codegen;

pub fn emit_expr(cg: &mut Codegen, expr: &Expr) -> Result<(), CompileError> {
    match expr {
        Expr::Int(v) => cg.code.push(Instr::ConstInt(*v)),
        Expr::Bool(v) => cg.code.push(Instr::ConstBool(*v)),
        Expr::Str(s) => cg.code.push(Instr::ConstStr(s.clone())),
        Expr::Var(name) => {
            let idx = cg.symbols.get(name)?;
            cg.code.push(Instr::Load(idx));
        }
        Expr::Index { name, index } => {
            let idx = cg.symbols.get(name)?;
            cg.code.push(Instr::Load(idx));
            emit_expr(cg, index)?;
            cg.code.push(Instr::LoadIndex);
        }
        Expr::Call(call) => call::emit_call_expr(cg, call)?,
        Expr::Unary { op, expr } => {
            emit_expr(cg, expr)?;
            match op {
                UnaryOp::Neg => cg.code.push(Instr::Neg),
                UnaryOp::Not => cg.code.push(Instr::Not),
            }
        }
        Expr::Binary { op, left, right } => {
            emit_expr(cg, left)?;
            emit_expr(cg, right)?;
            match op {
                BinaryOp::Add => cg.code.push(Instr::Add),
                BinaryOp::Sub => cg.code.push(Instr::Sub),
                BinaryOp::Mul => cg.code.push(Instr::Mul),
                BinaryOp::Div => cg.code.push(Instr::Div),
                BinaryOp::Eq => compare::emit_eq(cg, left, right),
                BinaryOp::Ne => compare::emit_ne(cg, left, right),
                BinaryOp::Lt => cg.code.push(Instr::Lt),
                BinaryOp::Le => cg.code.push(Instr::Le),
                BinaryOp::Gt => cg.code.push(Instr::Gt),
                BinaryOp::Ge => cg.code.push(Instr::Ge),
                BinaryOp::And => cg.code.push(Instr::And),
                BinaryOp::Or => cg.code.push(Instr::Or),
            }
        }
    }
    Ok(())
}
