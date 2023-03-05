use colored::*;

pub struct ConsolePrinter {}

impl ConsolePrinter {
    /// Logs a message to console on INFO level in blue color.
    ///
    /// # Arguments
    /// * `message` - string to log
    ///
    /// # Examples
    /// ```
    /// ConsolePrinter::info(String::of("Info"));
    /// ```
    pub fn info(message: String) -> () {
        ConsolePrinter::log_line(
            "INFO".bright_blue(),
            message
        );
    }

    /// Logs a message to console on WARN level in yellow color.
    ///
    /// # Arguments
    /// * `message` - string to log
    ///
    /// # Examples
    /// ```
    /// ConsolePrinter::info(String::of("Warning"));
    /// ```
    pub fn warn(message: String) -> () {
        ConsolePrinter::log_line(
            "WARN".yellow(),
            message
        );
    }

    /// Logs a message to console on ERROR level in red color.
    ///
    /// # Arguments
    /// * `message` - string to log
    ///
    /// # Examples
    /// ```
    /// ConsolePrinter::info(String::of("Error"));
    /// ```
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