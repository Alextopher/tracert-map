use serde::Deserialize;

/// A simple TOML config file.
///
/// At the moment it only includes the maxmind api token
#[derive(Debug, Deserialize)]
pub struct Config {
    pub token: String,
}

impl Config {
    /// Loads the config file from the given path
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let config = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&config)?)
    }
}
