use crate::configs::{CliArgs, DataFormat};
use crate::errors::ConverterErrors;

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::ops::DerefMut;

use clap::Parser;

use parser::errors::{ParsingError, SerializeError};
use parser::parsers::{BinRecord, CsvRecord, TxtRecord};
use parser::record::{BankRecord, BankRecordParser};

mod configs;
mod errors;

fn main() -> Result<(), ConverterErrors> {
  let cli = CliArgs::parse();

  let CliArgs {
    input,
    input_format,
    output_format,
  } = cli;

  let file = File::open(input)?;
  let mut buf_reader = BufReader::new(file);

  let mut parsed_records = vec![];
  while let Ok(record) = read_record_from_source(&input_format, &mut buf_reader)
  {
    parsed_records.push(record);
  }

  let stdout = io::stdout().lock();
  let mut buf_writer = BufWriter::new(stdout);

  for record in parsed_records {
    write_record_to_source(&output_format, record, &mut buf_writer)?;
  }

  buf_writer.flush()?;

  Ok(())
}

fn read_record_from_source(
  input_format: &DataFormat,
  mut buffer: &mut impl BufRead,
) -> Result<BankRecord, ParsingError> {
  let mut reader = buffer.deref_mut();

  match input_format {
    DataFormat::Bin => BinRecord::from_read(&mut reader),
    DataFormat::Csv => CsvRecord::from_read(&mut reader),
    DataFormat::Txt => TxtRecord::from_read(&mut reader),
  }
}

fn write_record_to_source(
  input_format: &DataFormat,
  record: BankRecord,
  mut buffer: &mut impl Write,
) -> Result<(), SerializeError> {
  let mut writer = buffer.deref_mut();

  match input_format {
    DataFormat::Bin => BinRecord(record).write_to(&mut writer),
    DataFormat::Csv => CsvRecord(record).write_to(&mut writer),
    DataFormat::Txt => TxtRecord(record).write_to(&mut writer),
  }
}

#[cfg(test)]
mod converter_test {

  // Invalid input file format
  // Missing input file
  // From all to all formats with assertion

  //cargo run -p converter -- -i ./mocks/records_example.bin --input-format bin --output-format txt > ./temp/temp.txt
}
