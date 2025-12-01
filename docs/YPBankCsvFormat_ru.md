# YPBank CSV Format Specification

## Overview

The format file is a comma-separated value (`CSV`) text file designed to store transaction data. The file has a strict
structure: a mandatory header line and subsequent lines, each representing a single transaction.

## File Structure

### Encoding

The file must be encoded in `UTF-8`.

### Header

The first line of the file must always contain a header with field names. The header must exactly match the following
line:

```
TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
```

### Data Records

Each line after the header represents a single transaction. Fields within a line are separated by commas. Empty lines in
the file are ignored by the parser.

## Field Descriptions

| Field Name     | Data Type          | Description                                                                                                                           |
|----------------|--------------------|---------------------------------------------------------------------------------------------------------------------------------------|
| `TX_ID`        | `integer (64-bit)` | Unique transaction identifier.                                                                                                        |
| `TX_TYPE`      | `string`           | Transaction type. Possible values: `DEPOSIT`, `TRANSFER`, `WITHDRAWAL`.                                                               |
| `FROM_USER_ID` | `integer (64-bit)` | Sending user ID. For system deposits (`DEPOSIT`), this value can be `0`.                                                              |
| `TO_USER_ID`   | `integer (64-bit)` | Receiving user ID. For system debits (`WITHDRAWAL`), this value can be `0`.                                                           |
| `AMOUNT`       | `integer (64-bit)` | Transaction amount in the smallest unit of currency (e.g., cents).                                                                    |
| `TIMESTAMP`    | `integer (64-bit)` | Transaction time in Unix time (milliseconds since the epoch).                                                                         |
| `STATUS`       | `string`           | Transaction status. Possible values: `SUCCESS`, `FAILURE`, `PENDING`.                                                                 |
| `DESCRIPTION`  | `string`           | Text description of the transaction. This field is the last field in the line and is always enclosed in double quotation marks (`"`). |

## Example

```csv
TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
1001,DEPOSIT,0,501,50000,1672531200000,SUCCESS,"Initial account funding"
1002,TRANSFER,501,502,15000,1672534800000,FAILURE,"Payment for services, invoice #123"
1003,WITHDRAWAL,502,0,1000,1672538400000,PENDING,"ATM withdrawal"
```