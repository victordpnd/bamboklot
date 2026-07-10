# Log Parser

A lightweight Rust console application for parsing application log files.

## Overview

Log Parser reads log entries, extracts severity levels, and generates a simple report.

The project demonstrates Rust modules, structs, vectors, enums, ownership, and formatted output.

## Features

- Parse log entries
- Count INFO/WARNING/ERROR messages
- Display formatted report
- Simple parser implementation
- Modular project structure

## Project Structure

```
.
├── Cargo.toml
├── README.md
└── src
    ├── main.rs
    ├── parser.rs
    └── log.rs
```

## Example Output

```
===== LOG REPORT =====

INFO: 2
WARNING: 1
ERROR: 2

Total entries: 5
```

## Technologies

- Rust
- Cargo
- Git
- GitHub

## Future Improvements

- Read logs from file
- JSON export
- Search by keyword
- Timestamp parsing
- Colored terminal output

