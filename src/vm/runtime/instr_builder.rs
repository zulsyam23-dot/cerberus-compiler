use crate::bytecode::{Instr, write_bytecode};
use crate::error::CompileError;

use super::super::ops::{pop, pop_bool, pop_int};
use super::super::value::Value;
use super::Vm;
use std::env;

impl Vm {
    pub(super) fn exec_builder(&mut self, instr: Instr) -> Result<(), CompileError> {
        match instr {
            Instr::BcNew => {
                let name = match pop(&mut self.stack)? {
                    Value::Str(s) => s,
                    _ => return Err(CompileError::new_simple("bc_new: expected string name")),
                };
                if env::var_os("CERBERUS_DEBUG_BOOTSTRAP").is_some() {
                    eprintln!("DEBUG: bc_new {}", name);
                }
                self.builder = Some(super::builder::BcBuilder::new(name));
            }
            Instr::BcMain => {
                if env::var_os("CERBERUS_DEBUG_BOOTSTRAP").is_some() {
                    eprintln!("DEBUG: bc_main");
                }
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_main: no builder"))?;
                b.begin_main()?;
            }
            Instr::BcEmitPrintStr => {
                let s = match pop(&mut self.stack)? {
                    Value::Str(s) => s,
                    _ => {
                        return Err(CompileError::new_simple(
                            "bc_emit_print_str: expected string",
                        ));
                    }
                };
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_print_str: no builder"))?;
                b.emit_print_str(s)?;
            }
            Instr::BcEmitConstInt => {
                let v = pop_int(&mut self.stack)?;
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_const_int: no builder"))?;
                b.emit_const_int(v)?;
            }
            Instr::BcEmitStore0 => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_store0: no builder"))?;
                b.emit_store0()?;
            }
            Instr::BcEmitLoad0 => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_load0: no builder"))?;
                b.emit_load0()?;
            }
            Instr::BcEmitPrintLn => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_println: no builder"))?;
                b.emit_println()?;
            }
            Instr::BcEmitConstBool => {
                let v = pop_bool(&mut self.stack)?;
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_const_bool: no builder"))?;
                b.emit_const_bool(v)?;
            }
            Instr::BcEmitConstStr => {
                let v = match pop(&mut self.stack)? {
                    Value::Str(s) => s,
                    _ => {
                        return Err(CompileError::new_simple(
                            "bc_emit_const_str: expected string",
                        ));
                    }
                };
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_const_str: no builder"))?;
                b.emit_const_str(v)?;
            }
            Instr::BcEmitLoad => {
                let idx = pop_int(&mut self.stack)?;
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_load: no builder"))?;
                b.emit_load(idx)?;
            }
            Instr::BcEmitStore => {
                let idx = pop_int(&mut self.stack)?;
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_store: no builder"))?;
                b.emit_store(idx)?;
            }
            Instr::BcEmitAdd => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_add: no builder"))?;
                b.emit_add()?;
            }
            Instr::BcEmitSub => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_sub: no builder"))?;
                b.emit_sub()?;
            }
            Instr::BcEmitMul => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_mul: no builder"))?;
                b.emit_mul()?;
            }
            Instr::BcEmitDiv => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_div: no builder"))?;
                b.emit_div()?;
            }
            Instr::BcEmitEq => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_eq: no builder"))?;
                b.emit_eq()?;
            }
            Instr::BcEmitNe => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_ne: no builder"))?;
                b.emit_ne()?;
            }
            Instr::BcEmitLt => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_lt: no builder"))?;
                b.emit_lt()?;
            }
            Instr::BcEmitLe => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_le: no builder"))?;
                b.emit_le()?;
            }
            Instr::BcEmitGt => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_gt: no builder"))?;
                b.emit_gt()?;
            }
            Instr::BcEmitGe => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_ge: no builder"))?;
                b.emit_ge()?;
            }
            Instr::BcEmitAnd => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_and: no builder"))?;
                b.emit_and()?;
            }
            Instr::BcEmitOr => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_or: no builder"))?;
                b.emit_or()?;
            }
            Instr::BcEmitNot => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_not: no builder"))?;
                b.emit_not()?;
            }
            Instr::BcEmitNeg => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_neg: no builder"))?;
                b.emit_neg()?;
            }
            Instr::BcEmitStrLen => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_strlen: no builder"))?;
                b.emit_strlen()?;
            }
            Instr::BcEmitStrConcat => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_concat: no builder"))?;
                b.emit_concat()?;
            }
            Instr::BcEmitStrSubstr => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_substr: no builder"))?;
                b.emit_substr()?;
            }
            Instr::BcEmitStrReplace => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_replace: no builder"))?;
                b.emit_replace()?;
            }
            Instr::BcEmitVecNew => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_vec_new: no builder"))?;
                b.emit_vec_new()?;
            }
            Instr::BcEmitVecLen => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_vec_len: no builder"))?;
                b.emit_vec_len()?;
            }
            Instr::BcEmitVecGet => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_vec_get: no builder"))?;
                b.emit_vec_get()?;
            }
            Instr::BcEmitVecSet => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_vec_set: no builder"))?;
                b.emit_vec_set()?;
            }
            Instr::BcEmitVecPush => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_vec_push: no builder"))?;
                b.emit_vec_push()?;
            }
            Instr::BcEmitVecRemove => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_vec_remove: no builder"))?;
                b.emit_vec_remove()?;
            }
            Instr::BcEmitVecLast => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_vec_last: no builder"))?;
                b.emit_vec_last()?;
            }
            Instr::BcEmitVecPop => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_vec_pop: no builder"))?;
                b.emit_vec_pop()?;
            }
            Instr::BcEmitMapNew => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_map_new: no builder"))?;
                b.emit_map_new()?;
            }
            Instr::BcEmitMapLen => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_map_len: no builder"))?;
                b.emit_map_len()?;
            }
            Instr::BcEmitMapSet => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_map_set: no builder"))?;
                b.emit_map_set()?;
            }
            Instr::BcEmitMapGet => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_map_get: no builder"))?;
                b.emit_map_get()?;
            }
            Instr::BcEmitMapHas => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_map_has: no builder"))?;
                b.emit_map_has()?;
            }
            Instr::BcEmitMapRemove => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_map_remove: no builder"))?;
                b.emit_map_remove()?;
            }
            Instr::BcEmitReadFile => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_readfile: no builder"))?;
                b.emit_readfile()?;
            }
            Instr::BcEmitWriteFile => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_writefile: no builder"))?;
                b.emit_writefile()?;
            }
            Instr::BcEmitArgCount => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_arg_count: no builder"))?;
                b.emit_arg_count()?;
            }
            Instr::BcEmitArg => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_arg: no builder"))?;
                b.emit_arg()?;
            }
            Instr::BcEmitStrClear => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_str_clear: no builder"))?;
                b.emit_str_clear()?;
            }
            Instr::BcEmitVecClear => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_vec_clear: no builder"))?;
                b.emit_vec_clear()?;
            }
            Instr::BcEmitMapClear => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_map_clear: no builder"))?;
                b.emit_map_clear()?;
            }
            Instr::BcEmitEnvGet => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_env_get: no builder"))?;
                b.emit_env_get()?;
            }
            Instr::BcEmitEnvHas => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_env_has: no builder"))?;
                b.emit_env_has()?;
            }
            Instr::BcEmitCwd => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_cwd: no builder"))?;
                b.emit_cwd()?;
            }
            Instr::BcEmitPathJoin => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_path_join: no builder"))?;
                b.emit_path_join()?;
            }
            Instr::BcEmitFsExists => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_fs_exists: no builder"))?;
                b.emit_fs_exists()?;
            }
            Instr::BcEmitFsListDir => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_fs_listdir: no builder"))?;
                b.emit_fs_listdir()?;
            }
            Instr::BcEmitNowTimestamp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_now_timestamp: no builder"))?;
                b.emit_now_timestamp()?;
            }
            Instr::BcEmitConstIntOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_const_int_op: no builder"))?;
                b.emit_bc_emit_const_int()?;
            }
            Instr::BcEmitConstBoolOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_const_bool_op: no builder"))?;
                b.emit_bc_emit_const_bool()?;
            }
            Instr::BcEmitConstStrOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_const_str_op: no builder"))?;
                b.emit_bc_emit_const_str()?;
            }
            Instr::BcEmitLoadOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_load_op: no builder"))?;
                b.emit_bc_emit_load()?;
            }
            Instr::BcEmitStoreOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_store_op: no builder"))?;
                b.emit_bc_emit_store()?;
            }
            Instr::BcEmitCallOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_call_op: no builder"))?;
                b.emit_bc_emit_call()?;
            }
            Instr::BcNewOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_new_op: no builder"))?;
                b.emit_bc_new()?;
            }
            Instr::BcWriteOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_write_op: no builder"))?;
                b.emit_bc_write()?;
            }
            Instr::BcLabelOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_label_op: no builder"))?;
                b.emit_bc_label()?;
            }
            Instr::BcJumpOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_jump_op: no builder"))?;
                b.emit_bc_jump()?;
            }
            Instr::BcJumpIfFalseOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_jump_if_false_op: no builder"))?;
                b.emit_bc_jump_if_false()?;
            }
            Instr::BcFuncBeginOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_func_begin_op: no builder"))?;
                b.emit_bc_func_begin()?;
            }
            Instr::BcMainOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_main_op: no builder"))?;
                b.emit_bc_main()?;
            }
            Instr::BcFuncEndOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_func_end_op: no builder"))?;
                b.emit_bc_func_end()?;
            }
            Instr::BcEmitHaltOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_halt_op: no builder"))?;
                b.emit_bc_emit_halt()?;
            }
            Instr::BcEmitRetOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_ret_op: no builder"))?;
                b.emit_bc_emit_ret()?;
            }
            Instr::BcEmitRetValOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_retval_op: no builder"))?;
                b.emit_bc_emit_retval()?;
            }
            Instr::BcEmitPrintLnOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_println_op: no builder"))?;
                b.emit_bc_emit_println()?;
            }
            Instr::BcEmitWriteFileOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_writefile_op: no builder"))?;
                b.emit_bc_emit_writefile()?;
            }
            Instr::BcEmitAddOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_add_op: no builder"))?;
                b.emit_bc_emit_add()?;
            }
            Instr::BcEmitSubOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_sub_op: no builder"))?;
                b.emit_bc_emit_sub()?;
            }
            Instr::BcEmitMulOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_mul_op: no builder"))?;
                b.emit_bc_emit_mul()?;
            }
            Instr::BcEmitDivOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_div_op: no builder"))?;
                b.emit_bc_emit_div()?;
            }
            Instr::BcEmitEqOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_eq_op: no builder"))?;
                b.emit_bc_emit_eq()?;
            }
            Instr::BcEmitNeOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_ne_op: no builder"))?;
                b.emit_bc_emit_ne()?;
            }
            Instr::BcEmitLtOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_lt_op: no builder"))?;
                b.emit_bc_emit_lt()?;
            }
            Instr::BcEmitLeOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_le_op: no builder"))?;
                b.emit_bc_emit_le()?;
            }
            Instr::BcEmitGtOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_gt_op: no builder"))?;
                b.emit_bc_emit_gt()?;
            }
            Instr::BcEmitGeOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_ge_op: no builder"))?;
                b.emit_bc_emit_ge()?;
            }
            Instr::BcEmitAndOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_and_op: no builder"))?;
                b.emit_bc_emit_and()?;
            }
            Instr::BcEmitOrOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_or_op: no builder"))?;
                b.emit_bc_emit_or()?;
            }
            Instr::BcEmitNotOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_not_op: no builder"))?;
                b.emit_bc_emit_not()?;
            }
            Instr::BcEmitNegOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_neg_op: no builder"))?;
                b.emit_bc_emit_neg()?;
            }
            Instr::BcEmitStrLenOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_strlen_op: no builder"))?;
                b.emit_bc_emit_strlen()?;
            }
            Instr::BcEmitStrConcatOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_concat_op: no builder"))?;
                b.emit_bc_emit_concat()?;
            }
            Instr::BcEmitStrSubstrOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_substr_op: no builder"))?;
                b.emit_bc_emit_substr()?;
            }
            Instr::BcEmitStrReplaceOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_replace_op: no builder"))?;
                b.emit_bc_emit_replace()?;
            }
            Instr::BcEmitVecNewOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_vec_new_op: no builder"))?;
                b.emit_bc_emit_vec_new()?;
            }
            Instr::BcEmitVecLenOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_vec_len_op: no builder"))?;
                b.emit_bc_emit_vec_len()?;
            }
            Instr::BcEmitVecGetOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_vec_get_op: no builder"))?;
                b.emit_bc_emit_vec_get()?;
            }
            Instr::BcEmitVecSetOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_vec_set_op: no builder"))?;
                b.emit_bc_emit_vec_set()?;
            }
            Instr::BcEmitVecPushOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_vec_push_op: no builder"))?;
                b.emit_bc_emit_vec_push()?;
            }
            Instr::BcEmitVecRemoveOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_vec_remove_op: no builder"))?;
                b.emit_bc_emit_vec_remove()?;
            }
            Instr::BcEmitVecLastOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_vec_last_op: no builder"))?;
                b.emit_bc_emit_vec_last()?;
            }
            Instr::BcEmitVecPopOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_vec_pop_op: no builder"))?;
                b.emit_bc_emit_vec_pop()?;
            }
            Instr::BcEmitVecClearOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_vec_clear_op: no builder"))?;
                b.emit_bc_emit_vec_clear()?;
            }
            Instr::BcEmitMapNewOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_map_new_op: no builder"))?;
                b.emit_bc_emit_map_new()?;
            }
            Instr::BcEmitMapLenOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_map_len_op: no builder"))?;
                b.emit_bc_emit_map_len()?;
            }
            Instr::BcEmitMapSetOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_map_set_op: no builder"))?;
                b.emit_bc_emit_map_set()?;
            }
            Instr::BcEmitMapGetOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_map_get_op: no builder"))?;
                b.emit_bc_emit_map_get()?;
            }
            Instr::BcEmitMapHasOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_map_has_op: no builder"))?;
                b.emit_bc_emit_map_has()?;
            }
            Instr::BcEmitMapRemoveOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_map_remove_op: no builder"))?;
                b.emit_bc_emit_map_remove()?;
            }
            Instr::BcEmitMapClearOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_map_clear_op: no builder"))?;
                b.emit_bc_emit_map_clear()?;
            }
            Instr::BcEmitReadFileOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_readfile_op: no builder"))?;
                b.emit_bc_emit_readfile()?;
            }
            Instr::BcEmitArgCountOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_arg_count_op: no builder"))?;
                b.emit_bc_emit_arg_count()?;
            }
            Instr::BcEmitArgOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_arg_op: no builder"))?;
                b.emit_bc_emit_arg()?;
            }
            Instr::BcEmitEnvGetOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_env_get_op: no builder"))?;
                b.emit_bc_emit_env_get()?;
            }
            Instr::BcEmitEnvHasOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_env_has_op: no builder"))?;
                b.emit_bc_emit_env_has()?;
            }
            Instr::BcEmitCwdOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_cwd_op: no builder"))?;
                b.emit_bc_emit_cwd()?;
            }
            Instr::BcEmitPathJoinOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_path_join_op: no builder"))?;
                b.emit_bc_emit_path_join()?;
            }
            Instr::BcEmitFsExistsOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_fs_exists_op: no builder"))?;
                b.emit_bc_emit_fs_exists()?;
            }
            Instr::BcEmitFsListDirOp => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_fs_listdir_op: no builder"))?;
                b.emit_bc_emit_fs_listdir()?;
            }
            Instr::BcEmitNowTimestampOp => {
                let b = self.builder.as_mut().ok_or_else(|| {
                    CompileError::new_simple("bc_emit_now_timestamp_op: no builder")
                })?;
                b.emit_bc_emit_now_timestamp()?;
            }
            Instr::MetaBcNew => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("meta_bc_new: no builder"))?;
                b.emit_meta_bc_new()?;
            }
            Instr::MetaBcMain => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("meta_bc_main: no builder"))?;
                b.emit_meta_bc_main()?;
            }
            Instr::MetaBcWrite => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("meta_bc_write: no builder"))?;
                b.emit_meta_bc_write()?;
            }
            Instr::MetaBcFuncBegin => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("meta_bc_func_begin: no builder"))?;
                b.emit_meta_bc_func_begin()?;
            }
            Instr::MetaBcFuncEnd => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("meta_bc_func_end: no builder"))?;
                b.emit_meta_bc_func_end()?;
            }
            Instr::MetaBcEmitHalt => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("meta_bc_emit_halt: no builder"))?;
                b.emit_meta_bc_emit_halt()?;
            }
            Instr::MetaBcLabel => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("meta_bc_label: no builder"))?;
                b.emit_meta_bc_label()?;
            }
            Instr::MetaBcJump => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("meta_bc_jump: no builder"))?;
                b.emit_meta_bc_jump()?;
            }
            Instr::MetaBcJumpIfFalse => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("meta_bc_jump_if_false: no builder"))?;
                b.emit_meta_bc_jump_if_false()?;
            }
            Instr::BcFuncBegin => {
                let param_count = pop_int(&mut self.stack)? as u32;
                let name = match pop(&mut self.stack)? {
                    Value::Str(s) => s,
                    _ => {
                        return Err(CompileError::new_simple(
                            "bc_func_begin: expected string name",
                        ));
                    }
                };
                if env::var_os("CERBERUS_DEBUG_BOOTSTRAP").is_some() {
                    eprintln!("DEBUG: bc_func_begin {} {}", name, param_count);
                }
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_func_begin: no builder"))?;
                b.begin_function(name, param_count, false)?;
            }
            Instr::BcFuncEnd => {
                if env::var_os("CERBERUS_DEBUG_BOOTSTRAP").is_some() {
                    let current = self
                        .builder
                        .as_ref()
                        .and_then(|b| b.current_name())
                        .unwrap_or("?");
                    eprintln!("DEBUG: bc_func_end {}", current);
                }
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_func_end: no builder"))?;
                b.end_function()?;
            }
            Instr::BcEmitCall => {
                let name = match pop(&mut self.stack)? {
                    Value::Str(s) => s,
                    _ => {
                        return Err(CompileError::new_simple(
                            "bc_emit_call: expected string name",
                        ));
                    }
                };
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_call: no builder"))?;
                b.emit_call_by_name(name)?;
            }
            Instr::BcEmitRet => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_ret: no builder"))?;
                b.emit_ret()?;
            }
            Instr::BcEmitRetVal => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_retval: no builder"))?;
                b.emit_retval()?;
            }
            Instr::BcLabel => {
                let stack_len = self.stack.len();
                let id = pop_int(&mut self.stack)?;
                if env::var_os("CERBERUS_DEBUG_LABELS").is_some() {
                    let func = self
                        .builder
                        .as_ref()
                        .and_then(|b| b.current_name())
                        .unwrap_or("?");
                    eprintln!(
                        "DEBUG: bc_label {} (fn={}) stack_before={}",
                        id, func, stack_len
                    );
                }
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_label: no builder"))?;
                b.emit_label(id)?;
            }
            Instr::BcJump => {
                let stack_len = self.stack.len();
                let id = pop_int(&mut self.stack)?;
                if env::var_os("CERBERUS_DEBUG_LABELS").is_some() {
                    let func = self
                        .builder
                        .as_ref()
                        .and_then(|b| b.current_name())
                        .unwrap_or("?");
                    eprintln!(
                        "DEBUG: bc_jump {} (fn={}) stack_before={}",
                        id, func, stack_len
                    );
                }
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_jump: no builder"))?;
                b.emit_jump(id)?;
            }
            Instr::BcJumpIfFalse => {
                let stack_len = self.stack.len();
                let id = pop_int(&mut self.stack)?;
                if env::var_os("CERBERUS_DEBUG_LABELS").is_some() {
                    let func = self
                        .builder
                        .as_ref()
                        .and_then(|b| b.current_name())
                        .unwrap_or("?");
                    eprintln!(
                        "DEBUG: bc_jump_if_false {} (fn={}) stack_before={}",
                        id, func, stack_len
                    );
                }
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_jump_if_false: no builder"))?;
                b.emit_jump_if_false(id)?;
            }
            Instr::BcEmitHalt => {
                let b = self
                    .builder
                    .as_mut()
                    .ok_or_else(|| CompileError::new_simple("bc_emit_halt: no builder"))?;
                b.emit_halt()?;
            }
            Instr::BcWrite => {
                let path = match pop(&mut self.stack)? {
                    Value::Str(s) => s,
                    _ => return Err(CompileError::new_simple("bc_write: expected string path")),
                };
                let b = self
                    .builder
                    .take()
                    .ok_or_else(|| CompileError::new_simple("bc_write: no builder"))?;
                let bc = b.finish()?;
                write_bytecode(&path, &bc)?;
            }
            _ => unreachable!("invalid builder instruction"),
        }
        Ok(())
    }
}
