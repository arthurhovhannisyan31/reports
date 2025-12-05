use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

use parser::errors::ParsingError;
use parser::parsers::BinRecord;
use parser::record::BankRecordParser;

fn main() -> Result<(), ParsingError> {
  let f = File::open("./mocks/records_example.bin")?;
  let mut mock_reader = BufReader::new(f);
  let mut write_buf = BufWriter::new(File::create("./temp/records.bin")?);

  while let Ok(record) = BinRecord::from_read(&mut mock_reader) {
    println!("{:#?}", record);

    let _ = BinRecord(record).write_to(&mut write_buf);
  }

  write_buf.flush()?;

  Ok(())
}
