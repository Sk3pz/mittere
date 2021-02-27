use std::path::Path;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Write, Read};

pub fn read_config_raw(file: &mut File) -> String {
    let mut config_content = String::new();
    file.read_to_string(&mut config_content).expect("Failed to read config - please make sure server-config.toml exists in `~/mittere-config/` and that the server has permissions.");
    config_content
}