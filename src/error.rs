use std::error::Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, ModelIoError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelIoError {
    code: i32,
    message: String,
}

impl ModelIoError {
    pub(crate) fn new(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    #[must_use]
    pub fn code(&self) -> i32 {
        self.code
    }

    #[must_use]
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
