use report_parser::BankRecordParser;
use report_parser::errors::ParsingError;
use report_parser::parsers::csv::CsvReportParser;
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), ParsingError> {
  let f = File::open("./report_files/records_example.csv")?;
  let mut reader = BufReader::new(f);

  let records = CsvReportParser::from_read(&mut reader)?;

  println!("{records:#?}");
  println!("{}", records.len());

  Ok(())
}
