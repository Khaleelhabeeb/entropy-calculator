use clap::Parser;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

const BUFFER_SIZE: usize = 256;

#[derive(Parser, Debug)]
#[command(name = "entropy_calculator")]
#[command(version = "0.1.0")]
#[command(about = "Entropy calculator for files, calculates either byte-level or bit-level entropy based on a command-line argument.")]
struct Args {
    /// Calculate bit-level informational entropy
    #[arg(short = 'b', long = "bit")]
    bit_level: bool,

    /// Files to analyze
    files: Vec<String>,
}

fn count_bits_set(byte: u8) -> u8 {
    byte.count_ones() as u8
}

fn calculate_byte_level_entropy(counts: &[u32; 256], total_bytes: u64) -> f64 {
    let mut entropy = 0.0;
    for &count in counts.iter() {
        if count > 0 {
            let prob = count as f64 / total_bytes as f64;
            entropy -= prob * prob.log2();
        }
    }
    entropy
}

fn calculate_bit_level_entropy(counts: &[u32; 256], total_bytes: u64) -> f64 {
    let mut bit_counts = [0u32; 9]; // 0 to 8 bits set
    
    for (byte_value, &count) in counts.iter().enumerate() {
        if count > 0 {
            let bits_set = count_bits_set(byte_value as u8);
            bit_counts[bits_set as usize] += count;
        }
    }

    let mut bit_entropy = 0.0;
    for &bit_count in bit_counts.iter() {
        if bit_count > 0 {
            let prob = bit_count as f64 / total_bytes as f64;
            bit_entropy -= prob * prob.log2();
        }
    }
    bit_entropy
}

fn calculate_entropy(filename: &str, bit_level: bool) {
    let path = Path::new(filename);
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file {}: {}", filename, e);
            return;
        }
    };

    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; BUFFER_SIZE];
    let mut counts = [0u32; 256];
    let mut total_bytes: u64 = 0;

    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(bytes_read) => {
                for i in 0..bytes_read {
                    counts[buffer[i] as usize] += 1;
                }
                total_bytes += bytes_read as u64;
            }
            Err(e) => {
                eprintln!("Error reading file {}: {}", filename, e);
                return;
            }
        }
    }

    if total_bytes == 0 {
        println!("File {} is empty", filename);
        return;
    }

    if bit_level {
        let bit_entropy = calculate_bit_level_entropy(&counts, total_bytes);
        
        println!("\n--- File: {} ---", filename);
        println!("---------------------------------------");
        println!("Bit-level informational entropy: {:.6} bits", bit_entropy);
        println!("---------------------------------------");
    } else {
        let entropy = calculate_byte_level_entropy(&counts, total_bytes);
        let entropy_per_byte = entropy / 8.0;
        let entropy_of_file = entropy * total_bytes as f64;

        println!("\n--- File: {} ---", filename);
        println!("---------------------------------------");
        println!("Entropy per byte              : {:.6} bits ({:.6} bytes)", entropy, entropy_per_byte);
        println!("Entropy of file               : {:.6} bits ({:.6} bytes)", entropy_of_file, entropy_of_file / 8.0);
        println!("Size of file                  : {} bytes", total_bytes);
        println!("Delta                         : {:.6} bytes (compressible theoretically)", total_bytes as f64 - entropy_of_file / 8.0);
        println!("Best Theoretical Coding ratio : {:.6}", 8.0 / entropy);
        println!("---------------------------------------");
    }
    println!();
}

fn main() {
    let args = Args::parse();

    if args.files.is_empty() {
        eprintln!("Error: Please provide one or more files to analyze");
        eprintln!("Usage: entropy_calculator [OPTIONS] <FILE>...");
        std::process::exit(1);
    }

    for file in &args.files {
        calculate_entropy(file, args.bit_level);
    }
}

