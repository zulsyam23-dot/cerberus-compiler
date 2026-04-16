use crate::ast::Type;
use crate::typecheck::env::{FuncSig, ProcSig, TypeEnv};

pub(super) fn register_core(env: &mut TypeEnv) {
    env.funcs.insert(
        "readfile".to_string(),
        FuncSig {
            params: vec![Type::String],
            ret: Type::String,
        },
    );
    env.procs.insert(
        "writefile".to_string(),
        ProcSig {
            params: vec![Type::String, Type::String],
        },
    );
    env.funcs.insert(
        "arg_count".to_string(),
        FuncSig {
            params: Vec::new(),
            ret: Type::Integer,
        },
    );
    env.funcs.insert(
        "arg".to_string(),
        FuncSig {
            params: vec![Type::Integer],
            ret: Type::String,
        },
    );
    env.funcs.insert(
        "strlen".to_string(),
        FuncSig {
            params: vec![Type::String],
            ret: Type::Integer,
        },
    );
    env.funcs.insert(
        "substr".to_string(),
        FuncSig {
            params: vec![Type::String, Type::Integer, Type::Integer],
            ret: Type::String,
        },
    );
    env.funcs.insert(
        "replace".to_string(),
        FuncSig {
            params: vec![Type::String, Type::String, Type::String],
            ret: Type::String,
        },
    );
    env.funcs.insert(
        "concat".to_string(),
        FuncSig {
            params: vec![Type::String, Type::String],
            ret: Type::String,
        },
    );
    env.funcs.insert(
        "string_clear".to_string(),
        FuncSig {
            params: vec![Type::String],
            ret: Type::String,
        },
    );
    env.funcs.insert(
        "env_get".to_string(),
        FuncSig {
            params: vec![Type::String],
            ret: Type::String,
        },
    );
    env.funcs.insert(
        "env_has".to_string(),
        FuncSig {
            params: vec![Type::String],
            ret: Type::Boolean,
        },
    );
    env.funcs.insert(
        "cwd".to_string(),
        FuncSig {
            params: Vec::new(),
            ret: Type::String,
        },
    );
    env.funcs.insert(
        "path_join".to_string(),
        FuncSig {
            params: vec![Type::String, Type::String],
            ret: Type::String,
        },
    );
    env.funcs.insert(
        "fs_exists".to_string(),
        FuncSig {
            params: vec![Type::String],
            ret: Type::Boolean,
        },
    );
    env.funcs.insert(
        "fs_listdir".to_string(),
        FuncSig {
            params: vec![Type::String],
            ret: Type::Vector(Box::new(Type::String)),
        },
    );
    env.funcs.insert(
        "os_exec".to_string(),
        FuncSig {
            params: vec![Type::String],
            ret: Type::Integer,
        },
    );
    env.funcs.insert(
        "now_timestamp".to_string(),
        FuncSig {
            params: Vec::new(),
            ret: Type::Integer,
        },
    );
    env.procs.insert(
        "sleep_ms".to_string(),
        ProcSig {
            params: vec![Type::Integer],
        },
    );
    env.procs.insert(
        "log_str".to_string(),
        ProcSig {
            params: vec![Type::String],
        },
    );
    env.procs.insert(
        "log_int".to_string(),
        ProcSig {
            params: vec![Type::Integer],
        },
    );
    env.procs.insert(
        "log_bool".to_string(),
        ProcSig {
            params: vec![Type::Boolean],
        },
    );
}
