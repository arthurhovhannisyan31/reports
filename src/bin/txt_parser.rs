use report_parser::{BankRecord, Status, TxType, record_field};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

fn main() -> io::Result<()> {
  let f = File::open("./report_files/records_example.txt")?;
  let reader = BufReader::new(f);

  let mut records: Vec<BankRecord> = vec![];
  let mut bank_record = BankRecord::new();

  for str in reader.lines().map_while(Result::ok) {
    if str.starts_with("#") {
      continue;
    }

    if str.is_empty() {
      if bank_record.tx_id != 0 {
        records.push(bank_record.clone());

        bank_record = BankRecord::new();
      }

      continue;
    }

    let parts = str.split(':');
    let mut parts_iter = parts.into_iter();

    let field_name = parts_iter.next().unwrap().trim();
    let field_value = parts_iter.next().unwrap().trim();

    // Collect to a common format:
    // TX_ID|TX_TYPE|FROM_USER_ID|TO_USER_ID|AMOUNT|TIMESTAMP|STATUS|DESCRIPTION and pass to BankRecord::from_read

    match field_name {
      record_field::TX_ID => {
        bank_record.tx_id = field_value.parse::<u64>().unwrap();
      }
      record_field::TX_TYPE => {
        bank_record.tx_type = TxType::from_str(field_value).unwrap();
      }
      record_field::FROM_USER_ID => {
        bank_record.from_user_id = field_value.parse::<u64>().unwrap();
      }
      record_field::TO_USER_ID => {
        bank_record.to_user_id = field_value.parse::<u64>().unwrap();
      }
      record_field::AMOUNT => {
        bank_record.amount = field_value.parse::<i64>().unwrap();
      }
      record_field::TIMESTAMP => {
        bank_record.timestamp = field_value.parse::<u64>().unwrap();
      }
      record_field::STATUS => {
        bank_record.status = Status::from_str(field_value).unwrap();
      }
      record_field::DESCRIPTION => {
        bank_record.description = field_value.replace('"', "");
      }
      _ => (),
    }
  }

  if bank_record.tx_id != 0 {
    records.push(bank_record.clone());
  }

  println!("{records:#?}");

  Ok(())
}
