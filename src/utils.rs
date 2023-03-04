use colored::*;

pub struct ConsolePrinter {}

impl ConsolePrinter {
    pub fn info(message: String) -> () {
        ConsolePrinter::log_line(
            "INFO".green(),
            message
        );
    }

    pub fn warn(message: String) -> () {
        ConsolePrinter::log_line(
            "WARN".yellow(),
            message
        );
    }

    pub fn error(message: String) -> () {
        ConsolePrinter::log_line(
            "ERR ".red(),
            message
        );
    }

    fn log_line(code: ColoredString, message: String) -> () {
        println!("{}:  {}", code, message);
    }
}