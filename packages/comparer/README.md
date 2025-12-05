<div align="center">
  <h1><code>report comparer</code></h1><sub>Built with 🦀</sub>
</div>

[![main](https://github.com/arthurhovhannisyan31/reports/actions/workflows/code-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/reports/actions/workflows/code-validation.yml)
[![main](https://github.com/arthurhovhannisyan31/reports/actions/workflows/packages-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/reports/actions/workflows/packages-validation.yml)

## Overview

This crate provides simple logic for comparing 2 reports and finding difference in records set.
Currently, only 3 data [formats](./src/configs.rs) supported.
Other data formats can be supported using custom [BankRecordParser](../parser/src/record.rs) crate.
Please see [parser](../parser/README.md) module for details.

## Synopsis

- `--file1 <FILE_PATH>` Path to first report file
- `--format1 <DATA_FORMAT>` First report file data [format](./src/configs.rs)
- `--file2 <FILE_PATH>` Path to second report file
- `--format2 <DATA_FORMAT>` Second report file data [format](./src/configs.rs)


- `-h, --help`  Print help
- `-V, --version`  Print version

## Description

Reports are checked against each other and difference for both reports reported to cli output. You can use cli tools to
catch output and write it to a file or other source.

```shell
  comparer --file1 ./mocks/records_example.bin --format1 bin --file2 ./mocks/records_example.txt --format2 txt
```

```shell
  comparer --file1 ./mocks/records_example.bin --format1 bin --file2 ./mocks/records_example.txt --format2 txt > compare_result.txt
```
