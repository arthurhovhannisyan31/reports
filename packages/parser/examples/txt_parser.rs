use parser::errors::ParsingError;
use parser::parsers::{CsvRecord, TxtRecord, csv::CVS_HEADERS};
use parser::record::BankRecordParser;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() -> Result<(), ParsingError> {
  let f = File::open("./temp/records_example.csv")?;
  let mut reader = BufReader::new(f);
  let mut write_buf = BufWriter::new(File::create("./temp/records.csv")?);

  writeln!(&mut write_buf, "{CVS_HEADERS}")?;
  while let Ok(record) = TxtRecord::from_read(&mut reader) {
    let _ = CsvRecord(record).write_to(&mut write_buf);
  }

  write_buf.flush()?;

  let mut result_reader = BufReader::new(File::open("./temp/records.csv")?);
  result_reader.read_line(&mut String::new())?;
  while let Ok(record) = CsvRecord::from_read(&mut result_reader) {
    println!("{:#?}", record);
  }

  Ok(())
}
