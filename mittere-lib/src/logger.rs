use std::path::Path;
use std::time::{SystemTime, Duration};
use std::net::TcpStream;
use std::fs::{File, OpenOptions};
use std::ffi::OsStr;
use std::io::Write;
use chrono::Local;
use chrono::format::{DelayedFormat, StrftimeItems};
use std::fs;
use std::ops::Add;
use better_term::style::{Style, Color};

/// A logging system to output to
pub struct Logger {
    pub output_console: bool, // defines if messages should be printed to console when created
    pub output_file: bool,
    pub show_verbose: bool,
    pub panic_on_err: bool,
    pub log_file: File,
}

impl Logger {
    /// Create a new logger
    pub fn new(file_path: &Path, output_console: bool, output_file: bool, panic_on_err: bool) -> Logger {
        let dir = file_path.parent().expect("Failed to get parent directory of log file.");
        //println!("Created log file: {}", file_path.to_str().expect("couldnt get path"));
        if !dir.exists() {
            match fs::create_dir_all(dir) {
                Ok(_) => (),
                Err(e) => panic!("Failed to create parent directories: {}", e)
            }
        }

        let mut log_file = match File::create(&file_path) {
            Ok(file) => file,
            Err(why) => panic!("couldn't open log file for writing: {}", why)
        };

        Logger {
            log_file,
            output_console,
            output_file,
            show_verbose: false,
            panic_on_err
        }
    }

    /// set if debug / verbose messages should be displayed
    pub fn show_verbose_msgs(&mut self, show: bool) {
        self.show_verbose = show;
    }

    /// Set if logger should panic on any error, or do nothing
    pub fn set_panic_on_err(&mut self, panic: bool) {
        self.panic_on_err = panic;
    }

    /// set if console output should be shown
    pub fn set_output_console(&mut self, output: bool) {
        self.output_console = output;
    }

    /// returns the current timestamp in HOURS:MINUTES:SECONDS MONTH/DAY/YEAR
    fn get_timestamp(&self) -> DelayedFormat<StrftimeItems> {
        let datetime = Local::now();
        return datetime.format("%H:%M:%S %m/%d/%Y");
    }

    fn output(&mut self, prefix: &str, s: &str) {
        let msg = format!("[{}] [{}] > {}\n", self.get_timestamp(), prefix, s).to_string();

        // Write to file
        if self.output_file {
            match self.log_file.write_all(msg.as_bytes()) {
                Ok(_) => (),
                Err(why) => {
                    if self.panic_on_err {
                        panic!("Failed to write to log file: {}", why);
                    } else if self.output_console {
                        self.output_console("ERROR", format!("Failed to write log message to file: {}", why).as_str());
                    }
                }
            }
        }
    }

    fn output_console(&mut self, prefix: &str, msg: &str) {
        if self.output_console {
            //println!("\x1b[0;90m[\x1b[0;37m{}\x1b[0;90m] [{}\x1b[0;90m] \x1b[0;37m> \x1b[0;97m{}", self.get_timestamp(), prefix, msg);
            println!("{}[{}{}{}] [{}{}] {}> {}{}",
                     Color::BrightBlack, // timestamp [
                     Color::White,       // timestamp color
                     self.get_timestamp(),                    // timestamp
                     Color::BrightBlack, // timestamp ] and prefix [
                     prefix,                                  // prefix
                     Color::BrightBlack, // prefix ]
                     Color::White, // >
                     Color::BrightWhite, // message color
                     msg                                      // display the message
            );
        }
    }

    /// Displays a verbose message (will not run if self.show_verbose is false).
    /// Should be run to display values of variables, output of calculations, and etc.
    pub fn verbose<S: Into<String>>(&mut self, msg: S) {
        let smsg = msg.into();
        if !self.show_verbose { return; }
        if self.output_console {
            println!("{}[{}{}{}] [{}VERBOSE{}] {}> {}{}",
                     Color::BrightBlack, // timestamp [
                     Color::White,       // timestamp color
                     self.get_timestamp(),                    // timestamp
                     Color::BrightBlack, // timestamp ] and prefix [
                     Color::White,       // VERBOSE color
                     Color::BrightBlack, // prefix ]
                     Color::White, // >
                     Color::BrightWhite, // message color
                     smsg                                      // display the message
            );
        }
        self.output("VERBOSE", smsg.as_str());
    }

    /// Displays an info message.
    /// Should be run to inform the user of various events in the program.
    pub fn info<S: Into<String>>(&mut self, msg: S) {
        let smsg = msg.into();
        // self.output_console("\x1b[0;36mINFO", msg);
        self.output_console(format!("{}INFO", Color::Cyan).as_str(), smsg.as_str());
        self.output("INFO", smsg.as_str());
    }

    /// Displays an important message.
    /// Should be run when something isn't an issue but is important information
    pub fn important<S: Into<String>>(&mut self, msg: S) {
        let smsg = msg.into();
        // self.output_console("\x1b[0;93mWARN", msg);
        if self.output_console {
            println!("{}[{}{}{}] {}[{}IMPORTANT{}] {}> {}{}",
                     Color::BrightBlack, // timestamp [
                     Color::White,       // timestamp color
                     self.get_timestamp(),                    // timestamp
                     Color::BrightBlack, // timestamp ]
                     Color::Green, // prefix [
                     Color::BrightGreen,       // prefix color
                     Color::Green, // prefix ]
                     Color::Green, // >
                     Color::BrightGreen, // message color
                     smsg                                      // display the message
            );
        }
        self.output("IMPORTANT", smsg.clone().as_str());
    }

    /// Displays a client's chat message
    /// Should be run when a client sends a chat message
    pub fn chat<S: Into<String>>(&mut self, msg: S, msg_color: String, display_name: String, name_color: String) {
        let smsg = msg.into();
        // self.output_console("\x1b[0;93mWARN", msg);
        if self.output_console {
            //println!("\x1b[0;90m[\x1b[0;37m{}\x1b[0;90m] [{}\x1b[0;90m] \x1b[0;37m> \x1b[0;97m{}", self.get_timestamp(), prefix, msg);
            println!("{}[{}{}{}] {}{} {}> {}{}",
                     Color::BrightBlack, // timestamp [
                     Color::White,       // timestamp color
                     self.get_timestamp(), // timestamp
                     Color::BrightBlack, // timestamp ]
                     name_color,  // The display name color
                     display_name, // display name
                     Color::White, // >
                     msg_color, // message color
                     smsg // display the message
            );
        }

        let file_msg = format!("[{}] {} > {}\n", self.get_timestamp(), display_name, smsg).to_string();

        // Write to file
        if self.output_file {
            match self.log_file.write_all(file_msg.as_bytes()) {
                Ok(_) => (),
                Err(why) => {
                    if self.panic_on_err {
                        panic!("Failed to write to log file: {}", why);
                    } else if self.output_console {
                        self.output_console("ERROR", format!("Failed to write log message to file: {}", why).as_str());
                    }
                }
            }
        }
    }

    /// Displays a warning message.
    /// Should be run when something could go wrong or the program caught an issue and fixed it.
    pub fn warn<S: Into<String>>(&mut self, msg: S) {
        let smsg = msg.into();
        // self.output_console("\x1b[0;93mWARN", msg);
        self.output_console(format!("{}WARN", Color::BrightYellow).as_str(), smsg.clone().as_str());
        self.output("WARN", smsg.clone().as_str());
    }

    /// Displays an error message.
    /// Should be run when an error occurs that will not outright crash the program
    pub fn error<S: Into<String>>(&mut self, msg: S) {
        let smsg = msg.into();
        // self.output_console("\x1b[0;91mERROR", msg);
        self.output_console(format!("{}ERROR", Color::BrightRed).as_str(), smsg.clone().as_str());
        self.output("ERROR", smsg.clone().as_str());
    }

    /// Displays a system failure message.
    /// Should be run when a major system failure has occurred, i.e. supposedly unreachable code, etc.
    pub fn failure<S: Into<String>>(&mut self, msg: S) {
        let smsg = msg.into();
        // self.output_console("\x1b[0;31mSYSTEM FAILURE", msg);
        self.output_console(format!("{}FAILURE", Color::Red).as_str(), smsg.clone().as_str());
        self.output("FAILURE", smsg.clone().as_str());
    }

}