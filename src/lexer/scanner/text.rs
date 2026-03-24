use crate::error::{CompileError, Span};

use super::super::numbers::parse_int_lit;
use super::Lexer;

impl<'a> Lexer<'a> {
    pub(super) fn read_ident(&mut self) -> String {
        let start = self.idx;
        while let Some(c) = self.peek_char() {
            if is_ident_continue(c) {
                self.advance();
            } else {
                break;
            }
        }
        self.src[start..self.idx].to_string()
    }

    pub(super) fn read_int_lit(
        &mut self,
        start_idx: usize,
        start_line: usize,
        start_col: usize,
    ) -> Result<i64, CompileError> {
        while let Some(c) = self.peek_char() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }
        parse_int_lit(self.src, start_idx, self.idx, start_line, start_col)
    }

    pub(super) fn read_string_lit(
        &mut self,
        start_line: usize,
        start_col: usize,
    ) -> Result<String, CompileError> {
        let mut out = String::new();
        loop {
            match self.peek_char() {
                Some('\'') => {
                    self.advance();
                    if self.peek_char() == Some('\'') {
                        // Escaped single quote
                        self.advance();
                        out.push('\'');
                        continue;
                    }
                    break;
                }
                Some(c) => {
                    if c == '\n' {
                        return Err(CompileError::new(
                            "unterminated string literal",
                            Span {
                                line: start_line,
                                col: start_col,
                                len: 1,
                            },
                        ));
                    }
                    self.advance();
                    out.push(c);
                }
                None => {
                    return Err(CompileError::new(
                        "unterminated string literal",
                        Span {
                            line: start_line,
                            col: start_col,
                            len: 1,
                        },
                    ));
                }
            }
        }
        Ok(out)
    }

    pub(super) fn peek_char(&self) -> Option<char> {
        self.bytes.get(self.idx).map(|b| *b as char)
    }

    pub(super) fn advance(&mut self) -> Option<char> {
        if self.idx >= self.bytes.len() {
            return None;
        }
        let c = self.bytes[self.idx] as char;
        self.idx += 1;
        if c == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(c)
    }
}

pub(super) fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

pub(super) fn is_ident_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}
