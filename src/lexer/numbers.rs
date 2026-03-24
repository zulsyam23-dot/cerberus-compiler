use crate::error::{CompileError, Span};

pub fn parse_int_lit(
    src: &str,
    start: usize,
    end: usize,
    line: usize,
    col: usize,
) -> Result<i64, CompileError> {
    let s = &src[start..end];
    s.parse::<i64>().map_err(|_| {
        CompileError::new(
            "integer literal is out of range",
            Span {
                line,
                col,
                len: end - start,
            },
        )
    })
}
