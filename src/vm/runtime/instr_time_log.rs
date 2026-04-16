use crate::bytecode::Instr;
use crate::error::CompileError;

use super::super::ops::{pop, pop_bool, pop_int};
use super::super::value::Value;
use super::Vm;

impl Vm {
    pub(super) fn exec_time_log(&mut self, instr: Instr) -> Result<(), CompileError> {
        match instr {
            Instr::NowTimestamp => {
                let ts = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_err(|e| CompileError::new_simple(format!("now_timestamp failed: {e}")))?;
                self.stack.push(Value::Int(ts.as_secs() as i64));
            }
            Instr::SleepMs => {
                let ms = pop_int(&mut self.stack)?;
                if ms < 0 {
                    return Err(CompileError::new_simple("sleep_ms: negative duration"));
                }
                let ms = ms as u64;
                std::thread::sleep(std::time::Duration::from_millis(ms));
            }
            Instr::LogStr => {
                let s = match pop(&mut self.stack)? {
                    Value::Str(s) => s,
                    _ => return Err(CompileError::new_simple("log_str: expected string")),
                };
                eprintln!("[log] {}", s);
            }
            Instr::LogInt => {
                let v = pop_int(&mut self.stack)?;
                eprintln!("[log] {}", v);
            }
            Instr::LogBool => {
                let v = pop_bool(&mut self.stack)?;
                eprintln!("[log] {}", if v { "true" } else { "false" });
            }
            _ => unreachable!("invalid time/log instruction"),
        }
        Ok(())
    }
}
