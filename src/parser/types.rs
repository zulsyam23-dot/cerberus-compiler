use crate::ast::Type;
use crate::error::CompileError;
use crate::lexer::TokenKind;

use super::Parser;

pub fn parse_type(p: &mut Parser<'_>) -> Result<Type, CompileError> {
    match p.current.kind.clone() {
        TokenKind::Array => {
            p.advance()?;
            p.expect_kind(TokenKind::LBracket, "'['")?;
            let len = match p.current.kind.clone() {
                TokenKind::IntLit(v) if v >= 0 => {
                    p.advance()?;
                    v as usize
                }
                _ => return Err(p.unexpected("array size integer")),
            };
            p.expect_kind(TokenKind::RBracket, "']'")?;
            p.expect_keyword(TokenKind::Of, "of")?;
            let elem = parse_type(p)?;
            Ok(Type::Array {
                elem: Box::new(elem),
                len,
            })
        }
        TokenKind::Integer => {
            p.advance()?;
            Ok(Type::Integer)
        }
        TokenKind::Boolean => {
            p.advance()?;
            Ok(Type::Boolean)
        }
        TokenKind::String => {
            p.advance()?;
            Ok(Type::String)
        }
        TokenKind::Option => {
            p.advance()?;
            let inner = parse_type(p)?;
            match inner {
                Type::Integer | Type::Boolean | Type::String => {
                    Ok(Type::Option(Box::new(inner)))
                }
                _ => Err(CompileError::new_simple(
                    "option only supports integer, boolean, or string",
                )),
            }
        }
        TokenKind::Result => {
            p.advance()?;
            let inner = parse_type(p)?;
            match inner {
                Type::Integer | Type::Boolean | Type::String => {
                    Ok(Type::Result(Box::new(inner)))
                }
                _ => Err(CompileError::new_simple(
                    "result only supports integer, boolean, or string",
                )),
            }
        }
        TokenKind::Vector | TokenKind::List => {
            p.advance()?;
            let inner = if p.current_is(TokenKind::Of) {
                p.advance()?;
                parse_type(p)?
            } else {
                Type::Integer
            };
            match inner {
                Type::Integer | Type::Boolean | Type::String => {
                    Ok(Type::Vector(Box::new(inner)))
                }
                _ => Err(CompileError::new_simple(
                    "vector only supports integer, boolean, or string",
                )),
            }
        }
        TokenKind::Stack => {
            p.advance()?;
            Ok(Type::StackInt)
        }
        TokenKind::Map => {
            p.advance()?;
            Ok(Type::MapStrStr)
        }
        TokenKind::Set => {
            p.advance()?;
            Ok(Type::SetStr)
        }
        _ => Err(p.unexpected("type name")),
    }
}
