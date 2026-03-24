mod block;
mod expr;
mod stmt;
mod types;

use crate::ast::Program;
use crate::error::CompileError;
use crate::lexer::{Lexer, Token, TokenKind};

use block::parse_block;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token,
    next: Token,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Result<Self, CompileError> {
        let mut lexer = Lexer::new(src);
        let current = lexer.next_token()?;
        let next = lexer.next_token()?;
        Ok(Self { lexer, current, next })
    }

    pub fn parse_program(&mut self) -> Result<Program, CompileError> {
        self.expect_keyword(TokenKind::Program, "program")?;
        let name = self.expect_ident()?;
        self.expect_kind(TokenKind::Semi, "';'")?;
        let uses = if self.current_is(TokenKind::Uses) {
            self.advance()?;
            let list = self.parse_ident_list()?;
            self.expect_kind(TokenKind::Semi, "';'")?;
            list
        } else {
            Vec::new()
        };
        let block = parse_block(self)?;
        self.expect_kind(TokenKind::Dot, "'.'")?;
        self.expect_kind(TokenKind::Eof, "end of file")?;
        Ok(Program { name, uses, block })
    }

    fn advance(&mut self) -> Result<(), CompileError> {
        self.current = std::mem::replace(&mut self.next, self.lexer.next_token()?);
        Ok(())
    }

    fn expect_keyword(&mut self, kind: TokenKind, expected: &str) -> Result<(), CompileError> {
        if std::mem::discriminant(&self.current.kind) == std::mem::discriminant(&kind) {
            self.advance()?;
            Ok(())
        } else {
            Err(self.unexpected(expected))
        }
    }

    fn expect_kind(&mut self, kind: TokenKind, expected: &str) -> Result<(), CompileError> {
        if std::mem::discriminant(&self.current.kind) == std::mem::discriminant(&kind) {
            self.advance()?;
            Ok(())
        } else {
            Err(self.unexpected(expected))
        }
    }

    fn expect_ident(&mut self) -> Result<String, CompileError> {
        match &self.current.kind {
            TokenKind::Ident(name) => {
                let name = name.clone();
                self.advance()?;
                Ok(name)
            }
            _ => Err(self.unexpected("identifier")),
        }
    }

    fn parse_ident_list(&mut self) -> Result<Vec<String>, CompileError> {
        let mut names = Vec::new();
        names.push(self.expect_ident()?);
        while self.current_is(TokenKind::Comma) {
            self.advance()?;
            names.push(self.expect_ident()?);
        }
        Ok(names)
    }

    fn unexpected(&self, expected: &str) -> CompileError {
        CompileError::new(
            format!("expected {expected}"),
            self.current.span,
        )
    }

    fn current_is(&self, kind: TokenKind) -> bool {
        std::mem::discriminant(&self.current.kind) == std::mem::discriminant(&kind)
    }

    fn current_is_ident(&self) -> bool {
        matches!(self.current.kind, TokenKind::Ident(_))
    }

    fn next_is(&self, kind: TokenKind) -> bool {
        std::mem::discriminant(&self.next.kind) == std::mem::discriminant(&kind)
    }
}
