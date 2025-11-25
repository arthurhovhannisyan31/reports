use report_parser::{BankRecord, Status, TxType, record_field};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> io::Result<()> {
  let f = File::open("./report_files/records_example.csv")?;
  let reader = BufReader::new(f);

  let mut records: Vec<BankRecord> = vec![];

  let mut lines = reader.lines();

  let first_line = lines.next();
  let headers = first_line.unwrap()?;
  let column_names: Vec<&str> = headers.split(',').collect();

  for str in lines.map_while(Result::ok) {
    let mut bank_record = BankRecord::new();
    let values: Vec<&str> = str.split(',').collect();

    column_names
      .iter()
      .zip(values)
      .for_each(|(&field_name, field_value)| match field_name {
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
      });

    records.push(bank_record);
  }

  println!("{records:#?}");

  Ok(())
}
