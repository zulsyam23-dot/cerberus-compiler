use crate::bytecode::Instr;
use crate::error::CompileError;

use super::helpers::ensure_vector_value;
use super::Vm;
use super::super::array::set_index;
use super::super::ops::{pop, pop_bool, pop_int};
use super::super::value::Value;

impl Vm {
    pub(super) fn exec_collections(&mut self, instr: Instr) -> Result<(), CompileError> {
        match instr {
            Instr::AllocArray(len) => {
                self.stack.push(Value::Array(vec![0; len as usize]));
            }
            Instr::LoadIndex => {
                let idx = pop_int(&mut self.stack)? as usize;
                let arr = match pop(&mut self.stack)? {
                    Value::Array(v) => v,
                    _ => return Err(CompileError::new_simple("load_index: expected array")),
                };
                let v = arr.get(idx).copied().ok_or_else(|| {
                    CompileError::new_simple("load_index: array index out of bounds")
                })?;
                self.stack.push(Value::Int(v));
            }
            Instr::StoreIndex => {
                let val = pop_int(&mut self.stack)?;
                let idx = pop_int(&mut self.stack)? as usize;
                let arr = match pop(&mut self.stack)? {
                    Value::Array(v) => v,
                    _ => return Err(CompileError::new_simple("store_index: expected array")),
                };
                let new_arr = set_index(arr, idx, val)?;
                self.stack.push(Value::Array(new_arr));
            }
            Instr::VecNew => {
                self.stack.push(Value::Vector(Vec::new()));
            }
            Instr::VecLen => {
                let v = match pop(&mut self.stack)? {
                    Value::Vector(v) => v,
                    _ => return Err(CompileError::new_simple("vector_len: expected vector")),
                };
                self.stack.push(Value::Int(v.len() as i64));
            }
            Instr::VecGet => {
                let idx = pop_int(&mut self.stack)? as usize;
                let v = match pop(&mut self.stack)? {
                    Value::Vector(v) => v,
                    _ => return Err(CompileError::new_simple("vector_get: expected vector")),
                };
                let out = v.get(idx).cloned().ok_or_else(|| {
                    CompileError::new_simple("vector_get: index out of range")
                })?;
                self.stack.push(out);
            }
            Instr::VecSet => {
                let val = pop(&mut self.stack)?;
                let idx = pop_int(&mut self.stack)? as usize;
                let mut v = match pop(&mut self.stack)? {
                    Value::Vector(v) => v,
                    _ => return Err(CompileError::new_simple("vector_set: expected vector")),
                };
                if idx >= v.len() {
                    return Err(CompileError::new_simple("vector_set: index out of range"));
                }
                ensure_vector_value(&v, &val)?;
                v[idx] = val;
                self.stack.push(Value::Vector(v));
            }
            Instr::VecPush => {
                let val = pop(&mut self.stack)?;
                let mut v = match pop(&mut self.stack)? {
                    Value::Vector(v) => v,
                    _ => return Err(CompileError::new_simple("vector_push: expected vector")),
                };
                ensure_vector_value(&v, &val)?;
                v.push(val);
                self.stack.push(Value::Vector(v));
            }
            Instr::VecRemove => {
                let idx = pop_int(&mut self.stack)? as usize;
                let mut v = match pop(&mut self.stack)? {
                    Value::Vector(v) => v,
                    _ => return Err(CompileError::new_simple("vector_remove: expected vector")),
                };
                if idx >= v.len() {
                    return Err(CompileError::new_simple("vector_remove: index out of range"));
                }
                v.remove(idx);
                self.stack.push(Value::Vector(v));
            }
            Instr::VecLast => {
                let v = match pop(&mut self.stack)? {
                    Value::Vector(v) => v,
                    _ => return Err(CompileError::new_simple("vector_last: expected vector")),
                };
                let out = v.last().cloned().ok_or_else(|| {
                    CompileError::new_simple("vector_last: empty vector")
                })?;
                self.stack.push(out);
            }
            Instr::VecPop => {
                let mut v = match pop(&mut self.stack)? {
                    Value::Vector(v) => v,
                    _ => return Err(CompileError::new_simple("vector_pop: expected vector")),
                };
                if v.pop().is_none() {
                    return Err(CompileError::new_simple("vector_pop: empty vector"));
                }
                self.stack.push(Value::Vector(v));
            }
            Instr::StackNew => {
                self.stack.push(Value::Stack(Vec::new()));
            }
            Instr::StackLen => {
                let v = match pop(&mut self.stack)? {
                    Value::Stack(v) => v,
                    _ => return Err(CompileError::new_simple("stack_len: expected stack")),
                };
                self.stack.push(Value::Int(v.len() as i64));
            }
            Instr::StackPush => {
                let val = pop_int(&mut self.stack)?;
                let mut v = match pop(&mut self.stack)? {
                    Value::Stack(v) => v,
                    _ => return Err(CompileError::new_simple("stack_push: expected stack")),
                };
                v.push(val);
                self.stack.push(Value::Stack(v));
            }
            Instr::StackTop => {
                let v = match pop(&mut self.stack)? {
                    Value::Stack(v) => v,
                    _ => return Err(CompileError::new_simple("stack_top: expected stack")),
                };
                let out = v.last().copied().ok_or_else(|| {
                    CompileError::new_simple("stack_top: empty stack")
                })?;
                self.stack.push(Value::Int(out));
            }
            Instr::StackPop => {
                let mut v = match pop(&mut self.stack)? {
                    Value::Stack(v) => v,
                    _ => return Err(CompileError::new_simple("stack_pop: expected stack")),
                };
                if v.pop().is_none() {
                    return Err(CompileError::new_simple("stack_pop: empty stack"));
                }
                self.stack.push(Value::Stack(v));
            }
            Instr::MapNew => {
                self.stack
                    .push(Value::Map(std::collections::HashMap::new()));
            }
            Instr::MapLen => {
                let m = match pop(&mut self.stack)? {
                    Value::Map(m) => m,
                    _ => return Err(CompileError::new_simple("map_len: expected map")),
                };
                self.stack.push(Value::Int(m.len() as i64));
            }
            Instr::MapSet => {
                let val = match pop(&mut self.stack)? {
                    Value::Str(v) => v,
                    _ => return Err(CompileError::new_simple("map_set: expected string value")),
                };
                let key = match pop(&mut self.stack)? {
                    Value::Str(v) => v,
                    _ => return Err(CompileError::new_simple("map_set: expected string key")),
                };
                let mut m = match pop(&mut self.stack)? {
                    Value::Map(m) => m,
                    _ => return Err(CompileError::new_simple("map_set: expected map")),
                };
                m.insert(key, val);
                self.stack.push(Value::Map(m));
            }
            Instr::MapGet => {
                let key = match pop(&mut self.stack)? {
                    Value::Str(v) => v,
                    _ => return Err(CompileError::new_simple("map_get: expected string key")),
                };
                let m = match pop(&mut self.stack)? {
                    Value::Map(m) => m,
                    _ => return Err(CompileError::new_simple("map_get: expected map")),
                };
                let out = m.get(&key).cloned().ok_or_else(|| {
                    CompileError::new_simple("map_get: key not found")
                })?;
                self.stack.push(Value::Str(out));
            }
            Instr::MapHas => {
                let key = match pop(&mut self.stack)? {
                    Value::Str(v) => v,
                    _ => return Err(CompileError::new_simple("map_has: expected string key")),
                };
                let m = match pop(&mut self.stack)? {
                    Value::Map(m) => m,
                    _ => return Err(CompileError::new_simple("map_has: expected map")),
                };
                self.stack.push(Value::Bool(m.contains_key(&key)));
            }
            Instr::MapRemove => {
                let key = match pop(&mut self.stack)? {
                    Value::Str(v) => v,
                    _ => {
                        return Err(CompileError::new_simple(
                            "map_remove: expected string key",
                        ))
                    }
                };
                let mut m = match pop(&mut self.stack)? {
                    Value::Map(m) => m,
                    _ => return Err(CompileError::new_simple("map_remove: expected map")),
                };
                m.remove(&key);
                self.stack.push(Value::Map(m));
            }
            Instr::SetNew => {
                self.stack
                    .push(Value::Set(std::collections::HashSet::new()));
            }
            Instr::SetLen => {
                let s = match pop(&mut self.stack)? {
                    Value::Set(s) => s,
                    _ => return Err(CompileError::new_simple("set_len: expected set")),
                };
                self.stack.push(Value::Int(s.len() as i64));
            }
            Instr::SetAdd => {
                let val = match pop(&mut self.stack)? {
                    Value::Str(v) => v,
                    _ => return Err(CompileError::new_simple("set_add: expected string")),
                };
                let mut s = match pop(&mut self.stack)? {
                    Value::Set(s) => s,
                    _ => return Err(CompileError::new_simple("set_add: expected set")),
                };
                s.insert(val);
                self.stack.push(Value::Set(s));
            }
            Instr::SetHas => {
                let val = match pop(&mut self.stack)? {
                    Value::Str(v) => v,
                    _ => return Err(CompileError::new_simple("set_has: expected string")),
                };
                let s = match pop(&mut self.stack)? {
                    Value::Set(s) => s,
                    _ => return Err(CompileError::new_simple("set_has: expected set")),
                };
                self.stack.push(Value::Bool(s.contains(&val)));
            }
            Instr::SetRemove => {
                let val = match pop(&mut self.stack)? {
                    Value::Str(v) => v,
                    _ => return Err(CompileError::new_simple("set_remove: expected string")),
                };
                let mut s = match pop(&mut self.stack)? {
                    Value::Set(s) => s,
                    _ => return Err(CompileError::new_simple("set_remove: expected set")),
                };
                s.remove(&val);
                self.stack.push(Value::Set(s));
            }
            Instr::OptSomeInt => {
                let v = pop_int(&mut self.stack)?;
                self.stack.push(Value::Option(Some(Box::new(Value::Int(v)))));
            }
            Instr::OptSomeBool => {
                let v = pop_bool(&mut self.stack)?;
                self.stack.push(Value::Option(Some(Box::new(Value::Bool(v)))));
            }
            Instr::OptSomeStr => {
                let v = match pop(&mut self.stack)? {
                    Value::Str(s) => s,
                    _ => return Err(CompileError::new_simple("option_some_str: expected string")),
                };
                self.stack.push(Value::Option(Some(Box::new(Value::Str(v)))));
            }
            Instr::OptNoneInt | Instr::OptNoneBool | Instr::OptNoneStr => {
                self.stack.push(Value::Option(None));
            }
            Instr::OptIsSomeInt => {
                let v = match pop(&mut self.stack)? {
                    Value::Option(v) => v,
                    _ => {
                        return Err(CompileError::new_simple(
                            "option_is_some_int: expected option",
                        ))
                    }
                };
                self.stack.push(Value::Bool(v.is_some()));
            }
            Instr::OptIsSomeBool => {
                let v = match pop(&mut self.stack)? {
                    Value::Option(v) => v,
                    _ => {
                        return Err(CompileError::new_simple(
                            "option_is_some_bool: expected option",
                        ))
                    }
                };
                self.stack.push(Value::Bool(v.is_some()));
            }
            Instr::OptIsSomeStr => {
                let v = match pop(&mut self.stack)? {
                    Value::Option(v) => v,
                    _ => {
                        return Err(CompileError::new_simple(
                            "option_is_some_str: expected option",
                        ))
                    }
                };
                self.stack.push(Value::Bool(v.is_some()));
            }
            Instr::OptUnwrapInt => {
                let v = match pop(&mut self.stack)? {
                    Value::Option(v) => v,
                    _ => return Err(CompileError::new_simple("option_unwrap_int: expected option")),
                };
                let inner = v.ok_or_else(|| CompileError::new_simple("option_unwrap_int: none"))?;
                match *inner {
                    Value::Int(i) => self.stack.push(Value::Int(i)),
                    _ => return Err(CompileError::new_simple("option_unwrap_int: type mismatch")),
                }
            }
            Instr::OptUnwrapBool => {
                let v = match pop(&mut self.stack)? {
                    Value::Option(v) => v,
                    _ => return Err(CompileError::new_simple("option_unwrap_bool: expected option")),
                };
                let inner = v.ok_or_else(|| CompileError::new_simple("option_unwrap_bool: none"))?;
                match *inner {
                    Value::Bool(i) => self.stack.push(Value::Bool(i)),
                    _ => return Err(CompileError::new_simple("option_unwrap_bool: type mismatch")),
                }
            }
            Instr::OptUnwrapStr => {
                let v = match pop(&mut self.stack)? {
                    Value::Option(v) => v,
                    _ => return Err(CompileError::new_simple("option_unwrap_str: expected option")),
                };
                let inner = v.ok_or_else(|| CompileError::new_simple("option_unwrap_str: none"))?;
                match *inner {
                    Value::Str(i) => self.stack.push(Value::Str(i)),
                    _ => return Err(CompileError::new_simple("option_unwrap_str: type mismatch")),
                }
            }
            Instr::OptUnwrapOrInt => {
                let default = pop_int(&mut self.stack)?;
                let v = match pop(&mut self.stack)? {
                    Value::Option(v) => v,
                    _ => return Err(CompileError::new_simple("option_unwrap_or_int: expected option")),
                };
                if let Some(inner) = v {
                    match *inner {
                        Value::Int(i) => self.stack.push(Value::Int(i)),
                        _ => return Err(CompileError::new_simple("option_unwrap_or_int: type mismatch")),
                    }
                } else {
                    self.stack.push(Value::Int(default));
                }
            }
            Instr::OptUnwrapOrBool => {
                let default = pop_bool(&mut self.stack)?;
                let v = match pop(&mut self.stack)? {
                    Value::Option(v) => v,
                    _ => return Err(CompileError::new_simple("option_unwrap_or_bool: expected option")),
                };
                if let Some(inner) = v {
                    match *inner {
                        Value::Bool(i) => self.stack.push(Value::Bool(i)),
                        _ => return Err(CompileError::new_simple("option_unwrap_or_bool: type mismatch")),
                    }
                } else {
                    self.stack.push(Value::Bool(default));
                }
            }
            Instr::OptUnwrapOrStr => {
                let default = match pop(&mut self.stack)? {
                    Value::Str(s) => s,
                    _ => {
                        return Err(CompileError::new_simple(
                            "option_unwrap_or_str: expected string default",
                        ))
                    }
                };
                let v = match pop(&mut self.stack)? {
                    Value::Option(v) => v,
                    _ => return Err(CompileError::new_simple("option_unwrap_or_str: expected option")),
                };
                if let Some(inner) = v {
                    match *inner {
                        Value::Str(i) => self.stack.push(Value::Str(i)),
                        _ => return Err(CompileError::new_simple("option_unwrap_or_str: type mismatch")),
                    }
                } else {
                    self.stack.push(Value::Str(default));
                }
            }
            Instr::ResOkInt => {
                let v = pop_int(&mut self.stack)?;
                self.stack
                    .push(Value::Result(Ok(Box::new(Value::Int(v)))));
            }
            Instr::ResOkBool => {
                let v = pop_bool(&mut self.stack)?;
                self.stack
                    .push(Value::Result(Ok(Box::new(Value::Bool(v)))));
            }
            Instr::ResOkStr => {
                let v = match pop(&mut self.stack)? {
                    Value::Str(s) => s,
                    _ => return Err(CompileError::new_simple("result_ok_str: expected string")),
                };
                self.stack
                    .push(Value::Result(Ok(Box::new(Value::Str(v)))));
            }
            Instr::ResErrInt => {
                let msg = match pop(&mut self.stack)? {
                    Value::Str(s) => s,
                    _ => return Err(CompileError::new_simple("result_err_int: expected string")),
                };
                self.stack.push(Value::Result(Err(msg)));
            }
            Instr::ResErrBool => {
                let msg = match pop(&mut self.stack)? {
                    Value::Str(s) => s,
                    _ => return Err(CompileError::new_simple("result_err_bool: expected string")),
                };
                self.stack.push(Value::Result(Err(msg)));
            }
            Instr::ResErrStr => {
                let msg = match pop(&mut self.stack)? {
                    Value::Str(s) => s,
                    _ => return Err(CompileError::new_simple("result_err_str: expected string")),
                };
                self.stack.push(Value::Result(Err(msg)));
            }
            Instr::ResIsOkInt => {
                let v = match pop(&mut self.stack)? {
                    Value::Result(v) => v,
                    _ => return Err(CompileError::new_simple("result_is_ok_int: expected result")),
                };
                self.stack.push(Value::Bool(v.is_ok()));
            }
            Instr::ResIsOkBool => {
                let v = match pop(&mut self.stack)? {
                    Value::Result(v) => v,
                    _ => return Err(CompileError::new_simple("result_is_ok_bool: expected result")),
                };
                self.stack.push(Value::Bool(v.is_ok()));
            }
            Instr::ResIsOkStr => {
                let v = match pop(&mut self.stack)? {
                    Value::Result(v) => v,
                    _ => return Err(CompileError::new_simple("result_is_ok_str: expected result")),
                };
                self.stack.push(Value::Bool(v.is_ok()));
            }
            Instr::ResUnwrapInt => {
                let v = match pop(&mut self.stack)? {
                    Value::Result(v) => v,
                    _ => return Err(CompileError::new_simple("result_unwrap_int: expected result")),
                };
                match v {
                    Ok(inner) => match *inner {
                        Value::Int(i) => self.stack.push(Value::Int(i)),
                        _ => return Err(CompileError::new_simple("result_unwrap_int: type mismatch")),
                    },
                    Err(msg) => {
                        return Err(CompileError::new_simple(format!(
                            "result_unwrap_int: {msg}"
                        )))
                    }
                }
            }
            Instr::ResUnwrapBool => {
                let v = match pop(&mut self.stack)? {
                    Value::Result(v) => v,
                    _ => return Err(CompileError::new_simple("result_unwrap_bool: expected result")),
                };
                match v {
                    Ok(inner) => match *inner {
                        Value::Bool(i) => self.stack.push(Value::Bool(i)),
                        _ => return Err(CompileError::new_simple("result_unwrap_bool: type mismatch")),
                    },
                    Err(msg) => {
                        return Err(CompileError::new_simple(format!(
                            "result_unwrap_bool: {msg}"
                        )))
                    }
                }
            }
            Instr::ResUnwrapStr => {
                let v = match pop(&mut self.stack)? {
                    Value::Result(v) => v,
                    _ => return Err(CompileError::new_simple("result_unwrap_str: expected result")),
                };
                match v {
                    Ok(inner) => match *inner {
                        Value::Str(i) => self.stack.push(Value::Str(i)),
                        _ => return Err(CompileError::new_simple("result_unwrap_str: type mismatch")),
                    },
                    Err(msg) => {
                        return Err(CompileError::new_simple(format!(
                            "result_unwrap_str: {msg}"
                        )))
                    }
                }
            }
            Instr::ResUnwrapOrInt => {
                let default = pop_int(&mut self.stack)?;
                let v = match pop(&mut self.stack)? {
                    Value::Result(v) => v,
                    _ => return Err(CompileError::new_simple("result_unwrap_or_int: expected result")),
                };
                match v {
                    Ok(inner) => match *inner {
                        Value::Int(i) => self.stack.push(Value::Int(i)),
                        _ => return Err(CompileError::new_simple("result_unwrap_or_int: type mismatch")),
                    },
                    Err(_) => self.stack.push(Value::Int(default)),
                }
            }
            Instr::ResUnwrapOrBool => {
                let default = pop_bool(&mut self.stack)?;
                let v = match pop(&mut self.stack)? {
                    Value::Result(v) => v,
                    _ => return Err(CompileError::new_simple("result_unwrap_or_bool: expected result")),
                };
                match v {
                    Ok(inner) => match *inner {
                        Value::Bool(i) => self.stack.push(Value::Bool(i)),
                        _ => return Err(CompileError::new_simple("result_unwrap_or_bool: type mismatch")),
                    },
                    Err(_) => self.stack.push(Value::Bool(default)),
                }
            }
            Instr::ResUnwrapOrStr => {
                let default = match pop(&mut self.stack)? {
                    Value::Str(s) => s,
                    _ => {
                        return Err(CompileError::new_simple(
                            "result_unwrap_or_str: expected string default",
                        ))
                    }
                };
                let v = match pop(&mut self.stack)? {
                    Value::Result(v) => v,
                    _ => return Err(CompileError::new_simple("result_unwrap_or_str: expected result")),
                };
                match v {
                    Ok(inner) => match *inner {
                        Value::Str(i) => self.stack.push(Value::Str(i)),
                        _ => return Err(CompileError::new_simple("result_unwrap_or_str: type mismatch")),
                    },
                    Err(_) => self.stack.push(Value::Str(default)),
                }
            }
            Instr::ResUnwrapErrInt => {
                let v = match pop(&mut self.stack)? {
                    Value::Result(v) => v,
                    _ => return Err(CompileError::new_simple("result_unwrap_err_int: expected result")),
                };
                match v {
                    Ok(_) => {
                        return Err(CompileError::new_simple(
                            "result_unwrap_err_int: ok value",
                        ))
                    }
                    Err(msg) => self.stack.push(Value::Str(msg)),
                }
            }
            Instr::ResUnwrapErrBool => {
                let v = match pop(&mut self.stack)? {
                    Value::Result(v) => v,
                    _ => return Err(CompileError::new_simple("result_unwrap_err_bool: expected result")),
                };
                match v {
                    Ok(_) => {
                        return Err(CompileError::new_simple(
                            "result_unwrap_err_bool: ok value",
                        ))
                    }
                    Err(msg) => self.stack.push(Value::Str(msg)),
                }
            }
            Instr::ResUnwrapErrStr => {
                let v = match pop(&mut self.stack)? {
                    Value::Result(v) => v,
                    _ => return Err(CompileError::new_simple("result_unwrap_err_str: expected result")),
                };
                match v {
                    Ok(_) => {
                        return Err(CompileError::new_simple(
                            "result_unwrap_err_str: ok value",
                        ))
                    }
                    Err(msg) => self.stack.push(Value::Str(msg)),
                }
            }
            Instr::VecClear => {
                let _ = match pop(&mut self.stack)? {
                    Value::Vector(v) => v,
                    _ => return Err(CompileError::new_simple("vector_clear: expected vector")),
                };
                self.stack.push(Value::Vector(Vec::new()));
            }
            Instr::StackClear => {
                let _ = match pop(&mut self.stack)? {
                    Value::Stack(v) => v,
                    _ => return Err(CompileError::new_simple("stack_clear: expected stack")),
                };
                self.stack.push(Value::Stack(Vec::new()));
            }
            Instr::MapClear => {
                let _ = match pop(&mut self.stack)? {
                    Value::Map(m) => m,
                    _ => return Err(CompileError::new_simple("map_clear: expected map")),
                };
                self.stack
                    .push(Value::Map(std::collections::HashMap::new()));
            }
            Instr::SetClear => {
                let _ = match pop(&mut self.stack)? {
                    Value::Set(s) => s,
                    _ => return Err(CompileError::new_simple("set_clear: expected set")),
                };
                self.stack
                    .push(Value::Set(std::collections::HashSet::new()));
            }
            _ => unreachable!("invalid collection instruction"),
        }
        Ok(())
    }
}
