use std::ffi::{CString, c_void};

use libloading::Library;

use crate::error::CompileError;

use super::super::ops::pop;
use super::super::value::Value;
use super::Vm;

impl Vm {
    pub(super) fn try_exec_c_intrinsic_call(
        &mut self,
        name: &str,
        param_count: usize,
    ) -> Result<bool, CompileError> {
        let out = match name {
            "c_open" => {
                Self::expect_arity(name, param_count, 1)?;
                let args = self.pop_call_args(param_count)?;
                let path = as_string(name, &args[0])?;
                let lib = unsafe { Library::new(&path) }
                    .map_err(|e| CompileError::new_simple(format!("c_open failed: {e}")))?;
                self.ffi_libraries.push(Some(lib));
                self.ffi_strings.clear();
                self.ffi_libraries.len() as i64
            }
            "c_close" => {
                Self::expect_arity(name, param_count, 1)?;
                let args = self.pop_call_args(param_count)?;
                let handle = as_int(name, &args[0])?;
                if handle <= 0 {
                    0
                } else {
                    let slot = (handle - 1) as usize;
                    if let Some(entry) = self.ffi_libraries.get_mut(slot) {
                        if entry.take().is_some() { 1 } else { 0 }
                    } else {
                        0
                    }
                }
            }
            "c_symbol" => {
                Self::expect_arity(name, param_count, 2)?;
                let args = self.pop_call_args(param_count)?;
                let handle = as_int(name, &args[0])?;
                let symbol = as_string(name, &args[1])?;
                let lib = self.lookup_library(handle)?;
                let mut name_buf = symbol.into_bytes();
                if !name_buf.ends_with(&[0]) {
                    name_buf.push(0);
                }
                let sym = unsafe { lib.get::<*const c_void>(&name_buf) }
                    .map_err(|e| CompileError::new_simple(format!("c_symbol failed: {e}")))?;
                (*sym as usize) as i64
            }
            "c_str_ptr" => {
                Self::expect_arity(name, param_count, 1)?;
                let args = self.pop_call_args(param_count)?;
                let text = as_string(name, &args[0])?;
                let cstr = CString::new(text)
                    .map_err(|_| CompileError::new_simple("c_str_ptr: string contains NUL byte"))?;
                let ptr = cstr.as_ptr() as usize as i64;
                self.ffi_strings.push(cstr);
                ptr
            }
            "c_call_i64_0" => {
                Self::expect_arity(name, param_count, 1)?;
                let args = self.pop_call_args(param_count)?;
                let sym = as_int(name, &args[0])?;
                call_i64_0(sym)?
            }
            "c_call_i64_1" => {
                Self::expect_arity(name, param_count, 2)?;
                let args = self.pop_call_args(param_count)?;
                let sym = as_int(name, &args[0])?;
                let a0 = as_int(name, &args[1])?;
                call_i64_1(sym, a0)?
            }
            "c_call_i64_2" => {
                Self::expect_arity(name, param_count, 3)?;
                let args = self.pop_call_args(param_count)?;
                let sym = as_int(name, &args[0])?;
                let a0 = as_int(name, &args[1])?;
                let a1 = as_int(name, &args[2])?;
                call_i64_2(sym, a0, a1)?
            }
            "c_call_i64_3" => {
                Self::expect_arity(name, param_count, 4)?;
                let args = self.pop_call_args(param_count)?;
                let sym = as_int(name, &args[0])?;
                let a0 = as_int(name, &args[1])?;
                let a1 = as_int(name, &args[2])?;
                let a2 = as_int(name, &args[3])?;
                call_i64_3(sym, a0, a1, a2)?
            }
            "c_call_i64_4" => {
                Self::expect_arity(name, param_count, 5)?;
                let args = self.pop_call_args(param_count)?;
                let sym = as_int(name, &args[0])?;
                let a0 = as_int(name, &args[1])?;
                let a1 = as_int(name, &args[2])?;
                let a2 = as_int(name, &args[3])?;
                let a3 = as_int(name, &args[4])?;
                call_i64_4(sym, a0, a1, a2, a3)?
            }
            _ => return Ok(false),
        };

        self.stack.push(Value::Int(out));
        Ok(true)
    }

    fn lookup_library(&self, handle: i64) -> Result<&Library, CompileError> {
        if handle <= 0 {
            return Err(CompileError::new_simple("c_symbol: invalid handle"));
        }
        let idx = (handle - 1) as usize;
        self.ffi_libraries
            .get(idx)
            .and_then(|slot| slot.as_ref())
            .ok_or_else(|| CompileError::new_simple("c_symbol: unknown or closed handle"))
    }

    fn pop_call_args(&mut self, param_count: usize) -> Result<Vec<Value>, CompileError> {
        let mut out = Vec::with_capacity(param_count);
        for _ in 0..param_count {
            out.push(pop(&mut self.stack)?);
        }
        out.reverse();
        Ok(out)
    }

    fn expect_arity(name: &str, got: usize, expected: usize) -> Result<(), CompileError> {
        if got != expected {
            return Err(CompileError::new_simple(format!(
                "{name}: invalid arity (expected {expected}, got {got})"
            )));
        }
        Ok(())
    }
}

fn as_int(name: &str, v: &Value) -> Result<i64, CompileError> {
    if let Value::Int(x) = v {
        Ok(*x)
    } else {
        Err(CompileError::new_simple(format!(
            "{name}: expected integer"
        )))
    }
}

fn as_string(name: &str, v: &Value) -> Result<String, CompileError> {
    if let Value::Str(s) = v {
        Ok(s.clone())
    } else {
        Err(CompileError::new_simple(format!("{name}: expected string")))
    }
}

fn checked_symbol_addr(symbol: i64, name: &str) -> Result<usize, CompileError> {
    if symbol <= 0 {
        return Err(CompileError::new_simple(format!("{name}: invalid symbol")));
    }
    Ok(symbol as usize)
}

fn call_i64_0(symbol: i64) -> Result<i64, CompileError> {
    let addr = checked_symbol_addr(symbol, "c_call_i64_0")?;
    let f: unsafe extern "C" fn() -> i64 = unsafe { std::mem::transmute(addr) };
    Ok(unsafe { f() })
}

fn call_i64_1(symbol: i64, a0: i64) -> Result<i64, CompileError> {
    let addr = checked_symbol_addr(symbol, "c_call_i64_1")?;
    let f: unsafe extern "C" fn(i64) -> i64 = unsafe { std::mem::transmute(addr) };
    Ok(unsafe { f(a0) })
}

fn call_i64_2(symbol: i64, a0: i64, a1: i64) -> Result<i64, CompileError> {
    let addr = checked_symbol_addr(symbol, "c_call_i64_2")?;
    let f: unsafe extern "C" fn(i64, i64) -> i64 = unsafe { std::mem::transmute(addr) };
    Ok(unsafe { f(a0, a1) })
}

fn call_i64_3(symbol: i64, a0: i64, a1: i64, a2: i64) -> Result<i64, CompileError> {
    let addr = checked_symbol_addr(symbol, "c_call_i64_3")?;
    let f: unsafe extern "C" fn(i64, i64, i64) -> i64 = unsafe { std::mem::transmute(addr) };
    Ok(unsafe { f(a0, a1, a2) })
}

fn call_i64_4(symbol: i64, a0: i64, a1: i64, a2: i64, a3: i64) -> Result<i64, CompileError> {
    let addr = checked_symbol_addr(symbol, "c_call_i64_4")?;
    let f: unsafe extern "C" fn(i64, i64, i64, i64) -> i64 = unsafe { std::mem::transmute(addr) };
    Ok(unsafe { f(a0, a1, a2, a3) })
}
