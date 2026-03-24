use crate::ast::CallExpr;
use crate::bytecode::Instr;
use crate::error::CompileError;

use super::super::super::Codegen;
use super::emit_args;

pub fn try_emit(cg: &mut Codegen, call: &CallExpr) -> Result<bool, CompileError> {
    match call.name.as_str() {
        "option_some_int" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("option_some_int expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::OptSomeInt);
            Ok(true)
        }
        "option_some_bool" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("option_some_bool expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::OptSomeBool);
            Ok(true)
        }
        "option_some_str" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("option_some_str expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::OptSomeStr);
            Ok(true)
        }
        "option_none_int" => {
            if !call.args.is_empty() {
                return Err(CompileError::new_simple("option_none_int expects 0 arguments"));
            }
            cg.code.push(Instr::OptNoneInt);
            Ok(true)
        }
        "option_none_bool" => {
            if !call.args.is_empty() {
                return Err(CompileError::new_simple("option_none_bool expects 0 arguments"));
            }
            cg.code.push(Instr::OptNoneBool);
            Ok(true)
        }
        "option_none_str" => {
            if !call.args.is_empty() {
                return Err(CompileError::new_simple("option_none_str expects 0 arguments"));
            }
            cg.code.push(Instr::OptNoneStr);
            Ok(true)
        }
        "option_is_some_int" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("option_is_some_int expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::OptIsSomeInt);
            Ok(true)
        }
        "option_is_some_bool" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("option_is_some_bool expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::OptIsSomeBool);
            Ok(true)
        }
        "option_is_some_str" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("option_is_some_str expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::OptIsSomeStr);
            Ok(true)
        }
        "option_unwrap_int" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("option_unwrap_int expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::OptUnwrapInt);
            Ok(true)
        }
        "option_unwrap_bool" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("option_unwrap_bool expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::OptUnwrapBool);
            Ok(true)
        }
        "option_unwrap_str" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("option_unwrap_str expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::OptUnwrapStr);
            Ok(true)
        }
        "option_unwrap_or_int" => {
            if call.args.len() != 2 {
                return Err(CompileError::new_simple("option_unwrap_or_int expects 2 arguments"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::OptUnwrapOrInt);
            Ok(true)
        }
        "option_unwrap_or_bool" => {
            if call.args.len() != 2 {
                return Err(CompileError::new_simple("option_unwrap_or_bool expects 2 arguments"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::OptUnwrapOrBool);
            Ok(true)
        }
        "option_unwrap_or_str" => {
            if call.args.len() != 2 {
                return Err(CompileError::new_simple("option_unwrap_or_str expects 2 arguments"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::OptUnwrapOrStr);
            Ok(true)
        }
        "result_ok_int" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("result_ok_int expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::ResOkInt);
            Ok(true)
        }
        "result_ok_bool" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("result_ok_bool expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::ResOkBool);
            Ok(true)
        }
        "result_ok_str" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("result_ok_str expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::ResOkStr);
            Ok(true)
        }
        "result_err_int" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("result_err_int expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::ResErrInt);
            Ok(true)
        }
        "result_err_bool" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("result_err_bool expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::ResErrBool);
            Ok(true)
        }
        "result_err_str" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("result_err_str expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::ResErrStr);
            Ok(true)
        }
        "result_is_ok_int" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("result_is_ok_int expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::ResIsOkInt);
            Ok(true)
        }
        "result_is_ok_bool" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("result_is_ok_bool expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::ResIsOkBool);
            Ok(true)
        }
        "result_is_ok_str" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("result_is_ok_str expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::ResIsOkStr);
            Ok(true)
        }
        "result_unwrap_int" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("result_unwrap_int expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::ResUnwrapInt);
            Ok(true)
        }
        "result_unwrap_bool" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("result_unwrap_bool expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::ResUnwrapBool);
            Ok(true)
        }
        "result_unwrap_str" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("result_unwrap_str expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::ResUnwrapStr);
            Ok(true)
        }
        "result_unwrap_or_int" => {
            if call.args.len() != 2 {
                return Err(CompileError::new_simple("result_unwrap_or_int expects 2 arguments"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::ResUnwrapOrInt);
            Ok(true)
        }
        "result_unwrap_or_bool" => {
            if call.args.len() != 2 {
                return Err(CompileError::new_simple("result_unwrap_or_bool expects 2 arguments"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::ResUnwrapOrBool);
            Ok(true)
        }
        "result_unwrap_or_str" => {
            if call.args.len() != 2 {
                return Err(CompileError::new_simple("result_unwrap_or_str expects 2 arguments"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::ResUnwrapOrStr);
            Ok(true)
        }
        "result_unwrap_err_int" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("result_unwrap_err_int expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::ResUnwrapErrInt);
            Ok(true)
        }
        "result_unwrap_err_bool" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("result_unwrap_err_bool expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::ResUnwrapErrBool);
            Ok(true)
        }
        "result_unwrap_err_str" => {
            if call.args.len() != 1 {
                return Err(CompileError::new_simple("result_unwrap_err_str expects 1 argument"));
            }
            emit_args(cg, &call.args)?;
            cg.code.push(Instr::ResUnwrapErrStr);
            Ok(true)
        }
        _ => Ok(false),
    }
}
