use crate::ast::CallExpr;
use crate::bytecode::Instr;
use crate::error::CompileError;

use super::super::super::Codegen;
use super::emit_args;

pub fn try_emit(cg: &mut Codegen, call: &CallExpr) -> Result<bool, CompileError> {
    match call.name.as_str() {
        "readfile" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("readfile expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::ReadFile);
            Ok(true)
        }
        "arg_count" => {
            if !call.args.is_empty() {
                return Err(CompileError::new_simple("arg_count expects 0 arguments"));
            }
            cg.code.push(Instr::ArgCount);
            Ok(true)
        }
        "arg" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("arg expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::Arg);
            Ok(true)
        }
        _ => Ok(false),
    }
}
