#[derive(Debug, Clone, Copy)]
pub struct VmLimits {
    pub max_steps: u64,
    pub max_stack_size: usize,
    pub max_call_depth: usize,
    pub max_locals_per_function: usize,
    pub max_functions: usize,
    pub max_instructions_per_function: usize,
}

impl Default for VmLimits {
    fn default() -> Self {
        Self {
            max_steps: 10_000_000,
            max_stack_size: 1_000_000,
            max_call_depth: 4_096,
            max_locals_per_function: 1_000_000,
            max_functions: 100_000,
            max_instructions_per_function: 10_000_000,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VmConfig {
    pub limits: VmLimits,
    pub validate_bytecode: bool,
}

impl Default for VmConfig {
    fn default() -> Self {
        Self {
            limits: VmLimits::default(),
            validate_bytecode: true,
        }
    }
}

impl VmConfig {
    #[cfg(test)]
    pub fn with_limits(limits: VmLimits) -> Self {
        Self {
            limits,
            ..Self::default()
        }
    }
}
