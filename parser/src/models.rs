//! Типы данных для представления финансовых транзакций.
//!
//! Модуль содержит структуру [`Transaction`], используемую всеми форматами
//! парсеров (CSV, TXT, BIN) для сериализации и десериализации данных.
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};
use crate::ParserError;

/// Банковская транзакция.
///
/// Представляет одну запись о транзакции.
#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    /// Уникальный идентификатор транзакции.
    pub id: u64,

    /// Идентификатор или номер счёта.
    pub account: String,

    /// Сумма транзакции.
    ///
    /// Может быть отрицательной (например, списание средств).
    pub amount: i64,

    /// Код валюты (например, `USD`, `EUR`).
    pub currency: String,
}

impl Transaction {
    /// Создаёт транзакцию из CSV-записи.
    ///
    /// Ожидаемый формат CSV:
    /// ```text
    /// id,account,amount,currency
    /// ```
    ///
    /// # Ошибки
    ///
    /// Возвращает [`ParserError::InvalidCsv`], если:
    /// - отсутствует одно из обязательных полей;
    /// - не удалось преобразовать поле в ожидаемый тип.
    pub fn from_csv_record(record: &csv::StringRecord) -> Result<Self, ParserError> {
        Ok(Self {
            id: record.get(0)
                .ok_or(ParserError::InvalidCsv)?
                .parse()
                .map_err(|_| ParserError::InvalidCsv)?,
            account: record.get(1).ok_or(ParserError::InvalidCsv)?.to_string(),
            amount: record.get(2)
                .ok_or(ParserError::InvalidCsv)?
                .parse()
                .map_err(|_| ParserError::InvalidCsv)?,
            currency: record.get(3).ok_or(ParserError::InvalidCsv)?.to_string(),
        })
    }
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
            self.account == other.account &&
            self.currency == other.currency &&
            self.amount == other.amount
    }
}

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: [{}], account: [{}], currency: [{}], amount: [{}]",
               self.id, self.account, self.currency, self.amount)
    }
}