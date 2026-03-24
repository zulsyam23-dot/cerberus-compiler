mod expr;
mod stmt;
mod symbols;

use std::collections::HashMap;

use crate::ast::{Param, Program};
use crate::bytecode::{Bytecode, Function};
use crate::error::CompileError;

use symbols::Symbols;

pub struct Codegen {
    module_name: String,
    program_name: String,
    symbols: Symbols,
    code: Vec<crate::bytecode::Instr>,
    functions: Vec<Function>,
    func_index: HashMap<String, u32>,
}

impl Codegen {
    pub fn new(program_name: &str, module_name: &str) -> Self {
        Self {
            module_name: module_name.to_string(),
            program_name: program_name.to_string(),
            symbols: Symbols::new(),
            code: Vec::new(),
            functions: Vec::new(),
            func_index: HashMap::new(),
        }
    }

    pub fn emit_program(&mut self, program: &Program) -> Result<Bytecode, CompileError> {
        let entry = self.register_function("main", 0)?;
        self.emit_function_body(entry, &[], &program.block)?;
        Ok(Bytecode {
            name: format!("{}::{}", self.program_name, self.module_name),
            functions: self.functions.clone(),
            entry,
        })
    }

    pub(crate) fn register_function(&mut self, name: &str, param_count: u32) -> Result<u32, CompileError> {
        if let Some(&idx) = self.func_index.get(name) {
            return Ok(idx);
        }
        let idx = self.functions.len() as u32;
        self.functions.push(Function {
            name: name.to_string(),
            param_count,
            locals: 0,
            code: Vec::new(),
        });
        self.func_index.insert(name.to_string(), idx);
        Ok(idx)
    }

    pub(crate) fn emit_function_body(
        &mut self,
        idx: u32,
        params: &[Param],
        block: &crate::ast::Block,
    ) -> Result<(), CompileError> {
        let saved_symbols = self.symbols.clone();
        let saved_code = std::mem::take(&mut self.code);

        self.symbols = Symbols::new();
        self.code = Vec::new();
        for p in params {
            let _ = self.symbols.declare(&p.name, &p.ty)?;
        }
        stmt::emit_block(self, block)?;
        self.code.push(crate::bytecode::Instr::Ret);

        let locals = self.symbols.count().max(params.len()) as u32;
        let func = Function {
            name: self.functions[idx as usize].name.clone(),
            param_count: params.len() as u32,
            locals,
            code: self.code.clone(),
        };
        self.functions[idx as usize] = func;

        self.symbols = saved_symbols;
        self.code = saved_code;
        Ok(())
    }
}
