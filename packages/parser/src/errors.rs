use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::num::ParseIntError;
use std::string::FromUtf8Error;

#[derive(Debug)] // try using thiserror::Error for fast error creation
pub enum ParsingError {
  IO(io::Error),
  ParseInt(ParseIntError),
  ParseTxType(TxTypeError),
  ParseStatus(StatusTypeError),
  ParseUtf8(FromUtf8Error),
  ParseBin {
    source: io::Error,
    description: String,
  },
  Custom(String),
}

#[derive(Debug)]
pub enum SerializeError {
  IO(io::Error),
}

#[derive(Debug)]
pub enum TxTypeError {
  InvalidSting(String),
  InvalidNumber(u8),
  NotFound,
}

#[derive(Debug)]
pub enum StatusTypeError {
  InvalidSting(String),
  InvalidNumber(u8),
  NotFound,
}

impl Display for ParsingError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::IO(err) => write!(f, "{:?}", err),
      Self::ParseInt(err) => write!(f, "{:?}", err),
      Self::ParseTxType(err) => write!(f, "{:?}", err),
      Self::ParseStatus(err) => write!(f, "{:?}", err),
      Self::ParseUtf8(err) => write!(f, "{:?}", err),
      Self::ParseBin {
        source,
        description,
      } => {
        write!(f, "{:?}\n {:?}", description, source)
      }
      Self::Custom(str) => write!(f, "{:?}", str),
    }
  }
}

impl std::error::Error for ParsingError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Self::IO(err) => Some(err),
      Self::ParseInt(err) => Some(err),
      Self::ParseTxType(_err) => None,
      Self::ParseStatus(_err) => None,
      Self::ParseUtf8(_err) => None,
      Self::ParseBin {
        source,
        description: _,
      } => Some(source),
      Self::Custom(_str) => None,
    }
  }
}

impl From<io::Error> for ParsingError {
  fn from(err: io::Error) -> Self {
    Self::IO(err)
  }
}

impl From<ParseIntError> for ParsingError {
  fn from(err: ParseIntError) -> Self {
    Self::ParseInt(err)
  }
}

impl From<TxTypeError> for ParsingError {
  fn from(err: TxTypeError) -> Self {
    Self::ParseTxType(err)
  }
}

impl From<StatusTypeError> for ParsingError {
  fn from(err: StatusTypeError) -> Self {
    Self::ParseStatus(err)
  }
}

impl From<FromUtf8Error> for ParsingError {
  fn from(err: FromUtf8Error) -> Self {
    Self::ParseUtf8(err)
  }
}

impl Display for TxTypeError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::InvalidSting(s) => {
        write!(f, "Invalid string transaction type: {:?}", s)
      }
      Self::InvalidNumber(n) => {
        write!(f, "Invalid number transaction type: {:?}", n)
      }
      Self::NotFound => {
        write!(f, "Type option does not exist")
      }
    }
  }
}

impl Display for SerializeError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::IO(err) => write!(f, "{:?}", err),
    }
  }
}

impl std::error::Error for SerializeError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Self::IO(err) => Some(err),
    }
  }
}

impl From<io::Error> for SerializeError {
  fn from(err: io::Error) -> Self {
    Self::IO(err)
  }
}

impl Display for StatusTypeError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::InvalidSting(s) => {
        write!(f, "Invalid string transaction type: {:?}", s)
      }
      Self::InvalidNumber(n) => {
        write!(f, "Invalid number transaction type: {:?}", n)
      }
      Self::NotFound => {
        write!(f, "Type option does not exist")
      }
    }
  }
}
