mod builtins;
mod env;
mod expr;
mod stmt;
mod types;

use crate::ast::{Block, Decl, FuncDecl, ProcDecl, Program, Type, VarDecl};
use crate::error::CompileError;

use env::{FuncSig, ProcSig, TypeEnv};
use std::collections::HashSet;

pub struct TypeChecker {
    env: TypeEnv,
}

impl TypeChecker {
    pub fn new() -> Self {
        let mut env = TypeEnv::new();
        builtins::register_builtins(&mut env);
        Self { env }
    }

    pub fn check_program(&mut self, program: &Program) -> Result<(), CompileError> {
        self.check_block(&program.block)
    }

    fn check_block(&mut self, block: &Block) -> Result<(), CompileError> {
        let saved = self.env.clone();
        let mut local_vars = HashSet::new();

        for decl in &block.declarations {
            match decl {
                Decl::Var(VarDecl { name, ty }) => {
                    if local_vars.contains(name) {
                        return Err(CompileError::new_simple(format!(
                            "duplicate variable '{}'",
                            name
                        )));
                    }
                    local_vars.insert(name.clone());
                    if let Type::Array { elem, .. } = ty {
                        if **elem != Type::Integer {
                            return Err(CompileError::new_simple(
                                "only arrays of integer are supported",
                            ));
                        }
                    }
                    self.env.vars.insert(name.clone(), ty.clone());
                }
                Decl::Proc(ProcDecl { name, params, .. }) => {
                    if self.env.procs.contains_key(name) || self.env.funcs.contains_key(name) {
                        return Err(CompileError::new_simple(format!(
                            "duplicate procedure '{}'",
                            name
                        )));
                    }
                    self.env.procs.insert(
                        name.clone(),
                        ProcSig {
                            params: params.iter().map(|p| p.ty.clone()).collect(),
                        },
                    );
                }
                Decl::Func(FuncDecl {
                    name,
                    return_type,
                    params,
                    ..
                }) => {
                    if self.env.procs.contains_key(name) || self.env.funcs.contains_key(name) {
                        return Err(CompileError::new_simple(format!(
                            "duplicate function '{}'",
                            name
                        )));
                    }
                    self.env.funcs.insert(
                        name.clone(),
                        FuncSig {
                            params: params.iter().map(|p| p.ty.clone()).collect(),
                            ret: return_type.clone(),
                        },
                    );
                }
            }
        }

        for decl in &block.declarations {
            match decl {
                Decl::Proc(ProcDecl {
                    params,
                    block,
                    name,
                }) => {
                    let saved_ret = self.env.current_return.take();
                    self.env.current_return = None;
                    for p in params {
                        self.env.vars.insert(p.name.clone(), p.ty.clone());
                    }
                    if let Some(sig) = self.env.procs.get_mut(name) {
                        sig.params = params.iter().map(|p| p.ty.clone()).collect();
                    }
                    self.check_block(block)?;
                    self.env.current_return = saved_ret;
                }
                Decl::Func(FuncDecl {
                    return_type,
                    params,
                    block,
                    name,
                }) => {
                    let saved_ret = self.env.current_return.take();
                    self.env.current_return = Some(return_type.clone());
                    for p in params {
                        self.env.vars.insert(p.name.clone(), p.ty.clone());
                    }
                    if let Some(sig) = self.env.funcs.get_mut(name) {
                        sig.params = params.iter().map(|p| p.ty.clone()).collect();
                        sig.ret = return_type.clone();
                    }
                    self.check_block(block)?;
                    self.env.current_return = saved_ret;
                }
                _ => {}
            }
        }

        for stmt in &block.statements {
            stmt::check_stmt(&mut self.env, stmt)?;
        }

        self.env = saved;
        Ok(())
    }
}
