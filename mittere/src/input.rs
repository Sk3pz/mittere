use mittere_lib::{read_console, CLEAR};
use std::sync::mpsc::Sender;
use std::net::TcpStream;
use better_term::style::Color;
use mittere_lib::network::event_io::write_event_message;
use std::process::Command;

///
/// processes commands and returns true if needs to disconnect
///
/// # Arguments
/// `stream`: for use if a command needs to send data to the server
/// `cmd`: the command to process
pub fn cmd_handler(stream: &TcpStream, cmd: String) -> bool {
    // get the arguments
    let mut args: Vec<String> = Vec::new();
    let split = cmd.split(" ");
    for a in cmd.split(" ") {
        args.push(a.to_string());
    }
    let mut command = args.get(0).unwrap().to_owned();
    args.remove(0);
    match command.to_lowercase().as_str() {
        "/help" | "/?" => {
            let primary = Color::BrightGreen;
            let secondary = Color:: Green;
            let desc = Color::BrightYellow;
            let dash = Color::White;
            println!("Temporary help menu - redo coming soon!\n\
            {}Commands:\n\
            {}/online {}- {}list online users\n\
            {}/color <{}name{}|{}message{}> <{}color{}> {}- {}set the color of your name or messages\n\
            {}/clear {}- {}Clears the screen\n\
            {}/exit {}- {}safely disconnect{}",
                     primary,
                     primary, dash, desc,
                     primary, secondary, primary, secondary, primary, secondary, primary, dash, desc,
                     primary, dash, desc,
                     primary, dash, desc,
                     Color::White);
            false
        }
        "/online" | "/list" => {
            // TODO: Request list of online users
            println!("This command is not yet implemented, and is planned for a later release. Sorry for the inconvenience!");
            false
        }
        "/color" => {
            if args.len() < 2 {
                println!("{}[{}ERROR{}] {}> {}Invalid usage of the color command. Do /help for help!{}", Color::BrightBlack, Color::BrightRed, Color::BrightBlack, Color::Red, Color::BrightRed, Color::White);
            }
            let mode = args.get(0).unwrap();
            let color = args.get(1).unwrap();
            println!("mode: {}, color: {}", mode, color);
            false
        }
        "/clear" => {
            let cmd = Command::new(CLEAR).status();
            if cmd.is_err() {
                println!("{}[{}ERROR{}] {}> {}Failed to clear the screen{}", Color::BrightBlack, Color::BrightRed, Color::BrightBlack, Color::Red, Color::BrightRed, Color::White);
            }
            false
        }
        "/exit" | "/quit" => {
            println!("Received exit command. Thanks for using Mittere!");
            true
        }
        _ => {
            // TODO: unknown command
            false
        }
    }
}

pub fn input_handler(stream: TcpStream) {
    loop {
        let line = read_console().replace("\n", ""); // TODO: remove all escape codes (there are some weird glitches right now)

        if line.starts_with("/") {
            if cmd_handler(&stream, line) {
                drop(stream);
                std::process::exit(0);
            }
        } else {
            write_event_message(&stream, line);
        }
    }
}