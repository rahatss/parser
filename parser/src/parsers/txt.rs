use std::io::{BufRead, Read, Write};
use crate::{Format, models::Transaction, errors::ParserError};

pub struct TxtParser;

impl Format for TxtParser {
    fn read<R: Read>(reader: R) -> Result<Vec<Transaction>, ParserError> {
        let reader = std::io::BufReader::new(reader);
        let mut res = Vec::new();

        for (i, line) in reader.lines().enumerate() {
            let line = line.map_err(|err| ParserError::Io(err))?;
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() != 4 {
                return Err(ParserError::Invalid(format!(
                    "Invalid line {}: {}",
                    i + 1,
                    line
                )));
            }

            res.push(Transaction{
                id: parts[0].parse().map_err(|_| ParserError::Invalid(line.clone()))?,
                account: parts[1].to_string(),
                amount: parts[2].parse().map_err(|_| ParserError::Invalid(line.clone()))?,
                currency: parts[3].to_string(),
            });
        }

        Ok(res)
    }

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