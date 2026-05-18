use std::error::Error;
use std::fmt;

/// Aliases the result shape used by the corresponding Model I/O wrappers.
pub type Result<T> = std::result::Result<T, ModelIoError>;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Wraps the corresponding Model I/O model io error counterpart.
pub struct ModelIoError {
    code: i32,
    message: String,
}

impl ModelIoError {
    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O model io error counterpart.
    pub(crate) fn new(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O model io error counterpart.
    pub fn code(&self) -> i32 {
        self.code
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O model io error counterpart.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ModelIoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (status {})", self.message, self.code)
    }
}

impl Error for ModelIoError {}
