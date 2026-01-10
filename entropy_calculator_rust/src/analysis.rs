use crate::entropy::{BUFFER_SIZE, calculate_byte_level_entropy, calculate_bit_level_entropy};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct FileAnalysis {
    pub filename: PathBuf,
    pub size_bytes: u64,
    pub byte_entropy: Option<f64>,
    pub bit_entropy: Option<f64>,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SlidingWindowResult {
    pub position: u64,
    pub chunk_size: u64,
    pub entropy: f64,
}

pub fn analyze_file(path: &Path, bit_level: bool) -> FileAnalysis {
    let filename = path.to_path_buf();
    
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            return FileAnalysis {
                filename,
                size_bytes: 0,
                byte_entropy: None,
                bit_entropy: None,
                error: Some(format!("Error opening file: {}", e)),
            };
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
                return FileAnalysis {
                    filename,
                    size_bytes: total_bytes,
                    byte_entropy: None,
                    bit_entropy: None,
                    error: Some(format!("Error reading file: {}", e)),
                };
            }
        }
    }

    if total_bytes == 0 {
        return FileAnalysis {
            filename,
            size_bytes: 0,
            byte_entropy: None,
            bit_entropy: None,
            error: Some("File is empty".to_string()),
        };
    }

    if bit_level {
        let bit_entropy = calculate_bit_level_entropy(&counts, total_bytes);
        FileAnalysis {
            filename,
            size_bytes: total_bytes,
            byte_entropy: None,
            bit_entropy: Some(bit_entropy),
            error: None,
        }
    } else {
        let byte_entropy = calculate_byte_level_entropy(&counts, total_bytes);
        FileAnalysis {
            filename,
            size_bytes: total_bytes,
            byte_entropy: Some(byte_entropy),
            bit_entropy: None,
            error: None,
        }
    }
}

pub fn analyze_file_sliding_window(
    path: &Path,
    window_size: u64,
) -> Result<Vec<SlidingWindowResult>, String> {
    if window_size == 0 {
        return Err("Window size must be greater than 0".to_string());
    }

    // Read entire file into memory for sliding window analysis
    // For very large files, we could use a different approach, but this is simpler
    let data = std::fs::read(path)
        .map_err(|e| format!("Error reading file: {}", e))?;

    if data.is_empty() {
        return Err("File is empty".to_string());
    }

    let mut results = Vec::new();
    let window_size_usize = window_size as usize;

    // Non-overlapping windows
    let mut position = 0u64;
    let mut start = 0;

    while start + window_size_usize <= data.len() {
        let window = &data[start..start + window_size_usize];
        let mut counts = [0u32; 256];
        
        for &byte in window {
            counts[byte as usize] += 1;
        }
        
        let entropy = calculate_byte_level_entropy(&counts, window_size);
        results.push(SlidingWindowResult {
            position,
            chunk_size: window_size,
            entropy,
        });
        
        position += window_size;
        start += window_size_usize;
    }

    // Process remaining bytes if any
    if start < data.len() {
        let remaining = &data[start..];
        let actual_size = remaining.len() as u64;
        let mut counts = [0u32; 256];
        
        for &byte in remaining {
            counts[byte as usize] += 1;
        }
        
        if actual_size > 0 {
            let entropy = calculate_byte_level_entropy(&counts, actual_size);
            results.push(SlidingWindowResult {
                position,
                chunk_size: actual_size,
                entropy,
            });
        }
    }

    Ok(results)
}

