use crate::bytecode::Instr;
use crate::error::CompileError;

use super::Vm;
use super::super::ops::{bin_bool, bin_cmp, bin_int, pop, pop_bool, pop_int};
use super::super::value::Value;

impl Vm {
    pub(super) fn exec_core(&mut self, instr: Instr) -> Result<(), CompileError> {
        match instr {
            Instr::ConstInt(v) => self.stack.push(Value::Int(v)),
            Instr::ConstBool(v) => self.stack.push(Value::Bool(v)),
            Instr::ConstStr(s) => self.stack.push(Value::Str(s)),
            Instr::Load(i) => {
                let v = self
                    .locals
                    .get(i as usize)
                    .cloned()
                    .ok_or_else(|| CompileError::new_simple("invalid local index"))?;
                self.stack.push(v);
            }
            Instr::Store(i) => {
                let v = pop(&mut self.stack)?;
                if let Some(slot) = self.locals.get_mut(i as usize) {
                    *slot = v;
                } else {
                    return Err(CompileError::new_simple("invalid local index"));
                }
            }
            Instr::Add => bin_int(&mut self.stack, |a, b| a + b)?,
            Instr::Sub => bin_int(&mut self.stack, |a, b| a - b)?,
            Instr::Mul => bin_int(&mut self.stack, |a, b| a * b)?,
            Instr::Div => bin_int(&mut self.stack, |a, b| a / b)?,
            Instr::Eq => {
                let b = pop(&mut self.stack)?;
                let a = pop(&mut self.stack)?;
                match (a, b) {
                    (Value::Int(x), Value::Int(y)) => {
                        self.stack.push(Value::Bool(x == y));
                    }
                    (Value::Bool(x), Value::Bool(y)) => {
                        self.stack.push(Value::Bool(x == y));
                    }
                    (Value::Str(x), Value::Str(y)) => {
                        self.stack.push(Value::Bool(x == y));
                    }
                    _ => return Err(CompileError::new_simple("eq: type mismatch")),
                }
            }
            Instr::Ne => {
                let b = pop(&mut self.stack)?;
                let a = pop(&mut self.stack)?;
                match (a, b) {
                    (Value::Int(x), Value::Int(y)) => {
                        self.stack.push(Value::Bool(x != y));
                    }
                    (Value::Bool(x), Value::Bool(y)) => {
                        self.stack.push(Value::Bool(x != y));
                    }
                    (Value::Str(x), Value::Str(y)) => {
                        self.stack.push(Value::Bool(x != y));
                    }
                    _ => return Err(CompileError::new_simple("ne: type mismatch")),
                }
            }
            Instr::Lt => bin_cmp(&mut self.stack, |a, b| a < b)?,
            Instr::Le => bin_cmp(&mut self.stack, |a, b| a <= b)?,
            Instr::Gt => bin_cmp(&mut self.stack, |a, b| a > b)?,
            Instr::Ge => bin_cmp(&mut self.stack, |a, b| a >= b)?,
            Instr::And => bin_bool(&mut self.stack, |a, b| a && b)?,
            Instr::Or => bin_bool(&mut self.stack, |a, b| a || b)?,
            Instr::Not => {
                let v = pop_bool(&mut self.stack)?;
                self.stack.push(Value::Bool(!v));
            }
            Instr::Neg => {
                let v = pop_int(&mut self.stack)?;
                self.stack.push(Value::Int(-v));
            }
            _ => unreachable!("invalid core instruction"),
        }
        Ok(())
    }
}
