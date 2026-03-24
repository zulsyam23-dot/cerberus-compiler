mod assembler;
mod collections;
mod core;
mod options;
mod results;

use super::env::TypeEnv;

pub fn register_builtins(env: &mut TypeEnv) {
    core::register_core(env);
    collections::register_collections(env);
    assembler::register_assembler(env);
    options::register_option_builtins(env);
    results::register_result_builtins(env);
}
