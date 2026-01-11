use crate::analysis::FileAnalysis;
use serde::Serialize;
use std::io::Write;

#[derive(Debug, Clone, Serialize)]
pub struct AggregateStatistics {
    pub total_files: usize,
    pub total_bytes: u64,
    pub entropy_min: Option<f64>,
    pub entropy_max: Option<f64>,
    pub entropy_avg: Option<f64>,
    pub entropy_stddev: Option<f64>,
    pub entropy_median: Option<f64>,
    pub entropy_p25: Option<f64>,
    pub entropy_p75: Option<f64>,
    pub entropy_p90: Option<f64>,
    pub entropy_p95: Option<f64>,
    pub entropy_p99: Option<f64>,
}

pub fn calculate_aggregate_statistics(results: &[FileAnalysis]) -> AggregateStatistics {
    let valid_results: Vec<&FileAnalysis> = results
        .iter()
        .filter(|r| r.error.is_none() && r.byte_entropy.is_some())
        .collect();

    if valid_results.is_empty() {
        return AggregateStatistics {
            total_files: results.len(),
            total_bytes: 0,
            entropy_min: None,
            entropy_max: None,
            entropy_avg: None,
            entropy_stddev: None,
            entropy_median: None,
            entropy_p25: None,
            entropy_p75: None,
            entropy_p90: None,
            entropy_p95: None,
            entropy_p99: None,
        };
    }

    let total_bytes: u64 = valid_results.iter().map(|r| r.size_bytes).sum();
    
    let mut entropies: Vec<f64> = valid_results
        .iter()
        .filter_map(|r| r.byte_entropy)
        .collect();
    
    entropies.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let count = entropies.len();
    
    let min = entropies.first().copied();
    let max = entropies.last().copied();
    let avg = if count > 0 {
        Some(entropies.iter().sum::<f64>() / count as f64)
    } else {
        None
    };

    let variance = if count > 1 && avg.is_some() {
        let mean = avg.unwrap();
        Some(
            entropies
                .iter()
                .map(|&x| (x - mean).powi(2))
                .sum::<f64>()
                / (count - 1) as f64
        )
    } else {
        None
    };

    let stddev = variance.map(|v| v.sqrt());

    let median = if count > 0 {
        if count % 2 == 0 {
            Some((entropies[count / 2 - 1] + entropies[count / 2]) / 2.0)
        } else {
            Some(entropies[count / 2])
        }
    } else {
        None
    };

    let percentile = |p: f64| -> Option<f64> {
        if count == 0 {
            return None;
        }
        let index = (p * (count - 1) as f64) as usize;
        if index < count {
            Some(entropies[index])
        } else {
            entropies.last().copied()
        }
    };

    AggregateStatistics {
        total_files: results.len(),
        total_bytes,
        entropy_min: min,
        entropy_max: max,
        entropy_avg: avg,
        entropy_stddev: stddev,
        entropy_median: median,
        entropy_p25: percentile(0.25),
        entropy_p75: percentile(0.75),
        entropy_p90: percentile(0.90),
        entropy_p95: percentile(0.95),
        entropy_p99: percentile(0.99),
    }
}

pub fn print_aggregate_statistics(stats: &AggregateStatistics, writer: &mut impl Write) -> std::io::Result<()> {
    writeln!(writer, "\n{}", "=".repeat(70))?;
    writeln!(writer, "Aggregate Statistics")?;
    writeln!(writer, "{}", "=".repeat(70))?;
    writeln!(writer, "Total files analyzed: {}", stats.total_files)?;
    writeln!(writer, "Total bytes: {}", stats.total_bytes)?;
    
    if let Some(min) = stats.entropy_min {
        writeln!(writer, "\nEntropy Statistics:")?;
        writeln!(writer, "  Minimum:     {:.6} bits", min)?;
        
        if let Some(max) = stats.entropy_max {
            writeln!(writer, "  Maximum:     {:.6} bits", max)?;
        }
        
        if let Some(avg) = stats.entropy_avg {
            writeln!(writer, "  Average:     {:.6} bits", avg)?;
        }
        
        if let Some(stddev) = stats.entropy_stddev {
            writeln!(writer, "  Std Dev:     {:.6} bits", stddev)?;
        }
        
        if let Some(median) = stats.entropy_median {
            writeln!(writer, "  Median:      {:.6} bits", median)?;
        }
        
        writeln!(writer, "\nPercentiles:")?;
        if let Some(p25) = stats.entropy_p25 {
            writeln!(writer, "  25th (Q1):   {:.6} bits", p25)?;
        }
        if let Some(p75) = stats.entropy_p75 {
            writeln!(writer, "  75th (Q3):   {:.6} bits", p75)?;
        }
        if let Some(p90) = stats.entropy_p90 {
            writeln!(writer, "  90th:        {:.6} bits", p90)?;
        }
        if let Some(p95) = stats.entropy_p95 {
            writeln!(writer, "  95th:        {:.6} bits", p95)?;
        }
        if let Some(p99) = stats.entropy_p99 {
            writeln!(writer, "  99th:        {:.6} bits", p99)?;
        }
    } else {
        writeln!(writer, "\nNo valid entropy data available")?;
    }
    
    writeln!(writer, "{}", "=".repeat(70))?;
    Ok(())
}
