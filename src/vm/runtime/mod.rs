use crate::bytecode::{Bytecode, Function};
use crate::error::CompileError;

use super::value::Value;

mod builder;
mod config;
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
mod validate;

#[cfg(test)]
mod tests;

pub use config::{VmConfig, VmLimits};

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
    limits: VmLimits,
    steps_executed: u64,
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
    pub fn new(bc: &Bytecode, args: Vec<String>) -> Result<Self, CompileError> {
        Self::with_config(bc, args, VmConfig::default())
    }

    pub fn with_config(
        bc: &Bytecode,
        args: Vec<String>,
        config: VmConfig,
    ) -> Result<Self, CompileError> {
        if config.validate_bytecode {
            validate::validate_bytecode(bc, config.limits)?;
        }

        let entry = bc.entry as usize;
        let entry_func = &bc.functions[entry];
        Ok(Self {
            ip: 0,
            stack: Vec::new(),
            locals: vec![Value::Int(0); entry_func.locals as usize],
            functions: bc.functions.clone(),
            call_stack: Vec::new(),
            current_func: entry,
            args,
            builder: None,
            limits: config.limits,
            steps_executed: 0,
        })
    }

    pub fn run(&mut self) -> Result<(), CompileError> {
        loop {
            if self.steps_executed >= self.limits.max_steps {
                return Err(
                    self.decorate_runtime_error(CompileError::new_simple(format!(
                        "execution step limit exceeded ({})",
                        self.limits.max_steps
                    ))),
                );
            }
            self.steps_executed += 1;

            let step = dispatch::dispatch(self).map_err(|e| self.decorate_runtime_error(e))?;

            if self.stack.len() > self.limits.max_stack_size {
                return Err(
                    self.decorate_runtime_error(CompileError::new_simple(format!(
                        "stack size exceeded limit ({} > {})",
                        self.stack.len(),
                        self.limits.max_stack_size
                    ))),
                );
            }

            match step {
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

    fn jump_to(&mut self, target: u32) -> Result<(), CompileError> {
        let code_len = self
            .functions
            .get(self.current_func)
            .map(|f| f.code.len())
            .ok_or_else(|| CompileError::new_simple("invalid current function index"))?;
        let target = target as usize;
        if target >= code_len {
            return Err(CompileError::new_simple(format!(
                "jump target out of bounds: {} (code len {})",
                target, code_len
            )));
        }
        self.ip = target;
        Ok(())
    }

    fn decorate_runtime_error(&self, err: CompileError) -> CompileError {
        let trace = self.stack_trace();
        if trace.is_empty() {
            return err;
        }

        let mut message = err.message;
        message.push('\n');
        message.push_str(&trace);

        if let Some(span) = err.span {
            CompileError::new(message, span)
        } else {
            CompileError::new_simple(message)
        }
    }

    fn stack_trace(&self) -> String {
        let mut frames = Vec::new();

        if let Some(func) = self.functions.get(self.current_func) {
            frames.push(format!(
                "  at {}#{} ip {}",
                func.name,
                self.current_func,
                self.ip.saturating_sub(1)
            ));
        }

        for frame in self.call_stack.iter().rev() {
            let name = self
                .functions
                .get(frame.func)
                .map(|f| f.name.as_str())
                .unwrap_or("<invalid-function>");
            frames.push(format!("  at {}#{} ip {}", name, frame.func, frame.ip));
        }

        if frames.is_empty() {
            String::new()
        } else {
            format!("stack trace:\n{}", frames.join("\n"))
        }
    }
}
