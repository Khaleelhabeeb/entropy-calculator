use crate::analysis::FileAnalysis;
use serde::Serialize;
use std::io::Write;

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
    Csv,
}

#[derive(Serialize)]
struct JsonResult {
    filename: String,
    size_bytes: u64,
    byte_entropy_per_byte: Option<f64>,
    byte_entropy_of_file: Option<f64>,
    bit_entropy: Option<f64>,
    entropy_per_byte_bytes: Option<f64>,
    delta_compressible_bytes: Option<f64>,
    best_theoretical_coding_ratio: Option<f64>,
    error: Option<String>,
}

pub struct OutputFormatter {
    format: OutputFormat,
}

impl OutputFormatter {
    pub fn new(format: OutputFormat) -> Self {
        Self { format }
    }

    pub fn format_results(&self, results: &[FileAnalysis], writer: impl Write) -> std::io::Result<()> {
        match self.format {
            OutputFormat::Text => self.format_text(results, writer),
            OutputFormat::Json => self.format_json(results, writer),
            OutputFormat::Csv => self.format_csv(results, writer),
        }
    }

    fn format_text(&self, results: &[FileAnalysis], mut writer: impl Write) -> std::io::Result<()> {
        for analysis in results {
            if let Some(ref error) = analysis.error {
                writeln!(writer, "\n--- File: {} ---", analysis.filename.display())?;
                writeln!(writer, "Error: {}", error)?;
                writeln!(writer)?;
                continue;
            }

            if analysis.bit_entropy.is_some() {
                let bit_entropy = analysis.bit_entropy.unwrap();
                writeln!(writer, "\n--- File: {} ---", analysis.filename.display())?;
                writeln!(writer, "---------------------------------------")?;
                writeln!(writer, "Bit-level informational entropy: {:.6} bits", bit_entropy)?;
                writeln!(writer, "---------------------------------------")?;
            } else if let Some(byte_entropy) = analysis.byte_entropy {
                let entropy_per_byte = byte_entropy / 8.0;
                let entropy_of_file = byte_entropy * analysis.size_bytes as f64;
                let delta = analysis.size_bytes as f64 - entropy_of_file / 8.0;
                let ratio = if byte_entropy > 0.0 { 8.0 / byte_entropy } else { f64::INFINITY };

                writeln!(writer, "\n--- File: {} ---", analysis.filename.display())?;
                writeln!(writer, "---------------------------------------")?;
                writeln!(
                    writer,
                    "Entropy per byte              : {:.6} bits ({:.6} bytes)",
                    byte_entropy, entropy_per_byte
                )?;
                writeln!(
                    writer,
                    "Entropy of file               : {:.6} bits ({:.6} bytes)",
                    entropy_of_file, entropy_of_file / 8.0
                )?;
                writeln!(writer, "Size of file                  : {} bytes", analysis.size_bytes)?;
                writeln!(
                    writer,
                    "Delta                         : {:.6} bytes (compressible theoretically)",
                    delta
                )?;
                writeln!(
                    writer,
                    "Best Theoretical Coding ratio : {:.6}",
                    ratio
                )?;
                writeln!(writer, "---------------------------------------")?;
            }
            writeln!(writer)?;
        }
        Ok(())
    }

    fn format_json(&self, results: &[FileAnalysis], writer: impl Write) -> std::io::Result<()> {
        let json_results: Vec<JsonResult> = results
            .iter()
            .map(|a| {
                let byte_entropy_per_byte = a.byte_entropy;
                let byte_entropy_of_file = a.byte_entropy.map(|e| e * a.size_bytes as f64);
                let entropy_per_byte_bytes = a.byte_entropy.map(|e| e / 8.0);
                let delta = a.byte_entropy.map(|e| {
                    let entropy_of_file = e * a.size_bytes as f64;
                    a.size_bytes as f64 - entropy_of_file / 8.0
                });
                let ratio = a.byte_entropy.map(|e| if e > 0.0 { 8.0 / e } else { f64::INFINITY });

                JsonResult {
                    filename: a.filename.display().to_string(),
                    size_bytes: a.size_bytes,
                    byte_entropy_per_byte,
                    byte_entropy_of_file,
                    bit_entropy: a.bit_entropy,
                    entropy_per_byte_bytes,
                    delta_compressible_bytes: delta,
                    best_theoretical_coding_ratio: ratio,
                    error: a.error.clone(),
                }
            })
            .collect();

        serde_json::to_writer_pretty(writer, &json_results)?;
        Ok(())
    }

    fn format_csv(&self, results: &[FileAnalysis], writer: impl Write) -> std::io::Result<()> {
        let mut wtr = csv::Writer::from_writer(writer);
        
        wtr.write_record(&[
            "filename",
            "size_bytes",
            "byte_entropy_per_byte",
            "byte_entropy_of_file",
            "bit_entropy",
            "entropy_per_byte_bytes",
            "delta_compressible_bytes",
            "best_theoretical_coding_ratio",
            "error",
        ])?;

        for analysis in results {
            let byte_entropy_per_byte = analysis.byte_entropy.map(|e| e.to_string()).unwrap_or_default();
            let byte_entropy_of_file = analysis
                .byte_entropy
                .map(|e| (e * analysis.size_bytes as f64).to_string())
                .unwrap_or_default();
            let bit_entropy = analysis.bit_entropy.map(|e| e.to_string()).unwrap_or_default();
            let entropy_per_byte_bytes = analysis
                .byte_entropy
                .map(|e| (e / 8.0).to_string())
                .unwrap_or_default();
            let delta = analysis
                .byte_entropy
                .map(|e| {
                    let entropy_of_file = e * analysis.size_bytes as f64;
                    (analysis.size_bytes as f64 - entropy_of_file / 8.0).to_string()
                })
                .unwrap_or_default();
            let ratio = analysis
                .byte_entropy
                .map(|e| {
                    if e > 0.0 {
                        (8.0 / e).to_string()
                    } else {
                        "inf".to_string()
                    }
                })
                .unwrap_or_default();
            let error = analysis.error.as_deref().unwrap_or("");

            wtr.write_record(&[
                &analysis.filename.display().to_string(),
                &analysis.size_bytes.to_string(),
                &byte_entropy_per_byte,
                &byte_entropy_of_file,
                &bit_entropy,
                &entropy_per_byte_bytes,
                &delta,
                &ratio,
                error,
            ])?;
        }

        wtr.flush()?;
        Ok(())
    }
}

pub fn format_results(format: OutputFormat, results: &[FileAnalysis], writer: impl Write) -> std::io::Result<()> {
    let formatter = OutputFormatter::new(format);
    formatter.format_results(results, writer)
}

