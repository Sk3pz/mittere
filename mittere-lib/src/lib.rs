
use better_term::style::Color;
use crate::logger::Logger;
use std::path::Path;
use chrono::Local;

pub mod logger;
pub mod network;
pub mod packet_capnp;
// pub mod proto_buff; // NO LONGER USED - SWITCHED TO CAPNP

pub fn make_logger(show_verbose: bool, output_console: bool, output_file: bool, panic_on_err: bool, client: Option<String>) -> Logger {
    // get the current directory
    let current_dir = std::env::current_dir().expect("Failed to get current directory. Please make sure the program has proper permissions!");
    // get the current time
    let datetime = Local::now().format("%H-%M-%S_%m-%d-%Y");
    // get the raw path of the log file
    let mut raw_path = String::new();
    if client.is_some() {
        let client_ip = format!("client.{}", client.unwrap());
        raw_path = format!("{}/logs/{}/{}.txt", current_dir.as_path().to_str().expect("Error occurred initializing the logger: could not form string from path"),
                               datetime, client_ip);
    } else {
        raw_path = format!("{}/logs/{}/{}.txt", current_dir.as_path().to_str().expect("Error occurred initializing the logger: could not form string from path"),
                               datetime, "runtime_logs");
    }

    // create the path
    let path = Path::new(raw_path.as_str());
    // create the logger using parameters and the path
    let mut logger = Logger::new(path, output_console, output_file, panic_on_err);
    // set to show verbose messages
    logger.show_verbose_msgs(show_verbose);

    logger
}