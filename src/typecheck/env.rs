use std::collections::HashMap;

use crate::ast::Type;

#[derive(Clone)]
pub struct TypeEnv {
    pub vars: HashMap<String, Type>,
    pub funcs: HashMap<String, FuncSig>,
    pub procs: HashMap<String, ProcSig>,
    pub current_return: Option<Type>,
}

impl TypeEnv {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            funcs: HashMap::new(),
            procs: HashMap::new(),
            current_return: None,
        }
    }
}

#[derive(Clone)]
pub struct FuncSig {
    pub params: Vec<Type>,
    pub ret: Type,
}

#[derive(Clone)]
pub struct ProcSig {
    pub params: Vec<Type>,
}
