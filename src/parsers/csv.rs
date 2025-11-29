use crate::constants::{RECORD_LINES_NUMBER, record_field};
use crate::errors::{ParsingError, SerializeError};
use crate::record::{BankRecord, BankRecordSerDe, Status, TxType};
use std::io;
use std::io::{BufRead, ErrorKind, Write};
use std::str::FromStr;

pub struct CsvRecord(pub BankRecord);

pub const CVS_HEADERS: &str =
  "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION";

impl BankRecordSerDe for CsvRecord {
  fn from_read<R: BufRead>(buffer: &mut R) -> Result<BankRecord, ParsingError> {
    let mut lines = buffer.lines();
    let mut bank_record = BankRecord::new();

    let line = lines.next().ok_or_else(|| {
      ParsingError::Custom("EOF: File has no lines to read".to_string())
    })?;
    let line = line?;

    let column_names: Vec<&str> = CVS_HEADERS.split(',').collect();
    let values: Vec<&str> = line.split(',').collect();

    let zip_iter = column_names.iter().zip(values);
    let columns_len = zip_iter.len();

    if columns_len != RECORD_LINES_NUMBER {
      return Err(ParsingError::IO(io::Error::new(
        ErrorKind::InvalidData,
        format!("Wrong number of columns in row: {line}"),
      )));
    }

    for (&field_name, field_value) in zip_iter {
      match field_name {
        record_field::TX_ID => {
          bank_record.tx_id = field_value.parse::<u64>()?;
        }
        record_field::TX_TYPE => {
          bank_record.tx_type =
            TxType::from_str(field_value).map_err(ParsingError::ParseTxType)?;
        }
        record_field::FROM_USER_ID => {
          bank_record.from_user_id = field_value.parse::<u64>()?;
        }
        record_field::TO_USER_ID => {
          bank_record.to_user_id = field_value.parse::<u64>()?;
        }
        record_field::AMOUNT => {
          bank_record.amount = field_value.parse::<u64>()?;
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

    Ok(bank_record)
  }
  fn write_to<W: Write>(
    &mut self,
    buffer: &mut W,
  ) -> Result<(), SerializeError> {
    let columns: Vec<String> = vec![
      self.0.tx_id.to_string(),
      self.0.tx_type.to_string(),
      self.0.from_user_id.to_string(),
      self.0.to_user_id.to_string(),
      self.0.amount.to_string(),
      self.0.timestamp.to_string(),
      self.0.status.to_string(),
      format!("{:?}", self.0.description.to_string()),
    ];

    writeln!(buffer, "{}", columns.join(","))?;

    Ok(())
  }
}

#[cfg(test)]
mod csv_parser_test {
  use crate::parsers::csv::CsvRecord;
  use crate::record::{BankRecord, BankRecordSerDe, Status, TxType};
  use std::io::{Cursor, Write};
  use std::str::FromStr;

  #[test]
  fn test_parse_valid_input() {
    let mut buff = Cursor::new(String::from(
      "1000000000000000,DEPOSIT,0,9223372036854775807,100,1633036860000,FAILURE,\"Record number 1\"",
    ));

    let rec_result = CsvRecord::from_read(&mut buff);
    assert!(rec_result.is_ok());

    let rec = rec_result.unwrap();
    assert_eq!(rec.tx_id, 1000000000000000u64);
    assert_eq!(rec.tx_type, TxType::from_str("DEPOSIT").unwrap());
    assert_eq!(rec.from_user_id, 0u64);
    assert_eq!(rec.to_user_id, 9223372036854775807u64);
    assert_eq!(rec.amount, 100);
    assert_eq!(rec.timestamp, 1633036860000u64);
    assert_eq!(rec.status, Status::from_str("FAILURE").unwrap());
    assert_eq!(rec.description, String::from("Record number 1"));
  }

  #[test]
  fn test_parse_missing_column() {
    let mut buff = Cursor::new(String::from(
      "DEPOSIT,0,9223372036854775807,100,1633036860000,FAILURE,\"Record number 1\"",
    ));

    let rec_result = CsvRecord::from_read(&mut buff);
    // Change errors to assert returned errors
    assert!(rec_result.is_err());
  }

  #[test]
  fn test_parse_extra_column() {
    let mut buff = Cursor::new(String::from(
      "DEPOSIT,0,9223372036854775807,100,1633036860000,FAILURE,\"Record number 1\"\"Hello Kitty\"",
    ));

    let rec_result = CsvRecord::from_read(&mut buff);
    assert!(rec_result.is_err());
  }

  #[test]
  fn test_parse_empty_line() {
    let mut buff = Cursor::new(String::from(""));

    let rec_result = CsvRecord::from_read(&mut buff);
    assert!(rec_result.is_err());
  }

  #[test]
  fn test_serialize_record() {
    let vec: Vec<u8> = vec![];
    let mut buffer = Cursor::new(vec);

    let record = BankRecord {
      tx_id: 1000000000000000,
      tx_type: TxType::Deposit,
      from_user_id: 0,
      to_user_id: 9223372036854775807,
      amount: 100,
      timestamp: 1633036860000,
      status: Status::Failure,
      description: String::from("Record number 1"),
    };

    let _ = CsvRecord(record).write_to(&mut buffer);
    buffer.flush().unwrap();

    let assert_result = String::from(
      "1000000000000000,DEPOSIT,0,9223372036854775807,100,1633036860000,FAILURE,\"Record number 1\"\n",
    );

    assert_eq!(buffer.into_inner(), assert_result.as_bytes());
  }
}
