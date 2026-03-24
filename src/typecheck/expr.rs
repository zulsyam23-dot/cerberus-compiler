use crate::ast::{BinaryOp, CallExpr, Expr, Type, UnaryOp};
use crate::error::CompileError;

use super::env::TypeEnv;
use super::types::{expect_bool, expect_int};

pub fn check_expr(env: &mut TypeEnv, expr: &Expr) -> Result<Type, CompileError> {
    match expr {
        Expr::Int(_) => Ok(Type::Integer),
        Expr::Bool(_) => Ok(Type::Boolean),
        Expr::Str(_) => Ok(Type::String),
        Expr::Var(name) => env
            .vars
            .get(name)
            .cloned()
            .ok_or_else(|| CompileError::new_simple(format!("unknown variable '{}'", name))),
        Expr::Index { name, index } => {
            let var_ty = env
                .vars
                .get(name)
                .cloned()
                .ok_or_else(|| CompileError::new_simple(format!("unknown variable '{}'", name)))?;
            let idx_ty = check_expr(env, index)?;
            expect_int(&idx_ty, "array index")?;
            match var_ty {
                Type::Array { elem, .. } => Ok(*elem),
                _ => Err(CompileError::new_simple("not an array")),
            }
        }
        Expr::Call(CallExpr { name, args }) => {
            if let Some(ty) = check_vector_call(env, name, args)? {
                return Ok(ty);
            }
            let sig = env
                .funcs
                .get(name)
                .cloned()
                .ok_or_else(|| CompileError::new_simple(format!("unknown function '{}'", name)))?;
            if sig.params.len() != args.len() {
                return Err(CompileError::new_simple(
                    "function argument count mismatch",
                ));
            }
            for (i, arg) in args.iter().enumerate() {
                let ty = check_expr(env, arg)?;
                if ty != sig.params[i] {
                    return Err(CompileError::new_simple(
                        "function argument type mismatch",
                    ));
                }
            }
            Ok(sig.ret)
        }
        Expr::Unary { op, expr } => {
            let ty = check_expr(env, expr)?;
            match op {
                UnaryOp::Neg => {
                    expect_int(&ty, "unary '-'")?;
                    Ok(Type::Integer)
                }
                UnaryOp::Not => {
                    expect_bool(&ty, "unary 'not'")?;
                    Ok(Type::Boolean)
                }
            }
        }
        Expr::Binary { op, left, right } => {
            let lt = check_expr(env, left)?;
            let rt = check_expr(env, right)?;
            match op {
                BinaryOp::Add
                | BinaryOp::Sub
                | BinaryOp::Mul
                | BinaryOp::Div => {
                    expect_int(&lt, "binary arithmetic")?;
                    expect_int(&rt, "binary arithmetic")?;
                    Ok(Type::Integer)
                }
                BinaryOp::Eq | BinaryOp::Ne => {
                    if lt != rt {
                        return Err(CompileError::new_simple(
                            "type error in comparison: operands must have same type",
                        ));
                    }
                    if !matches!(lt, Type::Integer | Type::Boolean | Type::String) {
                        return Err(CompileError::new_simple(
                            "type error in comparison: unsupported type",
                        ));
                    }
                    Ok(Type::Boolean)
                }
                BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => {
                    expect_int(&lt, "comparison")?;
                    expect_int(&rt, "comparison")?;
                    Ok(Type::Boolean)
                }
                BinaryOp::And | BinaryOp::Or => {
                    expect_bool(&lt, "boolean operator")?;
                    expect_bool(&rt, "boolean operator")?;
                    Ok(Type::Boolean)
                }
            }
        }
    }
}

fn check_vector_call(
    env: &mut TypeEnv,
    name: &str,
    args: &[Expr],
) -> Result<Option<Type>, CompileError> {
    match name {
        "vector_len" => {
            if args.len() != 1 {
                return Err(CompileError::new_simple("vector_len expects 1 argument"));
            }
            let ty = check_expr(env, &args[0])?;
            match ty {
                Type::Vector(_) => Ok(Some(Type::Integer)),
                _ => Err(CompileError::new_simple("vector_len expects vector")),
            }
        }
        "vector_get" => {
            if args.len() != 2 {
                return Err(CompileError::new_simple("vector_get expects 2 arguments"));
            }
            let vec_ty = check_expr(env, &args[0])?;
            let idx_ty = check_expr(env, &args[1])?;
            expect_int(&idx_ty, "vector_get index")?;
            match vec_ty {
                Type::Vector(inner) => Ok(Some(*inner)),
                _ => Err(CompileError::new_simple("vector_get expects vector")),
            }
        }
        "vector_set" => {
            if args.len() != 3 {
                return Err(CompileError::new_simple("vector_set expects 3 arguments"));
            }
            let vec_ty = check_expr(env, &args[0])?;
            let idx_ty = check_expr(env, &args[1])?;
            let val_ty = check_expr(env, &args[2])?;
            expect_int(&idx_ty, "vector_set index")?;
            match vec_ty {
                Type::Vector(inner) => {
                    if *inner != val_ty {
                        return Err(CompileError::new_simple("vector_set value type mismatch"));
                    }
                    Ok(Some(Type::Vector(inner)))
                }
                _ => Err(CompileError::new_simple("vector_set expects vector")),
            }
        }
        "vector_push" => {
            if args.len() != 2 {
                return Err(CompileError::new_simple("vector_push expects 2 arguments"));
            }
            let vec_ty = check_expr(env, &args[0])?;
            let val_ty = check_expr(env, &args[1])?;
            match vec_ty {
                Type::Vector(inner) => {
                    if *inner != val_ty {
                        return Err(CompileError::new_simple("vector_push value type mismatch"));
                    }
                    Ok(Some(Type::Vector(inner)))
                }
                _ => Err(CompileError::new_simple("vector_push expects vector")),
            }
        }
        "vector_remove" => {
            if args.len() != 2 {
                return Err(CompileError::new_simple("vector_remove expects 2 arguments"));
            }
            let vec_ty = check_expr(env, &args[0])?;
            let idx_ty = check_expr(env, &args[1])?;
            expect_int(&idx_ty, "vector_remove index")?;
            match vec_ty {
                Type::Vector(inner) => Ok(Some(Type::Vector(inner))),
                _ => Err(CompileError::new_simple("vector_remove expects vector")),
            }
        }
        "vector_last" => {
            if args.len() != 1 {
                return Err(CompileError::new_simple("vector_last expects 1 argument"));
            }
            let vec_ty = check_expr(env, &args[0])?;
            match vec_ty {
                Type::Vector(inner) => Ok(Some(*inner)),
                _ => Err(CompileError::new_simple("vector_last expects vector")),
            }
        }
        "vector_pop" => {
            if args.len() != 1 {
                return Err(CompileError::new_simple("vector_pop expects 1 argument"));
            }
            let vec_ty = check_expr(env, &args[0])?;
            match vec_ty {
                Type::Vector(inner) => Ok(Some(Type::Vector(inner))),
                _ => Err(CompileError::new_simple("vector_pop expects vector")),
            }
        }
        "vector_clear" => {
            if args.len() != 1 {
                return Err(CompileError::new_simple("vector_clear expects 1 argument"));
            }
            let vec_ty = check_expr(env, &args[0])?;
            match vec_ty {
                Type::Vector(inner) => Ok(Some(Type::Vector(inner))),
                _ => Err(CompileError::new_simple("vector_clear expects vector")),
            }
        }
        _ => Ok(None),
    }
}
