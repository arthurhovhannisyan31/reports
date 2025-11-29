use report_parser::errors::ParsingError;
use report_parser::parsers::txt::TxtRecord;
use report_parser::record::BankRecordSerDe;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

fn main() -> Result<(), ParsingError> {
  let f = File::open("./mocks/records_example.txt")?;
  let mut reader = BufReader::new(f);

  let mut write_buf = BufWriter::new(File::create("foo.txt")?);

  while let Ok(record) = TxtRecord::from_read(&mut reader) {
    let _ = TxtRecord(record).write_to(&mut write_buf);
  }

  write_buf.flush()?;

  Ok(())
}
