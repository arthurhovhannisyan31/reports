use crate::errors::ComparerError;
use clap::{Parser, ValueEnum};
use std::ffi::OsStr;
use std::io;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, ValueEnum, Clone, PartialEq)]
pub enum DataFormat {
  Bin,
  Csv,
  Txt,
}

pub(crate) const EXTENSION_WHITELIST: &[&str] = &["bin", "csv", "txt"];

#[derive(Debug, Parser)]
#[command(version, about, next_line_help = true)]
pub struct CliArgs {
  #[arg(long, value_name = "File path", value_parser = path_validation)]
  pub file1: PathBuf,
  #[arg(long, value_enum, value_name = "File Format")]
  pub format1: DataFormat,
  #[arg(long, value_name = "File path", value_parser = path_validation)]
  pub file2: PathBuf,
  #[arg(long, value_enum, value_name = "File Format")]
  pub format2: DataFormat,
}

pub fn path_validation(path: &str) -> Result<PathBuf, ComparerError> {
  let path = PathBuf::from_str(path).map_err(|_| {
    ComparerError::IO(io::Error::new(
      ErrorKind::NotFound,
      format!("Failed reading provided file path: {path}"),
    ))
  })?;

  if !path.exists() {
    return Err(ComparerError::NotFound);
  }

  if let Some(extension) = path.extension().and_then(OsStr::to_str) {
    if EXTENSION_WHITELIST.contains(&extension) {
      return Ok(path);
    }
  }

  Err(ComparerError::InvalidSourceFile)
}
