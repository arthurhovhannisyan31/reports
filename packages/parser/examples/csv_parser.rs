use parser::errors::ParsingError;
use parser::parsers::csv::{CsvRecord, CVS_HEADERS};
use parser::record::BankRecordParser;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() -> Result<(), ParsingError> {
  let f = File::open("./mocks/records_example.csv")?;
  let mut reader = BufReader::new(f);
  let mut header = String::new();
  let mut write_buf = BufWriter::new(File::create("./temp/records.csv")?);

  reader.read_line(&mut header)?;
  writeln!(&mut write_buf, "{CVS_HEADERS}")?;

  while let Ok(record) = CsvRecord::from_read(&mut reader) {
    let _ = CsvRecord(record).write_to(&mut write_buf);
  }

  write_buf.flush()?;

  Ok(())
}
