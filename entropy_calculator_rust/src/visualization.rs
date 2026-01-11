use crate::analysis::{FileAnalysis, SlidingWindowResult};
use std::cmp;
use std::io::Write;

const HISTOGRAM_WIDTH: usize = 60;
const HISTOGRAM_HEIGHT: usize = 20;

pub fn print_byte_distribution_histogram(analysis: &FileAnalysis, writer: &mut impl Write) -> std::io::Result<()> {
    if analysis.error.is_some() || analysis.byte_entropy.is_none() {
        return Ok(());
    }

    // Read file and count bytes
    use crate::entropy::BUFFER_SIZE;
    use std::fs::File;
    use std::io::{BufReader, Read};
    
    let file = File::open(&analysis.filename)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("Error opening file: {}", e)))?;
    
    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; BUFFER_SIZE];
    let mut counts = [0u32; 256];
    
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(bytes_read) => {
                for i in 0..bytes_read {
                    counts[buffer[i] as usize] += 1;
                }
            }
            Err(e) => {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Error reading file: {}", e)));
            }
        }
    }

    writeln!(writer, "\nByte Distribution Histogram: {}", analysis.filename.display())?;
    writeln!(writer, "{}", "=".repeat(70))?;
    
    // Group into bins for readability (16 bins of 16 bytes each)
    let num_bins = 16;
    let bin_size = 256 / num_bins;
    let mut bin_counts = vec![0u32; num_bins];
    
    for (i, &count) in counts.iter().enumerate() {
        bin_counts[i / bin_size] += count;
    }
    
    let max_bin_count = bin_counts.iter().max().copied().unwrap_or(1) as f64;
    
    for (bin_idx, &bin_count) in bin_counts.iter().enumerate() {
        let bar_length = if max_bin_count > 0.0 {
            ((bin_count as f64 / max_bin_count) * HISTOGRAM_WIDTH as f64) as usize
        } else {
            0
        };
        
        let bar = "█".repeat(bar_length);
        let range_start = bin_idx * bin_size;
        let range_end = cmp::min(range_start + bin_size - 1, 255);
        
        writeln!(
            writer,
            "{:02X}-{:02X} │{}│ {}",
            range_start, range_end, bar, bin_count
        )?;
    }
    
    writeln!(writer, "{}", "=".repeat(70))?;
    Ok(())
}

pub fn print_sliding_window_graph(results: &[SlidingWindowResult], writer: &mut impl Write) -> std::io::Result<()> {
    if results.is_empty() {
        return Ok(());
    }

    let min_entropy = results.iter().map(|r| r.entropy).fold(f64::INFINITY, f64::min);
    let max_entropy = results.iter().map(|r| r.entropy).fold(0.0, f64::max);
    let range = max_entropy - min_entropy;
    
    if range <= 0.0 {
        writeln!(writer, "\nEntropy Graph (constant value: {:.6})", min_entropy)?;
        writeln!(writer, "{}", "─".repeat(HISTOGRAM_WIDTH + 10))?;
        return Ok(());
    }

    writeln!(writer, "\nSliding Window Entropy Graph")?;
    writeln!(writer, "{}", "=".repeat(70))?;
    writeln!(writer, "Min: {:.6} bits, Max: {:.6} bits, Range: {:.6} bits", min_entropy, max_entropy, range)?;
    writeln!(writer, "{}", "─".repeat(70))?;
    
    // Create a grid for the graph
    let grid_height = HISTOGRAM_HEIGHT;
    let grid_width = HISTOGRAM_WIDTH;
    
    // Sample points if we have too many
    let step = if results.len() > grid_width {
        results.len() / grid_width
    } else {
        1
    };
    
    let sampled: Vec<_> = results.iter().step_by(step).collect();
    
    // Build the graph
    for row in (0..grid_height).rev() {
        let threshold = min_entropy + (range * row as f64 / grid_height as f64);
        let next_threshold = if row > 0 {
            min_entropy + (range * (row - 1) as f64 / grid_height as f64)
        } else {
            max_entropy
        };
        
        // Print y-axis label
        if row % 5 == 0 || row == grid_height - 1 {
            write!(writer, "{:6.2} │", next_threshold)?;
        } else {
            write!(writer, "       │")?;
        }
        
        // Print data points
        for result in &sampled {
            if result.entropy >= threshold && result.entropy < next_threshold {
                write!(writer, "█")?;
            } else if (result.entropy - threshold).abs() < range / grid_height as f64 {
                write!(writer, "▌")?;
            } else {
                write!(writer, " ")?;
            }
        }
        
        writeln!(writer)?;
    }
    
    // Print x-axis
    write!(writer, "       └")?;
    for _ in 0..sampled.len() {
        write!(writer, "─")?;
    }
    writeln!(writer)?;
    
    // Print position markers
    if !sampled.is_empty() {
        write!(writer, "        0")?;
        if sampled.len() > 10 {
            let mid = sampled.len() / 2;
            for _ in 0..(mid - 3) {
                write!(writer, " ")?;
            }
            write!(writer, "{}", sampled[mid].position)?;
            for _ in 0..(sampled.len() - mid - 5) {
                write!(writer, " ")?;
            }
            if sampled.len() > 5 {
                write!(writer, "{}", sampled.last().unwrap().position)?;
            }
        }
        writeln!(writer)?;
    }
    
    writeln!(writer, "{}", "─".repeat(70))?;
    Ok(())
}

pub fn print_frequency_chart(analysis: &FileAnalysis, top_n: usize, writer: &mut impl Write) -> std::io::Result<()> {
    if analysis.error.is_some() {
        return Ok(());
    }

    use crate::entropy::BUFFER_SIZE;
    use std::fs::File;
    use std::io::{BufReader, Read};
    
    let file = File::open(&analysis.filename)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("Error opening file: {}", e)))?;
    
    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; BUFFER_SIZE];
    let mut counts = [0u32; 256];
    
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(bytes_read) => {
                for i in 0..bytes_read {
                    counts[buffer[i] as usize] += 1;
                }
            }
            Err(e) => {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Error reading file: {}", e)));
            }
        }
    }

    // Create vector of (byte_value, count) pairs
    let mut freq_pairs: Vec<(u8, u32)> = counts
        .iter()
        .enumerate()
        .map(|(byte, &count)| (byte as u8, count))
        .filter(|(_, count)| *count > 0)
        .collect();
    
    // Sort by frequency (descending)
    freq_pairs.sort_by(|a, b| b.1.cmp(&a.1));
    
    let total = analysis.size_bytes as f64;
    let max_count = freq_pairs.first().map(|(_, c)| *c as f64).unwrap_or(1.0);
    
    writeln!(writer, "\nTop {} Most Frequent Bytes: {}", top_n, analysis.filename.display())?;
    writeln!(writer, "{}", "=".repeat(70))?;
    writeln!(writer, "{:<10} {:<10} {:<10} {:<10} {}", "Byte", "Hex", "Count", "Percent", "Bar")?;
    writeln!(writer, "{}", "─".repeat(70))?;
    
    for (byte_value, count) in freq_pairs.iter().take(top_n) {
        let percent = (*count as f64 / total) * 100.0;
        let bar_length = ((*count as f64 / max_count) * 30.0) as usize;
        let bar = "█".repeat(bar_length);
        
        let char_repr = if *byte_value >= 32 && *byte_value <= 126 {
            format!("'{}'", *byte_value as char)
        } else {
            "".to_string()
        };
        
        writeln!(
            writer,
            "{:<10} {:<10} {:<10} {:>6.2}%  {}",
            char_repr,
            format!("0x{:02X}", byte_value),
            count,
            percent,
            bar
        )?;
    }
    
    writeln!(writer, "{}", "─".repeat(70))?;
    Ok(())
}
