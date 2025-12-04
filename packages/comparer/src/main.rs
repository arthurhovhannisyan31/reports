use clap::Parser;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::ops::DerefMut;

use parser::errors::ParsingError;
use parser::parsers::{BinRecord, CsvRecord, TxtRecord};
use parser::record::{BankRecord, BankRecordParser};

mod configs;
mod errors;
use crate::configs::{CliArgs, DataFormat};
use crate::errors::ComparerError;

/*
For the purposes if this implementation I'd like to assume following things:
- Hashed records are unique since they have timestamp field (at least no
duplicates were found in data examples)
- Reports may be subset of each other, so line by line comparison is not an option
- Reports may have some intersection in records, and unique records of their own
*/

fn main() -> Result<(), ComparerError> {
  let cli = CliArgs::parse();
  let mut records1_set: HashSet<BankRecord> = HashSet::new();
  let mut records2_set: HashSet<BankRecord> = HashSet::new();

  let CliArgs {
    file1,
    format1,
    file2,
    format2,
  } = cli;

  let mut file1_reader = BufReader::new(File::open(&file1)?);
  let mut file2_reader = BufReader::new(File::open(&file2)?);

  if format1 == DataFormat::Csv {
    // Skip headers line
    file1_reader.read_line(&mut String::new())?;
  }
  while let Ok(record) = read_record_from_source(&mut file1_reader, &format1) {
    records1_set.insert(record);
  }

  if format2 == DataFormat::Csv {
    // Skip headers line
    file2_reader.read_line(&mut String::new())?;
  }
  while let Ok(record) = read_record_from_source(&mut file2_reader, &format2) {
    records2_set.insert(record);
  }

  let stdout = io::stdout().lock();
  let mut buf_writer = BufWriter::new(stdout);

  let file1_diff_count = records1_set.difference(&records2_set).count();
  let file2_diff_count = records2_set.difference(&records1_set).count();

  if file1_diff_count == 0 && file2_diff_count == 0 {
    writeln!(
      buf_writer,
      "The transaction records in {:?} and {:?} are identical.\nGreat job, now you can go home!",
      &file1.to_str().unwrap_or("Source file 1"),
      &file2.to_str().unwrap_or("Source file 2"),
    )?;
  } else {
    writeln!(
      buf_writer,
      "The following transactions didn't match between files:",
    )?;
    writeln!(buf_writer)?;

    for record in records1_set.difference(&records2_set) {
      writeln!(
        buf_writer,
        "File: {:?}\nRecord: {:#?} ",
        &file1.to_str().unwrap_or("File 1"),
        record,
      )
      .expect("Failed writing to stdout");
      writeln!(buf_writer)?;
    }
    for record in records2_set.difference(&records1_set) {
      writeln!(
        buf_writer,
        "File: {:?}\nRecord: {:#?} ",
        &file2.to_str().unwrap_or("File 2"),
        record,
      )
      .expect("Failed writing to stdout");
      writeln!(buf_writer)?;
    }

    writeln!(
      buf_writer,
      "Please revise your files and don't upset your manager",
    )?;
  }

  buf_writer.flush()?;

  Ok(())
}

fn read_record_from_source(
  mut buffer: &mut impl BufRead,
  input_format: &DataFormat,
) -> Result<BankRecord, ParsingError> {
  match input_format {
    DataFormat::Bin => BinRecord::from_read(buffer.deref_mut()),
    DataFormat::Csv => CsvRecord::from_read(buffer.deref_mut()),
    DataFormat::Txt => TxtRecord::from_read(buffer.deref_mut()),
  }
}
