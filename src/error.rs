#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub line: usize,
    pub col: usize,
    pub len: usize,
}

#[derive(Debug, Clone)]
pub struct CompileError {
    pub message: String,
    pub span: Option<Span>,
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: {}", self.message)
    }
}

impl std::error::Error for CompileError {}

impl CompileError {
    pub fn new(message: impl Into<String>, span: Span) -> Self {
        Self {
            message: message.into(),
            span: Some(span),
        }
    }

    pub fn new_simple(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            span: None,
        }
    }

    pub fn render(&self, source: &str) -> String {
        match self.span {
            None => format!("error: {}", self.message),
            Some(span) => {
                let line_text = source
                    .lines()
                    .nth(span.line.saturating_sub(1))
                    .unwrap_or("");
                let caret_len = span.len.max(1);
                let mut caret = String::new();
                for _ in 0..(span.col.saturating_sub(1)) {
                    caret.push(' ');
                }
                for _ in 0..caret_len {
                    caret.push('^');
                }
                format!(
                    "error: {}\n --> {}:{}\n  |\n{} | {}\n  | {}",
                    self.message, span.line, span.col, span.line, line_text, caret
                )
            }
        }
    }
}
