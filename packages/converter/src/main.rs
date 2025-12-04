use clap::Parser;
use parser::errors::{ParsingError, SerializeError};
use parser::parsers::{BinRecord, CsvRecord, TxtRecord, csv::CVS_HEADERS};
use parser::record::{BankRecord, BankRecordParser};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::ops::DerefMut;

mod configs;
mod errors;

use crate::configs::{CliArgs, DataFormat};
use crate::errors::ConverterErrors;

fn main() -> Result<(), ConverterErrors> {
  let cli = CliArgs::parse();

  let CliArgs {
    input,
    input_format,
    output_format,
  } = cli;

  let mut file_reader = BufReader::new(File::open(input)?);

  let stdout = io::stdout().lock();
  let mut buf_writer = BufWriter::new(stdout);

  convert(
    &mut file_reader,
    &mut buf_writer,
    input_format,
    output_format,
  )?;

  Ok(())
}

fn convert(
  reader: &mut impl BufRead,
  writer: &mut impl Write,
  input_format: DataFormat,
  output_format: DataFormat,
) -> Result<(), ConverterErrors> {
  if input_format == DataFormat::Csv {
    // Skip headers line
    reader.read_line(&mut String::new())?;
  }

  let mut parsed_records = vec![];
  while let Ok(record) = read_record_from_source(reader, &input_format) {
    parsed_records.push(record);
  }

  if output_format == DataFormat::Csv {
    // Write headers line
    writeln!(writer, "{}", CVS_HEADERS)?;
  }

  for record in parsed_records {
    write_record_to_source(writer, record, &output_format)?;
  }

  writer.flush()?;

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

fn write_record_to_source(
  mut buffer: &mut impl Write,
  record: BankRecord,
  input_format: &DataFormat,
) -> Result<(), SerializeError> {
  match input_format {
    DataFormat::Bin => BinRecord(record).write_to(buffer.deref_mut()),
    DataFormat::Csv => CsvRecord(record).write_to(buffer.deref_mut()),
    DataFormat::Txt => TxtRecord(record).write_to(buffer.deref_mut()),
  }
}

#[cfg(test)]
mod test_converter {
  use crate::configs::DataFormat;
  use crate::convert;
  use parser::parsers::bin;
  use parser::record::{Status, TxType};
  use std::io::Cursor;

  #[test]
  fn test_convert_txt_to_csv() {
    let source_data = String::from(
      "# Record 1 (DEPOSIT)
TX_TYPE: DEPOSIT
TO_USER_ID: 9223372036854775807
FROM_USER_ID: 0
TIMESTAMP: 1633036860000
DESCRIPTION: \"Record number 1\"
TX_ID: 1000000000000000
AMOUNT: 100
STATUS: FAILURE

# Record 2 (TRANSFER)
DESCRIPTION: \"Record number 2\"
TIMESTAMP: 1633036920000
STATUS: PENDING
AMOUNT: 200
TX_ID: 1000000000000001
TX_TYPE: TRANSFER
FROM_USER_ID: 9223372036854775807
TO_USER_ID: 9223372036854775807

",
    );
    let assert_data = String::from(
      "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
1000000000000000,DEPOSIT,0,9223372036854775807,100,1633036860000,FAILURE,\"Record number 1\"
1000000000000001,TRANSFER,9223372036854775807,9223372036854775807,200,1633036920000,PENDING,\"Record number 2\"
"
    );

    let mut input_buffer: Cursor<String> = Cursor::new(source_data.clone());
    let mut output_buffer: Vec<u8> = vec![];
    let input_format = DataFormat::Txt;
    let output_format = DataFormat::Csv;

    let result = convert(
      &mut input_buffer,
      &mut output_buffer,
      input_format,
      output_format,
    );

    assert!(result.is_ok());
    assert_eq!(output_buffer, assert_data.as_bytes());
  }

  #[test]
  fn test_convert_csv_to_bin() {
    let source_data = String::from(
      "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
1000000000000000,DEPOSIT,0,9223372036854775807,100,1633036860000,FAILURE,\"Record number 1\"
1000000000000001,TRANSFER,9223372036854775807,9223372036854775807,200,1633036920000,PENDING,\"Record number 2\"
"
    );
    let mut input_buffer = Cursor::new(source_data);
    let mut output_buffer: Vec<u8> = vec![];

    let mut assert_data: Vec<u8> = vec![];

    let record1_desc = String::from("Record number 1");
    assert_data.extend_from_slice(bin::RECORD_HEADER);
    assert_data.extend_from_slice(&61u32.to_be_bytes()[..]);
    assert_data.extend_from_slice(&1000000000000000u64.to_be_bytes()[..]);
    assert_data.extend_from_slice(&(TxType::Deposit as u8).to_be_bytes()[..]);
    assert_data.extend_from_slice(&0u64.to_be_bytes()[..]);
    assert_data.extend_from_slice(&9223372036854775807u64.to_be_bytes()[..]);
    assert_data.extend_from_slice(&100u64.to_be_bytes()[..]);
    assert_data.extend_from_slice(&1633036860000u64.to_be_bytes()[..]);
    assert_data.extend_from_slice(&(Status::Failure as u8).to_be_bytes()[..]);
    assert_data
      .extend_from_slice(&((record1_desc.len() + 2) as u32).to_be_bytes()[..]);
    assert_data.extend_from_slice("\"".as_bytes());
    assert_data.extend_from_slice(record1_desc.as_bytes());
    assert_data.extend_from_slice("\"".as_bytes());

    let record2_desc = String::from("Record number 2");
    assert_data.extend_from_slice(bin::RECORD_HEADER);
    assert_data.extend_from_slice(&61u32.to_be_bytes()[..]);
    assert_data.extend_from_slice(&1000000000000001u64.to_be_bytes()[..]);
    assert_data.extend_from_slice(&(TxType::Transfer as u8).to_be_bytes()[..]);
    assert_data.extend_from_slice(&9223372036854775807u64.to_be_bytes()[..]);
    assert_data.extend_from_slice(&9223372036854775807u64.to_be_bytes()[..]);
    assert_data.extend_from_slice(&200u64.to_be_bytes()[..]);
    assert_data.extend_from_slice(&1633036920000u64.to_be_bytes()[..]);
    assert_data.extend_from_slice(&(Status::Pending as u8).to_be_bytes()[..]);
    assert_data
      .extend_from_slice(&((record2_desc.len() + 2) as u32).to_be_bytes()[..]);
    assert_data.extend_from_slice("\"".as_bytes());
    assert_data.extend_from_slice(record2_desc.as_bytes());
    assert_data.extend_from_slice("\"".as_bytes());

    let input_format = DataFormat::Csv;
    let output_format = DataFormat::Bin;

    let result = convert(
      &mut input_buffer,
      &mut output_buffer,
      input_format,
      output_format,
    );

    assert!(result.is_ok());
    assert_eq!(output_buffer, assert_data);
  }

  #[test]
  fn test_convert_bin_to_txt() {
    let mut input_data: Vec<u8> = vec![];

    let record1_desc = String::from("Record number 1");
    input_data.extend_from_slice(bin::RECORD_HEADER);
    input_data.extend_from_slice(&63u32.to_be_bytes()[..]);
    input_data.extend_from_slice(&1000000000000000u64.to_be_bytes()[..]);
    input_data.extend_from_slice(&(TxType::Deposit as u8).to_be_bytes()[..]);
    input_data.extend_from_slice(&0u64.to_be_bytes()[..]);
    input_data.extend_from_slice(&9223372036854775807u64.to_be_bytes()[..]);
    input_data.extend_from_slice(&100u64.to_be_bytes()[..]);
    input_data.extend_from_slice(&1633036860000u64.to_be_bytes()[..]);
    input_data.extend_from_slice(&(Status::Failure as u8).to_be_bytes()[..]);
    input_data
      .extend_from_slice(&((record1_desc.len() + 2) as u32).to_be_bytes()[..]);
    input_data.extend_from_slice("\"".as_bytes());
    input_data.extend_from_slice(record1_desc.as_bytes());
    input_data.extend_from_slice("\"".as_bytes());

    let record2_desc = String::from("Record number 2");
    input_data.extend_from_slice(bin::RECORD_HEADER);
    input_data.extend_from_slice(&63u32.to_be_bytes()[..]);
    input_data.extend_from_slice(&1000000000000001u64.to_be_bytes()[..]);
    input_data.extend_from_slice(&(TxType::Transfer as u8).to_be_bytes()[..]);
    input_data.extend_from_slice(&9223372036854775807u64.to_be_bytes()[..]);
    input_data.extend_from_slice(&9223372036854775807u64.to_be_bytes()[..]);
    input_data.extend_from_slice(&200u64.to_be_bytes()[..]);
    input_data.extend_from_slice(&1633036920000u64.to_be_bytes()[..]);
    input_data.extend_from_slice(&(Status::Pending as u8).to_be_bytes()[..]);
    input_data
      .extend_from_slice(&((record2_desc.len() + 2) as u32).to_be_bytes()[..]);
    input_data.extend_from_slice("\"".as_bytes());
    input_data.extend_from_slice(record2_desc.as_bytes());
    input_data.extend_from_slice("\"".as_bytes());

    let mut input_buffer = Cursor::new(input_data);
    let mut output_buffer: Vec<u8> = vec![];

    let assert_data = String::from(
      "# Record 1 (DEPOSIT)
TX_ID: 1000000000000000
TX_TYPE: DEPOSIT
FROM_USER_ID: 0
TO_USER_ID: 9223372036854775807
AMOUNT: 100
TIMESTAMP: 1633036860000
STATUS: FAILURE
DESCRIPTION: \"Record number 1\"

# Record 2 (TRANSFER)
TX_ID: 1000000000000001
TX_TYPE: TRANSFER
FROM_USER_ID: 9223372036854775807
TO_USER_ID: 9223372036854775807
AMOUNT: 200
TIMESTAMP: 1633036920000
STATUS: PENDING
DESCRIPTION: \"Record number 2\"

",
    );

    let input_format = DataFormat::Bin;
    let output_format = DataFormat::Txt;

    let result = convert(
      &mut input_buffer,
      &mut output_buffer,
      input_format,
      output_format,
    );

    assert!(result.is_ok());
    assert_eq!(output_buffer, assert_data.as_bytes());
  }

  #[test]
  fn test_convert_txt_to_bin() {
    let source_data = String::from(
      "# Record 1 (DEPOSIT)
TX_TYPE: DEPOSIT
TO_USER_ID: 9223372036854775807
FROM_USER_ID: 0
TIMESTAMP: 1633036860000
DESCRIPTION: \"Record number 1\"
TX_ID: 1000000000000000
AMOUNT: 100
STATUS: FAILURE

# Record 2 (TRANSFER)
DESCRIPTION: \"Record number 2\"
TIMESTAMP: 1633036920000
STATUS: PENDING
AMOUNT: 200
TX_ID: 1000000000000001
TX_TYPE: TRANSFER
FROM_USER_ID: 9223372036854775807
TO_USER_ID: 9223372036854775807

",
    );
    let mut input_buffer = Cursor::new(source_data);

    let mut assert_data: Vec<u8> = vec![];

    let record1_desc = String::from("Record number 1");
    assert_data.extend_from_slice(bin::RECORD_HEADER);
    assert_data.extend_from_slice(&61u32.to_be_bytes()[..]);
    assert_data.extend_from_slice(&1000000000000000u64.to_be_bytes()[..]);
    assert_data.extend_from_slice(&(TxType::Deposit as u8).to_be_bytes()[..]);
    assert_data.extend_from_slice(&0u64.to_be_bytes()[..]);
    assert_data.extend_from_slice(&9223372036854775807u64.to_be_bytes()[..]);
    assert_data.extend_from_slice(&100u64.to_be_bytes()[..]);
    assert_data.extend_from_slice(&1633036860000u64.to_be_bytes()[..]);
    assert_data.extend_from_slice(&(Status::Failure as u8).to_be_bytes()[..]);
    assert_data
      .extend_from_slice(&((record1_desc.len() + 2) as u32).to_be_bytes()[..]);
    assert_data.extend_from_slice("\"".as_bytes());
    assert_data.extend_from_slice(record1_desc.as_bytes());
    assert_data.extend_from_slice("\"".as_bytes());

    let record2_desc = String::from("Record number 2");
    assert_data.extend_from_slice(bin::RECORD_HEADER);
    assert_data.extend_from_slice(&61u32.to_be_bytes()[..]);
    assert_data.extend_from_slice(&1000000000000001u64.to_be_bytes()[..]);
    assert_data.extend_from_slice(&(TxType::Transfer as u8).to_be_bytes()[..]);
    assert_data.extend_from_slice(&9223372036854775807u64.to_be_bytes()[..]);
    assert_data.extend_from_slice(&9223372036854775807u64.to_be_bytes()[..]);
    assert_data.extend_from_slice(&200u64.to_be_bytes()[..]);
    assert_data.extend_from_slice(&1633036920000u64.to_be_bytes()[..]);
    assert_data.extend_from_slice(&(Status::Pending as u8).to_be_bytes()[..]);
    assert_data
      .extend_from_slice(&((record2_desc.len() + 2) as u32).to_be_bytes()[..]);
    assert_data.extend_from_slice("\"".as_bytes());
    assert_data.extend_from_slice(record2_desc.as_bytes());
    assert_data.extend_from_slice("\"".as_bytes());

    let mut output_buffer: Vec<u8> = vec![];
    let input_format = DataFormat::Txt;
    let output_format = DataFormat::Bin;

    let result = convert(
      &mut input_buffer,
      &mut output_buffer,
      input_format,
      output_format,
    );

    assert!(result.is_ok());
    assert_eq!(output_buffer, assert_data);
  }

  #[test]
  fn test_convert_bin_to_csv() {
    let mut input_data: Vec<u8> = vec![];

    let record1_desc = String::from("Record number 1");
    input_data.extend_from_slice(bin::RECORD_HEADER);
    input_data.extend_from_slice(&63u32.to_be_bytes()[..]);
    input_data.extend_from_slice(&1000000000000000u64.to_be_bytes()[..]);
    input_data.extend_from_slice(&(TxType::Deposit as u8).to_be_bytes()[..]);
    input_data.extend_from_slice(&0u64.to_be_bytes()[..]);
    input_data.extend_from_slice(&9223372036854775807u64.to_be_bytes()[..]);
    input_data.extend_from_slice(&100u64.to_be_bytes()[..]);
    input_data.extend_from_slice(&1633036860000u64.to_be_bytes()[..]);
    input_data.extend_from_slice(&(Status::Failure as u8).to_be_bytes()[..]);
    input_data
      .extend_from_slice(&((record1_desc.len() + 2) as u32).to_be_bytes()[..]);
    input_data.extend_from_slice("\"".as_bytes());
    input_data.extend_from_slice(record1_desc.as_bytes());
    input_data.extend_from_slice("\"".as_bytes());

    let record2_desc = String::from("Record number 2");
    input_data.extend_from_slice(bin::RECORD_HEADER);
    input_data.extend_from_slice(&63u32.to_be_bytes()[..]);
    input_data.extend_from_slice(&1000000000000001u64.to_be_bytes()[..]);
    input_data.extend_from_slice(&(TxType::Transfer as u8).to_be_bytes()[..]);
    input_data.extend_from_slice(&9223372036854775807u64.to_be_bytes()[..]);
    input_data.extend_from_slice(&9223372036854775807u64.to_be_bytes()[..]);
    input_data.extend_from_slice(&200u64.to_be_bytes()[..]);
    input_data.extend_from_slice(&1633036920000u64.to_be_bytes()[..]);
    input_data.extend_from_slice(&(Status::Pending as u8).to_be_bytes()[..]);
    input_data
      .extend_from_slice(&((record2_desc.len() + 2) as u32).to_be_bytes()[..]);
    input_data.extend_from_slice("\"".as_bytes());
    input_data.extend_from_slice(record2_desc.as_bytes());
    input_data.extend_from_slice("\"".as_bytes());

    let mut input_buffer = Cursor::new(input_data);

    let assert_data = String::from(
      "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
1000000000000000,DEPOSIT,0,9223372036854775807,100,1633036860000,FAILURE,\"Record number 1\"
1000000000000001,TRANSFER,9223372036854775807,9223372036854775807,200,1633036920000,PENDING,\"Record number 2\"
"
    );

    let mut output_buffer: Vec<u8> = vec![];
    let input_format = DataFormat::Bin;
    let output_format = DataFormat::Csv;

    let result = convert(
      &mut input_buffer,
      &mut output_buffer,
      input_format,
      output_format,
    );

    assert!(result.is_ok());
    assert_eq!(output_buffer, assert_data.as_bytes());
  }

  #[test]
  fn test_convert_csv_to_txt() {
    let source_data = String::from(
      "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
1000000000000000,DEPOSIT,0,9223372036854775807,100,1633036860000,FAILURE,\"Record number 1\"
1000000000000001,TRANSFER,9223372036854775807,9223372036854775807,200,1633036920000,PENDING,\"Record number 2\"
"
    );
    let assert_data = String::from(
      "# Record 1 (DEPOSIT)
TX_ID: 1000000000000000
TX_TYPE: DEPOSIT
FROM_USER_ID: 0
TO_USER_ID: 9223372036854775807
AMOUNT: 100
TIMESTAMP: 1633036860000
STATUS: FAILURE
DESCRIPTION: \"Record number 1\"

# Record 2 (TRANSFER)
TX_ID: 1000000000000001
TX_TYPE: TRANSFER
FROM_USER_ID: 9223372036854775807
TO_USER_ID: 9223372036854775807
AMOUNT: 200
TIMESTAMP: 1633036920000
STATUS: PENDING
DESCRIPTION: \"Record number 2\"

",
    );

    let mut input_buffer: Cursor<String> = Cursor::new(source_data.clone());
    let mut output_buffer: Vec<u8> = vec![];
    let input_format = DataFormat::Csv;
    let output_format = DataFormat::Txt;

    let result = convert(
      &mut input_buffer,
      &mut output_buffer,
      input_format,
      output_format,
    );

    assert!(result.is_ok());
    assert_eq!(output_buffer, assert_data.as_bytes());
  }
}
