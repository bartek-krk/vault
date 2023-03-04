use crate::error::ErrorCode;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path;
use serde_json::from_str;

pub struct FileUtil {}

impl FileUtil {
    const CONFIG_FILE_FILENAME: &'static str = "vault_config.json";

    pub fn get_config_file_path() -> String {
        format!("{}/{}", env::var("HOME").unwrap(), FileUtil::CONFIG_FILE_FILENAME)
    }

    pub fn config_file_exists() -> bool {
        path::Path::new(FileUtil::get_config_file_path().as_str()).exists()
    }

    pub fn read_config_json() -> Result<serde_json::Value, ErrorCode> {
        let file_result = File::open(FileUtil::get_config_file_path());
        if file_result.is_ok() {
            let mut contents = String::new();
            let mut file = file_result.unwrap();
            let read_result = file.read_to_string(&mut contents);
            if read_result.is_ok() {
                let parse_result = from_str(&*String::from(contents));
                if parse_result.is_ok() {
                    Ok(parse_result.unwrap())
                } else {
                    Err(ErrorCode::InvalidJsonFormat)
                }
            } else {
                Err(ErrorCode::BrokenFile)
            }
        } else {
            Err(ErrorCode::FileDoesNotExist)
        }
    }

    pub fn read_config_json_property(name: &str) -> Result<String, ErrorCode> {
        let json_result = FileUtil::read_config_json();
        if json_result.is_ok() {
            let mut json = json_result.unwrap();
            let found_value = json[name].as_str();
            match found_value {
                None => Err(ErrorCode::JsonPropertyNotFound),
                _ => Ok(String::from(found_value.unwrap()))
            }
        } else {
            Err(json_result.err().unwrap())
        }
    }

}