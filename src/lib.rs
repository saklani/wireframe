//! Draw UI wireframes as screen mockups in the terminal, from an indented
//! outline of elements.
//!
//! ```
//! let wf = wireframe::parse("wireframe Pay width=24\n  button: Go").unwrap();
//! assert!(wireframe::render_fit(&wf, 0).text.contains("Go"));
//! ```

use std::fmt;

mod paint;
mod text;
mod wireframe;

pub use text::Fit;
pub use wireframe::{parse, render, render_fit, Align, El, Screen, Wireframe, DEFAULT_WIDTH, MIN_WIDTH};

/// A problem in wireframe source, with its 1-based line number when known.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Error {
    pub line: Option<usize>,
    /// 1-based character column on that line, when known.
    pub column: Option<usize>,
    pub message: String,
}

impl Error {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Error { line: None, column: None, message: message.into() }
    }

    pub(crate) fn at(line: usize, message: impl Into<String>) -> Self {
        Error { line: Some(line), ..Error::new(message) }
    }

    pub(crate) fn at_column(line: usize, column: usize, message: impl Into<String>) -> Self {
        Error { column: Some(column), ..Error::at(line, message) }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.line, self.column) {
            (Some(line), Some(column)) => write!(f, "line {line}, column {column}: {}", self.message),
            (Some(line), None) => write!(f, "line {line}: {}", self.message),
            _ => f.write_str(&self.message),
        }
    }
}

impl std::error::Error for Error {}
