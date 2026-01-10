use clap::Parser;
use entropy_calculator::{analyze_file, analyze_file_sliding_window, format_results, OutputFormat};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(name = "entropy_calculator")]
#[command(version = "0.1.0")]
#[command(about = "Advanced entropy calculator for files with multiple output formats, recursive directory analysis, and parallel processing.")]
struct Args {
    /// Calculate bit-level informational entropy
    #[arg(short = 'b', long = "bit")]
    bit_level: bool,

    /// Output format
    #[arg(short = 'f', long = "format", default_value = "text", value_enum)]
    format: OutputFormat,

    /// Analyze directories recursively
    #[arg(short = 'r', long = "recursive")]
    recursive: bool,

    /// Sliding window analysis (entropy per chunk)
    #[arg(short = 'w', long = "window", value_name = "SIZE")]
    window_size: Option<u64>,

    /// Filter files by extension (e.g., ".txt", ".bin")
    #[arg(short = 'e', long = "extension")]
    extension: Option<String>,

    /// Hide progress bars (by default progress bars are shown for multiple files)
    #[arg(long = "no-progress", action = clap::ArgAction::SetTrue)]
    no_progress: bool,

    /// Parallel processing (number of threads, 0 = auto)
    #[arg(long = "threads", default_value = "0")]
    threads: usize,

    /// Files or directories to analyze
    files: Vec<String>,
}

fn collect_files(paths: &[String], recursive: bool, extension: Option<&str>) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for path_str in paths {
        let path = Path::new(path_str);
        
        if path.is_file() {
            if let Some(ext) = extension {
                let ext_clean = ext.trim_start_matches('.');
                if path.extension()
                    .and_then(|e| e.to_str())
                    .map(|e| e == ext_clean)
                    .unwrap_or(false)
                {
                    files.push(path.to_path_buf());
                }
            } else {
                files.push(path.to_path_buf());
            }
        } else if path.is_dir() {
            if recursive {
                let walker = WalkDir::new(path)
                    .follow_links(false)
                    .into_iter()
                    .filter_map(|e| e.ok());

                for entry in walker {
                    let entry_path = entry.path();
                    if entry_path.is_file() {
                        if let Some(ext) = extension {
                            if entry_path
                                .extension()
                                .and_then(|e| e.to_str())
                                .map(|e| e == ext.trim_start_matches('.'))
                                .unwrap_or(false)
                            {
                                files.push(entry_path.to_path_buf());
                            }
                        } else {
                            files.push(entry_path.to_path_buf());
                        }
                    }
                }
            } else {
                eprintln!("Warning: {} is a directory. Use -r/--recursive to analyze directories.", path_str);
            }
        } else {
            eprintln!("Warning: {} does not exist or is not a file/directory", path_str);
        }
    }

    files
}

fn main() {
    let args = Args::parse();

    if args.files.is_empty() {
        eprintln!("Error: Please provide one or more files or directories to analyze");
        eprintln!("Usage: entropy_calculator [OPTIONS] <FILE|DIR>...");
        eprintln!("\nExamples:");
        eprintln!("  entropy_calculator file.txt");
        eprintln!("  entropy_calculator -r --extension .txt /path/to/dir");
        eprintln!("  entropy_calculator --format json -r .");
        eprintln!("  entropy_calculator --window 1024 large_file.bin");
        std::process::exit(1);
    }

    // Set up parallel processing
    if args.threads > 0 {
        rayon::ThreadPoolBuilder::new()
            .num_threads(args.threads)
            .build_global()
            .expect("Failed to initialize thread pool");
    }

    // Collect all files to analyze
    let files = collect_files(&args.files, args.recursive, args.extension.as_deref());

    if files.is_empty() {
        eprintln!("Error: No files found to analyze");
        std::process::exit(1);
    }

    // Handle sliding window analysis
    if let Some(window_size) = args.window_size {
        if files.len() > 1 {
            eprintln!("Warning: Sliding window analysis works on a single file. Analyzing first file only.");
        }

        if let Some(file) = files.first() {
            match analyze_file_sliding_window(file, window_size) {
                Ok(results) => {
                    println!("\n--- Sliding Window Entropy Analysis: {} ---", file.display());
                    println!("Window size: {} bytes", window_size);
                    println!("Number of windows: {}", results.len());
                    println!("\nPosition (bytes)\tWindow Size\tEntropy (bits)");
                    println!("{}", "-".repeat(60));
                    
                    for result in &results {
                        println!(
                            "{}\t\t{}\t\t{:.6}",
                            result.position, result.chunk_size, result.entropy
                        );
                    }

                    // Calculate statistics
                    if !results.is_empty() {
                        let min_entropy = results.iter().map(|r| r.entropy).fold(f64::INFINITY, f64::min);
                        let max_entropy = results.iter().map(|r| r.entropy).fold(0.0, f64::max);
                        let avg_entropy = results.iter().map(|r| r.entropy).sum::<f64>() / results.len() as f64;
                        
                        println!("\nStatistics:");
                        println!("  Minimum entropy: {:.6} bits", min_entropy);
                        println!("  Maximum entropy: {:.6} bits", max_entropy);
                        println!("  Average entropy: {:.6} bits", avg_entropy);
                    }
                }
                Err(e) => {
                    eprintln!("Error performing sliding window analysis: {}", e);
                    std::process::exit(1);
                }
            }
        }
        return;
    }

    // Regular entropy analysis
    let pb = if !args.no_progress && files.len() > 1 && args.window_size.is_none() {
        Some(
            ProgressBar::new(files.len() as u64).with_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files ({msg})")
                    .unwrap()
                    .progress_chars("#>-"),
            ),
        )
    } else {
        None
    };

    let results: Vec<_> = files
        .par_iter()
        .map(|file| {
            if let Some(ref bar) = pb {
                bar.set_message(file.file_name().and_then(|n| n.to_str()).unwrap_or("unknown").to_string());
            }
            
            let result = analyze_file(file, args.bit_level);
            
            if let Some(ref bar) = pb {
                bar.inc(1);
            }
            
            result
        })
        .collect();

    if let Some(bar) = pb {
        bar.finish_with_message("Complete");
    }

    // Output results
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    
    if let Err(e) = format_results(args.format, &results, &mut handle) {
        eprintln!("Error writing output: {}", e);
        std::process::exit(1);
    }
}
