use crate::ast::Type;
use crate::typecheck::env::{FuncSig, TypeEnv};

pub(super) fn register_result_builtins(env: &mut TypeEnv) {
    env.funcs.insert(
        "result_ok_int".to_string(),
        FuncSig {
            params: vec![Type::Integer],
            ret: Type::Result(Box::new(Type::Integer)),
        },
    );
    env.funcs.insert(
        "result_ok_bool".to_string(),
        FuncSig {
            params: vec![Type::Boolean],
            ret: Type::Result(Box::new(Type::Boolean)),
        },
    );
    env.funcs.insert(
        "result_ok_str".to_string(),
        FuncSig {
            params: vec![Type::String],
            ret: Type::Result(Box::new(Type::String)),
        },
    );
    env.funcs.insert(
        "result_err_int".to_string(),
        FuncSig {
            params: vec![Type::String],
            ret: Type::Result(Box::new(Type::Integer)),
        },
    );
    env.funcs.insert(
        "result_err_bool".to_string(),
        FuncSig {
            params: vec![Type::String],
            ret: Type::Result(Box::new(Type::Boolean)),
        },
    );
    env.funcs.insert(
        "result_err_str".to_string(),
        FuncSig {
            params: vec![Type::String],
            ret: Type::Result(Box::new(Type::String)),
        },
    );
    env.funcs.insert(
        "result_is_ok_int".to_string(),
        FuncSig {
            params: vec![Type::Result(Box::new(Type::Integer))],
            ret: Type::Boolean,
        },
    );
    env.funcs.insert(
        "result_is_ok_bool".to_string(),
        FuncSig {
            params: vec![Type::Result(Box::new(Type::Boolean))],
            ret: Type::Boolean,
        },
    );
    env.funcs.insert(
        "result_is_ok_str".to_string(),
        FuncSig {
            params: vec![Type::Result(Box::new(Type::String))],
            ret: Type::Boolean,
        },
    );
    env.funcs.insert(
        "result_unwrap_int".to_string(),
        FuncSig {
            params: vec![Type::Result(Box::new(Type::Integer))],
            ret: Type::Integer,
        },
    );
    env.funcs.insert(
        "result_unwrap_bool".to_string(),
        FuncSig {
            params: vec![Type::Result(Box::new(Type::Boolean))],
            ret: Type::Boolean,
        },
    );
    env.funcs.insert(
        "result_unwrap_str".to_string(),
        FuncSig {
            params: vec![Type::Result(Box::new(Type::String))],
            ret: Type::String,
        },
    );
    env.funcs.insert(
        "result_unwrap_or_int".to_string(),
        FuncSig {
            params: vec![Type::Result(Box::new(Type::Integer)), Type::Integer],
            ret: Type::Integer,
        },
    );
    env.funcs.insert(
        "result_unwrap_or_bool".to_string(),
        FuncSig {
            params: vec![Type::Result(Box::new(Type::Boolean)), Type::Boolean],
            ret: Type::Boolean,
        },
    );
    env.funcs.insert(
        "result_unwrap_or_str".to_string(),
        FuncSig {
            params: vec![Type::Result(Box::new(Type::String)), Type::String],
            ret: Type::String,
        },
    );
    env.funcs.insert(
        "result_unwrap_err_int".to_string(),
        FuncSig {
            params: vec![Type::Result(Box::new(Type::Integer))],
            ret: Type::String,
        },
    );
    env.funcs.insert(
        "result_unwrap_err_bool".to_string(),
        FuncSig {
            params: vec![Type::Result(Box::new(Type::Boolean))],
            ret: Type::String,
        },
    );
    env.funcs.insert(
        "result_unwrap_err_str".to_string(),
        FuncSig {
            params: vec![Type::Result(Box::new(Type::String))],
            ret: Type::String,
        },
    );
}
