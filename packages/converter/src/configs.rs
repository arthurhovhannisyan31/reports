use clap::{Parser, ValueEnum};
use std::io;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, ValueEnum, Clone)]
pub enum DataFormat {
  Binary,
  Csv,
  Text,
}

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

fn path_validation(path: &str) -> Result<PathBuf, io::Error> {
  let path =
    PathBuf::from_str(path).expect("Failed reading provided path value");

  if path.exists() {
    Ok(path)
  } else {
    Err(io::Error::new(ErrorKind::NotFound, "File path not found"))
  }
}
