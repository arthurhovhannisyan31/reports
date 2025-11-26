use crate::errors::{ParsingError, SerializeError};
use crate::{BankRecord, BankRecordParser, Status, TxType};
use std::io;
use std::io::{ErrorKind, Read, Write};

pub struct BinReportParser;

static RECORD_HEADER: &[u8; 4] = b"YPBN";

impl BankRecordParser for BinReportParser {
  fn from_read<R: Read>(
    reader: &mut R,
  ) -> Result<Vec<BankRecord>, ParsingError> {
    let mut records: Vec<BankRecord> = vec![];

    loop {
      let mut record_header_buf = [0u8; 4];
      if let Err(err) = reader.read_exact(&mut record_header_buf) {
        // EOF
        break;
      }

      assert_eq!(record_header_buf, *RECORD_HEADER);

      if record_header_buf != *RECORD_HEADER {
        return Err(ParsingError::IO(io::Error::new(
          ErrorKind::InvalidData,
          "Wrong record header",
        )));
      }

      reader.read_exact(&mut record_header_buf)?;
      let record_size = u32::from_be_bytes(record_header_buf);

      let mut record_body_buf = vec![0; record_size as usize];
      reader.read_exact(&mut record_body_buf)?;

      let tx_id: &[u8; 8] = &record_body_buf[0..8]
        .iter()
        .as_slice()
        .try_into()
        .expect("Failed parsing to slice");

      let tx_type: &[u8; 1] = &record_body_buf[8..9]
        .iter()
        .as_slice()
        .try_into()
        .expect("Failed parsing to slice");

      let from_user_id: &[u8; 8] = &record_body_buf[9..17]
        .iter()
        .as_slice()
        .try_into()
        .expect("Failed parsing to slice");

      let to_user_id: &[u8; 8] = &record_body_buf[17..25]
        .iter()
        .as_slice()
        .try_into()
        .expect("Failed parsing to slice");

      let amount: &[u8; 8] = &record_body_buf[25..33]
        .iter()
        .as_slice()
        .try_into()
        .expect("Failed parsing to slice");

      let timestamp: &[u8; 8] = &record_body_buf[33..41]
        .iter()
        .as_slice()
        .try_into()
        .expect("Failed parsing to slice");

      let status: &[u8; 1] = &record_body_buf[41..42]
        .iter()
        .as_slice()
        .try_into()
        .expect("Failed parsing to slice");

      let description: String =
        String::from_utf8(record_body_buf[46..].to_vec()).unwrap();

      records.push(BankRecord {
        tx_id: u64::from_be_bytes(*tx_id),
        tx_type: TxType::try_from(u8::from_be_bytes(*tx_type)).unwrap(),
        from_user_id: u64::from_be_bytes(*from_user_id),
        to_user_id: u64::from_be_bytes(*to_user_id),
        amount: i64::from_be_bytes(*amount),
        timestamp: u64::from_be_bytes(*timestamp),
        status: Status::try_from(u8::from_be_bytes(*status)).unwrap(),
        description,
      })
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
