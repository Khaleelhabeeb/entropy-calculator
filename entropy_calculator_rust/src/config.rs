use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub output: OutputConfig,
    
    #[serde(default)]
    pub analysis: AnalysisConfig,
    
    #[serde(default)]
    pub visualization: VisualizationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    #[serde(default = "default_format")]
    pub format: String,
    
    #[serde(default)]
    pub show_progress: bool,
    
    #[serde(default)]
    pub threads: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    #[serde(default)]
    pub bit_level: bool,
    
    #[serde(default)]
    pub recursive: bool,
    
    #[serde(default)]
    pub extension: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationConfig {
    #[serde(default = "default_true")]
    pub show_histogram: bool,
    
    #[serde(default = "default_true")]
    pub show_frequency_chart: bool,
    
    #[serde(default = "default_frequency_top_n")]
    pub frequency_top_n: usize,
}

fn default_format() -> String {
    "text".to_string()
}

fn default_true() -> bool {
    true
}

fn default_frequency_top_n() -> usize {
    10
}

impl Default for Config {
    fn default() -> Self {
        Config {
            output: OutputConfig::default(),
            analysis: AnalysisConfig::default(),
            visualization: VisualizationConfig::default(),
        }
    }
}

impl Default for OutputConfig {
    fn default() -> Self {
        OutputConfig {
            format: "text".to_string(),
            show_progress: true,
            threads: 0,
        }
    }
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        AnalysisConfig {
            bit_level: false,
            recursive: false,
            extension: None,
        }
    }
}

impl Default for VisualizationConfig {
    fn default() -> Self {
        VisualizationConfig {
            show_histogram: true,
            show_frequency_chart: true,
            frequency_top_n: 10,
        }
    }
}

pub fn load_config(path: &std::path::Path) -> Result<Config, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Error reading config file: {}", e))?;
    
    let config: Config = toml::from_str(&content)
        .map_err(|e| format!("Error parsing config file: {}", e))?;
    
    Ok(config)
}

pub fn save_config_template(path: &std::path::Path) -> Result<(), String> {
    let config = Config::default();
    let toml_string = toml::to_string_pretty(&config)
        .map_err(|e| format!("Error serializing config: {}", e))?;
    
    std::fs::write(path, toml_string)
        .map_err(|e| format!("Error writing config file: {}", e))?;
    
    Ok(())
}
