use std::io::{Read, Write};
use crate::errors::ParserError;
use crate::{Format};
use crate::models::Transaction;

pub struct Csv;

impl Format for Csv {
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

    fn write<W: Write>(writer: W, transaction: &[Transaction]) -> Result<(), ParserError> {
        let mut writer = csv::Writer::from_writer(writer);
        for tx in transaction {
            writer.serialize(tx).map_err(|err| ParserError::Csv(err))?;
        }

        Ok(())
    }
}