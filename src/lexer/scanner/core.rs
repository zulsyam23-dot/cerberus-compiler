use crate::error::{CompileError, Span};

use super::super::keywords::keyword_kind;
use super::super::token::{Token, TokenKind};
use super::text::is_ident_start;
use super::Lexer;

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            bytes: src.as_bytes(),
            idx: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, CompileError> {
        self.skip_whitespace_and_comments()?;
        let start_idx = self.idx;
        let start_line = self.line;
        let start_col = self.col;

        if self.idx >= self.bytes.len() {
            return Ok(Token {
                kind: TokenKind::Eof,
                span: Span {
                    line: start_line,
                    col: start_col,
                    len: 0,
                },
            });
        }

        let b = self.bytes[self.idx];
        if b >= 128 {
            return Err(CompileError::new(
                "non-ASCII character is not allowed",
                Span {
                    line: start_line,
                    col: start_col,
                    len: 1,
                },
            ));
        }

        let ch = b as char;
        match ch {
            ';' => {
                self.advance();
                Ok(Token {
                    kind: TokenKind::Semi,
                    span: Span {
                        line: start_line,
                        col: start_col,
                        len: 1,
                    },
                })
            }
            ':' => {
                self.advance();
                if self.peek_char() == Some('=') {
                    self.advance();
                    Ok(Token {
                        kind: TokenKind::Assign,
                        span: Span {
                            line: start_line,
                            col: start_col,
                            len: 2,
                        },
                    })
                } else {
                    Ok(Token {
                        kind: TokenKind::Colon,
                        span: Span {
                            line: start_line,
                            col: start_col,
                            len: 1,
                        },
                    })
                }
            }
            ',' => {
                self.advance();
                Ok(Token {
                    kind: TokenKind::Comma,
                    span: Span {
                        line: start_line,
                        col: start_col,
                        len: 1,
                    },
                })
            }
            '.' => {
                self.advance();
                Ok(Token {
                    kind: TokenKind::Dot,
                    span: Span {
                        line: start_line,
                        col: start_col,
                        len: 1,
                    },
                })
            }
            '+' => {
                self.advance();
                Ok(Token {
                    kind: TokenKind::Plus,
                    span: Span {
                        line: start_line,
                        col: start_col,
                        len: 1,
                    },
                })
            }
            '-' => {
                self.advance();
                Ok(Token {
                    kind: TokenKind::Minus,
                    span: Span {
                        line: start_line,
                        col: start_col,
                        len: 1,
                    },
                })
            }
            '*' => {
                self.advance();
                Ok(Token {
                    kind: TokenKind::Star,
                    span: Span {
                        line: start_line,
                        col: start_col,
                        len: 1,
                    },
                })
            }
            '/' => {
                self.advance();
                Ok(Token {
                    kind: TokenKind::Slash,
                    span: Span {
                        line: start_line,
                        col: start_col,
                        len: 1,
                    },
                })
            }
            '(' => {
                self.advance();
                Ok(Token {
                    kind: TokenKind::LParen,
                    span: Span {
                        line: start_line,
                        col: start_col,
                        len: 1,
                    },
                })
            }
            ')' => {
                self.advance();
                Ok(Token {
                    kind: TokenKind::RParen,
                    span: Span {
                        line: start_line,
                        col: start_col,
                        len: 1,
                    },
                })
            }
            '[' => {
                self.advance();
                Ok(Token {
                    kind: TokenKind::LBracket,
                    span: Span {
                        line: start_line,
                        col: start_col,
                        len: 1,
                    },
                })
            }
            ']' => {
                self.advance();
                Ok(Token {
                    kind: TokenKind::RBracket,
                    span: Span {
                        line: start_line,
                        col: start_col,
                        len: 1,
                    },
                })
            }
            '=' => {
                self.advance();
                Ok(Token {
                    kind: TokenKind::Eq,
                    span: Span {
                        line: start_line,
                        col: start_col,
                        len: 1,
                    },
                })
            }
            '<' => {
                self.advance();
                if self.peek_char() == Some('=') {
                    self.advance();
                    Ok(Token {
                        kind: TokenKind::Le,
                        span: Span {
                            line: start_line,
                            col: start_col,
                            len: 2,
                        },
                    })
                } else if self.peek_char() == Some('>') {
                    self.advance();
                    Ok(Token {
                        kind: TokenKind::Ne,
                        span: Span {
                            line: start_line,
                            col: start_col,
                            len: 2,
                        },
                    })
                } else {
                    Ok(Token {
                        kind: TokenKind::Lt,
                        span: Span {
                            line: start_line,
                            col: start_col,
                            len: 1,
                        },
                    })
                }
            }
            '>' => {
                self.advance();
                if self.peek_char() == Some('=') {
                    self.advance();
                    Ok(Token {
                        kind: TokenKind::Ge,
                        span: Span {
                            line: start_line,
                            col: start_col,
                            len: 2,
                        },
                    })
                } else {
                    Ok(Token {
                        kind: TokenKind::Gt,
                        span: Span {
                            line: start_line,
                            col: start_col,
                            len: 1,
                        },
                    })
                }
            }
            c if c.is_ascii_digit() => {
                let lit = self.read_int_lit(start_idx, start_line, start_col)?;
                Ok(Token {
                    kind: TokenKind::IntLit(lit),
                    span: Span {
                        line: start_line,
                        col: start_col,
                        len: self.idx - start_idx,
                    },
                })
            }
            c if is_ident_start(c) => {
                let ident = self.read_ident();
                let lower = ident.to_ascii_lowercase();
                let kind = keyword_kind(&lower);
                Ok(Token {
                    kind,
                    span: Span {
                        line: start_line,
                        col: start_col,
                        len: self.idx - start_idx,
                    },
                })
            }
            '\'' => {
                self.advance();
                let s = self.read_string_lit(start_line, start_col)?;
                Ok(Token {
                    kind: TokenKind::StringLit(s),
                    span: Span {
                        line: start_line,
                        col: start_col,
                        len: self.idx - start_idx,
                    },
                })
            }
            _ => Err(CompileError::new(
                format!("unexpected character '{}'", ch),
                Span {
                    line: start_line,
                    col: start_col,
                    len: 1,
                },
            )),
        }
    }
}
