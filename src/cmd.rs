use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::error::ErrorCode;

#[derive(Debug, EnumIter, PartialEq)]
pub enum Command {
    Help,
    GetValue,
    AddValue,
    ListAll
}

impl Command {
    /// Returns command code providable by the user.
    ///
    /// # Examples
    /// ```
    /// Command::Help.get_command();
    /// >> "help"
    /// ```
    ///
    /// # Returns
    /// - `String` command
    pub fn get_command(&self) -> String {
        match *self {
            Command::Help => String::from("help"),
            Command::GetValue => String::from("get"),
            Command::AddValue => String::from("add"),
            Command::ListAll => String::from("ls"),
        }
    }

    /// Determines operation by the command provided by the user.
    ///
    /// # Examples
    /// ```
    /// get_by_command_str("help");
    /// >> Command::Help
    /// ```
    ///
    /// # Returns
    /// - `String` command enum value
    pub fn get_by_command_str(command_str: &str) -> Result<Command, ErrorCode> {
        for command in Command::iter() {
            if command.get_command().eq(command_str) {
                return Ok(command);
            }
        }
        Err(ErrorCode::UnsupportedOperation)
    }
}

pub struct Cmd {}

impl Cmd {
    /// Determines operation type from program arguments.
    ///
    /// # Examples
    /// ```
    /// resolve_operation_type(env::args().collect());
    /// >> Command::Help
    /// ```
    ///
    /// # Returns
    /// - `String` command enum value
    pub fn resolve_operation_type(args: Vec<String>) -> Result<Command, ErrorCode> {
        if args.len() < 2 {
            return Err(ErrorCode::NonexistentArgument);
        } else {
            let command_str: String = args[1].to_string();
            Command::get_by_command_str(command_str.as_str())
        }
    }

    /// Determines supplementary argument from program arguments.
    ///
    /// # Examples
    /// ```
    /// >> vault get key
    /// get_supplementary_arg(env::args().collect());
    /// >> "key"
    /// ```
    ///
    /// # Returns
    /// - `String` supplementary argument provided by the user
    pub fn get_supplementary_arg(args: Vec<String>) -> Result<String, ErrorCode> {
        if args.len() < 3 {
            return Err(ErrorCode::NonexistentArgument);
        } else {
            Ok(args[2].to_string())
        }
    }
}