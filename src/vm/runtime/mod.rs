use crate::bytecode::{Bytecode, Function};
use crate::error::CompileError;

use super::value::Value;

mod builder;
mod dispatch;
mod helpers;
mod instr_builder;
mod instr_collections;
mod instr_control;
mod instr_core;
mod instr_env_fs;
mod instr_io;
mod instr_strings;
mod instr_time_log;

use builder::BcBuilder;

pub struct Vm {
    ip: usize,
    stack: Vec<Value>,
    locals: Vec<Value>,
    functions: Vec<Function>,
    call_stack: Vec<Frame>,
    current_func: usize,
    args: Vec<String>,
    builder: Option<BcBuilder>,
}

#[derive(Clone)]
struct Frame {
    ip: usize,
    locals: Vec<Value>,
    func: usize,
}

pub(super) enum Step {
    Continue,
    Halt,
}

impl Vm {
    pub fn new(bc: &Bytecode, args: Vec<String>) -> Self {
        let entry = bc.entry as usize;
        let entry_func = &bc.functions[entry];
        Self {
            ip: 0,
            stack: Vec::new(),
            locals: vec![Value::Int(0); entry_func.locals as usize],
            functions: bc.functions.clone(),
            call_stack: Vec::new(),
            current_func: entry,
            args,
            builder: None,
        }
    }

    pub fn run(&mut self) -> Result<(), CompileError> {
        loop {
            match dispatch::dispatch(self)? {
                Step::Continue => {}
                Step::Halt => break,
            }
        }
        Ok(())
    }

    fn store_local(&mut self, idx: u32, v: Value) -> Result<(), CompileError> {
        if let Some(slot) = self.locals.get_mut(idx as usize) {
            *slot = v;
            Ok(())
        } else {
            Err(CompileError::new_simple("invalid local index"))
        }
    }
}
