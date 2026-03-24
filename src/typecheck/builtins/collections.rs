use crate::ast::Type;
use crate::typecheck::env::{FuncSig, TypeEnv};

pub(super) fn register_collections(env: &mut TypeEnv) {
    env.funcs.insert(
        "vector_new_int".to_string(),
        FuncSig {
            params: Vec::new(),
            ret: Type::Vector(Box::new(Type::Integer)),
        },
    );
    env.funcs.insert(
        "vector_new".to_string(),
        FuncSig {
            params: Vec::new(),
            ret: Type::Vector(Box::new(Type::Integer)),
        },
    );
    env.funcs.insert(
        "vector_new_bool".to_string(),
        FuncSig {
            params: Vec::new(),
            ret: Type::Vector(Box::new(Type::Boolean)),
        },
    );
    env.funcs.insert(
        "vector_new_str".to_string(),
        FuncSig {
            params: Vec::new(),
            ret: Type::Vector(Box::new(Type::String)),
        },
    );
    env.funcs.insert(
        "stack_new".to_string(),
        FuncSig {
            params: Vec::new(),
            ret: Type::StackInt,
        },
    );
    env.funcs.insert(
        "stack_len".to_string(),
        FuncSig {
            params: vec![Type::StackInt],
            ret: Type::Integer,
        },
    );
    env.funcs.insert(
        "stack_push".to_string(),
        FuncSig {
            params: vec![Type::StackInt, Type::Integer],
            ret: Type::StackInt,
        },
    );
    env.funcs.insert(
        "stack_top".to_string(),
        FuncSig {
            params: vec![Type::StackInt],
            ret: Type::Integer,
        },
    );
    env.funcs.insert(
        "stack_pop".to_string(),
        FuncSig {
            params: vec![Type::StackInt],
            ret: Type::StackInt,
        },
    );
    env.funcs.insert(
        "stack_clear".to_string(),
        FuncSig {
            params: vec![Type::StackInt],
            ret: Type::StackInt,
        },
    );
    env.funcs.insert(
        "map_new".to_string(),
        FuncSig {
            params: Vec::new(),
            ret: Type::MapStrStr,
        },
    );
    env.funcs.insert(
        "map_len".to_string(),
        FuncSig {
            params: vec![Type::MapStrStr],
            ret: Type::Integer,
        },
    );
    env.funcs.insert(
        "map_set".to_string(),
        FuncSig {
            params: vec![Type::MapStrStr, Type::String, Type::String],
            ret: Type::MapStrStr,
        },
    );
    env.funcs.insert(
        "map_get".to_string(),
        FuncSig {
            params: vec![Type::MapStrStr, Type::String],
            ret: Type::String,
        },
    );
    env.funcs.insert(
        "map_has".to_string(),
        FuncSig {
            params: vec![Type::MapStrStr, Type::String],
            ret: Type::Boolean,
        },
    );
    env.funcs.insert(
        "map_remove".to_string(),
        FuncSig {
            params: vec![Type::MapStrStr, Type::String],
            ret: Type::MapStrStr,
        },
    );
    env.funcs.insert(
        "map_clear".to_string(),
        FuncSig {
            params: vec![Type::MapStrStr],
            ret: Type::MapStrStr,
        },
    );
    env.funcs.insert(
        "set_new".to_string(),
        FuncSig {
            params: Vec::new(),
            ret: Type::SetStr,
        },
    );
    env.funcs.insert(
        "set_len".to_string(),
        FuncSig {
            params: vec![Type::SetStr],
            ret: Type::Integer,
        },
    );
    env.funcs.insert(
        "set_add".to_string(),
        FuncSig {
            params: vec![Type::SetStr, Type::String],
            ret: Type::SetStr,
        },
    );
    env.funcs.insert(
        "set_has".to_string(),
        FuncSig {
            params: vec![Type::SetStr, Type::String],
            ret: Type::Boolean,
        },
    );
    env.funcs.insert(
        "set_remove".to_string(),
        FuncSig {
            params: vec![Type::SetStr, Type::String],
            ret: Type::SetStr,
        },
    );
    env.funcs.insert(
        "set_clear".to_string(),
        FuncSig {
            params: vec![Type::SetStr],
            ret: Type::SetStr,
        },
    );
}
