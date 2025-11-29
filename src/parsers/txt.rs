use crate::constants::{RECORD_LINES_NUMBER, record_field};
use crate::errors::{ParsingError, SerializeError};
use crate::record::{BankRecord, BankRecordSerDe, Status, TxType};
use std::io;
use std::io::{BufRead, ErrorKind, Write};
use std::str::FromStr;

pub struct TxtReportParser;
pub struct TxtRecord(pub BankRecord);

impl BankRecordSerDe for TxtRecord {
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

      // record line limit overflow
      if record_lines_count > RECORD_LINES_NUMBER {
        return Err(ParsingError::IO(io::Error::new(
          ErrorKind::UnexpectedEof,
          format!("Bank record has more than {RECORD_LINES_NUMBER} lines"),
        )));
      }

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

// #[cfg(test)]
// mod txt_parser_test {
//   // TODO Use cursor
//

//   #[test]
//   fn test_valid_input() {
//     todo!()
//   }
//
//   #[test]
//   fn test_missing_record_middle_line() {
//     todo!()
//   }
//
//   #[test]
//   fn test_missing_file_last_line() {
//     todo!()
//   }
//
//   #[test]
//   fn test_extra_record_line() {
//     todo!()
//   }
//
//   #[test]
//   fn test_extra_empty_line() {
//     todo!()
//   }
// }
