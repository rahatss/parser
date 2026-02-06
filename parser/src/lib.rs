mod errors;
mod models;
mod parsers;
mod tests;

use std::io::{Read, Write};
pub use crate::errors::ParserError;
pub use crate::models::Transaction;

pub use parsers::{
    binary, csv, txt
};

/// Общий интерфейс для чтения/записи транзакций в файл.
///
/// Реализуется для CSV, TXT и BIN форматов.
pub trait Format {
    /// Читает транзакции из входного потока.
    ///
    /// # Ошибки
    ///
    /// Возвращает ошибку, если данные повреждены
    /// или имеют неверный формат.
    fn read<R: Read>(reader: R) -> Result<Vec<Transaction>, ParserError>;

    /// Записывает транзакции в выходной поток.
    ///
    /// # Ошибки
    ///
    /// Возвращает ошибку записи.
    fn write<W: Write>(writer: W, transactions: &[Transaction]) -> Result<(), ParserError>;
}
