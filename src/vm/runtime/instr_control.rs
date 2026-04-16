use crate::bytecode::Instr;
use crate::error::CompileError;

use super::super::ops::{pop, pop_bool};
use super::super::value::Value;
use super::{Frame, Step, Vm};

impl Vm {
    pub(super) fn exec_control(&mut self, instr: Instr) -> Result<Step, CompileError> {
        match instr {
            Instr::Jump(target) => {
                self.jump_to(target)?;
                Ok(Step::Continue)
            }
            Instr::JumpIfFalse(target) => {
                let v = pop_bool(&mut self.stack)?;
                if !v {
                    self.jump_to(target)?;
                }
                Ok(Step::Continue)
            }
            Instr::Call(idx) => {
                if self.call_stack.len() >= self.limits.max_call_depth {
                    return Err(CompileError::new_simple(format!(
                        "call depth exceeded ({})",
                        self.limits.max_call_depth
                    )));
                }

                let callee = idx as usize;
                let (param_count, locals_count) = {
                    let func = self
                        .functions
                        .get(callee)
                        .ok_or_else(|| CompileError::new_simple("invalid function index"))?;
                    (func.param_count as usize, func.locals as usize)
                };

                if locals_count > self.limits.max_locals_per_function {
                    return Err(CompileError::new_simple(format!(
                        "function locals exceed VM limit ({} > {})",
                        locals_count, self.limits.max_locals_per_function
                    )));
                }

                let mut new_locals = vec![Value::Int(0); locals_count];
                for i in (0..param_count).rev() {
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
