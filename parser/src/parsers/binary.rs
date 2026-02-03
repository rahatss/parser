use std::io::{Read, Write};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crate::{Format, models::Transaction, errors::ParserError};

pub struct BinParser;

impl Format for BinParser {
    fn read<R: Read>(mut reader: R) -> Result<Vec<Transaction>, ParserError> {
        let count = reader.read_u32::<LittleEndian>().map_err(|err| ParserError::Io(err))?;
        let mut transactions = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let id = reader.read_u64::<LittleEndian>().map_err(|err| ParserError::Io(err))?;
            let amount = reader.read_i64::<LittleEndian>().map_err(|err| ParserError::Io(err))?;
            let account = read_string(&mut reader)?;
            let currency = read_string(&mut reader)?;

            transactions.push(Transaction{
                id,
                account,
                amount,
                currency,
            })
        }

        Ok(transactions)
    }

    fn write<W: Write>(mut writer: W, transactions: &[Transaction]) -> Result<(), ParserError> {
        writer.write_u32::<LittleEndian>(transactions.len() as u32).map_err(|err| ParserError::Io(err))?;

        for tx in transactions {
            writer.write_u64::<LittleEndian>(tx.id).map_err(|err| ParserError::Io(err))?;
            writer.write_i64::<LittleEndian>(tx.amount).map_err(|err| ParserError::Io(err))?;
            write_string(&mut writer, &tx.account)?;
            write_string(&mut writer, &tx.currency)?
        }

        Ok(())
    }
}

fn read_string<R: Read>(r: &mut R) -> Result<String, ParserError> {
    let n = r.read_u32::<LittleEndian>().map_err(|err| ParserError::Io(err))?;
    let mut buf = vec![0u8; n as usize];

    r.read_exact(&mut buf).map_err(|err| ParserError::Io(err))?;
    Ok(String::from_utf8_lossy(&buf).to_string())
}

fn write_string<W: Write>(w: &mut W, s: &str) -> Result<(), ParserError> {
    w.write_u32::<LittleEndian>(s.len() as u32).map_err(|err| ParserError::Io(err))?;
    w.write_all(s.as_bytes()).map_err(|err| ParserError::Io(err))?;

    Ok(())
}