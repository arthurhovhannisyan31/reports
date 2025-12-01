use crate::errors::ConverterErrors;
use clap::{Parser, ValueEnum};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, ValueEnum, Clone)]
pub enum DataFormat {
  Bin,
  Csv,
  Txt,
}

pub(crate) const EXTENSION_WHITELIST: &[&str] = &["bin", "csv", "txt"];

#[derive(Debug, Parser)]
#[command(version, about, next_line_help = true)]
pub struct CliArgs {
  #[arg(short = 'i', long, value_name = "File path", value_parser = path_validation)]
  pub input: PathBuf,
  #[arg(long, value_enum, value_name = "File Format")]
  pub input_format: DataFormat,
  #[arg(long, value_enum, value_name = "File Format")]
  pub output_format: DataFormat,
}

pub fn path_validation(path: &str) -> Result<PathBuf, ConverterErrors> {
  let path =
    PathBuf::from_str(path).expect("Failed reading provided path value");

  if path.exists() {
    if let Some(extension) = path.extension().and_then(OsStr::to_str) {
      if EXTENSION_WHITELIST.contains(&extension) {
        return Ok(path);
      }
    }

    Err(ConverterErrors::InvalidSourceFile)
  } else {
    Err(ConverterErrors::NotFound)
  }
}
