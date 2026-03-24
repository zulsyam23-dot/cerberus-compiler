use crate::bytecode::Instr;
use crate::error::CompileError;

use super::{Frame, Step, Vm};
use super::super::ops::{pop, pop_bool};
use super::super::value::Value;

impl Vm {
    pub(super) fn exec_control(&mut self, instr: Instr) -> Result<Step, CompileError> {
        match instr {
            Instr::Jump(target) => {
                self.ip = target as usize;
                Ok(Step::Continue)
            }
            Instr::JumpIfFalse(target) => {
                let v = pop_bool(&mut self.stack)?;
                if !v {
                    self.ip = target as usize;
                }
                Ok(Step::Continue)
            }
            Instr::Call(idx) => {
                let callee = idx as usize;
                let func = self.functions.get(callee).ok_or_else(|| {
                    CompileError::new_simple("invalid function index")
                })?;
                let mut new_locals = vec![Value::Int(0); func.locals as usize];
                for i in (0..func.param_count as usize).rev() {
                    let v = pop(&mut self.stack)?;
                    if i < new_locals.len() {
                        new_locals[i] = v;
                    } else {
                        return Err(CompileError::new_simple("invalid param index"));
                    }
                }
                let frame = Frame {
                    ip: self.ip,
                    locals: std::mem::replace(&mut self.locals, new_locals),
                    func: self.current_func,
                };
                self.call_stack.push(frame);
                self.current_func = callee;
                self.ip = 0;
                Ok(Step::Continue)
            }
            Instr::Ret => {
                if let Some(frame) = self.call_stack.pop() {
                    self.locals = frame.locals;
                    self.current_func = frame.func;
                    self.ip = frame.ip;
                    Ok(Step::Continue)
                } else {
                    Ok(Step::Halt)
                }
            }
            Instr::RetVal => {
                let ret = pop(&mut self.stack)?;
                if let Some(frame) = self.call_stack.pop() {
                    self.locals = frame.locals;
                    self.current_func = frame.func;
                    self.ip = frame.ip;
                    self.stack.push(ret);
                    Ok(Step::Continue)
                } else {
                    Ok(Step::Halt)
                }
            }
            Instr::Halt => Ok(Step::Halt),
            _ => unreachable!("invalid control instruction"),
        }
    }
}
