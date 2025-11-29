use report_parser::errors::ParsingError;
use report_parser::parsers::bin::BinRecord;
use report_parser::record::BankRecordSerDe;
use std::fs::File;
use std::io::BufReader;

pub const RECORD_HEADER: &[u8; 4] = b"YPBN";

fn main() -> Result<(), ParsingError> {
  let f = File::open("./mocks/records_example.bin")?;
  let mut reader = BufReader::new(f);
  let mut count = 0;

  while let Ok(record) = BinRecord::from_read(&mut reader) {
    count += 1;
  }

  println!("{count}");

  Ok(())
}
