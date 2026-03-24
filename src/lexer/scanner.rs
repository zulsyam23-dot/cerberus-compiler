mod core;
mod text;
mod trivia;

pub struct Lexer<'a> {
    pub(super) src: &'a str,
    pub(super) bytes: &'a [u8],
    pub(super) idx: usize,
    pub(super) line: usize,
    pub(super) col: usize,
}
