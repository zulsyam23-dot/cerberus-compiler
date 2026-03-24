use crate::ast::{Expr, Type};
use crate::bytecode::Instr;

use super::super::Codegen;

pub fn emit_eq(cg: &mut Codegen, left: &Expr, right: &Expr) {
    if is_string_expr(cg, left) && is_string_expr(cg, right) {
        cg.code.push(Instr::StrEq);
    } else {
        cg.code.push(Instr::Eq);
    }
}

pub fn emit_ne(cg: &mut Codegen, left: &Expr, right: &Expr) {
    if is_string_expr(cg, left) && is_string_expr(cg, right) {
        cg.code.push(Instr::StrNe);
    } else {
        cg.code.push(Instr::Ne);
    }
}

fn is_string_expr(cg: &Codegen, expr: &Expr) -> bool {
    match expr {
        Expr::Str(_) => true,
        Expr::Var(name) => matches!(cg.symbols.get_type(name), Some(Type::String)),
        _ => false,
    }
}
