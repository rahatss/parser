use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub enum ParserError {
    Io(std::io::Error),
    Csv(csv::Error),
    Invalid(String),
    InvalidCsv,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::Io(err) => write!(f, "IO error: {}", err.to_string()),
            ParserError::Csv(err) => write!(f, "CSV error: {}", err.to_string()),
            ParserError::Invalid(reason) => write!(f, "Invalid: {}", reason),
            ParserError::InvalidCsv => write!(f, "Invalid csv"),
        }
    }
}

impl std::error::Error for ParserError {}