//! CSV-парсер банковских транзакций.
//!
//! Формат CSV ожидается в следующем виде (без заголовков):
//!
//! ```text
//! id,account,amount,currency
//! ```
//!
//! Где:
//! - `id` — целое беззнаковое число (`u64`)
//! - `account` — строка
//! - `amount` — целое число (`i64`)
//! - `currency` — строка (например, `USD`)

use std::io::{Read, Write};
use crate::errors::ParserError;
use crate::{Format};
use crate::models::Transaction;


/// CSV-парсер транзакций.
///
/// Реализует трейт [`Format`] и позволяет:
/// - читать транзакции из CSV-потока;
/// - записывать транзакции в CSV-поток.
///
/// Используется для человеко-читаемого хранения и обмена данными.
pub struct Csv;

impl Format for Csv {
    /// Считывает транзакции из CSV-потока.
    ///
    /// Каждая строка CSV должна содержать ровно 4 поля:
    /// `id, account, amount, currency`.
    ///
    /// # Ошибки
    ///
    /// Возвращает:
    /// - [`ParserError::Csv`] — если CSV повреждён или имеет неверный формат;
    /// - [`ParserError::InvalidCsv`] — если запись не соответствует модели [`Transaction`].
    fn read<R: Read>(reader: R) -> Result<Vec<Transaction>, ParserError> {
        let mut rdr  = csv::Reader::from_reader(reader);
        let mut transactions = Vec::new();

        for record in rdr.records() {
            let record = record.map_err(|err| ParserError::Csv(err))?;
            let tx = Transaction::from_csv_record(&record)?;
            transactions.push(tx);
        }

        Ok(transactions)
    }

    /// Записывает транзакции в CSV-поток.
    ///
    /// Каждая транзакция сериализуется в отдельную строку
    /// без заголовков.
    ///
    /// # Ошибки
    ///
    /// Возвращает [`ParserError::Csv`], если произошла ошибка записи.
    fn write<W: Write>(writer: W, transaction: &[Transaction]) -> Result<(), ParserError> {
        let mut writer = csv::Writer::from_writer(writer);
        for tx in transaction {
            writer.serialize(tx).map_err(|err| ParserError::Csv(err))?;
        }

        Ok(())
    }
}