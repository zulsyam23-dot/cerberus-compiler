use crate::bytecode::Instr;
use crate::error::CompileError;

use super::Vm;
use super::super::ops::{pop, pop_int};
use super::super::value::Value;

impl Vm {
    pub(super) fn exec_strings(&mut self, instr: Instr) -> Result<(), CompileError> {
        match instr {
            Instr::StrEq => {
                let b = match pop(&mut self.stack)? {
                    Value::Str(v) => v,
                    _ => return Err(CompileError::new_simple("streq: expected string")),
                };
                let a = match pop(&mut self.stack)? {
                    Value::Str(v) => v,
                    _ => return Err(CompileError::new_simple("streq: expected string")),
                };
                self.stack.push(Value::Bool(a == b));
            }
            Instr::StrNe => {
                let b = match pop(&mut self.stack)? {
                    Value::Str(v) => v,
                    _ => return Err(CompileError::new_simple("strne: expected string")),
                };
                let a = match pop(&mut self.stack)? {
                    Value::Str(v) => v,
                    _ => return Err(CompileError::new_simple("strne: expected string")),
                };
                self.stack.push(Value::Bool(a != b));
            }
            Instr::StrLen => {
                let s = match pop(&mut self.stack)? {
                    Value::Str(v) => v,
                    _ => return Err(CompileError::new_simple("strlen: expected string")),
                };
                let len = s.chars().count() as i64;
                self.stack.push(Value::Int(len));
            }
            Instr::StrConcat => {
                let b = match pop(&mut self.stack)? {
                    Value::Str(v) => v,
                    _ => return Err(CompileError::new_simple("concat: expected string")),
                };
                let a = match pop(&mut self.stack)? {
                    Value::Str(v) => v,
                    _ => return Err(CompileError::new_simple("concat: expected string")),
                };
                let mut out = String::with_capacity(a.len() + b.len());
                out.push_str(&a);
                out.push_str(&b);
                self.stack.push(Value::Str(out));
            }
            Instr::StrSubstr => {
                let len = pop_int(&mut self.stack)? as isize;
                let start = pop_int(&mut self.stack)? as isize;
                let s = match pop(&mut self.stack)? {
                    Value::Str(v) => v,
                    _ => return Err(CompileError::new_simple("substr: expected string")),
                };
                if start < 0 || len < 0 {
                    return Err(CompileError::new_simple("substr: negative index"));
                }
                let chars: Vec<char> = s.chars().collect();
                let start_u = start as usize;
                let len_u = len as usize;
                if start_u > chars.len() || start_u + len_u > chars.len() {
                    return Err(CompileError::new_simple("substr: out of range"));
                }
                let out: String = chars[start_u..start_u + len_u].iter().collect();
                self.stack.push(Value::Str(out));
            }
            Instr::StrReplace => {
                let to = match pop(&mut self.stack)? {
                    Value::Str(v) => v,
                    _ => return Err(CompileError::new_simple("replace: expected string")),
                };
                let from = match pop(&mut self.stack)? {
                    Value::Str(v) => v,
                    _ => return Err(CompileError::new_simple("replace: expected string")),
                };
                let s = match pop(&mut self.stack)? {
                    Value::Str(v) => v,
                    _ => return Err(CompileError::new_simple("replace: expected string")),
                };
                let out = s.replace(&from, &to);
                self.stack.push(Value::Str(out));
            }
            Instr::StrClear => {
                let _ = match pop(&mut self.stack)? {
                    Value::Str(s) => s,
                    _ => return Err(CompileError::new_simple("string_clear: expected string")),
                };
                self.stack.push(Value::Str(String::new()));
            }
            _ => unreachable!("invalid string instruction"),
        }
        Ok(())
    }
}
