<div align="center">
  <h1><code>report converter</code></h1><sub>Built with 🦀</sub>
</div>

[![main](https://github.com/arthurhovhannisyan31/reports/actions/workflows/code-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/reports/actions/workflows/code-validation.yml)
[![main](https://github.com/arthurhovhannisyan31/reports/actions/workflows/packages-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/reports/actions/workflows/packages-validation.yml)

## Overview

This crate provides simple logic for reports conversion from one data formats into others.
Currently, only 3 data [formats](./src/configs.rs) supported.
Other data formats can be supported using custom [BankRecordParser](../parser/src/record.rs) crate.
Please see [parser](../parser/README.md) module for details.

## Synopsis

- `-i, --input <FILE_PATH>` Path to report file
- `--input_format <DATA_FORMAT>` Input report file data [format](./src/configs.rs)
- `--output_format <DATA_FORMAT>` Output report file data [format](./src/configs.rs)


- `--help`  Print help
- `-V, --version`  Print version

## Description

Input file records are parsed and reported to cli output according to selected `output format`. You can use cli tools to
catch output and write it to a file or other source.

```shell
  converter --input ./mocks/records_example.bin --input_format bin --output_format txt
```

```shell
  converter --input ./mocks/records_example.bin --input_format bin --output_format txt > convert_result.txt
```
