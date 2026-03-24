use crate::ast::Type;
use crate::typecheck::env::{FuncSig, TypeEnv};

pub(super) fn register_option_builtins(env: &mut TypeEnv) {
    env.funcs.insert(
        "option_some_int".to_string(),
        FuncSig {
            params: vec![Type::Integer],
            ret: Type::Option(Box::new(Type::Integer)),
        },
    );
    env.funcs.insert(
        "option_some_bool".to_string(),
        FuncSig {
            params: vec![Type::Boolean],
            ret: Type::Option(Box::new(Type::Boolean)),
        },
    );
    env.funcs.insert(
        "option_some_str".to_string(),
        FuncSig {
            params: vec![Type::String],
            ret: Type::Option(Box::new(Type::String)),
        },
    );
    env.funcs.insert(
        "option_none_int".to_string(),
        FuncSig {
            params: Vec::new(),
            ret: Type::Option(Box::new(Type::Integer)),
        },
    );
    env.funcs.insert(
        "option_none_bool".to_string(),
        FuncSig {
            params: Vec::new(),
            ret: Type::Option(Box::new(Type::Boolean)),
        },
    );
    env.funcs.insert(
        "option_none_str".to_string(),
        FuncSig {
            params: Vec::new(),
            ret: Type::Option(Box::new(Type::String)),
        },
    );
    env.funcs.insert(
        "option_is_some_int".to_string(),
        FuncSig {
            params: vec![Type::Option(Box::new(Type::Integer))],
            ret: Type::Boolean,
        },
    );
    env.funcs.insert(
        "option_is_some_bool".to_string(),
        FuncSig {
            params: vec![Type::Option(Box::new(Type::Boolean))],
            ret: Type::Boolean,
        },
    );
    env.funcs.insert(
        "option_is_some_str".to_string(),
        FuncSig {
            params: vec![Type::Option(Box::new(Type::String))],
            ret: Type::Boolean,
        },
    );
    env.funcs.insert(
        "option_unwrap_int".to_string(),
        FuncSig {
            params: vec![Type::Option(Box::new(Type::Integer))],
            ret: Type::Integer,
        },
    );
    env.funcs.insert(
        "option_unwrap_bool".to_string(),
        FuncSig {
            params: vec![Type::Option(Box::new(Type::Boolean))],
            ret: Type::Boolean,
        },
    );
    env.funcs.insert(
        "option_unwrap_str".to_string(),
        FuncSig {
            params: vec![Type::Option(Box::new(Type::String))],
            ret: Type::String,
        },
    );
    env.funcs.insert(
        "option_unwrap_or_int".to_string(),
        FuncSig {
            params: vec![Type::Option(Box::new(Type::Integer)), Type::Integer],
            ret: Type::Integer,
        },
    );
    env.funcs.insert(
        "option_unwrap_or_bool".to_string(),
        FuncSig {
            params: vec![Type::Option(Box::new(Type::Boolean)), Type::Boolean],
            ret: Type::Boolean,
        },
    );
    env.funcs.insert(
        "option_unwrap_or_str".to_string(),
        FuncSig {
            params: vec![Type::Option(Box::new(Type::String)), Type::String],
            ret: Type::String,
        },
    );
}
