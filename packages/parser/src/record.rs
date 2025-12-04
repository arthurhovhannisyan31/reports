use crate::constants;
use crate::errors::{
  ParsingError, SerializeError, StatusTypeError, TxTypeError,
};
use std::fmt::{Display, Formatter};
use std::io;
use std::io::{BufRead, ErrorKind, Write};
use std::str::FromStr;

#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub struct BankRecord {
  pub tx_id: u64,
  pub tx_type: TxType,
  pub from_user_id: u64,
  pub to_user_id: u64,
  pub amount: u64,
  pub timestamp: u64,
  pub status: Status,
  pub description: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum TxType {
  #[default]
  Deposit = 0,
  Transfer = 1,
  Withdrawal = 2,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum Status {
  #[default]
  Success = 0,
  Failure = 1,
  Pending = 2,
}

impl BankRecord {
  pub fn new() -> Self {
    Self::default()
  }
}

pub trait BankRecordParser {
  fn from_read<R: BufRead>(buffer: &mut R) -> Result<BankRecord, ParsingError>;
  fn write_to<W: Write>(
    &mut self,
    buffer: &mut W,
  ) -> Result<(), SerializeError>;
}

impl FromStr for TxType {
  type Err = TxTypeError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      constants::tx_type::DEPOSIT => Ok(TxType::Deposit),
      constants::tx_type::TRANSFER => Ok(TxType::Transfer),
      constants::tx_type::WITHDRAWAL => Ok(TxType::Withdrawal),
      _ => Err(TxTypeError::InvalidSting(s.to_string())),
    }
  }
}

impl Display for TxType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      TxType::Deposit => write!(f, "{}", constants::tx_type::DEPOSIT),
      TxType::Transfer => write!(f, "{}", constants::tx_type::TRANSFER),
      TxType::Withdrawal => write!(f, "{}", constants::tx_type::WITHDRAWAL),
    }
  }
}

impl TryFrom<u8> for TxType {
  type Error = TxTypeError; // TODO Add enum error for TxType

  fn try_from(v: u8) -> Result<Self, Self::Error> {
    match v {
      0 => Ok(TxType::Deposit),
      1 => Ok(TxType::Transfer),
      2 => Ok(TxType::Withdrawal),
      _ => Err(TxTypeError::NotFound),
    }
  }
}

impl FromStr for Status {
  type Err = io::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      constants::status::SUCCESS => Ok(Status::Success),
      constants::status::FAILURE => Ok(Status::Failure),
      constants::status::PENDING => Ok(Status::Pending),
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
      Status::Success => write!(f, "{}", constants::status::SUCCESS),
      Status::Failure => write!(f, "{}", constants::status::FAILURE),
      Status::Pending => write!(f, "{}", constants::status::PENDING),
    }
  }
}

impl TryFrom<u8> for Status {
  type Error = StatusTypeError;

  fn try_from(v: u8) -> Result<Self, Self::Error> {
    match v {
      0 => Ok(Status::Success),
      1 => Ok(Status::Failure),
      2 => Ok(Status::Pending),
      _ => Err(StatusTypeError::NotFound),
    }
  }
}
