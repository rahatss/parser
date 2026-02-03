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

pub trait Format {
    fn read<R: Read>(reader: R) -> Result<Vec<Transaction>, ParserError>;
    fn write<W: Write>(writer: W, transactions: &[Transaction]) -> Result<(), ParserError>;
}
