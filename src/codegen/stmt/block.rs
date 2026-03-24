use crate::ast::{Decl, FuncDecl, ProcDecl, VarDecl, Block};
use crate::bytecode::Instr;
use crate::error::CompileError;

use super::super::Codegen;
use super::emit_stmt;

pub(super) fn emit_block(cg: &mut Codegen, block: &Block) -> Result<(), CompileError> {
    for decl in &block.declarations {
        match decl {
            Decl::Var(VarDecl { name, ty }) => {
                let idx = cg.symbols.declare(name, ty)?;
                if let crate::ast::Type::Array { len, .. } = ty {
                    cg.code.push(Instr::AllocArray(*len as u32));
                    cg.code.push(Instr::Store(idx));
                }
            }
            Decl::Proc(ProcDecl { name, params, block }) => {
                let idx = cg.register_function(name, params.len() as u32)?;
                cg.emit_function_body(idx, params, block)?;
            }
            Decl::Func(FuncDecl {
                name,
                params,
                return_type: _,
                block,
            }) => {
                let idx = cg.register_function(name, params.len() as u32)?;
                cg.emit_function_body(idx, params, block)?;
            }
        }
    }
    for stmt in &block.statements {
        emit_stmt(cg, stmt)?;
    }
    Ok(())
}
