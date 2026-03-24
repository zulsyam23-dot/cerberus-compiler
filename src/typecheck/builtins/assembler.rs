use crate::ast::Type;
use crate::typecheck::env::{ProcSig, TypeEnv};

pub(super) fn register_assembler(env: &mut TypeEnv) {
    env.procs.insert(
        "bc_new".to_string(),
        ProcSig {
            params: vec![Type::String],
        },
    );
    env.procs.insert(
        "bc_main".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_print_str".to_string(),
        ProcSig {
            params: vec![Type::String],
        },
    );
    env.procs.insert(
        "bc_emit_const_int".to_string(),
        ProcSig {
            params: vec![Type::Integer],
        },
    );
    env.procs.insert(
        "bc_emit_store0".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_load0".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_println".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_const_bool".to_string(),
        ProcSig {
            params: vec![Type::Boolean],
        },
    );
    env.procs.insert(
        "bc_emit_const_str".to_string(),
        ProcSig {
            params: vec![Type::String],
        },
    );
    env.procs.insert(
        "bc_emit_load".to_string(),
        ProcSig {
            params: vec![Type::Integer],
        },
    );
    env.procs.insert(
        "bc_emit_store".to_string(),
        ProcSig {
            params: vec![Type::Integer],
        },
    );
    env.procs.insert(
        "bc_emit_add".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_sub".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_mul".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_div".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_eq".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_ne".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_lt".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_le".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_gt".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_ge".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_and".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_or".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_not".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_neg".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_strlen".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_concat".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_substr".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_replace".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_vec_new".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_vec_len".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_vec_get".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_vec_set".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_vec_push".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_vec_remove".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_vec_last".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_vec_pop".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_map_new".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_map_len".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_map_set".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_map_get".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_map_has".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_map_remove".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_readfile".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_writefile".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_arg_count".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_arg".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_str_clear".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_vec_clear".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_emit_map_clear".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_label".to_string(),
        ProcSig {
            params: vec![Type::Integer],
        },
    );
    env.procs.insert(
        "bc_jump".to_string(),
        ProcSig {
            params: vec![Type::Integer],
        },
    );
    env.procs.insert(
        "bc_jump_if_false".to_string(),
        ProcSig {
            params: vec![Type::Integer],
        },
    );
    env.procs.insert(
        "bc_emit_halt".to_string(),
        ProcSig { params: Vec::new() },
    );
    env.procs.insert(
        "bc_write".to_string(),
        ProcSig {
            params: vec![Type::String],
        },
    );
}
