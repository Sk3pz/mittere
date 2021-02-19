
use better_term::style::Color;
use crate::logger::Logger;
use std::path::Path;
use chrono::Local;
use chrono::format::{StrftimeItems, DelayedFormat};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::io::stdin;

pub mod logger;
pub mod network;
pub mod packet_capnp;
pub mod config;

#[cfg(target_os = "macos")]
pub const CLEAR: &str = "clear";
#[cfg(target_os = "linux")]
pub const CLEAR: &str = "clear";
#[cfg(target_os = "windows")]
pub const CLEAR: &str = "cls";

pub const KEEPALIVE_INTERVAL: u64 = 20; // Time in seconds to send keepalive packet

pub fn systime() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Fatal error occurred: System time moved backwards! Are you a time traveler?")
}

pub fn to_epoch(time: SystemTime) -> Duration {
    time.duration_since(UNIX_EPOCH)
        .expect("Fatal error occurred: System time moved backwards! Are you a time traveler?")
}

pub fn make_logger(show_verbose: bool, output_console: bool, output_file: bool, panic_on_err: bool) -> Logger {
    // get the current directory
    let current_dir = std::env::current_dir().expect("Failed to get current directory. Please make sure that Mittere is run as admin next time!");
    // get the current time
    let datetime = Local::now().format("%H-%M-%S_%m-%d-%Y");
    // get the raw path of the log file
    let mut raw_path = format!("{}/logs/{}_log.txt", current_dir.as_path().to_str().expect("Error occurred initializing the logger: could not form string from path"), datetime);

    // create the path
    let path = Path::new(raw_path.as_str());
    // create the logger using parameters and the path
    let mut logger = Logger::new(path, output_console, output_file, panic_on_err);
    // set to show verbose messages
    logger.show_verbose_msgs(show_verbose);

    logger
}

// TODO: This currently is somewhat broken, if a message is sent while a client is typing, it will cut the input in half,
//  although this glitch is only visual.
// TODO: May have to write custom output system :'( (or wait until GFX update!)
pub fn read_console() -> String {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("Error reading from terminal: could not read from input");
    line
}