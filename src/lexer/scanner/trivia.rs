use crate::error::{CompileError, Span};

use super::Lexer;

impl<'a> Lexer<'a> {
    pub(super) fn skip_whitespace_and_comments(&mut self) -> Result<(), CompileError> {
        loop {
            self.skip_whitespace();
            if self.peek_char() == Some('{') {
                self.advance(); // '{'
                let mut closed = false;
                while let Some(c) = self.peek_char() {
                    self.advance();
                    if c == '}' {
                        closed = true;
                        break;
                    }
                }
                if !closed {
                    return Err(CompileError::new(
                        "unterminated comment",
                        Span {
                            line: self.line,
                            col: self.col,
                            len: 1,
                        },
                    ));
                }
                continue;
            }
            break;
        }
        Ok(())
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if c == ' ' || c == '\t' || c == '\r' || c == '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }
}
