# Vigenère Cipher Implementation in Rust

A high-performance implementation of the Vigenère cipher in Rust, featuring both encryption and decryption capabilities.

## Description

This project implements the classic Vigenère cipher, a polyalphabetic substitution cipher that uses a keyword to encrypt and decrypt messages. The implementation is optimized for performance using unsafe Rust for direct pointer manipulation.

## Features

- Fast encryption and decryption
- Case-insensitive input handling (automatically converts to uppercase)
- Performance metrics (execution time in nanoseconds)
- Preserves non-alphabetic characters
- Command-line interface

## Installation

1. Clone the repository
2. Build the project:
```bash
cargo build --release
```
3. Optional: Create an alias for easier usage:
```bash
alias vigenere=./target/release/vigenere
```

## Usage

The program accepts three arguments:
- Operation mode (`encrypt` or `decrypt`)
- Key (any string)
- Text to process

### Command Format
```bash
vigenere <encrypt|decrypt> <key> <text>
```

### Examples

Encryption:
```bash
vigenere encrypt qwertyuiop TEST
# Output: JAWK
# Time taken: 250 nanoseconds
```

Decryption:
```bash
vigenere decrypt qwertyuiop JAWK
# Output: TEST
# Time taken: 125 nanoseconds
```

## Performance

The implementation is optimized for speed, typically processing operations in under 300 nanoseconds for short strings.

## Technical Details

- Uses unsafe Rust for optimized pointer manipulation
- Handles ASCII uppercase letters (A-Z)
- Preserves non-alphabetic characters in the input