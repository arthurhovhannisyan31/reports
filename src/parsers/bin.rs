use crate::errors::{ParsingError, SerializeError};
use crate::record::{BankRecord, BankRecordParser, Status, TxType};
use std::io::{IoSliceMut, Read, Write};

pub struct BinReportParser;

static RECORD_HEADER: &[u8; 4] = b"YPBN";

impl BankRecordParser for BinReportParser {
  fn from_read<R: Read>(
    reader: &mut R,
  ) -> Result<Vec<BankRecord>, ParsingError> {
    let mut records: Vec<BankRecord> = vec![];

    loop {
      let mut record_header_buf = [0u8; 4];
      if reader.read_exact(&mut record_header_buf).is_err() {
        // EOF
        break;
      }

      if record_header_buf != *RECORD_HEADER {
        // RECORD_HEADER is lost, moving on with 1 byte step
        let mut step = [0u8; 1];
        reader.read_exact(&mut step)?;

        continue;
      }

      reader.read_exact(&mut record_header_buf)?;

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
      let read = reader.read_vectored(&mut bufs)?;

      // Description buffer size needs to be calculated based on number bytes read
      // It allows to move cursor to the end of record section
      let mut desc_buf = vec![0u8; (record_size - read as u32) as usize];
      reader.read_exact(&mut desc_buf)?;

      records.push(BankRecord {
        tx_id: u64::from_be_bytes(tx_id),
        tx_type: TxType::try_from(u8::from_be_bytes(tx_type))?,
        from_user_id: u64::from_be_bytes(from_user_id),
        to_user_id: u64::from_be_bytes(to_user_id),
        amount: i64::from_be_bytes(amount),
        timestamp: u64::from_be_bytes(timestamp),
        status: Status::try_from(u8::from_be_bytes(status)).expect("aaa"),
        description: "".to_string(),
      });
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

// #[cfg(test)]
// mod bin_parser_test{
//   #[test]
//   fn parses_valid_input(){
//
//   }
// }
