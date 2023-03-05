use std::{env, io};
use std::fs::File;
use crate::clipboard::Clipboard;
use crate::cmd::{Cmd, Command};
use crate::utils::ConsolePrinter;
use crate::file::FileUtil;
use crate::help::Help;

mod utils;
mod file;
mod error;
mod cmd;
mod help;
mod clipboard;

fn main() {
    let stdin = io::stdin();

    let operation_type = Cmd::resolve_operation_type(
        env::args().collect()
    );

    if !FileUtil::config_file_exists() {
        match FileUtil::create_empty_config_file() {
            Ok(_val) => {
                ConsolePrinter::warn(String::from(
                    "Config file did not exist. Created empty one."
                ))
            }
            Err(error_code) => {
                ConsolePrinter::error(error_code.get_code())
            }
        }
    }

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
                                    Clipboard::copy_to_clipboard(val.as_str());
                                    ConsolePrinter::info(String::from("Value copied to clipboard!"))
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
                Command::AddValue => {
                    let mut key_buffer = String::new();
                    println!("{}", "Key?");
                    stdin.read_line(&mut key_buffer);
                    key_buffer = key_buffer.strip_suffix("\n").unwrap().to_string();
                    let mut value_buffer = String::new();
                    println!("{}", "Value?");
                    stdin.read_line(&mut value_buffer);
                    value_buffer = value_buffer.strip_suffix("\n").unwrap().to_string();

                    match FileUtil::read_config_json() {
                        Ok(mut json) => {
                            json = FileUtil::merge(&json, key_buffer, value_buffer);
                            FileUtil::overwrite_json_file(json);
                        }
                        Err(error_code) => {
                            ConsolePrinter::error(error_code.get_code());
                        }
                    };
                }
            }
        },
        Err(error_code) => {
            ConsolePrinter::error(error_code.get_code());
        }
    }
}
