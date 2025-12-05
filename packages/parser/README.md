<div align="center">
  <h1><code>report parser</code></h1><sub>Built with 🦀</sub>
</div>

[![main](https://github.com/arthurhovhannisyan31/reports/actions/workflows/code-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/reports/actions/workflows/code-validation.yml)
[![main](https://github.com/arthurhovhannisyan31/reports/actions/workflows/packages-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/reports/actions/workflows/packages-validation.yml)

## Overview

This is a core crate that hold logic for serialization and deserialization of reports.
Report parsers support any source of data which implements [Read](https://doc.rust-lang.org/std/io/trait.Read.html)
trait and serializes to sources that implement [Write](https://doc.rust-lang.org/std/io/trait.Write.html) trait.
Parsers should be used with [BufReader](https://doc.rust-lang.org/std/io/struct.BufReader.html)
and [BufWriter](https://doc.rust-lang.org/std/io/struct.BufWriter.html) to keep track of the bytes.

## Description

[BankRecord](./src/record.rs) is the core data model that describes a set of records in a report.
[BankRecordParser](./src/record.rs) trait provides basic logic for serializing and deserializing a single `BankRecord`
record.

Please see [Binary](./src/parsers/bin.rs), [Csv](./src/parsers/csv.rs) and [Text](./src/parsers/txt.rs) records
implementation for details. Custom reports support can be implemented using [BankRecordParser](./src/record.rs) trait.

Different types of reports have different structure, like multi-line [Text](./src/parsers/txt.rs) records, single
line [Csv](./src/parsers/csv.rs) records and byte mask layout [Binary](./src/parsers/bin.rs) records.
Some parser implementation support scan logic which helps to find the right spot from where the parsing should be
started. It helps skip junk data and parses broken files.

```rust
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

use parser::errors::ParsingError;
use parser::parsers::BinRecord;
use parser::record::BankRecordParser;

fn main() -> Result<(), ParsingError> {
  let f = File::open("records.bin")?;
  let mut reader = BufReader::new(f);
  let mut writer = BufWriter::new(File::create("records_output.bin")?);

  while let Ok(record) = BinRecord::from_read(&mut reader) {
    println!("{:#?}", record);

    let _ = BinRecord(record).write_to(&mut writer);
  }

  writer.flush()?;

  Ok(())
}
```