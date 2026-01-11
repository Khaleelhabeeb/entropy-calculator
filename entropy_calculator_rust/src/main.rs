use clap::Parser;
use entropy_calculator::{
    analyze_file, analyze_file_sliding_window, format_results, OutputFormat,
    print_byte_distribution_histogram, print_sliding_window_graph, print_frequency_chart,
    calculate_aggregate_statistics, print_aggregate_statistics,
    compare_files, print_comparison, compare_multiple_files,
    load_config, save_config_template, Config,
};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(name = "entropy_calculator")]
#[command(version = "0.2.0")]
#[command(about = "Advanced entropy calculator with visualization, statistics, comparison, and config support.")]
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

    /// Show byte distribution histogram
    #[arg(long = "histogram", action = clap::ArgAction::SetTrue)]
    histogram: bool,

    /// Show frequency chart (top N most frequent bytes)
    #[arg(long = "frequency", action = clap::ArgAction::SetTrue)]
    frequency: bool,

    /// Number of top bytes to show in frequency chart
    #[arg(long = "frequency-top", default_value = "10")]
    frequency_top: usize,

    /// Show sliding window graph (requires --window)
    #[arg(long = "graph", action = clap::ArgAction::SetTrue)]
    graph: bool,

    /// Show aggregate statistics (for multiple files)
    #[arg(long = "stats", action = clap::ArgAction::SetTrue)]
    stats: bool,

    /// Compare two files
    #[arg(long = "compare", value_name = "FILE2", num_args = 1)]
    compare: Option<String>,

    /// Baseline comparison mode (compare all files to first file)
    #[arg(long = "baseline", action = clap::ArgAction::SetTrue)]
    baseline: bool,

    /// Configuration file path
    #[arg(short = 'c', long = "config")]
    config: Option<PathBuf>,

    /// Generate a sample configuration file
    #[arg(long = "gen-config", value_name = "PATH")]
    gen_config: Option<PathBuf>,

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

fn apply_config_to_args(config: &Config, args: &mut Args) {
    // Apply config settings (CLI args take precedence)
    if !args.bit_level {
        args.bit_level = config.analysis.bit_level;
    }
    if !args.recursive {
        args.recursive = config.analysis.recursive;
    }
    if args.extension.is_none() {
        args.extension = config.analysis.extension.clone();
    }
    if !args.no_progress {
        args.no_progress = !config.output.show_progress;
    }
    if args.threads == 0 {
        args.threads = config.output.threads;
    }
    if !args.histogram {
        args.histogram = config.visualization.show_histogram;
    }
    if !args.frequency {
        args.frequency = config.visualization.show_frequency_chart;
    }
    if args.frequency_top == 10 {
        args.frequency_top = config.visualization.frequency_top_n;
    }
}

fn main() {
    let mut args = Args::parse();

    // Handle config file generation
    if let Some(config_path) = args.gen_config {
        match save_config_template(&config_path) {
            Ok(_) => {
                println!("Configuration template saved to: {}", config_path.display());
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("Error generating config file: {}", e);
                std::process::exit(1);
            }
        }
    }

    // Load config if provided
    if let Some(config_path) = &args.config {
        match load_config(config_path) {
            Ok(config) => {
                apply_config_to_args(&config, &mut args);
            }
            Err(e) => {
                eprintln!("Warning: Could not load config file: {}", e);
            }
        }
    }

    // Handle comparison mode
    if let Some(compare_file) = &args.compare {
        if args.files.len() != 1 {
            eprintln!("Error: --compare requires exactly one file as the first argument");
            eprintln!("Usage: entropy_calculator --compare FILE2 FILE1");
            std::process::exit(1);
        }

        let file1 = Path::new(&args.files[0]);
        let file2 = Path::new(compare_file);

        let analysis1 = analyze_file(file1, args.bit_level);
        let analysis2 = analyze_file(file2, args.bit_level);

        let comparison = compare_files(&analysis1, &analysis2);
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        
        if let Err(e) = print_comparison(&comparison, &mut handle) {
            eprintln!("Error writing comparison: {}", e);
            std::process::exit(1);
        }
        return;
    }

    // Handle baseline comparison
    if args.baseline {
        if args.files.len() < 2 {
            eprintln!("Error: --baseline requires at least 2 files");
            std::process::exit(1);
        }

        let files = collect_files(&args.files, args.recursive, args.extension.as_deref());
        if files.len() < 2 {
            eprintln!("Error: Need at least 2 files for baseline comparison");
            std::process::exit(1);
        }

        let results: Vec<_> = files
            .par_iter()
            .map(|file| analyze_file(file, args.bit_level))
            .collect();

        let stdout = io::stdout();
        let mut handle = stdout.lock();

        if let Err(e) = compare_multiple_files(&results, 0, &mut handle) {
            eprintln!("Error writing comparison: {}", e);
            std::process::exit(1);
        }
        return;
    }

    if args.files.is_empty() {
        eprintln!("Error: Please provide one or more files or directories to analyze");
        eprintln!("Usage: entropy_calculator [OPTIONS] <FILE|DIR>...");
        eprintln!("\nExamples:");
        eprintln!("  entropy_calculator file.txt");
        eprintln!("  entropy_calculator -r --extension .txt /path/to/dir");
        eprintln!("  entropy_calculator --format json -r .");
        eprintln!("  entropy_calculator --window 1024 --graph large_file.bin");
        eprintln!("  entropy_calculator --compare file2.txt file1.txt");
        eprintln!("  entropy_calculator --baseline file1.txt file2.txt file3.txt");
        eprintln!("  entropy_calculator --stats -r --extension .bin /path/to/dir");
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
                    let stdout = io::stdout();
                    let mut handle = stdout.lock();

                    println!("\n--- Sliding Window Entropy Analysis: {} ---", file.display());
                    println!("Window size: {} bytes", window_size);
                    println!("Number of windows: {}", results.len());

                    if args.graph {
                        if let Err(e) = print_sliding_window_graph(&results, &mut handle) {
                            eprintln!("Error printing graph: {}", e);
                        }
                    } else {
                        println!("\nPosition (bytes)\tWindow Size\tEntropy (bits)");
                        println!("{}", "-".repeat(60));
                        
                        for result in &results {
                            println!(
                                "{}\t\t{}\t\t{:.6}",
                                result.position, result.chunk_size, result.entropy
                            );
                        }
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
    let pb = if !args.no_progress && files.len() > 1 {
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
    
    // Show aggregate statistics if requested
    if args.stats && results.len() > 1 {
        let stats = calculate_aggregate_statistics(&results);
        if let Err(e) = print_aggregate_statistics(&stats, &mut handle) {
            eprintln!("Error writing statistics: {}", e);
        }
    }

    // Format and output results
    if let Err(e) = format_results(args.format, &results, &mut handle) {
        eprintln!("Error writing output: {}", e);
        std::process::exit(1);
    }

    // Show visualizations if requested (only for text format and single file)
    if matches!(args.format, OutputFormat::Text) && results.len() == 1 {
        let analysis = &results[0];
        
        if args.histogram && analysis.error.is_none() {
            if let Err(e) = print_byte_distribution_histogram(analysis, &mut handle) {
                eprintln!("Error printing histogram: {}", e);
            }
        }

        if args.frequency && analysis.error.is_none() {
            if let Err(e) = print_frequency_chart(analysis, args.frequency_top, &mut handle) {
                eprintln!("Error printing frequency chart: {}", e);
            }
        }
    } else if (args.histogram || args.frequency) && !matches!(args.format, OutputFormat::Text) {
        eprintln!("Warning: Visualization options (--histogram, --frequency) only work with text format and single files");
    }
}
