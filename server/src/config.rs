use std::fmt;
use std::fmt::{Display, Formatter};
use std::path::Path;
use serde::Deserialize;

#[derive(Debug)]
pub enum ConfigError {
    IoError(std::io::Error),
    TomlError(toml::de::Error),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ConfigError::IoError(e) => write!(f, "Config IO Error: {}", e),
            ConfigError::TomlError(e) => write!(f, "Config TOML Error: {}", e),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConnectionInfo {
    pub(crate) ip: String,
    pub(crate) port: u16,
}

impl Display for ConnectionInfo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.ip, self.port)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct General {
    pub(crate) show_msgs_on_server: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub(crate) conn: ConnectionInfo,
}

impl Config {
    fn create_default<S: AsRef<Path>>(path: S) -> Result<(), ConfigError> {
        let default_config = "# The IP and Port that the server should host on\
        \n[conn]\
        \nip = \"0.0.0.0\"\
        \nport = 2727\
        \n\
        \n# General settings\
        \n[general]\
        \nshow_msgs_on_server = true";

        // create the directory if it doesn't exist
        if let Some(dir) = path.as_ref().parent() {
            std::fs::create_dir_all(dir).map_err(ConfigError::IoError)?;
        }

        // write to the file
        std::fs::write(path, default_config).map_err(ConfigError::IoError)?;

        Ok(())
    }

    pub fn load<S: AsRef<Path>>(path: S) -> Result<Config, ConfigError> {
        // if the config doesnt exist, create it with default values
        if !path.as_ref().exists() {
            Self::create_default(&path)?;
        }

        let toml = std::fs::read_to_string(path).map_err(ConfigError::IoError)?;
        toml::from_str(&toml).map_err(ConfigError::TomlError)
    }
}