use crate::errors::{ParsingError, SerializeError};
use crate::{
  BankRecord, BankRecordParser, RECORD_LINES_NUMBER, Status, TxType,
  record_field,
};
use std::io;
use std::io::{BufRead, ErrorKind, Write};
use std::str::FromStr;

pub struct CsvReportParser;

impl BankRecordParser for CsvReportParser {
  fn from_read<R: BufRead>(
    reader: &mut R,
  ) -> Result<Vec<BankRecord>, ParsingError> {
    let mut records: Vec<BankRecord> = vec![];

    let mut lines = reader.lines();

    let first_line = lines.next();
    let headers = first_line.unwrap()?;
    let column_names: Vec<&str> = headers.split(',').collect();

    for (idx, str) in lines.map_while(Result::ok).enumerate() {
      let mut bank_record = BankRecord::new();
      let values: Vec<&str> = str.split(',').collect();
      let zip_iter = column_names.iter().zip(values);

      let columns_len = zip_iter.len();

      if columns_len != RECORD_LINES_NUMBER {
        return Err(ParsingError::IO(io::Error::new(
          ErrorKind::InvalidData,
          format!("Line: {}; Wrong number of columns in row: {str}", idx + 1,),
        )));
      }

      for (&field_name, field_value) in zip_iter {
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
          _ => (),
        }
      }

      records.push(bank_record);
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
