<div align="center">
  <h1><code>reports</code></h1><sub>Built with 🦀</sub>
</div>

[![main](https://github.com/arthurhovhannisyan31/reports/actions/workflows/code-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/reports/actions/workflows/code-validation.yml)
[![main](https://github.com/arthurhovhannisyan31/reports/actions/workflows/packages-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/reports/actions/workflows/packages-validation.yml)

## Overview

This is a simple reports parsing crate that provides basic functionality for [parsing](./packages/parser/README.md),
[conversion](./packages/converter/README.md) and [comparison](./packages/comparer/README.md) of reports.
Currently, only 3 type of reports are
supported: [Binary](./docs/YPBankBinFormat_ru.md), [CSV](./docs/YPBankCsvFormat_ru.md)
and [Text](./docs/YPBankTextFromat_ru.md).
In order to provide your own implementation, please see the `BankRecordParser` trait
in [record](./packages/parser/src/record.rs) module.

Would you want to provide your own report format, please see basic report
model [BankRecord](./packages/parser/src/record.rs), and it's serialization and deserialization
trait [BankRecordParser]((./packages/parser/src/record.rs)).

## Description

### Parser

Each report is considered as a set of serialized [BankRecord](./packages/parser/src/record.rs) instances.
The [BankRecordParser]((./packages/parser/src/record.rs)) works with a single record at a time,
hence [BufReader](https://doc.rust-lang.org/std/io/struct.BufReader.html)
and [BufWriter](https://doc.rust-lang.org/std/io/struct.BufWriter.html) should be used to keep track of bytes.

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

## Usage

Please find the latest build binaries in
the [GH Releases](https://github.com/arthurhovhannisyan31/reports/releases).
Download the archived binary for your OS and use the `converter` or `comparer` file from the `target/release` folder.
Make sure the binary has sufficient rights to make file manipulations.

### Converter

The converter is a cli tool that converts a report from one format into another. Currently,
only [3 data formats](./src/configs.rs) are supported.
Pass cli output to a file to save it or some other pipe command to process the result.

```shell
  converter --input ./mocks/records_example.bin --input_format bin --output_format txt > convert_result.txt
```

### Comparer

The comparer is a cli tool that compares 2 different reports and outputs any difference it finds. Pass cli output to a
file to save it or some other pipe command to process the result.

```shell
  comparer --file1 ./mocks/records_example.bin --format1 bin --file2 ./mocks/records_example.txt --format2 txt > compare_result.txt
```

## Stack

- Rust
- Clap

## Credits

Crate implemented as part of the [Yandex practicum](https://practicum.yandex.ru/) course.

## License

Licensed under either of at your option.

* Apache License, Version 2.0, [LICENSE-APACHE](./LICENSE_APACHE) or http://www.apache.org/licenses/LICENSE-2.0
* MIT license [LICENSE-MIT](./LICENSE_MIT) or http://opensource.org/licenses/MIT
