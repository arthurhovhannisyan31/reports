# YPBankText Text File Format Specification

## General Information

The YPBankText file format is a text structure used to store transaction records in the YPBank system. Each file
consists of sequential transaction records. The format is designed to be user-friendly and easy to parse
programmatically.

## Description

A YPBank file is a text file containing transaction records. Each record is a block of key-value pairs separated by a
blank line. A record contains the following required fields:

- `TX_ID` – a non-negative integer identifying the transaction.
- `TX_TYPE` – transaction type: `DEPOSIT`, `TRANSFER`, or `WITHDRAWAL`.
- `FROM_USER_ID` – a non-negative integer identifying the invoice sender (use `0` for DEPOSIT).
- `TO_USER_ID` – a non-negative integer identifying the invoice recipient (use `0` for WITHDRAWAL).
- `AMOUNT` – a non-negative integer representing the amount in the smallest unit of currency.
- `TIMESTAMP` – Unix epoch timestamp in milliseconds.
- `STATUS` – transaction status: `SUCCESS`, `FAILURE`, or `PENDING`.
- `DESCRIPTION` – an arbitrary text description, UTF-8, enclosed in double quotes.

Additional:

- Fields may be in any order.
- Each field appears exactly once.
- Transaction records are separated by blank lines.
- The file may contain single-line comments that begin with "#"; these lines are ignored during parsing.

## Examples

Example YPBank file contents:

```plain
# Record 1 (Deposit)
TX_ID: 1234567890123456
TX_TYPE: DEPOSIT
FROM_USER_ID: 0
TO_USER_ID: 9876543210987654
AMOUNT: 10000
TIMESTAMP: 1633036800000
STATUS: SUCCESS
DESCRIPTION: "Terminal deposit"

# Record 2 (Transfer)
TX_ID: 2312321321321321
TIMESTAMP: 1633056800000
STATUS: FAILURE
TX_TYPE: TRANSFER
FROM_USER_ID: 1231231231231231
TO_USER_ID: 9876543210987654
AMOUNT: 1000
DESCRIPTION: "User transfer"

#Record 3 (Withdrawal)
TX_ID: 3213213213213213
AMOUNT: 100
TX_TYPE: WITHDRAWAL
FROM_USER_ID: 9876543210987654
TO_USER_ID: 0
TIMESTAMP: 1633066800000
STATUS: SUCCESS
DESCRIPTION: "User withdrawal"
```