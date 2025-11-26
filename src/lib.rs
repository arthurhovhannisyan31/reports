pub mod errors;
pub mod parsers;

use crate::errors::{ParsingError, SerializeError};
use std::fmt::{Display, Formatter};
use std::io;
use std::io::{BufRead, ErrorKind, Write};
use std::str::FromStr;

pub static RECORD_LINES_NUMBER: usize = 8;

// TODO Delete clone
#[derive(Debug, Default, Clone)]
pub enum TxType {
  #[default]
  Deposit,
  Transfer,
  Withdrawal,
}

// TODO Delete clone
#[derive(Debug, Default, Clone)]
pub enum Status {
  #[default]
  Success,
  Failure,
  Pending,
}

#[derive(Debug, Default, Clone)]
pub struct BankRecord {
  pub tx_id: u64,
  pub tx_type: TxType,
  pub from_user_id: u64,
  pub to_user_id: u64,
  pub amount: i64,
  pub timestamp: u64,
  pub status: Status,
  pub description: String,
}

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

impl FromStr for TxType {
  type Err = String; // TODO Replace with proper type

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      tx_type::DEPOSIT => Ok(TxType::Deposit),
      tx_type::TRANSFER => Ok(TxType::Transfer),
      tx_type::WITHDRAWAL => Ok(TxType::Withdrawal),
      _ => Err(format!("Unknown type: {}", s)),
    }
  }
}

impl Display for TxType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      TxType::Deposit => write!(f, "{}", tx_type::DEPOSIT),
      TxType::Transfer => write!(f, "{}", tx_type::TRANSFER),
      TxType::Withdrawal => write!(f, "{}", tx_type::WITHDRAWAL),
    }
  }
}

impl TryFrom<u8> for TxType {
  type Error = (); // TODO Add enum error for TxType

  fn try_from(v: u8) -> Result<Self, Self::Error> {
    match v {
      0 => Ok(TxType::Deposit),
      1 => Ok(TxType::Transfer),
      2 => Ok(TxType::Withdrawal),
      _ => Err(()),
    }
  }
}

impl FromStr for Status {
  type Err = io::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      status::SUCCESS => Ok(Status::Success),
      status::FAILURE => Ok(Status::Failure),
      status::PENDING => Ok(Status::Pending),
      err => Err(io::Error::new(
        ErrorKind::InvalidData,
        format!("Failed parsing status from {s}: {err}"),
      )),
    }
  }
}

impl Display for Status {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Status::Success => write!(f, "{}", status::SUCCESS),
      Status::Failure => write!(f, "{}", status::FAILURE),
      Status::Pending => write!(f, "{}", status::PENDING),
    }
  }
}

impl TryFrom<u8> for Status {
  type Error = (); // TODO Add enum error for Status

  fn try_from(v: u8) -> Result<Self, Self::Error> {
    match v {
      0 => Ok(Status::Success),
      1 => Ok(Status::Failure),
      2 => Ok(Status::Pending),
      _ => Err(()),
    }
  }
}

impl BankRecord {
  pub fn new() -> Self {
    Self::default()
  }
}

pub trait BankRecordParser {
  fn from_read<R: BufRead>(_r: &mut R)
  -> Result<Vec<BankRecord>, ParsingError>;
  fn write_to<W: Write>(
    &mut self,
    _writer: &mut W,
  ) -> Result<(), SerializeError>;
}
