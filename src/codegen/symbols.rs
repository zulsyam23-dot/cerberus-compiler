use std::collections::HashMap;

use crate::ast::Type;
use crate::error::CompileError;

#[derive(Default, Clone)]
pub struct Symbols {
    locals: HashMap<String, u32>,
    types: HashMap<String, Type>,
}

impl Symbols {
    pub fn new() -> Self {
        Self {
            locals: HashMap::new(),
            types: HashMap::new(),
        }
    }

    pub fn count(&self) -> usize {
        self.locals.len()
    }

    pub fn declare(&mut self, name: &str, ty: &Type) -> Result<u32, CompileError> {
        if self.locals.contains_key(name) {
            return Err(CompileError::new_simple(format!(
                "duplicate variable '{}'",
                name
            )));
        }
        let idx = self.locals.len() as u32;
        self.locals.insert(name.to_string(), idx);
        self.types.insert(name.to_string(), ty.clone());
        Ok(idx)
    }

    pub fn get(&self, name: &str) -> Result<u32, CompileError> {
        self.locals
            .get(name)
            .copied()
            .ok_or_else(|| CompileError::new_simple(format!("unknown variable '{}'", name)))
    }

    pub fn get_type(&self, name: &str) -> Option<&Type> {
        self.types.get(name)
    }
}
