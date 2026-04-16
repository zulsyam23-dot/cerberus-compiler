use crate::ast::CallExpr;
use crate::bytecode::Instr;
use crate::error::CompileError;

use super::super::super::Codegen;
use super::emit_args;

pub fn try_emit(cg: &mut Codegen, call: &CallExpr) -> Result<bool, CompileError> {
    match call.name.as_str() {
        "env_get" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("env_get expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::EnvGet);
            Ok(true)
        }
        "env_has" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("env_has expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::EnvHas);
            Ok(true)
        }
        "cwd" => {
            if !call.args.is_empty() {
                return Err(CompileError::new_simple("cwd expects 0 arguments"));
            }
            cg.code.push(Instr::Cwd);
            Ok(true)
        }
        "path_join" => {
            if call.args.len() != 2 {
                return Err(CompileError::new_simple("path_join expects 2 arguments"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::PathJoin);
            Ok(true)
        }
        "fs_exists" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("fs_exists expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::FsExists);
            Ok(true)
        }
        "fs_listdir" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("fs_listdir expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::FsListDir);
            Ok(true)
        }
        "now_timestamp" => {
            if !call.args.is_empty() {
                return Err(CompileError::new_simple(
                    "now_timestamp expects 0 arguments",
                ));
            }
            cg.code.push(Instr::NowTimestamp);
            Ok(true)
        }
        _ => Ok(false),
    }
}
