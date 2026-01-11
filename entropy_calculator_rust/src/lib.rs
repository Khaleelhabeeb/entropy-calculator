pub mod entropy;
pub mod analysis;
pub mod output;
pub mod visualization;
pub mod statistics;
pub mod comparison;
pub mod config;

pub use entropy::{calculate_byte_level_entropy, calculate_bit_level_entropy};
pub use analysis::{FileAnalysis, analyze_file, analyze_file_sliding_window};
pub use output::{OutputFormat, OutputFormatter, format_results};
pub use visualization::{print_byte_distribution_histogram, print_sliding_window_graph, print_frequency_chart};
pub use statistics::{calculate_aggregate_statistics, print_aggregate_statistics, AggregateStatistics};
pub use comparison::{compare_files, print_comparison, compare_multiple_files, ComparisonResult};
pub use config::{Config, load_config, save_config_template};

