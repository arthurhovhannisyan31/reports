use report_parser::errors::ParsingError;
use report_parser::parsers::csv::CsvRecord;
use report_parser::record::BankRecordSerDe;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), ParsingError> {
  let f = File::open("./mocks/records_example.csv")?;
  let mut reader = BufReader::new(f);
  let mut count = 0;

  let mut header = String::new();
  reader.read_line(&mut header)?;

  while let Ok(record) = CsvRecord::from_read(&mut reader) {
    count += 1;
  }
  println!("{:#?}", count);

  Ok(())
}
