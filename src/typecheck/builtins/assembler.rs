use crate::ast::Type;
use crate::typecheck::env::{ProcSig, TypeEnv};

fn reg0(env: &mut TypeEnv, name: &str) {
    env.procs
        .insert(name.to_string(), ProcSig { params: Vec::new() });
}

fn reg1(env: &mut TypeEnv, name: &str, ty: Type) {
    env.procs
        .insert(name.to_string(), ProcSig { params: vec![ty] });
}

fn reg2(env: &mut TypeEnv, name: &str, a: Type, b: Type) {
    env.procs
        .insert(name.to_string(), ProcSig { params: vec![a, b] });
}

pub(super) fn register_assembler(env: &mut TypeEnv) {
    reg1(env, "bc_new", Type::String);
    reg0(env, "bc_new_op");
    reg0(env, "bc_main");
    reg0(env, "bc_main_op");
    reg2(env, "bc_func_begin", Type::String, Type::Integer);
    reg0(env, "bc_func_begin_op");
    reg0(env, "bc_func_end");
    reg0(env, "bc_func_end_op");
    reg1(env, "bc_label", Type::Integer);
    reg0(env, "bc_label_op");
    reg1(env, "bc_jump", Type::Integer);
    reg0(env, "bc_jump_op");
    reg1(env, "bc_jump_if_false", Type::Integer);
    reg0(env, "bc_jump_if_false_op");
    reg0(env, "bc_emit_halt");
    reg0(env, "bc_emit_halt_op");
    reg1(env, "bc_write", Type::String);
    reg0(env, "bc_write_op");
    reg1(env, "bc_emit_call", Type::String);
    reg0(env, "bc_emit_call_op");
    reg0(env, "bc_emit_ret");
    reg0(env, "bc_emit_ret_op");
    reg0(env, "bc_emit_retval");
    reg0(env, "bc_emit_retval_op");

    reg1(env, "builder_new", Type::String);
    reg0(env, "builder_main");
    reg1(env, "builder_write", Type::String);
    reg2(env, "builder_func_begin", Type::String, Type::Integer);
    reg0(env, "builder_func_end");
    reg0(env, "builder_emit_halt");
    reg1(env, "builder_label", Type::Integer);
    reg1(env, "builder_jump", Type::Integer);
    reg1(env, "builder_jump_if_false", Type::Integer);

    for name in [
        "emit_bcop_new",
        "emit_bcop_main",
        "emit_bcop_write",
        "emit_bcop_func_begin",
        "emit_bcop_func_end",
        "emit_bcop_halt",
        "emit_bcop_label",
        "emit_bcop_jump",
        "emit_bcop_jump_if_false",
    ] {
        reg0(env, name);
    }

    reg1(env, "bc_emit_print_str", Type::String);
    reg1(env, "bc_emit_const_int", Type::Integer);
    reg0(env, "bc_emit_const_int_op");
    reg0(env, "bc_emit_store0");
    reg0(env, "bc_emit_load0");
    reg0(env, "bc_emit_println");
    reg0(env, "bc_emit_println_op");
    reg1(env, "bc_emit_const_bool", Type::Boolean);
    reg0(env, "bc_emit_const_bool_op");
    reg1(env, "bc_emit_const_str", Type::String);
    reg0(env, "bc_emit_const_str_op");
    reg1(env, "bc_emit_load", Type::Integer);
    reg0(env, "bc_emit_load_op");
    reg1(env, "bc_emit_store", Type::Integer);
    reg0(env, "bc_emit_store_op");

    for name in [
        "bc_emit_add",
        "bc_emit_sub",
        "bc_emit_mul",
        "bc_emit_div",
        "bc_emit_eq",
        "bc_emit_ne",
        "bc_emit_lt",
        "bc_emit_le",
        "bc_emit_gt",
        "bc_emit_ge",
        "bc_emit_and",
        "bc_emit_or",
        "bc_emit_not",
        "bc_emit_neg",
        "bc_emit_strlen",
        "bc_emit_concat",
        "bc_emit_substr",
        "bc_emit_replace",
        "bc_emit_vec_new",
        "bc_emit_vec_len",
        "bc_emit_vec_get",
        "bc_emit_vec_set",
        "bc_emit_vec_push",
        "bc_emit_vec_remove",
        "bc_emit_vec_last",
        "bc_emit_vec_pop",
        "bc_emit_map_new",
        "bc_emit_map_len",
        "bc_emit_map_set",
        "bc_emit_map_get",
        "bc_emit_map_has",
        "bc_emit_map_remove",
        "bc_emit_map_clear",
        "bc_emit_readfile",
        "bc_emit_writefile",
        "bc_emit_arg_count",
        "bc_emit_arg",
        "bc_emit_str_clear",
        "bc_emit_vec_clear",
        "bc_emit_env_get",
        "bc_emit_env_has",
        "bc_emit_cwd",
        "bc_emit_path_join",
        "bc_emit_fs_exists",
        "bc_emit_fs_listdir",
        "bc_emit_now_timestamp",
    ] {
        reg0(env, name);
        reg0(env, &format!("{}_op", name));
    }
}
