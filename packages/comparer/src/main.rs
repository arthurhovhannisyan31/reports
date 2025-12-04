use clap::Parser;
use std::collections::HashSet;
use std::ffi::OsStr;
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

  let CliArgs {
    file1,
    format1,
    file2,
    format2,
  } = cli;

  let stdout = io::stdout().lock();
  let mut buf_writer = BufWriter::new(stdout);
  let file_1_name = file1
    .file_name()
    .unwrap_or(OsStr::new("File 1"))
    .to_str()
    .unwrap();
  let file_2_name = file2
    .file_name()
    .unwrap_or(OsStr::new("File 2"))
    .to_str()
    .unwrap();
  let mut file1_reader = BufReader::new(File::open(&file1)?);
  let mut file2_reader = BufReader::new(File::open(&file2)?);

  compare(
    &mut file1_reader,
    &mut file2_reader,
    &mut buf_writer,
    format1,
    format2,
    file_1_name,
    file_2_name,
  )?;

  Ok(())
}

fn compare(
  reader1: &mut impl BufRead,
  reader2: &mut impl BufRead,
  buf_writer: &mut impl Write,
  format1: DataFormat,
  format2: DataFormat,
  file_1_name: &str,
  file_2_name: &str,
) -> Result<(), ComparerError> {
  let mut records1_set: HashSet<BankRecord> = HashSet::new();
  let mut records2_set: HashSet<BankRecord> = HashSet::new();

  if format1 == DataFormat::Csv {
    // Skip headers line
    reader1.read_line(&mut String::new())?;
  }
  while let Ok(record) = read_record_from_source(reader1, &format1) {
    records1_set.insert(record);
  }

  if format2 == DataFormat::Csv {
    // Skip headers line
    reader2.read_line(&mut String::new())?;
  }
  while let Ok(record) = read_record_from_source(reader2, &format2) {
    records2_set.insert(record);
  }

  let file1_diff_count = records1_set.difference(&records2_set).count();
  let file2_diff_count = records2_set.difference(&records1_set).count();

  if file1_diff_count == 0 && file2_diff_count == 0 {
    writeln!(
      buf_writer,
      "The transaction records in {:?} and {:?} are identical.\nGreat job, now you can go home!",
      file_1_name, file_2_name,
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
        "File: {:?}\nRecord id: {} ",
        file_1_name, record.tx_id,
      )
      .expect("Failed writing to stdout");
      writeln!(buf_writer)?;
    }
    for record in records2_set.difference(&records1_set) {
      writeln!(
        buf_writer,
        "File: {:?}\nRecord id: {} ",
        file_2_name, record.tx_id,
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

#[cfg(test)]
mod test_comparer {
  use crate::compare;
  use crate::configs::DataFormat;
  use crate::errors::ComparerError;
  use std::ffi::OsStr;
  use std::fs::File;
  use std::io::BufReader;
  use std::path::Path;

  #[test]
  fn test_matrix() -> Result<(), ComparerError> {
    let file_configs = [
      (Path::new("./tests/stub_files/records.csv"), DataFormat::Csv),
      (Path::new("./tests/stub_files/records.bin"), DataFormat::Bin),
      (Path::new("./tests/stub_files/records.txt"), DataFormat::Txt),
    ];

    // Run matrix of tests for bin, csv and test files
    for (file_path_1, data_format_1) in &file_configs {
      for (file_path_2, data_format_2) in &file_configs {
        let file_1 = File::open(file_path_1)?;
        let file_2 = File::open(file_path_2)?;
        let file_1_name = file_path_1
          .file_name()
          .unwrap_or(OsStr::new("File 1"))
          .to_str()
          .unwrap();
        let file_2_name = file_path_2
          .file_name()
          .unwrap_or(OsStr::new("File 2"))
          .to_str()
          .unwrap();
        let mut file_1_reader = BufReader::new(file_1);
        let mut file_2_reader = BufReader::new(file_2);
        let mut output_buffer: Vec<u8> = vec![];

        let result = compare(
          &mut file_1_reader,
          &mut file_2_reader,
          &mut output_buffer,
          data_format_1.clone(),
          data_format_2.clone(),
          file_1_name,
          file_2_name,
        );

        let assert_output = format!(
          "The transaction records in {:?} and {:?} are identical.\nGreat job, now you can go home!\n",
          file_1_name, file_2_name
        );

        assert!(result.is_ok());
        assert_eq!(output_buffer, assert_output.as_bytes());
      }
    }

    Ok(())
  }

  #[test]
  fn test_reports_diffs() -> Result<(), ComparerError> {
    let file_configs_1 = [
      (Path::new("./tests/stub_files/records.bin"), DataFormat::Bin),
      (Path::new("./tests/stub_files/records.csv"), DataFormat::Csv),
      (Path::new("./tests/stub_files/records.txt"), DataFormat::Txt),
    ];
    let file_configs_2 = [
      (
        Path::new("./tests/stub_files/records_short.bin"),
        DataFormat::Bin,
      ),
      (
        Path::new("./tests/stub_files/records_short.csv"),
        DataFormat::Csv,
      ),
      (
        Path::new("./tests/stub_files/records_short.txt"),
        DataFormat::Txt,
      ),
    ];

    for (file_path_1, data_format_1) in &file_configs_1 {
      for (file_path_2, data_format_2) in &file_configs_2 {
        let file_1 = File::open(file_path_1)?;
        let file_2 = File::open(file_path_2)?;
        let file_1_name = file_path_1
          .file_name()
          .unwrap_or(OsStr::new("File 1"))
          .to_str()
          .unwrap();
        let file_2_name = file_path_2
          .file_name()
          .unwrap_or(OsStr::new("File 2"))
          .to_str()
          .unwrap();
        let mut file_1_reader = BufReader::new(file_1);
        let mut file_2_reader = BufReader::new(file_2);
        let mut output_buffer: Vec<u8> = vec![];

        let result = compare(
          &mut file_1_reader,
          &mut file_2_reader,
          &mut output_buffer,
          data_format_1.clone(),
          data_format_2.clone(),
          file_1_name,
          file_2_name,
        );

        let mut assert_output = String::from(
          "The following transactions didn't match between files:\n\n",
        );
        // With multiple missing records order of reporting was quite not stable for order of records, so I left only 1 record missing
        assert_output.push_str(&format!(
          "File: {:?}\nRecord id: {} \n\n",
          file_1_name, 1000000000000003u64
        ));
        assert_output
          .push_str("Please revise your files and don't upset your manager\n");

        assert!(result.is_ok());
        assert_eq!(output_buffer, assert_output.as_bytes());
      }
    }

    Ok(())
  }
}
