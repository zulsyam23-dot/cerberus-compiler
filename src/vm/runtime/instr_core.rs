use crate::bytecode::Instr;
use crate::error::CompileError;

use super::super::ops::{
    bin_bool, bin_cmp, bin_int_checked, pop, pop_bool, pop_int, unary_int_checked,
};
use super::super::value::Value;
use super::Vm;

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
            Instr::Add => bin_int_checked(&mut self.stack, "add", i64::checked_add)?,
            Instr::Sub => bin_int_checked(&mut self.stack, "sub", i64::checked_sub)?,
            Instr::Mul => bin_int_checked(&mut self.stack, "mul", i64::checked_mul)?,
            Instr::Div => {
                let b = pop_int(&mut self.stack)?;
                if b == 0 {
                    return Err(CompileError::new_simple("div: division by zero"));
                }
                let a = pop_int(&mut self.stack)?;
                let out = a
                    .checked_div(b)
                    .ok_or_else(|| CompileError::new_simple("div: integer overflow"))?;
                self.stack.push(Value::Int(out));
            }
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
            Instr::Neg => unary_int_checked(&mut self.stack, "neg", i64::checked_neg)?,
            _ => unreachable!("invalid core instruction"),
        }
        Ok(())
    }
}
