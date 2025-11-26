use crate::errors::{ParsingError, SerializeError};
use crate::{
  BankRecord, BankRecordParser, RECORD_LINES_NUMBER, Status, TxType,
  record_field,
};
use std::io;
use std::io::{BufRead, ErrorKind, Write};
use std::str::FromStr;

pub struct TxtReportParser;

impl BankRecordParser for TxtReportParser {
  fn from_read<R: BufRead>(
    reader: &mut R,
  ) -> Result<Vec<BankRecord>, ParsingError> {
    let mut records: Vec<BankRecord> = vec![];
    let mut bank_record = BankRecord::new();

    let mut file_lines_count: usize = 0;
    let mut record_lines_count: usize = 0;

    for line in reader.lines() {
      file_lines_count += 1;

      let line = line?;

      if line.starts_with("#") {
        continue;
      }

      if line.is_empty() {
        if record_lines_count == 0 {
          // redundant empty line in file or EOF
          break;
        }

        // Empty line withing record lines
        if (1..RECORD_LINES_NUMBER).contains(&record_lines_count) {
          return Err(ParsingError::IO(io::Error::new(
            ErrorKind::UnexpectedEof,
            format!(
              "2 Line: {file_lines_count}; Bank record should have at least {RECORD_LINES_NUMBER} lines"
            ),
          )));
        }

        // record_lines_count is 8
        // valid empty line between records
        record_lines_count = 0;

        continue;
      }

      record_lines_count += 1;

      // record line limit overflow
      if record_lines_count > RECORD_LINES_NUMBER {
        return Err(ParsingError::IO(io::Error::new(
          ErrorKind::UnexpectedEof,
          format!(
            "Line: {file_lines_count}; Bank record has more than {RECORD_LINES_NUMBER} lines"
          ),
        )));
      }

      let parts = line.split(':');
      let mut parts_iter = parts.into_iter();

      let (field_name, field_value) = (parts_iter.next(), parts_iter.next());

      if field_name.is_none() || field_value.is_none() {
        return Err(ParsingError::IO(io::Error::new(
          ErrorKind::InvalidData,
          format!("Line: {file_lines_count}; Failed parsing record line"),
        )));
      }

      let field_name = field_name.unwrap().trim();
      let field_value = field_value.unwrap().trim();

      match field_name {
        record_field::TX_ID => {
          bank_record.tx_id = field_value.parse::<u64>()?;
        }
        record_field::TX_TYPE => {
          bank_record.tx_type = TxType::from_str(field_value)?;
        }
        record_field::FROM_USER_ID => {
          bank_record.from_user_id = field_value.parse::<u64>()?;
        }
        record_field::TO_USER_ID => {
          bank_record.to_user_id = field_value.parse::<u64>()?;
        }
        record_field::AMOUNT => {
          bank_record.amount = field_value.parse::<i64>()?;
        }
        record_field::TIMESTAMP => {
          bank_record.timestamp = field_value.parse::<u64>()?;
        }
        record_field::STATUS => {
          bank_record.status = Status::from_str(field_value)?;
        }
        record_field::DESCRIPTION => {
          bank_record.description = field_value.replace('"', "");
        }
        unknown_field => {
          return Err(ParsingError::IO(io::Error::new(
            ErrorKind::InvalidData,
            format!(
              "Line: {file_lines_count}; Unknown record field: {unknown_field}"
            ),
          )));
        }
      }

      if record_lines_count == RECORD_LINES_NUMBER {
        let record_clone = bank_record.clone();

        records.push(record_clone);
        bank_record = BankRecord::new();
      }
    }

    if (1..RECORD_LINES_NUMBER).contains(&record_lines_count) {
      // Last line ended unexpectedly
      return Err(ParsingError::IO(io::Error::new(
        ErrorKind::UnexpectedEof,
        format!(
          "2 Line: {file_lines_count}; Bank record should have at least {RECORD_LINES_NUMBER} lines"
        ),
      )));
    }

    Ok(records)
  }
  fn write_to<W: Write>(
    &mut self,
    _writer: &mut W,
  ) -> Result<(), SerializeError> {
    todo!()
  }
}
