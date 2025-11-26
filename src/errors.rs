use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::num::ParseIntError;

// TODO Replace with thiserror
#[derive(Debug)]
pub enum ParsingError {
  IO(io::Error),
  ParseIntError(ParseIntError),
  NotFound, // TODO Delete
}

// TODO Replace with thiserror
#[derive(Debug)]
pub enum SerializeError {
  NotFound,
}

impl Display for ParsingError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::IO(err) => write!(f, "{:?}", err),
      Self::ParseIntError(err) => write!(f, "{:?}", err),
      Self::NotFound => write!(f, "Not found"),
    }
  }
}

impl std::error::Error for ParsingError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Self::IO(err) => Some(err),
      Self::ParseIntError(err) => Some(err),
      Self::NotFound => None,
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
    Self::ParseIntError(err)
  }
}

impl From<String> for ParsingError {
  fn from(value: String) -> Self {
    todo!()
  }
}
