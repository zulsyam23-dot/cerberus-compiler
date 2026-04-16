use crate::ast::CallExpr;
use crate::bytecode::Instr;
use crate::error::CompileError;

use super::super::super::Codegen;
use super::emit_args;

pub fn try_emit(cg: &mut Codegen, call: &CallExpr) -> Result<bool, CompileError> {
    match call.name.as_str() {
        "vector_new" | "vector_new_int" | "vector_new_bool" | "vector_new_str" => {
            if !call.args.is_empty() {
                return Err(CompileError::new_simple("vector_new_* expects 0 arguments"));
            }
            cg.code.push(Instr::VecNew);
            Ok(true)
        }
        "vector_len" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("vector_len expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::VecLen);
            Ok(true)
        }
        "vector_get" => {
            if call.args.len() != 2 {
                return Err(CompileError::new_simple("vector_get expects 2 arguments"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::VecGet);
            Ok(true)
        }
        "vector_set" => {
            if call.args.len() != 3 {
                return Err(CompileError::new_simple("vector_set expects 3 arguments"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::VecSet);
            Ok(true)
        }
        "vector_push" => {
            if call.args.len() != 2 {
                return Err(CompileError::new_simple("vector_push expects 2 arguments"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::VecPush);
            Ok(true)
        }
        "vector_remove" => {
            if call.args.len() != 2 {
                return Err(CompileError::new_simple(
                    "vector_remove expects 2 arguments",
                ));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::VecRemove);
            Ok(true)
        }
        "vector_last" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("vector_last expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::VecLast);
            Ok(true)
        }
        "vector_pop" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("vector_pop expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::VecPop);
            Ok(true)
        }
        "vector_clear" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("vector_clear expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::VecClear);
            Ok(true)
        }
        "stack_new" => {
            if !call.args.is_empty() {
                return Err(CompileError::new_simple("stack_new expects 0 arguments"));
            }
            cg.code.push(Instr::StackNew);
            Ok(true)
        }
        "stack_len" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("stack_len expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::StackLen);
            Ok(true)
        }
        "stack_push" => {
            if call.args.len() != 2 {
                return Err(CompileError::new_simple("stack_push expects 2 arguments"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::StackPush);
            Ok(true)
        }
        "stack_top" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("stack_top expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::StackTop);
            Ok(true)
        }
        "stack_pop" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("stack_pop expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::StackPop);
            Ok(true)
        }
        "stack_clear" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("stack_clear expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::StackClear);
            Ok(true)
        }
        "map_new" => {
            if !call.args.is_empty() {
                return Err(CompileError::new_simple("map_new expects 0 arguments"));
            }
            cg.code.push(Instr::MapNew);
            Ok(true)
        }
        "map_len" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("map_len expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::MapLen);
            Ok(true)
        }
        "map_set" => {
            if call.args.len() != 3 {
                return Err(CompileError::new_simple("map_set expects 3 arguments"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::MapSet);
            Ok(true)
        }
        "map_get" => {
            if call.args.len() != 2 {
                return Err(CompileError::new_simple("map_get expects 2 arguments"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::MapGet);
            Ok(true)
        }
        "map_has" => {
            if call.args.len() != 2 {
                return Err(CompileError::new_simple("map_has expects 2 arguments"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::MapHas);
            Ok(true)
        }
        "map_remove" => {
            if call.args.len() != 2 {
                return Err(CompileError::new_simple("map_remove expects 2 arguments"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::MapRemove);
            Ok(true)
        }
        "map_clear" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("map_clear expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::MapClear);
            Ok(true)
        }
        "set_new" => {
            if !call.args.is_empty() {
                return Err(CompileError::new_simple("set_new expects 0 arguments"));
            }
            cg.code.push(Instr::SetNew);
            Ok(true)
        }
        "set_len" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("set_len expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::SetLen);
            Ok(true)
        }
        "set_add" => {
            if call.args.len() != 2 {
                return Err(CompileError::new_simple("set_add expects 2 arguments"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::SetAdd);
            Ok(true)
        }
        "set_has" => {
            if call.args.len() != 2 {
                return Err(CompileError::new_simple("set_has expects 2 arguments"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::SetHas);
            Ok(true)
        }
        "set_remove" => {
            if call.args.len() != 2 {
                return Err(CompileError::new_simple("set_remove expects 2 arguments"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::SetRemove);
            Ok(true)
        }
        "set_clear" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("set_clear expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::SetClear);
            Ok(true)
        }
        _ => Ok(false),
    }
}
