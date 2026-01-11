# Entropy Calculator (Rust) - Advanced Edition

An advanced entropy calculator for files with multiple output formats, recursive directory analysis, parallel processing, and sliding window entropy analysis. This tool calculates Shannon entropy (byte-level or bit-level) and provides comprehensive statistics on information content.

## Features

### Tier 1 Features ✨

- **Multiple Output Formats**: Text (default), JSON, and CSV
- **Recursive Directory Analysis**: Analyze entire directory trees with filtering
- **Progress Bars**: Visual progress indicators for multiple file analysis
- **Parallel Processing**: Multi-threaded analysis for faster processing
- **Sliding Window Entropy**: Analyze entropy changes across file position

### Tier 2 Features 🎨

- **Visualization**: ASCII histograms, frequency charts, and line graphs
- **Statistical Summaries**: Aggregate statistics (min/max/avg, percentiles, std dev)
- **Comparative Analysis**: Compare files side-by-side or against a baseline
- **Configuration Files**: TOML-based configuration for default settings

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

## Visualization Features

### Byte Distribution Histogram

Display an ASCII histogram showing byte distribution across 16 bins:

```bash
./target/release/entropy_calculator --histogram file1.txt
```

Example output:
```
Byte Distribution Histogram: file1.txt
======================================================================
00-0F │█│ 1
10-1F ││ 0
20-2F │████████████████████████│ 15
...
======================================================================
```

### Frequency Chart

Show the top N most frequent bytes in a file:

```bash
./target/release/entropy_calculator --frequency --frequency-top 10 file1.txt
```

Controls the number of top bytes displayed (default: 10).

### Sliding Window Graph

Visualize entropy changes across file position with an ASCII line graph:

```bash
./target/release/entropy_calculator --window 1024 --graph large_file.bin
```

Shows entropy as a function of file position, useful for detecting patterns and anomalies.

## Statistical Summaries

Generate aggregate statistics across multiple files:

```bash
./target/release/entropy_calculator --stats -r --extension .bin /path/to/directory
```

Output includes:
- Minimum, maximum, average entropy
- Standard deviation
- Median
- Percentiles (25th, 75th, 90th, 95th, 99th)
- Total files and bytes analyzed

Example output:
```
======================================================================
Aggregate Statistics
======================================================================
Total files analyzed: 10
Total bytes: 1048576

Entropy Statistics:
  Minimum:     3.234567 bits
  Maximum:     7.891234 bits
  Average:     5.123456 bits
  Std Dev:     1.234567 bits
  Median:      5.000000 bits

Percentiles:
  25th (Q1):   4.500000 bits
  75th (Q3):   5.800000 bits
  90th:        6.500000 bits
  95th:        7.000000 bits
  99th:        7.500000 bits
======================================================================
```

## Comparative Analysis

### Compare Two Files

Compare entropy and size between two files:

```bash
./target/release/entropy_calculator --compare file2.txt file1.txt
```

Shows:
- Entropy comparison (absolute and percentage difference)
- Size comparison (absolute and percentage difference)

### Baseline Comparison

Compare all files against the first file (baseline):

```bash
./target/release/entropy_calculator --baseline baseline.txt file1.txt file2.txt file3.txt
```

Useful for batch comparison to identify files with similar or different entropy patterns.

## Configuration Files

### Generate Configuration Template

Create a configuration template:

```bash
./target/release/entropy_calculator --gen-config config.toml
```

This creates a TOML file with default settings that can be customized.

### Configuration File Format

Example `config.toml`:

```toml
[output]
format = "text"
show_progress = true
threads = 0

[analysis]
bit_level = false
recursive = false
extension = ".txt"  # Optional

[visualization]
show_histogram = true
show_frequency_chart = true
frequency_top_n = 10
```

### Using Configuration Files

```bash
./target/release/entropy_calculator -c config.toml file1.txt
```

Configuration settings can be overridden by command-line arguments. CLI arguments always take precedence.

## Complete Examples

### Example 1: Analyze all text files in a directory (JSON output)

```bash
./target/release/entropy_calculator -r --extension txt --format json /path/to/directory > results.json
```

### Example 2: Sliding window analysis with graph visualization

```bash
./target/release/entropy_calculator --window 512 --graph large_file.bin
```

### Example 3: Batch analysis with CSV export and statistics

```bash
./target/release/entropy_calculator --format csv --stats -r --extension bin /data/binary_files > entropy_report.csv
```

### Example 4: Full analysis with all visualizations

```bash
./target/release/entropy_calculator --histogram --frequency --frequency-top 15 file1.txt
```

### Example 5: Compare files and generate statistics

```bash
./target/release/entropy_calculator --compare file2.bin file1.bin
./target/release/entropy_calculator --stats file1.bin file2.bin file3.bin
```

### Example 6: Using configuration file

```bash
./target/release/entropy_calculator --gen-config myconfig.toml
# Edit myconfig.toml as needed
./target/release/entropy_calculator -c myconfig.toml -r /path/to/directory
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
      --histogram              Show byte distribution histogram (text format only)
      --frequency              Show frequency chart (top N bytes)
      --frequency-top <N>      Number of top bytes in frequency chart [default: 10]
      --graph                  Show sliding window graph (requires --window)
      --stats                  Show aggregate statistics (multiple files)
      --compare <FILE2>        Compare two files (requires exactly one file argument)
      --baseline               Compare all files against first file (baseline mode)
  -c, --config <PATH>          Configuration file path
      --gen-config <PATH>      Generate a sample configuration file
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
├── output.rs        # Output formatting (text, JSON, CSV)
├── visualization.rs # Visualization features (histograms, graphs, charts)
├── statistics.rs    # Statistical summaries and aggregate statistics
├── comparison.rs    # File comparison and baseline analysis
└── config.rs        # Configuration file support (TOML)
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
