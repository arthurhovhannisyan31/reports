use crate::constants::{RECORD_LINES_NUMBER, record_field};
use crate::errors::{ParsingError, SerializeError};
use crate::record::{BankRecord, BankRecordParser, Status, TxType};
use std::io;
use std::io::{BufRead, ErrorKind, Write};
use std::str::FromStr;

pub struct TxtRecord(pub BankRecord);

impl BankRecordParser for TxtRecord {
  fn from_read<R: BufRead>(buffer: &mut R) -> Result<BankRecord, ParsingError> {
    let mut bank_record = BankRecord::new();
    let mut record_lines_count: usize = 0;

    for line in buffer.lines() {
      let line = line?;

      if line.starts_with("#") {
        continue;
      }

      if line.is_empty() {
        match record_lines_count {
          0 => {
            // empty leading line is allowed
            continue;
          }
          1..RECORD_LINES_NUMBER => {
            return Err(ParsingError::IO(io::Error::new(
              ErrorKind::UnexpectedEof,
              format!(
                "Bank record should have at least {RECORD_LINES_NUMBER} lines"
              ),
            )));
          }
          _ => {
            return Err(ParsingError::Custom(format!(
              "Invalid record data, should have {RECORD_LINES_NUMBER} lines"
            )));
          }
        }
      }

      record_lines_count += 1;

      let parts = line.split(':');
      let mut parts_iter = parts.into_iter();

      let (field_name, field_value) = (parts_iter.next(), parts_iter.next());

      if field_name.is_none() || field_value.is_none() {
        return Err(ParsingError::IO(io::Error::new(
          ErrorKind::InvalidData,
          "Failed parsing record line",
        )));
      }

      let field_name = field_name.unwrap().trim();
      let field_value = field_value.unwrap().trim();

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
        unknown_field => {
          return Err(ParsingError::IO(io::Error::new(
            ErrorKind::InvalidData,
            format!("Unknown record field: {unknown_field}"),
          )));
        }
      }

      if record_lines_count == RECORD_LINES_NUMBER {
        return Ok(bank_record);
      }
    }

    Err(ParsingError::IO(io::Error::new(
      ErrorKind::UnexpectedEof,
      format!("Bank record should have at least {RECORD_LINES_NUMBER} lines"),
    )))
  }
  fn write_to<W: Write>(
    &mut self,
    buffer: &mut W,
  ) -> Result<(), SerializeError> {
    let tx_id_10k_mod = self.0.tx_id % 10000 + 1;

    // Leading comment line
    writeln!(buffer, "# Record {} ({})", tx_id_10k_mod, self.0.tx_type)?;
    writeln!(buffer, "{}: {}", record_field::TX_ID, self.0.tx_id)?;
    writeln!(buffer, "{}: {}", record_field::TX_TYPE, self.0.tx_type)?;
    writeln!(
      buffer,
      "{}: {}",
      record_field::FROM_USER_ID,
      self.0.from_user_id
    )?;
    writeln!(
      buffer,
      "{}: {}",
      record_field::TO_USER_ID,
      self.0.to_user_id
    )?;
    writeln!(buffer, "{}: {}", record_field::AMOUNT, self.0.amount)?;
    writeln!(buffer, "{}: {}", record_field::TIMESTAMP, self.0.timestamp)?;
    writeln!(buffer, "{}: {}", record_field::STATUS, self.0.status)?;
    writeln!(
      buffer,
      "{}: {:?}",
      record_field::DESCRIPTION,
      self.0.description
    )?;
    // Empty line separator
    writeln!(buffer)?;

    Ok(())
  }
}

#[cfg(test)]
mod txt_parser_test {
  use crate::parsers::txt::TxtRecord;
  use crate::record::{BankRecord, BankRecordParser, Status, TxType};
  use std::io::{Cursor, Write};
  use std::str::FromStr;

  #[test]
  fn test_parse_valid_input() {
    let mut buff = Cursor::new(String::from(
      "# Record 1 (DEPOSIT)
TX_ID: 1000000000000000
TX_TYPE: DEPOSIT
FROM_USER_ID: 0
TO_USER_ID: 9223372036854775807
AMOUNT: 100
TIMESTAMP: 1633036860000
STATUS: FAILURE
DESCRIPTION: \"Record number 1\"",
    ));

    let rec_result = TxtRecord::from_read(&mut buff);
    assert!(rec_result.is_ok());

    let rec = rec_result.unwrap();
    assert_eq!(rec.tx_id, 1000000000000000u64);
    assert_eq!(rec.tx_type, TxType::from_str("DEPOSIT").unwrap());
    assert_eq!(rec.from_user_id, 0u64);
    assert_eq!(rec.to_user_id, 9223372036854775807u64);
    assert_eq!(rec.amount, 100);
    assert_eq!(rec.timestamp, 1633036860000u64);
    assert_eq!(rec.status, Status::from_str("FAILURE").unwrap());
    // No need to escape quotes, serialization writes string quoted
    assert_eq!(rec.description, String::from("Record number 1"));
  }

  #[test]
  fn test_parse_leading_empty_lines() {
    let mut buff = Cursor::new(String::from(
      "

# Record 1 (DEPOSIT)

TX_ID: 1000000000000000
TX_TYPE: DEPOSIT
FROM_USER_ID: 0
TO_USER_ID: 9223372036854775807
AMOUNT: 100
TIMESTAMP: 1633036860000
STATUS: FAILURE
DESCRIPTION: \"Record number 1\"",
    ));

    let rec_result = TxtRecord::from_read(&mut buff);
    assert!(rec_result.is_ok());

    let rec = rec_result.unwrap();
    assert_eq!(rec.tx_id, 1000000000000000u64);
    assert_eq!(rec.tx_type, TxType::from_str("DEPOSIT").unwrap());
    assert_eq!(rec.from_user_id, 0u64);
    assert_eq!(rec.to_user_id, 9223372036854775807u64);
    assert_eq!(rec.amount, 100);
    assert_eq!(rec.timestamp, 1633036860000u64);
    assert_eq!(rec.status, Status::from_str("FAILURE").unwrap());
    // No need to escape quotes, serialization writes string quoted
    assert_eq!(rec.description, String::from("Record number 1"));
  }

  #[test]
  fn test_parse_trailing_empty_lines() {
    let mut buff = Cursor::new(String::from(
      "# Record 1 (DEPOSIT)
TX_ID: 1000000000000000
TX_TYPE: DEPOSIT
FROM_USER_ID: 0
TO_USER_ID: 9223372036854775807
AMOUNT: 100
TIMESTAMP: 1633036860000
STATUS: FAILURE
DESCRIPTION: \"Record number 1\"

      
",
    ));

    let rec_result = TxtRecord::from_read(&mut buff);
    assert!(rec_result.is_ok());

    let rec = rec_result.unwrap();
    assert_eq!(rec.tx_id, 1000000000000000u64);
    assert_eq!(rec.tx_type, TxType::from_str("DEPOSIT").unwrap());
    assert_eq!(rec.from_user_id, 0u64);
    assert_eq!(rec.to_user_id, 9223372036854775807u64);
    assert_eq!(rec.amount, 100);
    assert_eq!(rec.timestamp, 1633036860000u64);
    assert_eq!(rec.status, Status::from_str("FAILURE").unwrap());
    // No need to escape quotes, serialization writes string quoted
    assert_eq!(rec.description, String::from("Record number 1"));
  }

  #[test]
  fn test_parse_missing_description() {
    let mut buff = Cursor::new(String::from(
      "# Record 1 (DEPOSIT)
TX_ID: 1000000000000000
TX_TYPE: DEPOSIT
FROM_USER_ID: 0
TO_USER_ID: 9223372036854775807
AMOUNT: 100
TIMESTAMP: 1633036860000
STATUS: FAILURE",
    ));
    let rec_result = TxtRecord::from_read(&mut buff);

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
      // No need to escape quotes, serialization writes string quoted
      description: String::from("Record number 1"),
    };

    let _ = TxtRecord(record).write_to(&mut buffer);
    buffer.flush().unwrap();

    // Pay attention that new line is doubled to break the line and make empty one
    let assert_result = String::from(
      "# Record 1 (DEPOSIT)
TX_ID: 1000000000000000
TX_TYPE: DEPOSIT
FROM_USER_ID: 0
TO_USER_ID: 9223372036854775807
AMOUNT: 100
TIMESTAMP: 1633036860000
STATUS: FAILURE
DESCRIPTION: \"Record number 1\"

",
    );

    assert_eq!(buffer.into_inner(), assert_result.as_bytes());
  }
}
