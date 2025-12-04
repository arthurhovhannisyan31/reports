use crate::errors::{ParsingError, SerializeError};
use crate::record::{BankRecord, BankRecordParser, Status, TxType};
use std::cmp::min;
use std::io;
use std::io::{BufRead, IoSlice, IoSliceMut, Write};

pub struct BinRecord(pub BankRecord);

pub static RECORD_HEADER: &[u8; 4] = b"YPBN";

impl BankRecordParser for BinRecord {
  fn from_read<R: BufRead>(buffer: &mut R) -> Result<BankRecord, ParsingError> {
    let mut record_header_buf = [0u8; 4];

    buffer
      .read_exact(&mut record_header_buf)
      .map_err(ParsingError::IO)?;

    loop {
      if record_header_buf == *RECORD_HEADER {
        break;
      }

      // RECORD_HEADER is lost, scan 4 bytes window with 1 byte step
      // Move buffer cursor forward by 1 byte
      let mut step = [0u8; 1];
      buffer.read_exact(&mut step)?;

      // Shift last 3 bytes to front, fill the last byte from step buffer
      record_header_buf.copy_within(1.., 0);
      record_header_buf[3] = step[0];

      // Repeat the loop to check if header is found
      continue;
    }

    buffer.read_exact(&mut record_header_buf)?;

    let record_size = u32::from_be_bytes(record_header_buf);

    let mut tx_id = [0u8; 8];
    let mut tx_type = [0u8; 1];
    let mut from_user_id = [0u8; 8];
    let mut to_user_id = [0u8; 8];
    let mut amount = [0u8; 8];
    let mut timestamp = [0u8; 8];
    let mut status = [0u8; 1];
    let mut description_len = [0u8; 4];

    let mut bufs = [
      IoSliceMut::new(&mut tx_id),
      IoSliceMut::new(&mut tx_type),
      IoSliceMut::new(&mut from_user_id),
      IoSliceMut::new(&mut to_user_id),
      IoSliceMut::new(&mut amount),
      IoSliceMut::new(&mut timestamp),
      IoSliceMut::new(&mut status),
      IoSliceMut::new(&mut description_len),
    ];
    let read_bytes = buffer.read_vectored(&mut bufs)?;

    let description_len = u32::from_be_bytes(description_len);
    // Description buffer leftover size needs to be calculated based on number bytes read
    // It allows to move cursor to the end of record section
    let buffer_leftover = record_size - read_bytes as u32;
    let description_buf_size = min(description_len, buffer_leftover);
    let mut desc_buf = vec![0u8; description_buf_size as usize];

    buffer.read_exact(&mut desc_buf)?;

    let description =
      String::try_from(desc_buf.clone()).unwrap_or("".to_string());
    // Escaped quotes are not needed in model
    let normalized_description = description.replace("\"", "");

    Ok(BankRecord {
      tx_id: u64::from_be_bytes(tx_id),
      tx_type: TxType::try_from(u8::from_be_bytes(tx_type))?,
      from_user_id: u64::from_be_bytes(from_user_id),
      to_user_id: u64::from_be_bytes(to_user_id),
      amount: u64::from_be_bytes(amount),
      timestamp: u64::from_be_bytes(timestamp),
      status: Status::try_from(u8::from_be_bytes(status))?,
      description: normalized_description,
    })
  }
  fn write_to<W: Write>(
    &mut self,
    buffer: &mut W,
  ) -> Result<(), SerializeError> {
    let tx_id_buf = self.0.tx_id.to_be_bytes();
    let tx_type_buf = (self.0.tx_type.clone() as u8).to_be_bytes();
    let from_user_id_buf = self.0.from_user_id.to_be_bytes();
    let to_user_id_buf = self.0.to_user_id.to_be_bytes();
    let amount_buf = self.0.amount.to_be_bytes();
    let timestamp_buf = self.0.timestamp.to_be_bytes();
    let status_buf = (self.0.status.clone() as u8).to_be_bytes();
    let description_len = self.0.description.len();
    // Need to add 2 bytes for escaped quotes, to prevent data model layout shift
    let adjusted_description_len = if description_len == 0 {
      0
    } else {
      description_len + 2
    };
    let description_len_buf = (adjusted_description_len as u32).to_be_bytes();

    let bufs = [
      IoSlice::new(&tx_id_buf),
      IoSlice::new(&tx_type_buf),
      IoSlice::new(&from_user_id_buf),
      IoSlice::new(&to_user_id_buf),
      IoSlice::new(&amount_buf),
      IoSlice::new(&timestamp_buf),
      IoSlice::new(&status_buf),
      IoSlice::new(&description_len_buf),
    ];

    let record_size: u32 = (bufs.iter().map(|slice| slice.len()).sum::<usize>()
      + self.0.description.len()) as u32;

    // Write record header
    buffer.write_all(RECORD_HEADER)?;
    buffer.write_all(&record_size.to_be_bytes())?;

    // Write record body
    let write_bytes = buffer.write_vectored(&bufs)?;

    if write_bytes == 0 {
      return Err(SerializeError::IO(io::Error::other(
        "Source no longer able to accept bytes",
      )));
    }

    if description_len > 0 {
      // Write escaped quotes to record model
      write!(buffer, "\"")?;
      buffer.write_all(self.0.description.as_bytes())?;
      write!(buffer, "\"")?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod bin_parser_test {
  use crate::parsers::bin::{BinRecord, RECORD_HEADER};
  use crate::record::{BankRecord, BankRecordParser, Status, TxType};
  use std::io::{Cursor, Write};

  #[test]
  fn test_parse_valid_input() {
    let mut buff: Vec<u8> = vec![];
    // String quotes need to be escaped, values are written as is
    let description = String::from("Record number 1");

    buff.extend_from_slice(RECORD_HEADER);
    buff.extend_from_slice(&63u32.to_be_bytes()[..]);
    buff.extend_from_slice(&1000000000000000u64.to_be_bytes()[..]);
    buff.extend_from_slice(&(TxType::Deposit as u8).to_be_bytes()[..]);
    buff.extend_from_slice(&0u64.to_be_bytes()[..]);
    buff.extend_from_slice(&9223372036854775807u64.to_be_bytes()[..]);
    buff.extend_from_slice(&100u64.to_be_bytes()[..]);
    buff.extend_from_slice(&1633036860000u64.to_be_bytes()[..]);
    buff.extend_from_slice(&(Status::Failure as u8).to_be_bytes()[..]);
    buff.extend_from_slice(&((description.len() + 2) as u32).to_be_bytes()[..]);
    buff.extend_from_slice("\"".as_bytes());
    buff.extend_from_slice(description.as_bytes());
    buff.extend_from_slice("\"".as_bytes());

    let mut buff = Cursor::new(buff);
    let rec_result = BinRecord::from_read(&mut buff);

    assert!(rec_result.is_ok());

    let rec = rec_result.unwrap();

    assert_eq!(rec.tx_id, 1000000000000000u64);
    assert_eq!(rec.tx_type, TxType::Deposit);
    assert_eq!(rec.from_user_id, 0u64);
    assert_eq!(rec.to_user_id, 9223372036854775807u64);
    assert_eq!(rec.amount, 100u64);
    assert_eq!(rec.timestamp, 1633036860000u64);
    assert_eq!(rec.timestamp, 1633036860000u64);
    assert_eq!(rec.status, Status::Failure);
    assert_eq!(rec.description, description);
  }

  #[test]
  fn test_parse_missing_description() {
    let mut buff: Vec<u8> = vec![];
    let description = String::from("");

    buff.extend_from_slice(RECORD_HEADER);
    buff.extend_from_slice(&63u32.to_be_bytes()[..]);
    buff.extend_from_slice(&1000000000000000u64.to_be_bytes()[..]);
    buff.extend_from_slice(&(TxType::Deposit as u8).to_be_bytes()[..]);
    buff.extend_from_slice(&0u64.to_be_bytes()[..]);
    buff.extend_from_slice(&9223372036854775807u64.to_be_bytes()[..]);
    buff.extend_from_slice(&100u64.to_be_bytes()[..]);
    buff.extend_from_slice(&1633036860000u64.to_be_bytes()[..]);
    buff.extend_from_slice(&(Status::Failure as u8).to_be_bytes()[..]);
    buff.extend_from_slice(&(description.len() as u32).to_be_bytes()[..]);

    let mut buff = Cursor::new(buff);
    let rec_result = BinRecord::from_read(&mut buff);

    assert!(rec_result.is_ok());

    let rec = rec_result.unwrap();

    assert_eq!(rec.tx_id, 1000000000000000u64);
    assert_eq!(rec.tx_type, TxType::Deposit);
    assert_eq!(rec.from_user_id, 0u64);
    assert_eq!(rec.to_user_id, 9223372036854775807u64);
    assert_eq!(rec.amount, 100u64);
    assert_eq!(rec.timestamp, 1633036860000u64);
    assert_eq!(rec.timestamp, 1633036860000u64);
    assert_eq!(rec.status, Status::Failure);
  }

  #[test]
  fn test_parse_shifted_header_key() {
    let mut buff: Vec<u8> = vec![];
    // String quotes need to be escaped, values are written as is
    let description = String::from("Record number 1");

    buff.extend_from_slice("\"Hello Kitty\"".as_bytes());
    buff.extend_from_slice(RECORD_HEADER);
    buff.extend_from_slice(&63u32.to_be_bytes()[..]);
    buff.extend_from_slice(&1000000000000000u64.to_be_bytes()[..]);
    buff.extend_from_slice(&(TxType::Deposit as u8).to_be_bytes()[..]);
    buff.extend_from_slice(&0u64.to_be_bytes()[..]);
    buff.extend_from_slice(&9223372036854775807u64.to_be_bytes()[..]);
    buff.extend_from_slice(&100u64.to_be_bytes()[..]);
    buff.extend_from_slice(&1633036860000u64.to_be_bytes()[..]);
    buff.extend_from_slice(&(Status::Failure as u8).to_be_bytes()[..]);
    buff.extend_from_slice(&((description.len() + 2) as u32).to_be_bytes()[..]);
    buff.extend_from_slice("\"".as_bytes());
    buff.extend_from_slice(description.as_bytes());
    buff.extend_from_slice("\"".as_bytes());

    let mut buff = Cursor::new(buff);
    let rec_result = BinRecord::from_read(&mut buff);

    assert!(rec_result.is_ok());

    let rec = rec_result.unwrap();

    assert_eq!(rec.tx_id, 1000000000000000u64);
    assert_eq!(rec.tx_type, TxType::Deposit);
    assert_eq!(rec.from_user_id, 0u64);
    assert_eq!(rec.to_user_id, 9223372036854775807u64);
    assert_eq!(rec.amount, 100u64);
    assert_eq!(rec.timestamp, 1633036860000u64);
    assert_eq!(rec.timestamp, 1633036860000u64);
    assert_eq!(rec.status, Status::Failure);
    assert_eq!(rec.description, description);
  }

  #[test]
  fn test_parse_missing_header_key() {
    let mut buff: Vec<u8> = vec![];

    // Missing header slice
    buff.extend_from_slice(&46u32.to_be_bytes()[..]);
    buff.extend_from_slice(&1000000000000000u64.to_be_bytes()[..]);
    buff.extend_from_slice(&(TxType::Deposit as u8).to_be_bytes()[..]);
    buff.extend_from_slice(&0u64.to_be_bytes()[..]);
    buff.extend_from_slice(&9223372036854775807u64.to_be_bytes()[..]);
    buff.extend_from_slice(&100u64.to_be_bytes()[..]);
    buff.extend_from_slice(&1633036860000u64.to_be_bytes()[..]);
    buff.extend_from_slice(&(Status::Failure as u8).to_be_bytes()[..]);
    buff.extend_from_slice(&0u32.to_be_bytes()[..]);

    let mut buff = Cursor::new(buff);
    let rec_result = BinRecord::from_read(&mut buff);

    assert!(rec_result.is_err());
  }

  #[test]
  fn test_parse_data_layout_shift() {
    let mut buff: Vec<u8> = vec![];
    // String quotes need to be escaped, values are written as is
    let description = String::from("Record number 1");

    buff.extend_from_slice(RECORD_HEADER);
    buff.extend_from_slice(&63u32.to_be_bytes()[..]);
    buff.extend_from_slice(&1000000000000000u64.to_be_bytes()[..]);
    buff.extend_from_slice(&(TxType::Deposit as u8).to_be_bytes()[..]);
    buff.extend_from_slice(&0u64.to_be_bytes()[..]);
    buff.extend_from_slice(&9223372036854775807u64.to_be_bytes()[..]);
    // Missing amount field
    buff.extend_from_slice(&1633036860000u64.to_be_bytes()[..]);
    buff.extend_from_slice(&(Status::Failure as u8).to_be_bytes()[..]);
    buff.extend_from_slice(&0u32.to_be_bytes()[..]);

    let mut buff = Cursor::new(buff);
    let rec_result = BinRecord::from_read(&mut buff);

    assert!(rec_result.is_ok());

    let rec = rec_result.unwrap();

    assert_eq!(rec.tx_id, 1000000000000000u64);
    assert_eq!(rec.tx_type, TxType::Deposit);
    assert_eq!(rec.from_user_id, 0u64);
    assert_eq!(rec.to_user_id, 9223372036854775807u64);
    assert_ne!(rec.amount, 100u64);
    assert_ne!(rec.timestamp, 1633036860000u64);
    assert_ne!(rec.timestamp, 1633036860000u64);
    assert_ne!(rec.status, Status::Failure);
    assert_ne!(rec.description, description);
  }

  #[test]
  fn test_serialize_record() {
    let mut assert_buffer: Vec<u8> = vec![];
    let description = String::from("Record number 1");

    assert_buffer.extend_from_slice(RECORD_HEADER);
    assert_buffer.extend_from_slice(&61u32.to_be_bytes()[..]);
    assert_buffer.extend_from_slice(&1000000000000000u64.to_be_bytes()[..]);
    assert_buffer.extend_from_slice(&(TxType::Deposit as u8).to_be_bytes()[..]);
    assert_buffer.extend_from_slice(&0u64.to_be_bytes()[..]);
    assert_buffer.extend_from_slice(&9223372036854775807u64.to_be_bytes()[..]);
    assert_buffer.extend_from_slice(&100u64.to_be_bytes()[..]);
    assert_buffer.extend_from_slice(&1633036860000u64.to_be_bytes()[..]);
    assert_buffer.extend_from_slice(&(Status::Failure as u8).to_be_bytes()[..]);
    assert_buffer
      .extend_from_slice(&((description.len() + 2) as u32).to_be_bytes()[..]);
    assert_buffer.extend_from_slice("\"".as_bytes());
    assert_buffer.extend_from_slice(description.as_bytes());
    assert_buffer.extend_from_slice("\"".as_bytes());

    let vec: Vec<u8> = vec![];
    let mut write_buffer = Cursor::new(vec);

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

    let _ = BinRecord(record).write_to(&mut write_buffer);
    write_buffer.flush().unwrap();

    assert_eq!(write_buffer.into_inner(), assert_buffer);
  }

  #[test]
  fn test_serialize_missing_description() {
    let mut assert_buffer: Vec<u8> = vec![];

    assert_buffer.extend_from_slice(RECORD_HEADER);
    // Content size does not include description field at all
    assert_buffer.extend_from_slice(&46u32.to_be_bytes()[..]);
    assert_buffer.extend_from_slice(&1000000000000000u64.to_be_bytes()[..]);
    assert_buffer.extend_from_slice(&(TxType::Deposit as u8).to_be_bytes()[..]);
    assert_buffer.extend_from_slice(&0u64.to_be_bytes()[..]);
    assert_buffer.extend_from_slice(&9223372036854775807u64.to_be_bytes()[..]);
    assert_buffer.extend_from_slice(&100u64.to_be_bytes()[..]);
    assert_buffer.extend_from_slice(&1633036860000u64.to_be_bytes()[..]);
    assert_buffer.extend_from_slice(&(Status::Failure as u8).to_be_bytes()[..]);
    assert_buffer.extend_from_slice(&0u32.to_be_bytes()[..]);

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
      description: String::from(""),
    };

    let _ = BinRecord(record).write_to(&mut buffer);
    buffer.flush().unwrap();

    assert_eq!(buffer.into_inner(), assert_buffer);
  }
}
