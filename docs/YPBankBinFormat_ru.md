# YPBankBin Binary Format Specification

## Overview

The YPBankBin binary format is a compact, binary representation of the same transaction data described in the YPBankText
text format.
The file is a sequential stream of records; each record begins with a small header to simplify parsing and verification.

## Record Header

| Offset | Size    | Field         | Description                                                                                                     |
|--------|---------|---------------|-----------------------------------------------------------------------------------------------------------------|
| 0x00   | 4 bytes | `MAGIC`       | Constant value `0x59 0x50 0x42 0x4E` (`'YPBN'`), identifying the record header.                                 |
| 0x04   | 4 bytes | `RECORD_SIZE` | An unsigned 32-bit little-endian integer specifying the number of bytes to follow (i.e., the record body size). |

All multi-byte integers are encoded in big-endian format.

## Record Body (Field Order is Fixed)

| Field          | Size             | Type                                                    | Notes                                                                                     |
|----------------|------------------|---------------------------------------------------------|-------------------------------------------------------------------------------------------|
| `TX_ID`        | 8 bytes          | unsigned 64-bit                                         | Unique transaction identifier.                                                            |
| `TX_TYPE`      | 1 byte           | enumeration (0 = DEPOSIT, 1 = TRANSFER, 2 = WITHDRAWAL) |                                                                                           |
| `FROM_USER_ID` | 8 bytes          | unsigned 64-bit                                         | Sender account; `0` for DEPOSIT.                                                          |
| `TO_USER_ID`   | 8 bytes          | unsigned 64-bit                                         | Recipient account; `0` for WITHDRAWAL.                                                    |
| `AMOUNT`       | 8 bytes          | signed 64-bit                                           | Amount in the smallest monetary unit (cents). Positive for deposits, negative for debits. |
| `TIMESTAMP`    | 8 bytes          | unsigned 64-bit                                         | Transaction execution time in milliseconds from the Unix epoch.                           |
| `STATUS`       | 1 byte           | enumeration (0 = SUCCESS, 1 = FAILURE, 2 = PENDING)     |                                                                                           |
| `DESC_LEN`     | 4 bytes          | unsigned 32-bit                                         | Length of the next description in UTF-8 encoding.                                         |
| `DESCRIPTION`  | `DESC_LEN` bytes | UTF-8                                                   | Optional text description. If no description is present, `DESC_LEN` is set to `0`.        |

No alignment bytes are inserted; fields are arranged contiguously.

## File Structure

A file is a sequence of the following records:

```
[HEADER][BODY][HEADER][BODY]...
```

The presence of the `MAGIC` value at the beginning of each record allows the reader to resynchronize in the event of a
lost record boundary or data corruption.