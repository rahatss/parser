//! Ошибки, возникающие при парсинге и сериализации данных.
//!
//! Модуль содержит тип [`ParserError`], который используется во всех парсерах
//! (CSV, TXT, BIN) для унифицированной обработки ошибок ввода-вывода,
//! формата и валидации данных.
use std::fmt::{Debug, Formatter};


/// Унифицированная ошибка парсера.
///
/// Используется всеми реализациями [`Format`] для возврата ошибок
/// при чтении и записи данных.
#[derive(Debug)]
pub enum ParserError {
    /// Ошибка ввода-вывода (чтение или запись файлов).
    Io(std::io::Error),

    /// Ошибка, возникшая при работе с CSV.
    Csv(csv::Error),

    /// Ошибка валидации входных данных с поясняющим сообщением.
    Invalid(String),

    /// Некорректный CSV без детализированной причины.
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