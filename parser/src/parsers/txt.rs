//! Текстовый (TXT) парсер банковских транзакций.
//!
//! Формат TXT предполагает **одну транзакцию на строку**:
//!
//! ```text
//! <id> <account> <amount> <currency>
//! ```
//!
//! Поля разделяются пробелами:
//! - `id` — `u64`
//! - `account` — строка без пробелов
//! - `amount` — `i64`
//! - `currency` — строка без пробелов (например `USD`)

use std::fmt::format;
use std::io::{BufRead, Read, Write};
use crate::{Format, models::Transaction, errors::ParserError};

/// TXT-парсер транзакций.
///
/// Использует построчное чтение и пробелы в качестве разделителей.
/// Предназначен для простого человеко-читаемого формата без кавычек
/// и экранирования.
pub struct TxtParser;

impl Format for TxtParser {
    /// Считывает транзакции из текстового потока.
    ///
    /// Каждая строка должна содержать ровно 4 поля,
    /// разделённых пробелами.
    ///
    /// # Ошибки
    ///
    /// Возвращает:
    /// - [`ParserError::Io`] — при ошибке чтения потока;
    /// - [`ParserError::Invalid`] — если строка имеет неверный формат
    ///   или поле не удалось распарсить.
    fn read<R: Read>(reader: R) -> Result<Vec<Transaction>, ParserError> {
        let reader = std::io::BufReader::new(reader);
        let mut res = Vec::new();

        for (i, line) in reader.lines().enumerate() {
            let line = line.map_err(|err| ParserError::Io(err))?;
            let mut split = line.split_whitespace();

            let id = split
                .next()
                .ok_or_else(|| ParserError::Invalid(format!("Invalid id, line {}: {}", i+1, line)))?
                .parse()
                .map_err(|_| ParserError::Invalid(format!("Invalid id, line {}: {}", i+1, line)))?;

            let account = split
                .next()
                .ok_or_else(|| ParserError::Invalid(format!("Invalid account, line {}: {}", i + 1, line)))?
                .to_string();

            let amount = split
                .next()
                .ok_or_else(|| ParserError::Invalid(format!("Invalid amount, line {}: {}", i+1, line)))?
                .parse()
                .map_err(|_| ParserError::Invalid(format!("Invalid amount, line {}: {}", i+1, line)))?;

            let currency = split
                .next()
                .ok_or_else(|| ParserError::Invalid(format!("Invalid currency, line {}: {}", i+1, line)))?
                .to_string();

            if split.next().is_some() {
                return Err(ParserError::Invalid(format!(
                    "Invalid line {}: {}",
                    i + 1,
                    line
                )));
            }

            res.push(Transaction{
                id,
                account,
                amount,
                currency,
            });
        }

        Ok(res)
    }

    /// Записывает транзакции в текстовый поток.
    ///
    /// Каждая транзакция сериализуется в одну строку:
    ///
    /// ```text
    /// id account amount currency
    /// ```
    ///
    /// # Ошибки
    ///
    /// Возвращает [`ParserError::Invalid`], если произошла ошибка записи.
    fn write<W: Write>(mut writer: W, transaction: &[Transaction]) -> Result<(), ParserError> {
        for tx in transaction {
            writeln!(
                writer,
                "{} {} {} {}",
                tx.id, tx.account, tx.amount, tx.currency,
            ).map_err(|_| ParserError::Invalid(format!(
                "Invalid transaction {}", tx.id
            )))?;
        }

        Ok(())
    }
}