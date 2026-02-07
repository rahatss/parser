use parser::{Format, ParserError, binary, csv, txt};
use std::{env, fs::File, io::stdout};

fn main() {
    let mut args = env::args().skip(1);

    let mut input_file: Option<String> = None;
    let mut input_format: Option<String> = None;
    let mut output_format: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input" => input_file = args.next(),
            "--input-format" => input_format = args.next(),
            "--output-format" => output_format = args.next(),
            _ => usage(),
        }
    }

    let input_file = input_file.expect("Missing --input");
    let input_format = input_format.expect("Missing --input-format");
    let output_format = output_format.expect("Missing --output-format");

    let input = File::open(input_file.clone())
        .map_err(|err| {
            ParserError::Invalid(format!(
                "Failed to open input file '{}' (provided via --input): {}",
                input_file, err
            ))
        })
        .unwrap();

    let transactions = match input_format.as_str() {
        "bin" => binary::BinParser::read(input),
        "csv" => csv::Csv::read(input),
        "txt" => txt::TxtParser::read(input),
        _ => panic!("unsupported format"),
    }
    .unwrap();

    let out = stdout();

    match output_format.as_str() {
        "csv" => csv::Csv::write(out, &transactions),
        "txt" => txt::TxtParser::write(out, &transactions),
        "bin" => binary::BinParser::write(out, &transactions),
        _ => panic!("unknown output format"),
    }
    .unwrap();
}

fn usage() {
    panic!(
        "Usage:
         converter --input <file> --input-format <csv|txt|bin> --output-format <csv|txt|bin>"
    );
}
