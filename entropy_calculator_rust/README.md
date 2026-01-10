# Entropy Calculator (Rust)

This program calculates the entropy of one or more files, offering statistics on the information content including entropy per byte, total entropy, file size, compressible bytes, and best theoretical coding ratio, while supporting multiple file analysis.

## How to run

Make sure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs/).

### Build the project

```bash
cd entropy_calculator_rust
cargo build --release
```

The binary will be located at `target/release/entropy_calculator` (or `target/release/entropy_calculator.exe` on Windows).

**Important**: When using `cargo run`, you need to add `--` to separate cargo arguments from your program arguments.

### To run a single file

```bash
cargo run --release -- file1.txt
```

Or using the compiled binary (no `--` needed):

```bash
./target/release/entropy_calculator file1.txt
```

### To run multiple files

```bash
cargo run --release -- file1.txt file2.txt file3.txt
```

Or using the compiled binary:

```bash
./target/release/entropy_calculator file1.txt file2.txt file3.txt
```

### To calculate bit-level informational entropy

#### Single File with --bit flag:

```bash
cargo run --release -- --bit file1.txt
```

Or using the short form:

```bash
cargo run --release -- -b file1.txt
```

#### Multiple Files with --bit flag

```bash
cargo run --release -- --bit file1.txt file2.txt file3.txt
```

## Test Files

Sample test files are included in the project directory for testing:

- `test1.txt` - A sample text file with normal entropy
- `test2.bin` - A binary file with random data (high entropy)
- `test3_low_entropy.txt` - A file with repetitive data (low entropy)
- `test4_multiline.txt` - A multi-line text file

You can test the program with these files:

```bash
cargo run --release -- test1.txt test2.bin test3_low_entropy.txt
```

## Development

### Run in debug mode (faster compilation, slower execution)

```bash
cargo run -- file1.txt
```

### Run tests

```bash
cargo test
```

