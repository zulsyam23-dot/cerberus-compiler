use crate::ast::{Assign, AssignIndex, CallStmt, Stmt};
use crate::error::CompileError;

use super::env::TypeEnv;
use super::expr::check_expr;
use super::types::expect_printable;

pub fn check_stmt(env: &mut TypeEnv, stmt: &Stmt) -> Result<(), CompileError> {
    match stmt {
        Stmt::Assign(a) => check_assign(env, a),
        Stmt::AssignIndex(a) => check_assign_index(env, a),
        Stmt::Writeln(expr) => {
            let ty = check_expr(env, expr)?;
            expect_printable(&ty, "writeln")?;
            Ok(())
        }
        Stmt::Readln(names) => {
            for name in names {
                let ty = env
                    .vars
                    .get(name)
                    .ok_or_else(|| CompileError::new_simple(format!("unknown variable '{}'", name)))?
                    .clone();
                expect_printable(&ty, "readln")?;
            }
            Ok(())
        }
        Stmt::Call(CallStmt { name, args }) => {
            let sig = env.procs.get(name).cloned();
            if sig.is_none() {
                return Err(CompileError::new_simple(format!(
                    "unknown procedure '{}'",
                    name
                )));
            }
            let sig = sig.unwrap();
            if sig.params.len() != args.len() {
                return Err(CompileError::new_simple(
                    "procedure argument count mismatch",
                ));
            }
            for (i, arg) in args.iter().enumerate() {
                let ty = check_expr(env, arg)?;
                if ty != sig.params[i] {
                    return Err(CompileError::new_simple(
                        "procedure argument type mismatch",
                    ));
                }
            }
            Ok(())
        }
        Stmt::Return(expr) => {
            let expected = env.current_return.clone();
            match (expected, expr) {
                (None, None) => Ok(()),
                (None, Some(_)) => Err(CompileError::new_simple(
                    "return with value is not allowed in procedure",
                )),
                (Some(_), None) => Err(CompileError::new_simple(
                    "return without value is not allowed in function",
                )),
                (Some(expected), Some(e)) => {
                    let ty = check_expr(env, e)?;
                    if ty != expected {
                        return Err(CompileError::new_simple(
                            "return type does not match function type",
                        ));
                    }
                    Ok(())
                }
            }
        }
        Stmt::If(s) => {
            let cond = check_expr(env, &s.cond)?;
            super::types::expect_bool(&cond, "if condition")?;
            check_stmt(env, &s.then_branch)?;
            if let Some(else_branch) = &s.else_branch {
                check_stmt(env, else_branch)?;
            }
            Ok(())
        }
        Stmt::While(s) => {
            let cond = check_expr(env, &s.cond)?;
            super::types::expect_bool(&cond, "while condition")?;
            check_stmt(env, &s.body)?;
            Ok(())
        }
        Stmt::Compound(stmts) => {
            for s in stmts {
                check_stmt(env, s)?;
            }
            Ok(())
        }
        Stmt::Empty => Ok(()),
    }
}

fn check_assign(env: &mut TypeEnv, a: &Assign) -> Result<(), CompileError> {
    let var_ty = env
        .vars
        .get(&a.name)
        .ok_or_else(|| CompileError::new_simple(format!("unknown variable '{}'", a.name)))?
        .clone();
    let expr_ty = check_expr(env, &a.expr)?;
    if var_ty != expr_ty {
        return Err(CompileError::new_simple(format!(
            "type mismatch: '{}' is {:?} but assigned {:?}",
            a.name, var_ty, expr_ty
        )));
    }
    Ok(())
}

fn check_assign_index(env: &mut TypeEnv, a: &AssignIndex) -> Result<(), CompileError> {
    let var_ty = env
        .vars
        .get(&a.name)
        .ok_or_else(|| CompileError::new_simple(format!("unknown variable '{}'", a.name)))?
        .clone();
    let idx_ty = check_expr(env, &a.index)?;
    super::types::expect_int(&idx_ty, "array index")?;
    let expr_ty = check_expr(env, &a.expr)?;
    match var_ty {
        crate::ast::Type::Array { elem, .. } => {
            if *elem != expr_ty {
                return Err(CompileError::new_simple("type mismatch in array assignment"));
            }
            Ok(())
        }
        _ => Err(CompileError::new_simple("not an array")),
    }
}
