use crate::analysis::FileAnalysis;
use std::io::Write;

pub struct ComparisonResult {
    pub file1: String,
    pub file2: String,
    pub entropy1: Option<f64>,
    pub entropy2: Option<f64>,
    pub entropy_diff: Option<f64>,
    pub entropy_diff_percent: Option<f64>,
    pub size1: u64,
    pub size2: u64,
    pub size_diff: i64,
    pub size_diff_percent: f64,
}

pub fn compare_files(analysis1: &FileAnalysis, analysis2: &FileAnalysis) -> ComparisonResult {
    let entropy1 = analysis1.byte_entropy;
    let entropy2 = analysis2.byte_entropy;
    
    let entropy_diff = match (entropy1, entropy2) {
        (Some(e1), Some(e2)) => Some(e2 - e1),
        _ => None,
    };
    
    let entropy_diff_percent = match (entropy1, entropy2) {
        (Some(e1), Some(e2)) if e1 > 0.0 => Some(((e2 - e1) / e1) * 100.0),
        _ => None,
    };
    
    let size_diff = analysis2.size_bytes as i64 - analysis1.size_bytes as i64;
    let size_diff_percent = if analysis1.size_bytes > 0 {
        ((analysis2.size_bytes as f64 - analysis1.size_bytes as f64) / analysis1.size_bytes as f64) * 100.0
    } else {
        0.0
    };

    ComparisonResult {
        file1: analysis1.filename.display().to_string(),
        file2: analysis2.filename.display().to_string(),
        entropy1,
        entropy2,
        entropy_diff,
        entropy_diff_percent,
        size1: analysis1.size_bytes,
        size2: analysis2.size_bytes,
        size_diff,
        size_diff_percent,
    }
}

pub fn print_comparison(comparison: &ComparisonResult, writer: &mut impl Write) -> std::io::Result<()> {
    writeln!(writer, "\n{}", "=".repeat(70))?;
    writeln!(writer, "File Comparison")?;
    writeln!(writer, "{}", "=".repeat(70))?;
    
    writeln!(writer, "\nFile 1: {}", comparison.file1)?;
    writeln!(writer, "File 2: {}", comparison.file2)?;
    
    writeln!(writer, "\nEntropy Comparison:")?;
    if let Some(e1) = comparison.entropy1 {
        writeln!(writer, "  File 1: {:.6} bits", e1)?;
    } else {
        writeln!(writer, "  File 1: N/A")?;
    }
    
    if let Some(e2) = comparison.entropy2 {
        writeln!(writer, "  File 2: {:.6} bits", e2)?;
    } else {
        writeln!(writer, "  File 2: N/A")?;
    }
    
    if let Some(diff) = comparison.entropy_diff {
        let symbol = if diff >= 0.0 { "+" } else { "" };
        writeln!(writer, "  Difference: {}{:.6} bits", symbol, diff)?;
        
        if let Some(percent) = comparison.entropy_diff_percent {
            writeln!(writer, "  Percentage: {}{:.2}%", if percent >= 0.0 { "+" } else { "" }, percent)?;
        }
    } else {
        writeln!(writer, "  Difference: N/A")?;
    }
    
    writeln!(writer, "\nSize Comparison:")?;
    writeln!(writer, "  File 1: {} bytes", comparison.size1)?;
    writeln!(writer, "  File 2: {} bytes", comparison.size2)?;
    writeln!(writer, "  Difference: {} bytes ({}{:.2}%)", 
        comparison.size_diff,
        if comparison.size_diff_percent >= 0.0 { "+" } else { "" },
        comparison.size_diff_percent
    )?;
    
    writeln!(writer, "{}", "=".repeat(70))?;
    Ok(())
}

pub fn compare_multiple_files(results: &[FileAnalysis], baseline_index: usize, writer: &mut impl Write) -> std::io::Result<()> {
    if baseline_index >= results.len() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Baseline index out of range"
        ));
    }

    let baseline = &results[baseline_index];
    
    writeln!(writer, "\n{}", "=".repeat(70))?;
    writeln!(writer, "Baseline Comparison (baseline: {})", baseline.filename.display())?;
    writeln!(writer, "{}", "=".repeat(70))?;
    
    writeln!(writer, "\n{:<40} {:<15} {:<15} {:<15}", "File", "Entropy", "Diff", "Diff %")?;
    writeln!(writer, "{}", "─".repeat(85))?;
    
    if let Some(baseline_entropy) = baseline.byte_entropy {
        for (idx, result) in results.iter().enumerate() {
            if idx == baseline_index {
                continue;
            }
            
            let filename = result.filename.display().to_string();
            let display_name = if filename.len() > 38 {
                format!("...{}", &filename[filename.len() - 35..])
            } else {
                filename
            };
            
            if let Some(entropy) = result.byte_entropy {
                let diff = entropy - baseline_entropy;
                let diff_percent = if baseline_entropy > 0.0 {
                    ((entropy - baseline_entropy) / baseline_entropy) * 100.0
                } else {
                    0.0
                };
                
                writeln!(
                    writer,
                    "{:<40} {:<15.6} {:<15.6} {:<15.2}%",
                    display_name,
                    entropy,
                    diff,
                    diff_percent
                )?;
            } else {
                writeln!(writer, "{:<40} {:<15}", display_name, "N/A")?;
            }
        }
    }
    
    writeln!(writer, "{}", "=".repeat(70))?;
    Ok(())
}
