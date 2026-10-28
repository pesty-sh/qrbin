use std::fmt;

#[derive(Debug)]
pub enum QrError {
    EmptyPayload,
    Io(std::io::Error),
    Encoding(String),
    Output(String),
}

impl fmt::Display for QrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPayload => write!(f, "payload is empty"),
            Self::Io(error) => write!(f, "I/O error: {error}"),
            Self::Encoding(message) => write!(f, "QR encoding error: {message}"),
            Self::Output(message) => write!(f, "Output error: {message}"),
        }
    }
}

impl std::error::Error for QrError {}

impl From<std::io::Error> for QrError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
