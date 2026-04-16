use crate::ast::{CallStmt, Expr};
use crate::bytecode::Instr;
use crate::error::CompileError;

use super::super::Codegen;
use super::super::expr::emit_expr;
use super::builtins;

pub(super) fn emit_writeln(cg: &mut Codegen, expr: &Expr) -> Result<(), CompileError> {
    emit_expr(cg, expr)?;
    cg.code.push(Instr::PrintLn);
    Ok(())
}

pub(super) fn emit_readln(cg: &mut Codegen, names: &[String]) -> Result<(), CompileError> {
    for name in names {
        let idx = cg.symbols.get(name)?;
        let ty = cg
            .symbols
            .get_type(name)
            .ok_or_else(|| CompileError::new_simple(format!("unknown variable '{}'", name)))?;
        match ty {
            crate::ast::Type::Integer => cg.code.push(Instr::ReadInt(idx)),
            crate::ast::Type::Boolean => cg.code.push(Instr::ReadBool(idx)),
            crate::ast::Type::String => cg.code.push(Instr::ReadStr(idx)),
            crate::ast::Type::Array { .. } => {
                return Err(CompileError::new_simple(
                    "readln does not support array types",
                ));
            }
            crate::ast::Type::Vector(_) => {
                return Err(CompileError::new_simple(
                    "readln does not support vector types",
                ));
            }
            crate::ast::Type::StackInt => {
                return Err(CompileError::new_simple(
                    "readln does not support stack types",
                ));
            }
            crate::ast::Type::MapStrStr => {
                return Err(CompileError::new_simple(
                    "readln does not support map types",
                ));
            }
            crate::ast::Type::SetStr => {
                return Err(CompileError::new_simple(
                    "readln does not support set types",
                ));
            }
            crate::ast::Type::Option(_) => {
                return Err(CompileError::new_simple(
                    "readln does not support option types",
                ));
            }
            crate::ast::Type::Result(_) => {
                return Err(CompileError::new_simple(
                    "readln does not support result types",
                ));
            }
        }
    }
    Ok(())
}

pub(super) fn emit_call(cg: &mut Codegen, call: &CallStmt) -> Result<(), CompileError> {
    if builtins::emit_builtin_call(cg, &call.name, &call.args)? {
        return Ok(());
    }
    for arg in &call.args {
        emit_expr(cg, arg)?;
    }
    let idx = cg.register_function(&call.name, call.args.len() as u32)?;
    cg.code.push(Instr::Call(idx as u32));
    Ok(())
}
