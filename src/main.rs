use std::env;
use crate::cmd::{Cmd, Command};
use crate::utils::ConsolePrinter;
use crate::file::FileUtil;
use crate::help::Help;

mod utils;
mod file;
mod error;
mod cmd;
mod help;

fn main() {
    let operation_type = Cmd::resolve_operation_type(
        env::args().collect()
    );

    match operation_type {
        Ok(operation) => {
            match operation {
                Command::Help => {
                    Help::get_help();
                },
                Command::GetValue => {
                    match Cmd::get_supplementary_arg(env::args().collect()) {
                        Ok(key) => {
                            match FileUtil::read_config_json_property(key.as_str()) {
                                Ok(val) => {
                                    ConsolePrinter::info(format!("Value for key \"{}\" is \"{}\"", key, val));
                                }
                                Err(error) => {
                                    ConsolePrinter::warn(format!("Key was \"{}\"", key));
                                    ConsolePrinter::error(error.get_code());
                                }
                            };
                        },
                        Err(error_code) => {
                            ConsolePrinter::error(error_code.get_code());
                        }
                    }
                }
            }
        },
        Err(error_code) => {
            ConsolePrinter::error(error_code.get_code());
        }
    }
}
