use std::fmt::Formatter;
use serde::{Deserialize, Serialize};
use crate::ParserError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: u64,
    pub account: String,
    pub amount: i64,
    pub currency: String,
}

impl Transaction {
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