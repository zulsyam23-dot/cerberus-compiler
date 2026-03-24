use crate::ast::CallExpr;
use crate::bytecode::Instr;
use crate::error::CompileError;

use super::super::super::Codegen;
use super::emit_args;

pub fn try_emit(cg: &mut Codegen, call: &CallExpr) -> Result<bool, CompileError> {
    match call.name.as_str() {
        "strlen" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("strlen expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::StrLen);
            Ok(true)
        }
        "substr" => {
            if call.args.len() != 3 {
                return Err(CompileError::new_simple("substr expects 3 arguments"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::StrSubstr);
            Ok(true)
        }
        "replace" => {
            if call.args.len() != 3 {
                return Err(CompileError::new_simple("replace expects 3 arguments"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::StrReplace);
            Ok(true)
        }
        "concat" => {
            if call.args.len() != 2 {
                return Err(CompileError::new_simple("concat expects 2 arguments"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::StrConcat);
            Ok(true)
        }
        "string_clear" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("string_clear expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::StrClear);
            Ok(true)
        }
        _ => Ok(false),
    }
}
