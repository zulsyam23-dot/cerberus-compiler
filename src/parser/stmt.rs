use crate::ast::{Assign, AssignIndex, CallStmt, IfStmt, Stmt, WhileStmt};
use crate::error::CompileError;
use crate::lexer::TokenKind;

use super::block::parse_compound_statement;
use super::expr::parse_expr;
use super::Parser;

pub(crate) fn parse_statement(p: &mut Parser<'_>) -> Result<Stmt, CompileError> {
    match &p.current.kind {
        TokenKind::Ident(_) => {
            if p.next_is(TokenKind::Assign) {
                parse_assignment(p).map(Stmt::Assign)
            } else if p.next_is(TokenKind::LParen) {
                parse_call_stmt(p).map(Stmt::Call)
            } else if p.next_is(TokenKind::LBracket) {
                parse_index_assignment(p).map(Stmt::AssignIndex)
            } else {
                Err(p.unexpected("assignment or call"))
            }
        }
        TokenKind::Writeln => parse_writeln(p).map(Stmt::Writeln),
        TokenKind::Readln => parse_readln(p).map(Stmt::Readln),
        TokenKind::Return => parse_return(p).map(Stmt::Return),
        TokenKind::If => parse_if(p).map(Stmt::If),
        TokenKind::While => parse_while(p).map(Stmt::While),
        TokenKind::Begin => {
            let stmts = parse_compound_statement(p)?;
            Ok(Stmt::Compound(stmts))
        }
        _ => Err(p.unexpected("statement")),
    }
}

fn parse_assignment(p: &mut Parser<'_>) -> Result<Assign, CompileError> {
    let name = p.expect_ident()?;
    p.expect_kind(TokenKind::Assign, "':='")?;
    let expr = parse_expr(p)?;
    Ok(Assign { name, expr })
}

fn parse_writeln(p: &mut Parser<'_>) -> Result<crate::ast::Expr, CompileError> {
    p.expect_keyword(TokenKind::Writeln, "writeln")?;
    p.expect_kind(TokenKind::LParen, "'('")?;
    let expr = parse_expr(p)?;
    p.expect_kind(TokenKind::RParen, "')'")?;
    Ok(expr)
}

fn parse_readln(p: &mut Parser<'_>) -> Result<Vec<String>, CompileError> {
    p.expect_keyword(TokenKind::Readln, "readln")?;
    p.expect_kind(TokenKind::LParen, "'('")?;
    let mut names = Vec::new();
    names.push(p.expect_ident()?);
    while p.current_is(TokenKind::Comma) {
        p.advance()?;
        names.push(p.expect_ident()?);
    }
    p.expect_kind(TokenKind::RParen, "')'")?;
    Ok(names)
}

fn parse_index_assignment(p: &mut Parser<'_>) -> Result<AssignIndex, CompileError> {
    let name = p.expect_ident()?;
    p.expect_kind(TokenKind::LBracket, "'['")?;
    let index = parse_expr(p)?;
    p.expect_kind(TokenKind::RBracket, "']'")?;
    p.expect_kind(TokenKind::Assign, "':='")?;
    let expr = parse_expr(p)?;
    Ok(AssignIndex { name, index, expr })
}

fn parse_call_stmt(p: &mut Parser<'_>) -> Result<CallStmt, CompileError> {
    let name = p.expect_ident()?;
    let args = parse_call_args(p)?;
    Ok(CallStmt { name, args })
}

pub(crate) fn parse_call_args(p: &mut Parser<'_>) -> Result<Vec<crate::ast::Expr>, CompileError> {
    p.expect_kind(TokenKind::LParen, "'('")?;
    let mut args = Vec::new();
    if !p.current_is(TokenKind::RParen) {
        args.push(parse_expr(p)?);
        while p.current_is(TokenKind::Comma) {
            p.advance()?;
            args.push(parse_expr(p)?);
        }
    }
    p.expect_kind(TokenKind::RParen, "')'")?;
    Ok(args)
}

fn parse_return(p: &mut Parser<'_>) -> Result<Option<crate::ast::Expr>, CompileError> {
    p.expect_keyword(TokenKind::Return, "return")?;
    if p.current_is(TokenKind::Semi) || p.current_is(TokenKind::End) {
        Ok(None)
    } else {
        Ok(Some(parse_expr(p)?))
    }
}

fn parse_if(p: &mut Parser<'_>) -> Result<IfStmt, CompileError> {
    p.expect_keyword(TokenKind::If, "if")?;
    let cond = parse_expr(p)?;
    p.expect_keyword(TokenKind::Then, "then")?;
    let then_branch = parse_statement(p)?;
    let else_branch = if p.current_is(TokenKind::Else) {
        p.advance()?;
        Some(Box::new(parse_statement(p)?))
    } else {
        None
    };
    Ok(IfStmt {
        cond,
        then_branch: Box::new(then_branch),
        else_branch,
    })
}

fn parse_while(p: &mut Parser<'_>) -> Result<WhileStmt, CompileError> {
    p.expect_keyword(TokenKind::While, "while")?;
    let cond = parse_expr(p)?;
    p.expect_keyword(TokenKind::Do, "do")?;
    let body = parse_statement(p)?;
    Ok(WhileStmt {
        cond,
        body: Box::new(body),
    })
}
