# FSP — File Storage Protocol

## Overview

Binary protocol over TCP for file upload, download, listing, and deletion.  
All multi-byte integers are **big-endian (network byte order)**.

---

## Message Structure

Every message (request & response) follows this layout:

```
+--------+---------+----------------+-----------------+
| MAGIC  | VERSION | HEADER         | PAYLOAD         |
| 2 bytes| 1 byte  | variable       | variable        |
+--------+---------+----------------+-----------------+
```

### Magic Number

```
0x46 0x53   ("FS")
```

Used to validate that the incoming data is actually an FSP message.  
If magic doesn't match — drop the connection.

### Version

```
0x01
```

Current protocol version. Allows future backwards-compatible changes.

---

## Request Header

```
+--------+---------+--------+------------------+------------------+
| MAGIC  | VERSION | OPCODE | FILENAME_LEN     | PAYLOAD_LEN      |
| 2 bytes| 1 byte  | 1 byte | 2 bytes          | 8 bytes          |
+--------+---------+--------+------------------+------------------+
```

Total fixed header size: **14 bytes**

### Opcodes

| Opcode | Command    | Description              |
|--------|------------|--------------------------|
| `0x01` | `LIST`     | List all files on server |
| `0x02` | `UPLOAD`   | Upload a file            |
| `0x03` | `DOWNLOAD` | Download a file          |
| `0x04` | `DELETE`   | Delete a file            |

### Fields

- **FILENAME_LEN** (u16): Length of the filename string in bytes. `0` for LIST.
- **PAYLOAD_LEN** (u64): Length of the payload in bytes. `0` for LIST, DOWNLOAD, DELETE.

### Request Payload

```
+--------------------+--------------------+
| FILENAME           | FILE_DATA          |
| FILENAME_LEN bytes | PAYLOAD_LEN bytes  |
+--------------------+--------------------+
```

- **FILENAME**: UTF-8 encoded filename (no path separators allowed).
- **FILE_DATA**: Raw file bytes. Only present in UPLOAD.

### Request Examples

**LIST** — no filename, no payload:
```
46 53 01 01 00 00 00 00 00 00 00 00 00 00
^^^^^ ^^ ^^ ^^^^^ ^^^^^^^^^^^^^^^^^^^^^^^^
magic v  op fn=0  payload=0
```

**UPLOAD** "hello.txt" (9 bytes filename, 13 bytes content "Hello, World!"):
```
46 53 01 02 00 09 00 00 00 00 00 00 00 0D
^^^^^ ^^ ^^ ^^^^^ ^^^^^^^^^^^^^^^^^^^^^^^^
magic v  op fn=9  payload=13

68 65 6C 6C 6F 2E 74 78 74
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ filename: "hello.txt"

48 65 6C 6C 6F 2C 20 57 6F 72 6C 64 21
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ file data: "Hello, World!"
```

**DOWNLOAD** "hello.txt":
```
46 53 01 03 00 09 00 00 00 00 00 00 00 00
^^^^^ ^^ ^^ ^^^^^ ^^^^^^^^^^^^^^^^^^^^^^^^
magic v  op fn=9  payload=0

68 65 6C 6C 6F 2E 74 78 74
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ filename: "hello.txt"
```

---

## Response Header

```
+--------+---------+--------+------------------+
| MAGIC  | VERSION | STATUS | PAYLOAD_LEN      |
| 2 bytes| 1 byte  | 1 byte | 8 bytes          |
+--------+---------+--------+------------------+
```

Total fixed header size: **12 bytes**

### Status Codes

| Status | Meaning          |
|--------|------------------|
| `0x00` | `OK`             |
| `0x01` | `ErrorNotFound`|
| `0x02` | `ERROR_EXISTS`   |
| `0x03` | `ERROR_IO`       |
| `0xFF` | `ERROR_UNKNOWN`  |

### Response Payload

Depends on the original request:

**LIST response** — payload is a list of file entries:
```
+-------------+-------------------------------------------+
| FILE_COUNT  | ENTRIES                                   |
| 4 bytes     | repeated FILE_COUNT times                 |
+-------------+-------------------------------------------+

Each entry:
+-------------+-------------+-------------+
| NAME_LEN    | NAME        | FILE_SIZE   |
| 2 bytes     | N bytes     | 8 bytes     |
+-------------+-------------+-------------+
```

**DOWNLOAD response** — payload is raw file bytes.

**UPLOAD / DELETE response** — payload is empty on OK, or UTF-8 error message on error.

---

## Constraints

- Max filename length: **255 bytes** (UTF-8)
- Max file size: **u64::MAX** (theoretical), practically limited by disk/memory
- Filenames must not contain: `/`, `\`, `..`, null bytes
- One request per connection, or keep-alive with sequential request-response pairs

---

## Flow Diagrams

### Upload
```
Client                          Server
  |                               |
  |--- [UPLOAD header + data] --->|
  |                               |-- save to disk
  |<------ [OK response] ---------|
```

### Download
```
Client                          Server
  |                               |
  |--- [DOWNLOAD header] -------->|
  |                               |-- read from disk
  |<--- [OK + file data] ---------|
```

### List
```
Client                          Server
  |                               |
  |--- [LIST header] ------------>|
  |                               |-- read directory
  |<--- [OK + file list] ---------|
```
