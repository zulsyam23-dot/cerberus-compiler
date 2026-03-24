use crate::ast::Expr;
use crate::bytecode::Instr;
use crate::error::CompileError;

use super::super::super::expr::emit_expr;
use super::super::super::Codegen;

pub(super) fn emit_bc_emit(
    cg: &mut Codegen,
    name: &str,
    args: &[Expr],
) -> Result<bool, CompileError> {
    if name == "bc_emit_print_str" {
        if !args.is_empty() && args.len() != 1 {
            return Err(CompileError::new_simple("bc_emit_print_str expects 0 or 1 argument"));
        }
        if !args.is_empty() {
            emit_expr(cg, &args[0])?;
        }
        cg.code.push(Instr::BcEmitPrintStr);
        return Ok(true);
    }
    if name == "bc_emit_const_int" {
        if !args.is_empty() && args.len() != 1 {
            return Err(CompileError::new_simple("bc_emit_const_int expects 0 or 1 argument"));
        }
        if !args.is_empty() {
            emit_expr(cg, &args[0])?;
        }
        cg.code.push(Instr::BcEmitConstInt);
        return Ok(true);
    }
    if name == "bc_emit_store0" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_store0 expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitStore0);
        return Ok(true);
    }
    if name == "bc_emit_load0" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_load0 expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitLoad0);
        return Ok(true);
    }
    if name == "bc_emit_println" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_println expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitPrintLn);
        return Ok(true);
    }
    if name == "bc_emit_const_bool" {
        if !args.is_empty() && args.len() != 1 {
            return Err(CompileError::new_simple("bc_emit_const_bool expects 0 or 1 argument"));
        }
        if !args.is_empty() {
            emit_expr(cg, &args[0])?;
        }
        cg.code.push(Instr::BcEmitConstBool);
        return Ok(true);
    }
    if name == "bc_emit_const_str" {
        if !args.is_empty() && args.len() != 1 {
            return Err(CompileError::new_simple("bc_emit_const_str expects 0 or 1 argument"));
        }
        if !args.is_empty() {
            emit_expr(cg, &args[0])?;
        }
        cg.code.push(Instr::BcEmitConstStr);
        return Ok(true);
    }
    if name == "bc_emit_load" {
        if !args.is_empty() && args.len() != 1 {
            return Err(CompileError::new_simple("bc_emit_load expects 0 or 1 argument"));
        }
        if !args.is_empty() {
            emit_expr(cg, &args[0])?;
        }
        cg.code.push(Instr::BcEmitLoad);
        return Ok(true);
    }
    if name == "bc_emit_store" {
        if !args.is_empty() && args.len() != 1 {
            return Err(CompileError::new_simple("bc_emit_store expects 0 or 1 argument"));
        }
        if !args.is_empty() {
            emit_expr(cg, &args[0])?;
        }
        cg.code.push(Instr::BcEmitStore);
        return Ok(true);
    }
    if name == "bc_emit_add" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_add expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitAdd);
        return Ok(true);
    }
    if name == "bc_emit_sub" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_sub expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitSub);
        return Ok(true);
    }
    if name == "bc_emit_mul" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_mul expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitMul);
        return Ok(true);
    }
    if name == "bc_emit_div" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_div expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitDiv);
        return Ok(true);
    }
    if name == "bc_emit_eq" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_eq expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitEq);
        return Ok(true);
    }
    if name == "bc_emit_ne" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_ne expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitNe);
        return Ok(true);
    }
    if name == "bc_emit_lt" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_lt expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitLt);
        return Ok(true);
    }
    if name == "bc_emit_le" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_le expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitLe);
        return Ok(true);
    }
    if name == "bc_emit_gt" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_gt expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitGt);
        return Ok(true);
    }
    if name == "bc_emit_ge" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_ge expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitGe);
        return Ok(true);
    }
    if name == "bc_emit_and" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_and expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitAnd);
        return Ok(true);
    }
    if name == "bc_emit_or" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_or expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitOr);
        return Ok(true);
    }
    if name == "bc_emit_not" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_not expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitNot);
        return Ok(true);
    }
    if name == "bc_emit_neg" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_neg expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitNeg);
        return Ok(true);
    }
    if name == "bc_emit_strlen" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_strlen expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitStrLen);
        return Ok(true);
    }
    if name == "bc_emit_concat" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_concat expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitStrConcat);
        return Ok(true);
    }
    if name == "bc_emit_substr" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_substr expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitStrSubstr);
        return Ok(true);
    }
    if name == "bc_emit_replace" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_replace expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitStrReplace);
        return Ok(true);
    }
    if name == "bc_emit_vec_new" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_vec_new expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitVecNew);
        return Ok(true);
    }
    if name == "bc_emit_vec_len" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_vec_len expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitVecLen);
        return Ok(true);
    }
    if name == "bc_emit_vec_get" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_vec_get expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitVecGet);
        return Ok(true);
    }
    if name == "bc_emit_vec_set" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_vec_set expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitVecSet);
        return Ok(true);
    }
    if name == "bc_emit_vec_push" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_vec_push expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitVecPush);
        return Ok(true);
    }
    if name == "bc_emit_vec_remove" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_vec_remove expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitVecRemove);
        return Ok(true);
    }
    if name == "bc_emit_vec_last" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_vec_last expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitVecLast);
        return Ok(true);
    }
    if name == "bc_emit_vec_pop" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_vec_pop expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitVecPop);
        return Ok(true);
    }
    if name == "bc_emit_map_new" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_map_new expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitMapNew);
        return Ok(true);
    }
    if name == "bc_emit_map_len" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_map_len expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitMapLen);
        return Ok(true);
    }
    if name == "bc_emit_map_set" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_map_set expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitMapSet);
        return Ok(true);
    }
    if name == "bc_emit_map_get" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_map_get expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitMapGet);
        return Ok(true);
    }
    if name == "bc_emit_map_has" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_map_has expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitMapHas);
        return Ok(true);
    }
    if name == "bc_emit_map_remove" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_map_remove expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitMapRemove);
        return Ok(true);
    }
    if name == "bc_emit_readfile" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_readfile expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitReadFile);
        return Ok(true);
    }
    if name == "bc_emit_writefile" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_writefile expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitWriteFile);
        return Ok(true);
    }
    if name == "bc_emit_arg_count" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_arg_count expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitArgCount);
        return Ok(true);
    }
    if name == "bc_emit_arg" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_arg expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitArg);
        return Ok(true);
    }
    if name == "bc_emit_str_clear" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_str_clear expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitStrClear);
        return Ok(true);
    }
    if name == "bc_emit_vec_clear" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_vec_clear expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitVecClear);
        return Ok(true);
    }
    if name == "bc_emit_map_clear" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_map_clear expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitMapClear);
        return Ok(true);
    }
    if name == "bc_emit_env_get" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_env_get expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitEnvGet);
        return Ok(true);
    }
    if name == "bc_emit_env_has" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_env_has expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitEnvHas);
        return Ok(true);
    }
    if name == "bc_emit_cwd" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_cwd expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitCwd);
        return Ok(true);
    }
    if name == "bc_emit_path_join" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_path_join expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitPathJoin);
        return Ok(true);
    }
    if name == "bc_emit_fs_exists" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_fs_exists expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitFsExists);
        return Ok(true);
    }
    if name == "bc_emit_fs_listdir" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_fs_listdir expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitFsListDir);
        return Ok(true);
    }
    if name == "bc_emit_now_timestamp" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_now_timestamp expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitNowTimestamp);
        return Ok(true);
    }
    if name == "bc_emit_const_int_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple(
                "bc_emit_const_int_op expects 0 arguments",
            ));
        }
        cg.code.push(Instr::BcEmitConstIntOp);
        return Ok(true);
    }
    if name == "bc_emit_const_bool_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple(
                "bc_emit_const_bool_op expects 0 arguments",
            ));
        }
        cg.code.push(Instr::BcEmitConstBoolOp);
        return Ok(true);
    }
    if name == "bc_emit_const_str_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple(
                "bc_emit_const_str_op expects 0 arguments",
            ));
        }
        cg.code.push(Instr::BcEmitConstStrOp);
        return Ok(true);
    }
    if name == "bc_emit_load_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_load_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitLoadOp);
        return Ok(true);
    }
    if name == "bc_emit_store_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_store_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitStoreOp);
        return Ok(true);
    }
    if name == "bc_emit_println_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_println_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitPrintLnOp);
        return Ok(true);
    }
    if name == "bc_emit_writefile_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_writefile_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitWriteFileOp);
        return Ok(true);
    }
    if name == "bc_emit_add_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_add_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitAddOp);
        return Ok(true);
    }
    if name == "bc_emit_sub_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_sub_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitSubOp);
        return Ok(true);
    }
    if name == "bc_emit_mul_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_mul_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitMulOp);
        return Ok(true);
    }
    if name == "bc_emit_div_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_div_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitDivOp);
        return Ok(true);
    }
    if name == "bc_emit_eq_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_eq_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitEqOp);
        return Ok(true);
    }
    if name == "bc_emit_ne_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_ne_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitNeOp);
        return Ok(true);
    }
    if name == "bc_emit_lt_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_lt_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitLtOp);
        return Ok(true);
    }
    if name == "bc_emit_le_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_le_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitLeOp);
        return Ok(true);
    }
    if name == "bc_emit_gt_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_gt_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitGtOp);
        return Ok(true);
    }
    if name == "bc_emit_ge_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_ge_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitGeOp);
        return Ok(true);
    }
    if name == "bc_emit_and_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_and_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitAndOp);
        return Ok(true);
    }
    if name == "bc_emit_or_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_or_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitOrOp);
        return Ok(true);
    }
    if name == "bc_emit_not_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_not_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitNotOp);
        return Ok(true);
    }
    if name == "bc_emit_neg_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_neg_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitNegOp);
        return Ok(true);
    }
    if name == "bc_emit_strlen_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_strlen_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitStrLenOp);
        return Ok(true);
    }
    if name == "bc_emit_concat_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_concat_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitStrConcatOp);
        return Ok(true);
    }
    if name == "bc_emit_substr_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_substr_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitStrSubstrOp);
        return Ok(true);
    }
    if name == "bc_emit_replace_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_replace_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitStrReplaceOp);
        return Ok(true);
    }
    if name == "bc_emit_vec_new_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_vec_new_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitVecNewOp);
        return Ok(true);
    }
    if name == "bc_emit_vec_len_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_vec_len_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitVecLenOp);
        return Ok(true);
    }
    if name == "bc_emit_vec_get_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_vec_get_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitVecGetOp);
        return Ok(true);
    }
    if name == "bc_emit_vec_set_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_vec_set_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitVecSetOp);
        return Ok(true);
    }
    if name == "bc_emit_vec_push_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_vec_push_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitVecPushOp);
        return Ok(true);
    }
    if name == "bc_emit_vec_remove_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_vec_remove_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitVecRemoveOp);
        return Ok(true);
    }
    if name == "bc_emit_vec_last_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_vec_last_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitVecLastOp);
        return Ok(true);
    }
    if name == "bc_emit_vec_pop_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_vec_pop_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitVecPopOp);
        return Ok(true);
    }
    if name == "bc_emit_vec_clear_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_vec_clear_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitVecClearOp);
        return Ok(true);
    }
    if name == "bc_emit_map_new_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_map_new_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitMapNewOp);
        return Ok(true);
    }
    if name == "bc_emit_map_len_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_map_len_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitMapLenOp);
        return Ok(true);
    }
    if name == "bc_emit_map_set_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_map_set_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitMapSetOp);
        return Ok(true);
    }
    if name == "bc_emit_map_get_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_map_get_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitMapGetOp);
        return Ok(true);
    }
    if name == "bc_emit_map_has_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_map_has_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitMapHasOp);
        return Ok(true);
    }
    if name == "bc_emit_map_remove_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_map_remove_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitMapRemoveOp);
        return Ok(true);
    }
    if name == "bc_emit_map_clear_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_map_clear_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitMapClearOp);
        return Ok(true);
    }
    if name == "bc_emit_readfile_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_readfile_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitReadFileOp);
        return Ok(true);
    }
    if name == "bc_emit_arg_count_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_arg_count_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitArgCountOp);
        return Ok(true);
    }
    if name == "bc_emit_arg_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_arg_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitArgOp);
        return Ok(true);
    }
    if name == "bc_emit_env_get_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_env_get_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitEnvGetOp);
        return Ok(true);
    }
    if name == "bc_emit_env_has_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_env_has_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitEnvHasOp);
        return Ok(true);
    }
    if name == "bc_emit_cwd_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_cwd_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitCwdOp);
        return Ok(true);
    }
    if name == "bc_emit_path_join_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_path_join_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitPathJoinOp);
        return Ok(true);
    }
    if name == "bc_emit_fs_exists_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_fs_exists_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitFsExistsOp);
        return Ok(true);
    }
    if name == "bc_emit_fs_listdir_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_fs_listdir_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitFsListDirOp);
        return Ok(true);
    }
    if name == "bc_emit_now_timestamp_op" {
        if !args.is_empty() {
            return Err(CompileError::new_simple("bc_emit_now_timestamp_op expects 0 arguments"));
        }
        cg.code.push(Instr::BcEmitNowTimestampOp);
        return Ok(true);
    }
    Ok(false)
}
