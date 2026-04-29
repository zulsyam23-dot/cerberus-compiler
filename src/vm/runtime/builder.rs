use crate::bytecode::{Bytecode, Function, Instr};
use crate::error::CompileError;
use std::collections::{HashMap, HashSet};

use super::intrinsics;

pub(super) struct BcBuilder {
    name: String,
    functions: Vec<Function>,
    func_index: HashMap<String, u32>,
    defined: HashSet<String>,
    current: Option<BuildFn>,
    entry: Option<u32>,
}

struct BuildFn {
    name: String,
    param_count: u32,
    code: Vec<Instr>,
    labels: HashMap<i64, u32>,
    fixups: Vec<Fixup>,
    locals_max: u32,
}

struct Fixup {
    pos: usize,
    label: i64,
    kind: FixupKind,
}

enum FixupKind {
    Jump,
    JumpIfFalse,
}

impl BcBuilder {
    pub(super) fn new(name: String) -> Self {
        Self {
            name,
            functions: Vec::new(),
            func_index: HashMap::new(),
            defined: HashSet::new(),
            current: None,
            entry: None,
        }
    }

    pub(super) fn current_name(&self) -> Option<&str> {
        self.current.as_ref().map(|cur| cur.name.as_str())
    }

    pub(super) fn begin_main(&mut self) -> Result<(), CompileError> {
        self.begin_function("main".to_string(), 0, true)
    }

    pub(super) fn begin_function(
        &mut self,
        name: String,
        param_count: u32,
        is_entry: bool,
    ) -> Result<(), CompileError> {
        if self.current.is_some() {
            return Err(CompileError::new_simple(
                "bc_func_begin: function already open",
            ));
        }
        let idx = if let Some(existing) = self.func_index.get(&name).copied() {
            if self.defined.contains(&name) {
                return Err(CompileError::new_simple(format!(
                    "bc_func_begin: function '{}' already defined",
                    name
                )));
            }
            existing
        } else {
            let idx = self.functions.len() as u32;
            self.func_index.insert(name.clone(), idx);
            self.functions.push(Function {
                name: name.clone(),
                param_count,
                locals: 0,
                code: Vec::new(),
            });
            idx
        };
        if is_entry {
            self.entry = Some(idx);
        }
        self.current = Some(BuildFn {
            name,
            param_count,
            code: Vec::new(),
            labels: HashMap::new(),
            fixups: Vec::new(),
            locals_max: param_count,
        });
        Ok(())
    }

    pub(super) fn end_function(&mut self) -> Result<(), CompileError> {
        let build = self
            .current
            .take()
            .ok_or_else(|| CompileError::new_simple("bc_func_end: no open function"))?;
        let func = self.finalize_function(build)?;
        let idx = *self
            .func_index
            .get(&func.name)
            .ok_or_else(|| CompileError::new_simple("bc_func_end: unknown function"))?;
        if let Some(slot) = self.functions.get_mut(idx as usize) {
            *slot = func;
        }
        self.defined
            .insert(self.functions[idx as usize].name.clone());
        Ok(())
    }

    fn current_mut(&mut self) -> Result<&mut BuildFn, CompileError> {
        self.current.as_mut().ok_or_else(|| {
            CompileError::new_simple(
                "bc_emit: no open function; self-host bootstrap likely hit a dynamic bc_func_begin/bc_new/bc_write path that is not lowered yet",
            )
        })
    }

    pub(super) fn emit_print_str(&mut self, s: String) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::ConstStr(s));
        cur.code.push(Instr::PrintLn);
        Ok(())
    }

    pub(super) fn emit_halt(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::Halt);
        Ok(())
    }

    pub(super) fn emit_const_int(&mut self, v: i64) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::ConstInt(v));
        Ok(())
    }

    pub(super) fn emit_store0(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::Store(0));
        Ok(())
    }

    pub(super) fn emit_load0(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::Load(0));
        Ok(())
    }

    pub(super) fn emit_println(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::PrintLn);
        Ok(())
    }

    pub(super) fn emit_const_str(&mut self, v: String) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::ConstStr(v));
        Ok(())
    }

    pub(super) fn emit_const_bool(&mut self, v: bool) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::ConstBool(v));
        Ok(())
    }

    pub(super) fn emit_load(&mut self, idx: i64) -> Result<(), CompileError> {
        let u = to_u32_index(idx, "bc_emit_load")?;
        let cur = self.current_mut()?;
        cur.locals_max = cur.locals_max.max(u + 1);
        cur.code.push(Instr::Load(u));
        Ok(())
    }

    pub(super) fn emit_store(&mut self, idx: i64) -> Result<(), CompileError> {
        let u = to_u32_index(idx, "bc_emit_store")?;
        let cur = self.current_mut()?;
        cur.locals_max = cur.locals_max.max(u + 1);
        cur.code.push(Instr::Store(u));
        Ok(())
    }

    pub(super) fn emit_add(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::Add);
        Ok(())
    }

    pub(super) fn emit_sub(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::Sub);
        Ok(())
    }

    pub(super) fn emit_mul(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::Mul);
        Ok(())
    }

    pub(super) fn emit_div(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::Div);
        Ok(())
    }

    pub(super) fn emit_eq(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::Eq);
        Ok(())
    }

    pub(super) fn emit_ne(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::Ne);
        Ok(())
    }

    pub(super) fn emit_lt(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::Lt);
        Ok(())
    }

    pub(super) fn emit_le(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::Le);
        Ok(())
    }

    pub(super) fn emit_gt(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::Gt);
        Ok(())
    }

    pub(super) fn emit_ge(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::Ge);
        Ok(())
    }

    pub(super) fn emit_and(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::And);
        Ok(())
    }

    pub(super) fn emit_or(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::Or);
        Ok(())
    }

    pub(super) fn emit_not(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::Not);
        Ok(())
    }

    pub(super) fn emit_neg(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::Neg);
        Ok(())
    }

    pub(super) fn emit_strlen(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::StrLen);
        Ok(())
    }

    pub(super) fn emit_concat(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::StrConcat);
        Ok(())
    }

    pub(super) fn emit_substr(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::StrSubstr);
        Ok(())
    }

    pub(super) fn emit_replace(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::StrReplace);
        Ok(())
    }

    pub(super) fn emit_vec_new(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::VecNew);
        Ok(())
    }

    pub(super) fn emit_vec_len(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::VecLen);
        Ok(())
    }

    pub(super) fn emit_vec_get(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::VecGet);
        Ok(())
    }

    pub(super) fn emit_vec_set(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::VecSet);
        Ok(())
    }

    pub(super) fn emit_vec_push(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::VecPush);
        Ok(())
    }

    pub(super) fn emit_vec_remove(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::VecRemove);
        Ok(())
    }

    pub(super) fn emit_vec_last(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::VecLast);
        Ok(())
    }

    pub(super) fn emit_vec_pop(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::VecPop);
        Ok(())
    }

    pub(super) fn emit_map_new(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::MapNew);
        Ok(())
    }

    pub(super) fn emit_map_len(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::MapLen);
        Ok(())
    }

    pub(super) fn emit_map_set(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::MapSet);
        Ok(())
    }

    pub(super) fn emit_map_get(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::MapGet);
        Ok(())
    }

    pub(super) fn emit_map_has(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::MapHas);
        Ok(())
    }

    pub(super) fn emit_map_remove(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::MapRemove);
        Ok(())
    }

    pub(super) fn emit_readfile(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::ReadFile);
        Ok(())
    }

    pub(super) fn emit_writefile(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::WriteFile);
        Ok(())
    }

    pub(super) fn emit_arg_count(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::ArgCount);
        Ok(())
    }

    pub(super) fn emit_arg(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::Arg);
        Ok(())
    }

    pub(super) fn emit_str_clear(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::StrClear);
        Ok(())
    }

    pub(super) fn emit_vec_clear(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::VecClear);
        Ok(())
    }

    pub(super) fn emit_map_clear(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::MapClear);
        Ok(())
    }

    pub(super) fn emit_env_get(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::EnvGet);
        Ok(())
    }

    pub(super) fn emit_env_has(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::EnvHas);
        Ok(())
    }

    pub(super) fn emit_cwd(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::Cwd);
        Ok(())
    }

    pub(super) fn emit_path_join(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::PathJoin);
        Ok(())
    }

    pub(super) fn emit_fs_exists(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::FsExists);
        Ok(())
    }

    pub(super) fn emit_fs_listdir(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::FsListDir);
        Ok(())
    }

    pub(super) fn emit_now_timestamp(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::NowTimestamp);
        Ok(())
    }

    pub(super) fn emit_bc_emit_const_int(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::BcEmitConstInt);
        Ok(())
    }

    pub(super) fn emit_bc_emit_const_bool(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::BcEmitConstBool);
        Ok(())
    }

    pub(super) fn emit_bc_emit_const_str(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::BcEmitConstStr);
        Ok(())
    }

    pub(super) fn emit_bc_emit_load(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::BcEmitLoad);
        Ok(())
    }

    pub(super) fn emit_bc_emit_store(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::BcEmitStore);
        Ok(())
    }

    pub(super) fn emit_bc_emit_call(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::BcEmitCall);
        Ok(())
    }

    pub(super) fn emit_bc_new(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::BcNew);
        Ok(())
    }

    pub(super) fn emit_bc_write(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::BcWrite);
        Ok(())
    }

    pub(super) fn emit_bc_label(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::BcLabel);
        Ok(())
    }

    pub(super) fn emit_bc_jump(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::BcJump);
        Ok(())
    }

    pub(super) fn emit_bc_jump_if_false(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::BcJumpIfFalse);
        Ok(())
    }

    pub(super) fn emit_bc_func_begin(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::BcFuncBegin);
        Ok(())
    }

    fn emit_builder_instr(&mut self, instr: Instr) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(instr);
        Ok(())
    }

    pub(super) fn emit_bc_emit_println(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitPrintLn)
    }

    pub(super) fn emit_bc_emit_writefile(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitWriteFile)
    }

    pub(super) fn emit_bc_emit_add(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitAdd)
    }

    pub(super) fn emit_bc_emit_sub(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitSub)
    }

    pub(super) fn emit_bc_emit_mul(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitMul)
    }

    pub(super) fn emit_bc_emit_div(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitDiv)
    }

    pub(super) fn emit_bc_emit_eq(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitEq)
    }

    pub(super) fn emit_bc_emit_ne(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitNe)
    }

    pub(super) fn emit_bc_emit_lt(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitLt)
    }

    pub(super) fn emit_bc_emit_le(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitLe)
    }

    pub(super) fn emit_bc_emit_gt(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitGt)
    }

    pub(super) fn emit_bc_emit_ge(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitGe)
    }

    pub(super) fn emit_bc_emit_and(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitAnd)
    }

    pub(super) fn emit_bc_emit_or(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitOr)
    }

    pub(super) fn emit_bc_emit_not(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitNot)
    }

    pub(super) fn emit_bc_emit_neg(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitNeg)
    }

    pub(super) fn emit_bc_emit_strlen(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitStrLen)
    }

    pub(super) fn emit_bc_emit_concat(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitStrConcat)
    }

    pub(super) fn emit_bc_emit_substr(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitStrSubstr)
    }

    pub(super) fn emit_bc_emit_replace(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitStrReplace)
    }

    pub(super) fn emit_bc_emit_vec_new(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitVecNew)
    }

    pub(super) fn emit_bc_emit_vec_len(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitVecLen)
    }

    pub(super) fn emit_bc_emit_vec_get(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitVecGet)
    }

    pub(super) fn emit_bc_emit_vec_set(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitVecSet)
    }

    pub(super) fn emit_bc_emit_vec_push(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitVecPush)
    }

    pub(super) fn emit_bc_emit_vec_remove(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitVecRemove)
    }

    pub(super) fn emit_bc_emit_vec_last(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitVecLast)
    }

    pub(super) fn emit_bc_emit_vec_pop(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitVecPop)
    }

    pub(super) fn emit_bc_emit_vec_clear(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitVecClear)
    }

    pub(super) fn emit_bc_emit_map_new(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitMapNew)
    }

    pub(super) fn emit_bc_emit_map_len(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitMapLen)
    }

    pub(super) fn emit_bc_emit_map_set(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitMapSet)
    }

    pub(super) fn emit_bc_emit_map_get(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitMapGet)
    }

    pub(super) fn emit_bc_emit_map_has(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitMapHas)
    }

    pub(super) fn emit_bc_emit_map_remove(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitMapRemove)
    }

    pub(super) fn emit_bc_emit_map_clear(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitMapClear)
    }

    pub(super) fn emit_bc_emit_readfile(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitReadFile)
    }

    pub(super) fn emit_bc_emit_arg_count(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitArgCount)
    }

    pub(super) fn emit_bc_emit_arg(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitArg)
    }

    pub(super) fn emit_bc_emit_env_get(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitEnvGet)
    }

    pub(super) fn emit_bc_emit_env_has(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitEnvHas)
    }

    pub(super) fn emit_bc_emit_cwd(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitCwd)
    }

    pub(super) fn emit_bc_emit_path_join(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitPathJoin)
    }

    pub(super) fn emit_bc_emit_fs_exists(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitFsExists)
    }

    pub(super) fn emit_bc_emit_fs_listdir(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitFsListDir)
    }

    pub(super) fn emit_bc_emit_now_timestamp(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::BcEmitNowTimestamp)
    }

    pub(super) fn emit_meta_bc_new(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::MetaBcNew)
    }

    pub(super) fn emit_meta_bc_main(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::MetaBcMain)
    }

    pub(super) fn emit_meta_bc_write(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::MetaBcWrite)
    }

    pub(super) fn emit_meta_bc_func_begin(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::MetaBcFuncBegin)
    }

    pub(super) fn emit_meta_bc_func_end(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::MetaBcFuncEnd)
    }

    pub(super) fn emit_meta_bc_emit_halt(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::MetaBcEmitHalt)
    }

    pub(super) fn emit_meta_bc_label(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::MetaBcLabel)
    }

    pub(super) fn emit_meta_bc_jump(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::MetaBcJump)
    }

    pub(super) fn emit_meta_bc_jump_if_false(&mut self) -> Result<(), CompileError> {
        self.emit_builder_instr(Instr::MetaBcJumpIfFalse)
    }

    pub(super) fn emit_bc_main(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::BcMain);
        Ok(())
    }

    pub(super) fn emit_bc_func_end(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::BcFuncEnd);
        Ok(())
    }

    pub(super) fn emit_bc_emit_halt(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::BcEmitHalt);
        Ok(())
    }

    pub(super) fn emit_bc_emit_ret(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::BcEmitRet);
        Ok(())
    }

    pub(super) fn emit_bc_emit_retval(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::BcEmitRetVal);
        Ok(())
    }

    pub(super) fn emit_ret(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::Ret);
        Ok(())
    }

    pub(super) fn emit_retval(&mut self) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        cur.code.push(Instr::RetVal);
        Ok(())
    }

    pub(super) fn emit_call_by_name(&mut self, name: String) -> Result<(), CompileError> {
        if let Some(instr) = builtin_call_instr(&name) {
            let cur = self.current_mut()?;
            cur.code.push(instr);
            return Ok(());
        }
        if let Some(arity) = intrinsics::c_intrinsic_arity(&name) {
            let idx = if let Some(existing) = self.func_index.get(&name).copied() {
                let existing_fn = self
                    .functions
                    .get(existing as usize)
                    .ok_or_else(|| CompileError::new_simple("invalid function index"))?;
                if existing_fn.param_count != arity {
                    return Err(CompileError::new_simple(format!(
                        "intrinsic '{}' called with inconsistent arity",
                        name
                    )));
                }
                existing
            } else {
                let idx = self.functions.len() as u32;
                self.func_index.insert(name.clone(), idx);
                self.functions.push(Function {
                    name: name.clone(),
                    param_count: arity,
                    locals: arity.max(1),
                    code: vec![Instr::Halt],
                });
                self.defined.insert(name.clone());
                idx
            };
            let cur = self.current_mut()?;
            cur.code.push(Instr::Call(idx));
            return Ok(());
        }
        let idx = if let Some(idx) = self.func_index.get(&name).copied() {
            idx
        } else {
            let idx = self.functions.len() as u32;
            self.func_index.insert(name.clone(), idx);
            self.functions.push(Function {
                name,
                param_count: 0,
                locals: 0,
                code: Vec::new(),
            });
            idx
        };
        let cur = self.current_mut()?;
        cur.code.push(Instr::Call(idx));
        Ok(())
    }

    pub(super) fn emit_label(&mut self, id: i64) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        if id < 0 {
            return Err(CompileError::new_simple("bc_label: negative label id"));
        }
        let pos = cur.code.len() as u32;
        cur.labels.insert(id, pos);
        Ok(())
    }

    pub(super) fn emit_jump(&mut self, id: i64) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        if id < 0 {
            return Err(CompileError::new_simple("bc_jump: negative label id"));
        }
        let pos = cur.code.len();
        cur.code.push(Instr::Jump(0));
        cur.fixups.push(Fixup {
            pos,
            label: id,
            kind: FixupKind::Jump,
        });
        Ok(())
    }

    pub(super) fn emit_jump_if_false(&mut self, id: i64) -> Result<(), CompileError> {
        let cur = self.current_mut()?;
        if id < 0 {
            return Err(CompileError::new_simple(
                "bc_jump_if_false: negative label id",
            ));
        }
        let pos = cur.code.len();
        cur.code.push(Instr::JumpIfFalse(0));
        cur.fixups.push(Fixup {
            pos,
            label: id,
            kind: FixupKind::JumpIfFalse,
        });
        Ok(())
    }

    fn finalize_function(&self, build: BuildFn) -> Result<Function, CompileError> {
        let mut code = build.code;
        for fixup in &build.fixups {
            let target = build.labels.get(&fixup.label).ok_or_else(|| {
                CompileError::new_simple(format!(
                    "unresolved label {} in function {}",
                    fixup.label, build.name
                ))
            })?;
            let instr = match fixup.kind {
                FixupKind::Jump => Instr::Jump(*target),
                FixupKind::JumpIfFalse => Instr::JumpIfFalse(*target),
            };
            if fixup.pos < code.len() {
                code[fixup.pos] = instr;
            } else {
                return Err(CompileError::new_simple("label fixup out of range"));
            }
        }
        let locals = build.locals_max.max(build.param_count).max(1);
        Ok(Function {
            name: build.name,
            param_count: build.param_count,
            locals,
            code,
        })
    }

    pub(super) fn finish(mut self) -> Result<Bytecode, CompileError> {
        if let Some(build) = self.current.take() {
            let func = self.finalize_function(build)?;
            let idx = *self
                .func_index
                .get(&func.name)
                .ok_or_else(|| CompileError::new_simple("finish: unknown function"))?;
            if let Some(slot) = self.functions.get_mut(idx as usize) {
                *slot = func;
            }
            self.defined
                .insert(self.functions[idx as usize].name.clone());
        }
        for f in &self.functions {
            if !self.defined.contains(&f.name) && !intrinsics::is_c_intrinsic(&f.name) {
                return Err(CompileError::new_simple(format!(
                    "finish: unknown function '{}'",
                    f.name
                )));
            }
        }
        let entry = self.entry.unwrap_or(0);
        Ok(Bytecode {
            name: self.name,
            entry,
            functions: self.functions,
        })
    }
}

fn to_u32_index(idx: i64, ctx: &str) -> Result<u32, CompileError> {
    if idx < 0 {
        return Err(CompileError::new_simple(format!("{ctx}: negative index")));
    }
    if idx > u32::MAX as i64 {
        return Err(CompileError::new_simple(format!("{ctx}: index too large")));
    }
    Ok(idx as u32)
}

fn builtin_call_instr(name: &str) -> Option<Instr> {
    match name {
        "option_some_int" => Some(Instr::OptSomeInt),
        "option_some_bool" => Some(Instr::OptSomeBool),
        "option_some_str" => Some(Instr::OptSomeStr),
        "option_none_int" => Some(Instr::OptNoneInt),
        "option_none_bool" => Some(Instr::OptNoneBool),
        "option_none_str" => Some(Instr::OptNoneStr),
        "option_is_some_int" => Some(Instr::OptIsSomeInt),
        "option_is_some_bool" => Some(Instr::OptIsSomeBool),
        "option_is_some_str" => Some(Instr::OptIsSomeStr),
        "option_unwrap_int" => Some(Instr::OptUnwrapInt),
        "option_unwrap_bool" => Some(Instr::OptUnwrapBool),
        "option_unwrap_str" => Some(Instr::OptUnwrapStr),
        "option_unwrap_or_int" => Some(Instr::OptUnwrapOrInt),
        "option_unwrap_or_bool" => Some(Instr::OptUnwrapOrBool),
        "option_unwrap_or_str" => Some(Instr::OptUnwrapOrStr),
        "result_ok_int" => Some(Instr::ResOkInt),
        "result_ok_bool" => Some(Instr::ResOkBool),
        "result_ok_str" => Some(Instr::ResOkStr),
        "result_err_int" => Some(Instr::ResErrInt),
        "result_err_bool" => Some(Instr::ResErrBool),
        "result_err_str" => Some(Instr::ResErrStr),
        "result_is_ok_int" => Some(Instr::ResIsOkInt),
        "result_is_ok_bool" => Some(Instr::ResIsOkBool),
        "result_is_ok_str" => Some(Instr::ResIsOkStr),
        "result_unwrap_int" => Some(Instr::ResUnwrapInt),
        "result_unwrap_bool" => Some(Instr::ResUnwrapBool),
        "result_unwrap_str" => Some(Instr::ResUnwrapStr),
        "result_unwrap_or_int" => Some(Instr::ResUnwrapOrInt),
        "result_unwrap_or_bool" => Some(Instr::ResUnwrapOrBool),
        "result_unwrap_or_str" => Some(Instr::ResUnwrapOrStr),
        "result_unwrap_err_int" => Some(Instr::ResUnwrapErrInt),
        "result_unwrap_err_bool" => Some(Instr::ResUnwrapErrBool),
        "result_unwrap_err_str" => Some(Instr::ResUnwrapErrStr),
        "os_exec" => Some(Instr::OsExec),
        _ => None,
    }
}
