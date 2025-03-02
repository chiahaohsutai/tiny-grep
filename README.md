# Tiny Grep

A super simple command-line utility for searching text within a single file, similar to grep. This tool allows case-sensitive and case-insensitive searches. It supports searching for a specific pattern and case-sensitive and case-insensitive searches.

## Installation

* Clone the repository and build the CLI:
```bash
git clone <repository-url>
cd tiny-grep
cargo build --release
```

## Usage

```bash
./target/release/tiny-grep <pattern> <file>
```

### Example Usage:

1. Case-sensitive search:

```bash
./target/release/tiny-grep "hello" sample.txt
```

This will return all lines containing the exact match for "hello".

2. Case-insensitive search:

```bash
IGNORECASE=1 ./target/release/tiny-grep "hello" sample.txt
```

This will return all lines containing "hello" regardless of case (e.g., "Hello", "HELLO").

## Limitations

* Only supports searching within a single file
* Does not support regex pattern matching (only exact substring search)
