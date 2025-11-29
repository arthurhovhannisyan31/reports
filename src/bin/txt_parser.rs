use report_parser::errors::ParsingError;
use report_parser::parsers::txt::TxtRecord;
use report_parser::record::BankRecordSerDe;
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), ParsingError> {
  let f = File::open("./mocks/records_example.txt")?;
  let mut reader = BufReader::new(f);
  let mut count = 0;

  while let Ok(record) = TxtRecord::from_read(&mut reader) {
    count += 1;
  }

  println!("{:#?}", count);

  Ok(())
}
