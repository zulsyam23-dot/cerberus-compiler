use crate::ast::{BinaryOp, Expr, UnaryOp};
use crate::error::CompileError;
use crate::lexer::TokenKind;

use super::Parser;
use super::stmt::parse_call_args;

pub fn parse_expr(p: &mut Parser<'_>) -> Result<Expr, CompileError> {
    parse_or(p)
}

fn parse_or(p: &mut Parser<'_>) -> Result<Expr, CompileError> {
    let mut left = parse_and(p)?;
    while p.current_is(TokenKind::Or) {
        p.advance()?;
        let right = parse_and(p)?;
        left = Expr::Binary {
            op: BinaryOp::Or,
            left: Box::new(left),
            right: Box::new(right),
        };
    }
    Ok(left)
}

fn parse_and(p: &mut Parser<'_>) -> Result<Expr, CompileError> {
    let mut left = parse_compare(p)?;
    while p.current_is(TokenKind::And) {
        p.advance()?;
        let right = parse_compare(p)?;
        left = Expr::Binary {
            op: BinaryOp::And,
            left: Box::new(left),
            right: Box::new(right),
        };
    }
    Ok(left)
}

fn parse_compare(p: &mut Parser<'_>) -> Result<Expr, CompileError> {
    let mut left = parse_add(p)?;
    loop {
        let op = match p.current.kind {
            TokenKind::Eq => Some(BinaryOp::Eq),
            TokenKind::Ne => Some(BinaryOp::Ne),
            TokenKind::Lt => Some(BinaryOp::Lt),
            TokenKind::Le => Some(BinaryOp::Le),
            TokenKind::Gt => Some(BinaryOp::Gt),
            TokenKind::Ge => Some(BinaryOp::Ge),
            _ => None,
        };
        if let Some(op) = op {
            p.advance()?;
            let right = parse_add(p)?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        } else {
            break;
        }
    }
    Ok(left)
}

fn parse_add(p: &mut Parser<'_>) -> Result<Expr, CompileError> {
    let mut left = parse_mul(p)?;
    loop {
        let op = match p.current.kind {
            TokenKind::Plus => Some(BinaryOp::Add),
            TokenKind::Minus => Some(BinaryOp::Sub),
            _ => None,
        };
        if let Some(op) = op {
            p.advance()?;
            let right = parse_mul(p)?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        } else {
            break;
        }
    }
    Ok(left)
}

fn parse_mul(p: &mut Parser<'_>) -> Result<Expr, CompileError> {
    let mut left = parse_unary(p)?;
    loop {
        let op = match p.current.kind {
            TokenKind::Star => Some(BinaryOp::Mul),
            TokenKind::Slash => Some(BinaryOp::Div),
            _ => None,
        };
        if let Some(op) = op {
            p.advance()?;
            let right = parse_unary(p)?;
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        } else {
            break;
        }
    }
    Ok(left)
}

fn parse_unary(p: &mut Parser<'_>) -> Result<Expr, CompileError> {
    match p.current.kind {
        TokenKind::Minus => {
            p.advance()?;
            let expr = parse_unary(p)?;
            Ok(Expr::Unary {
                op: UnaryOp::Neg,
                expr: Box::new(expr),
            })
        }
        TokenKind::Not => {
            p.advance()?;
            let expr = parse_unary(p)?;
            Ok(Expr::Unary {
                op: UnaryOp::Not,
                expr: Box::new(expr),
            })
        }
        _ => parse_primary(p),
    }
}

fn parse_primary(p: &mut Parser<'_>) -> Result<Expr, CompileError> {
    match p.current.kind.clone() {
        TokenKind::IntLit(v) => {
            p.advance()?;
            Ok(Expr::Int(v))
        }
        TokenKind::True => {
            p.advance()?;
            Ok(Expr::Bool(true))
        }
        TokenKind::False => {
            p.advance()?;
            Ok(Expr::Bool(false))
        }
        TokenKind::StringLit(s) => {
            p.advance()?;
            Ok(Expr::Str(s))
        }
        TokenKind::Ident(name) => {
            p.advance()?;
            if p.current_is(TokenKind::LParen) {
                let args = parse_call_args(p)?;
                Ok(Expr::Call(crate::ast::CallExpr { name, args }))
            } else if p.current_is(TokenKind::LBracket) {
                p.advance()?;
                let index = parse_expr(p)?;
                p.expect_kind(TokenKind::RBracket, "']'")?;
                Ok(Expr::Index {
                    name,
                    index: Box::new(index),
                })
            } else {
                Ok(Expr::Var(name))
            }
        }
        TokenKind::LParen => {
            p.advance()?;
            let expr = parse_expr(p)?;
            p.expect_kind(TokenKind::RParen, "')'")?;
            Ok(expr)
        }
        _ => Err(p.unexpected("expression")),
    }
}
