mod bc_emit;

use crate::ast::Expr;
use crate::bytecode::Instr;
use crate::error::CompileError;

use super::super::expr::emit_expr;
use super::super::Codegen;

pub(super) fn emit_builtin_call(
    cg: &mut Codegen,
    name: &str,
    args: &[Expr],
) -> Result<bool, CompileError> {
    if emit_basic(cg, name, args)? {
        return Ok(true);
    }
    if emit_bc_control(cg, name, args)? {
        return Ok(true);
    }
    bc_emit::emit_bc_emit(cg, name, args)
}

fn emit_basic(cg: &mut Codegen, name: &str, args: &[Expr]) -> Result<bool, CompileError> {
    if emit_meta(cg, name, args)? {
        return Ok(true);
    }
    if name == "writefile" {
        if args.len() != 2 {
            return Err(CompileError::new_simple("writefile expects 2 arguments"));
        }
        for arg in args {
            emit_expr(cg, arg)?;
        }
        cg.code.push(Instr::WriteFile);
        return Ok(true);
    }
    if name == "sleep_ms" {
        if args.len() != 1 {
            return Err(CompileError::new_simple("sleep_ms expects 1 argument"));
        }
        emit_expr(cg, &args[0])?;
        cg.code.push(Instr::SleepMs);
        return Ok(true);
    }
    if name == "log_str" {
        if args.len() != 1 {
            return Err(CompileError::new_simple("log_str expects 1 argument"));
        }
        emit_expr(cg, &args[0])?;
        cg.code.push(Instr::LogStr);
        return Ok(true);
    }
    if name == "log_int" {
        if args.len() != 1 {
            return Err(CompileError::new_simple("log_int expects 1 argument"));
        }
        emit_expr(cg, &args[0])?;
        cg.code.push(Instr::LogInt);
        return Ok(true);
    }
    if name == "log_bool" {
        if args.len() != 1 {
            return Err(CompileError::new_simple("log_bool expects 1 argument"));
        }
        emit_expr(cg, &args[0])?;
        cg.code.push(Instr::LogBool);
        return Ok(true);
    }
    Ok(false)
}

fn emit_meta(cg: &mut Codegen, name: &str, args: &[Expr]) -> Result<bool, CompileError> {
    if name == "emit_bcop_new" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("emit_bcop_new expects 0 arguments"));
        }
        cg.code.push(Instr::MetaBcNew);
        return Ok(true);
    }
    if name == "emit_bcop_main" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("emit_bcop_main expects 0 arguments"));
        }
        cg.code.push(Instr::MetaBcMain);
        return Ok(true);
    }
    if name == "emit_bcop_write" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("emit_bcop_write expects 0 arguments"));
        }
        cg.code.push(Instr::MetaBcWrite);
        return Ok(true);
    }
    if name == "emit_bcop_func_begin" {
        if !args.is_empty() {
            return Err(CompileError::new_simple(
                "emit_bcop_func_begin expects 0 arguments",
            ));
        }
        cg.code.push(Instr::MetaBcFuncBegin);
        return Ok(true);
    }
    if name == "emit_bcop_func_end" {
        if !args.is_empty() {
            return Err(CompileError::new_simple(
                "emit_bcop_func_end expects 0 arguments",
            ));
        }
        cg.code.push(Instr::MetaBcFuncEnd);
        return Ok(true);
    }
    if name == "emit_bcop_halt" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("emit_bcop_halt expects 0 arguments"));
        }
        cg.code.push(Instr::MetaBcEmitHalt);
        return Ok(true);
    }
    if name == "emit_bcop_label" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("emit_bcop_label expects 0 arguments"));
        }
        cg.code.push(Instr::MetaBcLabel);
        return Ok(true);
    }
    if name == "emit_bcop_jump" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("emit_bcop_jump expects 0 arguments"));
        }
        cg.code.push(Instr::MetaBcJump);
        return Ok(true);
    }
    if name == "emit_bcop_jump_if_false" {
        if !args.is_empty() {
            return Err(CompileError::new_simple(
                "emit_bcop_jump_if_false expects 0 arguments",
            ));
        }
        cg.code.push(Instr::MetaBcJumpIfFalse);
        return Ok(true);
    }
    Ok(false)
}

fn emit_bc_control(cg: &mut Codegen, name: &str, args: &[Expr]) -> Result<bool, CompileError> {
    if name == "builder_new" {
        if args.len() != 1 {
            return Err(CompileError::new_simple("builder_new expects 1 argument"));
        }
        emit_expr(cg, &args[0])?;
        cg.code.push(Instr::BcNew);
        return Ok(true);
    }
    if name == "builder_main" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("builder_main expects 0 arguments"));
        }
        cg.code.push(Instr::BcMain);
        return Ok(true);
    }
    if name == "builder_write" {
        if args.len() != 1 {
            return Err(CompileError::new_simple("builder_write expects 1 argument"));
        }
        emit_expr(cg, &args[0])?;
        cg.code.push(Instr::BcWrite);
        return Ok(true);
    }
    if name == "builder_func_begin" {
        if args.len() != 2 {
            return Err(CompileError::new_simple("builder_func_begin expects 2 arguments"));
        }
        for arg in args {
            emit_expr(cg, arg)?;
        }
        cg.code.push(Instr::BcFuncBegin);
        return Ok(true);
    }
    if name == "builder_func_end" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("builder_func_end expects 0 arguments"));
        }
        cg.code.push(Instr::BcFuncEnd);
        return Ok(true);
    }
    if name == "builder_emit_halt" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("builder_emit_halt expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitHalt);
        return Ok(true);
    }
    if name == "builder_label" {
        if args.len() != 1 {
            return Err(CompileError::new_simple("builder_label expects 1 argument"));
        }
        emit_expr(cg, &args[0])?;
        cg.code.push(Instr::BcLabel);
        return Ok(true);
    }
    if name == "builder_jump" {
        if args.len() != 1 {
            return Err(CompileError::new_simple("builder_jump expects 1 argument"));
        }
        emit_expr(cg, &args[0])?;
        cg.code.push(Instr::BcJump);
        return Ok(true);
    }
    if name == "builder_jump_if_false" {
        if args.len() != 1 {
            return Err(CompileError::new_simple("builder_jump_if_false expects 1 argument"));
        }
        emit_expr(cg, &args[0])?;
        cg.code.push(Instr::BcJumpIfFalse);
        return Ok(true);
    }
    if name == "bc_new" {
        if !args.is_empty() && args.len() != 1 {
            return Err(CompileError::new_simple("bc_new expects 0 or 1 argument"));
        }
        if !args.is_empty() {
            emit_expr(cg, &args[0])?;
        }
        cg.code.push(Instr::BcNew);
        return Ok(true);
    }
    if name == "bc_new_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_new_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcNewOp);
        return Ok(true);
    }
    if name == "bc_main" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_main expects 0 arguments"));
        }
        cg.code.push(Instr::BcMain);
        return Ok(true);
    }
    if name == "bc_main_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_main_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcMainOp);
        return Ok(true);
    }
    if name == "bc_func_begin" {
        if !args.is_empty() && args.len() != 2 {
            return Err(CompileError::new_simple("bc_func_begin expects 0 or 2 arguments"));
        }
        for arg in args {
            emit_expr(cg, arg)?;
        }
        cg.code.push(Instr::BcFuncBegin);
        return Ok(true);
    }
    if name == "bc_func_begin_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_func_begin_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcFuncBeginOp);
        return Ok(true);
    }
    if name == "bc_func_end" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_func_end expects 0 arguments"));
        }
        cg.code.push(Instr::BcFuncEnd);
        return Ok(true);
    }
    if name == "bc_func_end_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_func_end_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcFuncEndOp);
        return Ok(true);
    }
    if name == "bc_emit_call" {
        if !args.is_empty() && args.len() != 1 {
            return Err(CompileError::new_simple("bc_emit_call expects 0 or 1 argument"));
        }
        if !args.is_empty() {
            emit_expr(cg, &args[0])?;
        }
        cg.code.push(Instr::BcEmitCall);
        return Ok(true);
    }
    if name == "bc_emit_call_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_call_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitCallOp);
        return Ok(true);
    }
    if name == "bc_emit_ret" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_ret expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitRet);
        return Ok(true);
    }
    if name == "bc_emit_ret_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_ret_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitRetOp);
        return Ok(true);
    }
    if name == "bc_emit_retval" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_retval expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitRetVal);
        return Ok(true);
    }
    if name == "bc_emit_retval_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_retval_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitRetValOp);
        return Ok(true);
    }
    if name == "bc_label_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_label_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcLabelOp);
        return Ok(true);
    }
    if name == "bc_jump_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_jump_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcJumpOp);
        return Ok(true);
    }
    if name == "bc_jump_if_false_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_jump_if_false_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcJumpIfFalseOp);
        return Ok(true);
    }
    if name == "bc_label" {
        if !args.is_empty() && args.len() != 1 {
            return Err(CompileError::new_simple("bc_label expects 0 or 1 argument"));
        }
        if !args.is_empty() {
            emit_expr(cg, &args[0])?;
        }
        cg.code.push(Instr::BcLabel);
        return Ok(true);
    }
    if name == "bc_jump" {
        if !args.is_empty() && args.len() != 1 {
            return Err(CompileError::new_simple("bc_jump expects 0 or 1 argument"));
        }
        if !args.is_empty() {
            emit_expr(cg, &args[0])?;
        }
        cg.code.push(Instr::BcJump);
        return Ok(true);
    }
    if name == "bc_jump_if_false" {
        if !args.is_empty() && args.len() != 1 {
            return Err(CompileError::new_simple("bc_jump_if_false expects 0 or 1 argument"));
        }
        if !args.is_empty() {
            emit_expr(cg, &args[0])?;
        }
        cg.code.push(Instr::BcJumpIfFalse);
        return Ok(true);
    }
    if name == "bc_emit_halt" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_halt expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitHalt);
        return Ok(true);
    }
    if name == "bc_emit_halt_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_halt_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitHaltOp);
        return Ok(true);
    }
    if name == "bc_write" {
        if !args.is_empty() && args.len() != 1 {
            return Err(CompileError::new_simple("bc_write expects 0 or 1 argument"));
        }
        if !args.is_empty() {
            emit_expr(cg, &args[0])?;
        }
        cg.code.push(Instr::BcWrite);
        return Ok(true);
    }
    if name == "bc_write_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_write_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcWriteOp);
        return Ok(true);
    }
    Ok(false)
}
