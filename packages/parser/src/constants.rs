pub const RECORD_LINES_NUMBER: usize = 8;

pub mod tx_type {
  pub const DEPOSIT: &str = "DEPOSIT";
  pub const TRANSFER: &str = "TRANSFER";
  pub const WITHDRAWAL: &str = "WITHDRAWAL";
}

pub mod status {
  pub const SUCCESS: &str = "SUCCESS";
  pub const FAILURE: &str = "FAILURE";
  pub const PENDING: &str = "PENDING";
}

pub mod record_field {
  pub const TX_ID: &str = "TX_ID";
  pub const TX_TYPE: &str = "TX_TYPE";
  pub const FROM_USER_ID: &str = "FROM_USER_ID";
  pub const TO_USER_ID: &str = "TO_USER_ID";
  pub const AMOUNT: &str = "AMOUNT";
  pub const TIMESTAMP: &str = "TIMESTAMP";
  pub const STATUS: &str = "STATUS";
  pub const DESCRIPTION: &str = "DESCRIPTION";
}
