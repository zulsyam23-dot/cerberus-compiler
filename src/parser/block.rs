use crate::ast::{Block, Decl, FuncDecl, Param, ProcDecl, Stmt, VarDecl};
use crate::error::CompileError;
use crate::lexer::TokenKind;

use super::stmt::parse_statement;
use super::types::parse_type;
use super::Parser;

pub fn parse_block(p: &mut Parser<'_>) -> Result<Block, CompileError> {
    let mut declarations = Vec::new();
    if p.current_is(TokenKind::Var) {
        p.advance()?;
        while p.current_is_ident() {
            let names = parse_ident_list(p)?;
            p.expect_kind(TokenKind::Colon, "':'")?;
            let ty = parse_type(p)?;
            p.expect_kind(TokenKind::Semi, "';'")?;
            for name in names {
                declarations.push(Decl::Var(VarDecl { name, ty: ty.clone() }));
            }
        }
    }

    while p.current_is(TokenKind::Procedure) || p.current_is(TokenKind::Function) {
        if p.current_is(TokenKind::Procedure) {
            let decl = parse_proc_decl(p)?;
            declarations.push(Decl::Proc(decl));
        } else {
            let decl = parse_func_decl(p)?;
            declarations.push(Decl::Func(decl));
        }
    }
    let statements = parse_compound_statement(p)?;
    Ok(Block {
        declarations,
        statements,
    })
}

fn parse_ident_list(p: &mut Parser<'_>) -> Result<Vec<String>, CompileError> {
    let mut names = Vec::new();
    names.push(p.expect_ident()?);
    while p.current_is(TokenKind::Comma) {
        p.advance()?;
        names.push(p.expect_ident()?);
    }
    Ok(names)
}

pub(crate) fn parse_compound_statement(p: &mut Parser<'_>) -> Result<Vec<Stmt>, CompileError> {
    p.expect_keyword(TokenKind::Begin, "begin")?;
    let mut statements = Vec::new();
    while !p.current_is(TokenKind::End) {
        if p.current_is(TokenKind::Semi) {
            p.advance()?;
            statements.push(Stmt::Empty);
            continue;
        }
        let stmt = parse_statement(p)?;
        statements.push(stmt);
        if p.current_is(TokenKind::Semi) {
            p.advance()?;
        } else if !p.current_is(TokenKind::End) {
            return Err(p.unexpected("';' or 'end'"));
        }
    }
    p.expect_keyword(TokenKind::End, "end")?;
    Ok(statements)
}

fn parse_proc_decl(p: &mut Parser<'_>) -> Result<ProcDecl, CompileError> {
    p.expect_keyword(TokenKind::Procedure, "procedure")?;
    let name = p.expect_ident()?;
    let params = parse_param_list(p)?;
    p.expect_kind(TokenKind::Semi, "';'")?;
    let block = parse_block(p)?;
    p.expect_kind(TokenKind::Semi, "';'")?;
    Ok(ProcDecl { name, params, block })
}

fn parse_func_decl(p: &mut Parser<'_>) -> Result<FuncDecl, CompileError> {
    p.expect_keyword(TokenKind::Function, "function")?;
    let name = p.expect_ident()?;
    let params = parse_param_list(p)?;
    p.expect_kind(TokenKind::Colon, "':'")?;
    let return_type = parse_type(p)?;
    p.expect_kind(TokenKind::Semi, "';'")?;
    let block = parse_block(p)?;
    p.expect_kind(TokenKind::Semi, "';'")?;
    Ok(FuncDecl {
        name,
        params,
        return_type,
        block,
    })
}

fn parse_param_list(p: &mut Parser<'_>) -> Result<Vec<Param>, CompileError> {
    if !p.current_is(TokenKind::LParen) {
        return Ok(Vec::new());
    }
    p.expect_kind(TokenKind::LParen, "'('")?;
    let mut params = Vec::new();
    if !p.current_is(TokenKind::RParen) {
        loop {
            let names = parse_ident_list(p)?;
            p.expect_kind(TokenKind::Colon, "':'")?;
            let ty = parse_type(p)?;
            for name in names {
                params.push(Param { name, ty: ty.clone() });
            }
            if p.current_is(TokenKind::Semi) {
                p.advance()?;
                continue;
            }
            break;
        }
    }
    p.expect_kind(TokenKind::RParen, "')'")?;
    Ok(params)
}
