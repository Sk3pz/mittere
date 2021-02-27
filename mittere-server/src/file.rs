use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::fs::{File, OpenOptions};
use mittere_lib::file::read_config_raw;
use std::io::Write;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub general: Option<General>,
    pub server: Option<Server>,
}

#[derive(Debug, Deserialize)]
pub struct General {
    pub connections: Option<i64>,
    pub motd: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub ip: Option<String>,
    pub port: Option<String>
}

pub fn read_config(path: &Path, default: String) -> Config {
    let dir = path.parent().expect("Failed to get parent location of config file. Invalid permissions?");
    if !dir.exists() {
        match fs::create_dir_all(dir) {
            Ok(_) => (),
            Err(e) => panic!("Failed to create parent directories: {}", e)
        }
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .open(path.clone())
        .expect("An error occurred in opening the config file.");

    let mut data = read_config_raw(&mut file);

    if data.is_empty() {
        match file.write_all(default.as_bytes()) {
            Ok(_) => (),
            Err(why) => {
                panic!("Failed to write defaults to config file: {}", why);
            }
        }
        data = default;
    }

    toml::from_str(data.as_str()).expect("Could not read config: Please make sure it is valid and has all keys defined, according to the server-config-example.toml")
}