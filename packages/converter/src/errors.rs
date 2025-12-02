use parser::errors::SerializeError;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io;

#[derive(Debug)]
pub enum ConverterErrors {
  IO(io::Error),
  InvalidSourceFile,
  NotFound,
}

impl Display for ConverterErrors {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::IO(err) => write!(f, "{:?}", err),
      Self::InvalidSourceFile => {
        write!(
          f,
          "Only following file types are supported: {:?}",
          crate::configs::EXTENSION_WHITELIST
        )
      }
      Self::NotFound => {
        write!(f, "File not found",)
      }
    }
  }
}

impl Error for ConverterErrors {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      Self::IO(err) => Some(err),
      Self::NotFound => None,
      Self::InvalidSourceFile => None,
    }
  }
}

impl From<io::Error> for ConverterErrors {
  fn from(err: io::Error) -> Self {
    Self::IO(err)
  }
}

impl From<SerializeError> for ConverterErrors {
  fn from(value: SerializeError) -> Self {
    match value {
      SerializeError::IO(err) => Self::IO(err),
    }
  }
}
