// ypbank_converter \
//   --input <input_file> \
//   --input-format <format> \
//   --output-format <format> \
//   > output_file.txt

// read from stdin | file
// input validation

use crate::configs::CliArgs;
use clap::Parser;
use parser::record::BankRecordParser;
use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter, Write};

mod configs;

fn main() -> io::Result<()> {
  let cli = CliArgs::parse();
  let stdout = io::stdout().lock();

  let CliArgs {
    input,
    input_format,
    output_format,
  } = cli;

  let file = File::open(input)?;
  let mut buf_reader = BufReader::new(file);
  let mut buf_writer = BufWriter::new(stdout);

  // let deserializer =  match input_format {
  //   DataFormat::Binary => BankRecord<CsvRecord2>,
  //   DataFormat::Csv => {
  //     while let Ok(record) = CsvRecord::from_read(&mut buf_reader) {
  //       let _ = CsvRecord(record).write_to(&mut buf_writer);
  //     }
  //   }
  //   DataFormat::Text => {
  //     while let Ok(record) = TxtRecord::from_read(&mut buf_reader) {
  //       let _ = TxtRecord(record).write_to(&mut buf_writer);
  //     }
  //   }
  // };

  buf_writer.flush()?;

  Ok(())
}
