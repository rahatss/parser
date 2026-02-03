use parser::{Format, ParserError, Transaction, binary, csv, txt};
use std::fs::File;
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);

    let mut file1: Option<String> = None;
    let mut format1: Option<String> = None;
    let mut file2: Option<String> = None;
    let mut format2: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--file1" => file1 = args.next(),
            "--format1" => format1 = args.next(),
            "--file2" => file2 = args.next(),
            "--format2" => format2 = args.next(),
            _ => {
                return Err(format!("Unknown argument: {}", arg).into());
            }
        }
    }

    let file1 = file1.ok_or("Missing --file1")?;
    let format1 = format1.ok_or("Missing --format1")?;
    let file2 = file2.ok_or("Missing --file2")?;
    let format2 = format2.ok_or("Missing --format2")?;

    let a = read(file1, format1.as_str())?;
    let b = read(file2, format2.as_str())?;

    for (tx1, tx2) in a.iter().zip(b) {
        if *tx1 != tx2 {
            println!("Mismatch tx, [{}] vs [{}]", *tx1, tx2);
            break;
        }
    }

    println!("Files are same!");
    Ok(())
}

fn read(path: String, format: &str) -> Result<Vec<Transaction>, ParserError> {
    let file = File::open(path).map_err(|err| ParserError::Io(err))?;

    match format {
        "csv" => Ok(csv::Csv::read(file)?),
        "txt" => Ok(binary::BinParser::read(file)?),
        "bin" => Ok(txt::TxtParser::read(file)?),
        _ => Err(ParserError::Invalid(format!("Unknown format: {}", format))),
    }
}
