use crate::bytecode::Instr;
use crate::error::CompileError;

use super::Vm;
use super::super::ops::pop;
use super::super::value::Value;

impl Vm {
    pub(super) fn exec_env_fs(&mut self, instr: Instr) -> Result<(), CompileError> {
        match instr {
            Instr::EnvGet => {
                let key = match pop(&mut self.stack)? {
                    Value::Str(s) => s,
                    _ => return Err(CompileError::new_simple("env_get: expected string key")),
                };
                let val = std::env::var(&key).unwrap_or_default();
                self.stack.push(Value::Str(val));
            }
            Instr::EnvHas => {
                let key = match pop(&mut self.stack)? {
                    Value::Str(s) => s,
                    _ => return Err(CompileError::new_simple("env_has: expected string key")),
                };
                self.stack.push(Value::Bool(std::env::var(&key).is_ok()));
            }
            Instr::Cwd => {
                let cwd = std::env::current_dir()
                    .map_err(|e| CompileError::new_simple(format!("cwd failed: {e}")))?;
                let s = cwd.to_string_lossy().to_string();
                self.stack.push(Value::Str(s));
            }
            Instr::PathJoin => {
                let b = match pop(&mut self.stack)? {
                    Value::Str(s) => s,
                    _ => return Err(CompileError::new_simple("path_join: expected string")),
                };
                let a = match pop(&mut self.stack)? {
                    Value::Str(s) => s,
                    _ => return Err(CompileError::new_simple("path_join: expected string")),
                };
                let joined = std::path::Path::new(&a).join(b);
                self.stack
                    .push(Value::Str(joined.to_string_lossy().to_string()));
            }
            Instr::FsExists => {
                let path = match pop(&mut self.stack)? {
                    Value::Str(s) => s,
                    _ => return Err(CompileError::new_simple("fs_exists: expected string path")),
                };
                self.stack.push(Value::Bool(std::path::Path::new(&path).exists()));
            }
            Instr::FsListDir => {
                let path = match pop(&mut self.stack)? {
                    Value::Str(s) => s,
                    _ => return Err(CompileError::new_simple("fs_listdir: expected string path")),
                };
                let mut out = Vec::new();
                let entries = std::fs::read_dir(&path).map_err(|e| {
                    CompileError::new_simple(format!("fs_listdir failed: {e}"))
                })?;
                for e in entries {
                    let e = e.map_err(|e| {
                        CompileError::new_simple(format!("fs_listdir failed: {e}"))
                    })?;
                    let name = e.file_name().to_string_lossy().to_string();
                    out.push(Value::Str(name));
                }
                self.stack.push(Value::Vector(out));
            }
            _ => unreachable!("invalid env/fs instruction"),
        }
        Ok(())
    }
}
