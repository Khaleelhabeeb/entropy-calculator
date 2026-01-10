# Entropy Calculator (Rust) - Advanced Edition

An advanced entropy calculator for files with multiple output formats, recursive directory analysis, parallel processing, and sliding window entropy analysis. This tool calculates Shannon entropy (byte-level or bit-level) and provides comprehensive statistics on information content.

## Features

### Tier 1 Features ✨

- **Multiple Output Formats**: Text (default), JSON, and CSV
- **Recursive Directory Analysis**: Analyze entire directory trees with filtering
- **Progress Bars**: Visual progress indicators for multiple file analysis
- **Parallel Processing**: Multi-threaded analysis for faster processing
- **Sliding Window Entropy**: Analyze entropy changes across file position

### Core Features

- Byte-level entropy calculation (Shannon entropy)
- Bit-level entropy calculation
- Multiple file support
- Detailed statistics (entropy per byte, total entropy, compression ratio, etc.)

## Installation

Make sure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs/).

### Build the project

```bash
cd entropy_calculator_rust
cargo build --release
```

The binary will be located at `target/release/entropy_calculator` (or `target/release/entropy_calculator.exe` on Windows).

## Usage

**Important**: When using `cargo run`, you need to add `--` to separate cargo arguments from your program arguments.

### Basic Usage

#### Single File

```bash
./target/release/entropy_calculator file1.txt
```

#### Multiple Files

```bash
./target/release/entropy_calculator file1.txt file2.txt file3.txt
```

### Output Formats

#### Text Format (Default)

```bash
./target/release/entropy_calculator file1.txt
```

Outputs human-readable text with formatted tables.

#### JSON Format

```bash
./target/release/entropy_calculator --format json file1.txt
```

Outputs structured JSON data, perfect for scripting and automation:

```json
[
  {
    "filename": "file1.txt",
    "size_bytes": 80,
    "byte_entropy_per_byte": 4.143381,
    "byte_entropy_of_file": 331.470466,
    "bit_entropy": null,
    "entropy_per_byte_bytes": 0.517923,
    "delta_compressible_bytes": 38.566192,
    "best_theoretical_coding_ratio": 1.930790,
    "error": null
  }
]
```

#### CSV Format

```bash
./target/release/entropy_calculator --format csv file1.txt file2.txt > results.csv
```

Outputs comma-separated values, perfect for spreadsheet import and data analysis.

### Recursive Directory Analysis

#### Analyze All Files in a Directory

```bash
./target/release/entropy_calculator -r /path/to/directory
```

#### Filter by File Extension

```bash
# Analyze only .txt files recursively
./target/release/entropy_calculator -r --extension .txt /path/to/directory

# Analyze only .bin files
./target/release/entropy_calculator -r --extension bin /path/to/directory
```

The extension filter works with or without the leading dot.

### Bit-Level Entropy

Calculate bit-level informational entropy instead of byte-level:

```bash
./target/release/entropy_calculator --bit file1.txt
# or short form
./target/release/entropy_calculator -b file1.txt
```

### Sliding Window Analysis

Analyze entropy changes across file position using sliding windows:

```bash
./target/release/entropy_calculator --window 1024 large_file.bin
```

This will:
- Divide the file into chunks of the specified size (in bytes)
- Calculate entropy for each chunk
- Show position, chunk size, and entropy for each window
- Display statistics (min, max, average entropy)

Example output:
```
--- Sliding Window Entropy Analysis: large_file.bin ---
Window size: 1024 bytes
Number of windows: 256

Position (bytes)	Window Size	Entropy (bits)
------------------------------------------------------------
0		1024		7.823456
1024		1024		7.912345
2048		1024		7.789012
...

Statistics:
  Minimum entropy: 7.234567 bits
  Maximum entropy: 7.987654 bits
  Average entropy: 7.812345 bits
```

**Note**: Sliding window analysis works on a single file. If multiple files are provided, only the first file will be analyzed.

### Parallel Processing

By default, the tool uses all available CPU cores for parallel processing. You can specify the number of threads:

```bash
# Use 4 threads
./target/release/entropy_calculator --threads 4 file1.txt file2.txt file3.txt

# Use auto-detection (default)
./target/release/entropy_calculator --threads 0 -r /path/to/dir
```

### Progress Bars

Progress bars are automatically shown when analyzing multiple files. To disable:

```bash
./target/release/entropy_calculator --no-progress file1.txt file2.txt
```

## Complete Examples

### Example 1: Analyze all text files in a directory (JSON output)

```bash
./target/release/entropy_calculator -r --extension txt --format json /path/to/directory > results.json
```

### Example 2: Sliding window analysis with bit-level entropy

```bash
./target/release/entropy_calculator --bit --window 512 encrypted_file.bin
```

### Example 3: Batch analysis with CSV export

```bash
./target/release/entropy_calculator --format csv -r --extension bin /data/binary_files > entropy_report.csv
```

### Example 4: Compare entropy across multiple files

```bash
./target/release/entropy_calculator --format csv file1.bin file2.bin file3.bin | sort -t, -k3 -nr
```

## Test Files

Sample test files are included in the project directory:

- `test1.txt` - A sample text file with normal entropy
- `test2.bin` - A binary file with random data (high entropy)
- `test3_low_entropy.txt` - A file with repetitive data (low entropy)
- `test4_multiline.txt` - A multi-line text file

You can test the program with these files:

```bash
./target/release/entropy_calculator test1.txt test2.bin test3_low_entropy.txt
```

## Command-Line Options

```
Options:
  -b, --bit                    Calculate bit-level informational entropy
  -f, --format <FORMAT>        Output format [default: text] [possible values: text, json, csv]
  -r, --recursive              Analyze directories recursively
  -w, --window <SIZE>          Sliding window analysis (entropy per chunk)
  -e, --extension <EXTENSION>  Filter files by extension (e.g., ".txt", ".bin")
      --no-progress            Hide progress bars (by default progress bars are shown)
      --threads <THREADS>      Parallel processing (number of threads, 0 = auto) [default: 0]
  -h, --help                   Print help
  -V, --version                Print version
```

## Output Fields

### Byte-Level Entropy Output

- **Entropy per byte**: Shannon entropy in bits (and bytes)
- **Entropy of file**: Total entropy of the entire file
- **Size of file**: File size in bytes
- **Delta**: Theoretically compressible bytes (size - entropy)
- **Best Theoretical Coding ratio**: Optimal compression ratio (8 / entropy)

### Bit-Level Entropy Output

- **Bit-level informational entropy**: Entropy based on bit distribution

### JSON/CSV Fields

- `filename`: Path to the analyzed file
- `size_bytes`: File size in bytes
- `byte_entropy_per_byte`: Shannon entropy per byte (bits)
- `byte_entropy_of_file`: Total entropy of file (bits)
- `bit_entropy`: Bit-level entropy (if `--bit` flag used)
- `entropy_per_byte_bytes`: Entropy per byte in bytes
- `delta_compressible_bytes`: Theoretically compressible bytes
- `best_theoretical_coding_ratio`: Optimal compression ratio
- `error`: Error message if analysis failed

## Performance

The tool is optimized for performance:

- **Parallel Processing**: Automatically uses all CPU cores
- **Buffered I/O**: Efficient file reading with buffering
- **Memory Efficient**: Streams file data instead of loading entire files (except for sliding window)
- **Progress Tracking**: Real-time progress bars for batch operations

For very large files or directory trees, consider:
- Using `--threads` to limit CPU usage
- Filtering by extension to reduce the number of files analyzed
- Redirecting output to a file for better performance

## Development

### Run in debug mode (faster compilation, slower execution)

```bash
cargo run -- file1.txt
```

### Run tests

```bash
cargo test
```

### Code Structure

```
src/
├── main.rs          # CLI interface and argument parsing
├── lib.rs           # Library interface
├── entropy.rs       # Entropy calculation functions
├── analysis.rs      # File analysis logic
└── output.rs        # Output formatting (text, JSON, CSV)
```

## License

This project is part of the entropy-calculator collection.

## Contributing

Contributions are welcome! This is an advanced implementation with Tier 1 features implemented. Future enhancements could include:

- Additional entropy metrics (Rényi, Tsallis)
- Visualization features
- Compression testing
- Security-focused analysis
- Interactive TUI mode
