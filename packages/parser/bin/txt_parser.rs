use parser::errors::ParsingError;
use parser::parsers::TxtRecord;
use parser::record::BankRecordParser;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

fn main() -> Result<(), ParsingError> {
  let f = File::open("./mocks/records_example.txt")?;
  let mut reader = BufReader::new(f);
  let mut write_buf = BufWriter::new(File::create("./temp/records.txt")?);

  while let Ok(record) = TxtRecord::from_read(&mut reader) {
    let _ = TxtRecord(record).write_to(&mut write_buf);
  }

  write_buf.flush()?;

  Ok(())
}
