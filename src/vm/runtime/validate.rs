use crate::bytecode::{Bytecode, Function, Instr};
use crate::error::CompileError;

use super::config::VmLimits;

pub(super) fn validate_bytecode(bc: &Bytecode, limits: VmLimits) -> Result<(), CompileError> {
    if bc.functions.is_empty() {
        return Err(CompileError::new_simple("invalid bytecode: no functions"));
    }

    if bc.functions.len() > limits.max_functions {
        return Err(CompileError::new_simple(format!(
            "invalid bytecode: function count {} exceeds limit {}",
            bc.functions.len(),
            limits.max_functions
        )));
    }

    let entry = bc.entry as usize;
    if entry >= bc.functions.len() {
        return Err(CompileError::new_simple(format!(
            "invalid bytecode: entry function index {} out of range (function count {})",
            entry,
            bc.functions.len()
        )));
    }

    for (func_idx, func) in bc.functions.iter().enumerate() {
        validate_function(func_idx, func, bc.functions.len(), limits)?;
    }

    Ok(())
}

fn validate_function(
    func_idx: usize,
    func: &Function,
    function_count: usize,
    limits: VmLimits,
) -> Result<(), CompileError> {
    let locals = func.locals as usize;
    let params = func.param_count as usize;

    if locals > limits.max_locals_per_function {
        return Err(CompileError::new_simple(format!(
            "invalid bytecode: function '{}'#{} uses {} locals (limit {})",
            func.name, func_idx, locals, limits.max_locals_per_function
        )));
    }

    if params > locals {
        return Err(CompileError::new_simple(format!(
            "invalid bytecode: function '{}'#{} has {} params but only {} locals",
            func.name, func_idx, params, locals
        )));
    }

    if func.code.len() > limits.max_instructions_per_function {
        return Err(CompileError::new_simple(format!(
            "invalid bytecode: function '{}'#{} has {} instructions (limit {})",
            func.name,
            func_idx,
            func.code.len(),
            limits.max_instructions_per_function
        )));
    }

    if func.code.is_empty() {
        return Err(CompileError::new_simple(format!(
            "invalid bytecode: function '{}'#{} has empty code",
            func.name, func_idx
        )));
    }

    let code_len = func.code.len();
    for (ip, instr) in func.code.iter().enumerate() {
        match instr {
            Instr::Load(local)
            | Instr::Store(local)
            | Instr::ReadInt(local)
            | Instr::ReadBool(local)
            | Instr::ReadStr(local) => {
                validate_local(func, func_idx, ip, *local, locals)?;
            }
            Instr::Jump(target) | Instr::JumpIfFalse(target) => {
                validate_jump(func, func_idx, ip, *target, code_len)?;
            }
            Instr::Call(target) => {
                let callee = *target as usize;
                if callee >= function_count {
                    return Err(CompileError::new_simple(format!(
                        "invalid bytecode: function '{}'#{} ip {} calls invalid function index {}",
                        func.name, func_idx, ip, callee
                    )));
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn validate_local(
    func: &Function,
    func_idx: usize,
    ip: usize,
    local: u32,
    locals: usize,
) -> Result<(), CompileError> {
    let local_idx = local as usize;
    if local_idx >= locals {
        return Err(CompileError::new_simple(format!(
            "invalid bytecode: function '{}'#{} ip {} accesses local {} but locals={}",
            func.name, func_idx, ip, local_idx, locals
        )));
    }
    Ok(())
}

fn validate_jump(
    func: &Function,
    func_idx: usize,
    ip: usize,
    target: u32,
    code_len: usize,
) -> Result<(), CompileError> {
    let jump_target = target as usize;
    if jump_target >= code_len {
        return Err(CompileError::new_simple(format!(
            "invalid bytecode: function '{}'#{} ip {} jumps to {} but code_len={}",
            func.name, func_idx, ip, jump_target, code_len
        )));
    }
    Ok(())
}
