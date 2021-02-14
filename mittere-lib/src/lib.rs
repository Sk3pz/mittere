
use better_term::style::Color;
use crate::logger::Logger;
use std::path::Path;
use chrono::Local;

pub mod logger;
mod proto_buff;
mod network;

pub fn make_logger(show_verbose: bool, output_console: bool, output_file: bool, panic_on_err: bool) -> Logger {
    // get the current directory
    let current_dir = std::env::current_dir().expect("Failed to get current directory. Please make sure the program has proper permissions!");
    // get the current time
    let datetime = Local::now().format("%H-%M-%S_%m-%d-%Y");
    // get the raw path of the log file
    let raw_path = format!("{}/logs/log{}.txt", current_dir.as_path().to_str().expect("Error occurred initializing the logger: could not form string from path"), datetime);
    // create the path
    let path = Path::new(raw_path.as_str());
    // create the logger using parameters and the path
    let mut logger = Logger::new(path, output_console, output_file, panic_on_err);
    // set to show verbose messages
    logger.show_verbose_msgs(show_verbose);

    logger
}