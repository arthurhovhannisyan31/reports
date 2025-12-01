// ypbank_converter \
//   --input <input_file> \
//   --input-format <format> \
//   --output-format <format> \
//   > output_file.txt

// read from stdin | file
// input validation

use crate::configs::{CliArgs, DataFormat};
use crate::errors::ConverterErrors;
use clap::Parser;
use parser::parsers::csv::CVS_HEADERS;
use parser::parsers::{BinRecord, CsvRecord, TxtRecord};
use parser::record::{BankRecord, BankRecordParser};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};

mod configs;
mod errors;

fn main() -> Result<(), ConverterErrors> {
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

  let parsed_records: Vec<BankRecord> = match input_format {
    DataFormat::Bin => {
      let mut vec = vec![];
      while let Ok(record) = BinRecord::from_read(&mut buf_reader) {
        vec.push(record);
      }
      vec
    }
    DataFormat::Csv => {
      let mut vec = vec![];
      // skip reading first line
      buf_reader.read_line(&mut String::new())?;
      while let Ok(record) = CsvRecord::from_read(&mut buf_reader) {
        vec.push(record);
      }
      vec
    }
    DataFormat::Txt => {
      let mut vec = vec![];
      while let Ok(record) = TxtRecord::from_read(&mut buf_reader) {
        vec.push(record);
      }
      vec
    }
  };

  match output_format {
    DataFormat::Bin => {
      for record in parsed_records {
        let _ = BinRecord(record).write_to(&mut buf_writer);
      }
    }
    DataFormat::Csv => {
      writeln!(&mut buf_writer, "{}", CVS_HEADERS)?;
      for record in parsed_records {
        let _ = CsvRecord(record).write_to(&mut buf_writer);
      }
    }
    DataFormat::Txt => {
      for record in parsed_records {
        let _ = TxtRecord(record).write_to(&mut buf_writer);
      }
    }
  }

  buf_writer.flush()?;

  Ok(())
}
