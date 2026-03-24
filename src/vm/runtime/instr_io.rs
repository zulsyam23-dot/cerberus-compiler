use crate::bytecode::Instr;
use crate::error::CompileError;

use super::Vm;
use super::super::io::read_line;
use super::super::ops::{pop, pop_int};
use super::super::value::Value;

impl Vm {
    pub(super) fn exec_io(&mut self, instr: Instr) -> Result<(), CompileError> {
        match instr {
            Instr::ReadFile => {
                let path = match pop(&mut self.stack)? {
                    Value::Str(s) => s,
                    _ => return Err(CompileError::new_simple("readfile: expected string path")),
                };
                let content = std::fs::read_to_string(&path).map_err(|e| {
                    CompileError::new_simple(format!("readfile failed: {e}"))
                })?;
                self.stack.push(Value::Str(content));
            }
            Instr::WriteFile => {
                let content = match pop(&mut self.stack)? {
                    Value::Str(s) => s,
                    _ => {
                        return Err(CompileError::new_simple(
                            "writefile: expected string content",
                        ))
                    }
                };
                let path = match pop(&mut self.stack)? {
                    Value::Str(s) => s,
                    _ => return Err(CompileError::new_simple("writefile: expected string path")),
                };
                std::fs::write(&path, content).map_err(|e| {
                    CompileError::new_simple(format!("writefile failed: {e}"))
                })?;
            }
            Instr::ArgCount => {
                self.stack.push(Value::Int(self.args.len() as i64));
            }
            Instr::Arg => {
                let idx = pop_int(&mut self.stack)? as usize;
                let v = self.args.get(idx).cloned().ok_or_else(|| {
                    CompileError::new_simple("arg: index out of range")
                })?;
                self.stack.push(Value::Str(v));
            }
            Instr::PrintLn => {
                let v = pop(&mut self.stack)?;
                match v {
                    Value::Int(i) => println!("{}", i),
                    Value::Bool(b) => println!("{}", if b { "true" } else { "false" }),
                    Value::Str(s) => println!("{}", s),
                    Value::Array(_) => println!("<array>"),
                    Value::Vector(_) => println!("<vector>"),
                    Value::Stack(_) => println!("<stack>"),
                    Value::Map(_) => println!("<map>"),
                    Value::Set(_) => println!("<set>"),
                    Value::Option(_) => println!("<option>"),
                    Value::Result(_) => println!("<result>"),
                }
            }
            Instr::ReadInt(idx) => {
                let v = read_line()?;
                let parsed = v.parse::<i64>().map_err(|_| {
                    CompileError::new_simple("readln: expected integer input")
                })?;
                self.store_local(idx, Value::Int(parsed))?;
            }
            Instr::ReadBool(idx) => {
                let v = read_line()?;
                let lower = v.to_ascii_lowercase();
                let parsed = match lower.as_str() {
                    "true" => true,
                    "false" => false,
                    _ => {
                        return Err(CompileError::new_simple(
                            "readln: expected boolean input (true/false)",
                        ))
                    }
                };
                self.store_local(idx, Value::Bool(parsed))?;
            }
            Instr::ReadStr(idx) => {
                let v = read_line()?;
                self.store_local(idx, Value::Str(v))?;
            }
            _ => unreachable!("invalid io instruction"),
        }
        Ok(())
    }
}
