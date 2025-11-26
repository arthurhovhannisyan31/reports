use report_parser::BankRecordParser;
use report_parser::errors::ParsingError;
use report_parser::parsers::txt::TxtReportParser;
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), ParsingError> {
  let f = File::open("./report_files/records_example.txt")?;
  let mut reader = BufReader::new(f);

  let records = TxtReportParser::from_read(&mut reader)?;

  println!("{records:#?}");
  println!("{}", records.len());

  Ok(())
}
