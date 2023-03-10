use std::collections::HashMap;
use crate::error::ErrorCode;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path;
use serde_json::{from_str, Map, Value};

pub struct FileUtil {}

impl FileUtil {
    const CONFIG_FILE_FILENAME: &'static str = "vault_config.json";

    /// Returns the path to config file in HOME directory.
    ///
    /// # Examples
    /// ```
    /// println!("{}", get_config_file_path());
    /// >> /Users/username/vault/vault_config.json
    /// ```
    pub fn get_config_file_path() -> String {
        format!("{}/vault/{}", env::var("HOME").unwrap(), FileUtil::CONFIG_FILE_FILENAME)
    }

    /// Returns the boolean value based on config file
    /// existence.
    ///
    /// # Examples
    /// ```
    /// println!("{}", config_file_exists());
    /// >> true
    /// ```
    pub fn config_file_exists() -> bool {
        path::Path::new(FileUtil::get_config_file_path().as_str()).exists()
    }


    /// Creates empty JSON config file in HOME directory.
    ///
    /// # Examples
    /// ```
    /// create_empty_config_file();
    /// ```
    ///
    /// # Returns
    /// - `Ok(true)` if file was created successfully
    /// - `Err(ErrorCode::FileWriteFailed)` if the file was not created due
    /// to an error
    pub fn create_empty_config_file() -> Result<bool, ErrorCode> {
        let create_file_result = File::create(FileUtil::get_config_file_path());
        if create_file_result.is_err() {
            return Err(ErrorCode::FileWriteFailed);
        }
        let mut f_result = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(FileUtil::get_config_file_path());
        if f_result.is_ok() {
            let mut f = f_result.unwrap();
            let write_result = f.write_all(b"{}");
            if write_result.is_ok() {
                f.flush();
                Ok(true)
            } else {
                Err(ErrorCode::FileWriteFailed)
            }
        } else {
            Err(ErrorCode::FileWriteFailed)
        }
    }

    /// Reads JSON config file.
    ///
    /// # Examples
    /// ```
    /// read_config_json();
    /// ```
    ///
    /// # Returns
    /// - `Ok(Value)` if file was read successfully (Value representing
    /// the JSON file)
    /// - `Err(ErrorCode::InvalidJsonFormat)` if the JSON file was not formatted
    /// correctly
    /// - `Err(ErrorCode::BrokenFile)` if the file seemed to be broken while
    /// attempting to open it
    /// - `Err(ErrorCode::FileDoesNotExist)` if the file could not be found
    pub fn read_config_json() -> Result<Value, ErrorCode> {
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

    /// Fetches specified JSON property from the file.
    ///
    /// # Arguments
    /// * `name` - The key of yhe property to read
    ///
    /// # Examples
    /// ```
    /// read_config_json_property(name: &str);
    /// ```
    ///
    /// # Returns
    /// - `Ok(Value)` if file was read successfully (Value representing
    /// the JSON file)
    /// - `Err(ErrorCode::InvalidJsonFormat)` if the JSON file was not formatted
    /// correctly
    /// - `Err(ErrorCode::BrokenFile)` if the file seemed to be broken while
    /// attempting to open it
    /// - `Err(ErrorCode::FileDoesNotExist)` if the file could not be found
    /// - `Err(ErrorCode::JsonPropertyNotFound)` if desired property was
    /// not found
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

    /// Fetches a map of all JSON properties from the file.
    ///
    /// # Examples
    /// ```
    /// list_all_config_json_properties();
    /// ```
    ///
    /// # Returns
    /// - `Ok(Value)` if file was read successfully (Value representing
    /// the JSON file)
    /// - `Err(ErrorCode::InvalidJsonFormat)` if the JSON file was not formatted
    /// correctly
    /// - `Err(ErrorCode::BrokenFile)` if the file seemed to be broken while
    /// attempting to open it
    /// - `Err(ErrorCode::FileDoesNotExist)` if the file could not be found
    pub fn list_all_config_json_properties() -> Result<Map<String, Value>, ErrorCode> {
        let json_result = FileUtil::read_config_json();
        if json_result.is_ok() {
            let mut json = json_result.unwrap();
            Ok(json.as_object().unwrap().clone())
        } else {
            Err(json_result.err().unwrap())
        }
    }

    /// Overwrites existing JSON file wit new value.
    ///
    /// # Arguments
    /// * `new_json` - The JSON string to overwrite current config file
    /// with
    ///
    /// # Examples
    /// ```
    /// overwrite_json_file(new_json: Value);
    /// ```
    ///
    /// # Returns
    /// - `Ok(Value)` if file was read successfully (Value representing
    /// the JSON file)
    /// - `Err(ErrorCode::FileWriteFailed)` if the file failed to be
    /// overwritten
    pub fn overwrite_json_file(new_json: Value) -> Result<bool, ErrorCode> {
        let mut f_result = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(FileUtil::get_config_file_path());
        if f_result.is_ok() {
            let mut f = f_result.unwrap();
            let write_result = f.write_all(serde_json::to_string(&new_json).unwrap().as_bytes());
            if write_result.is_ok() {
                f.flush();
                Ok(true)
            } else {
                Err(ErrorCode::FileWriteFailed)
            }
        } else {
            Err(ErrorCode::FileWriteFailed)
        }
    }

    /// Overwrites existing JSON file wit new value.
    ///
    /// # Arguments
    /// * `v` - old JSON object
    /// * `key` - JSON key to add
    /// * `value` - value for the given key
    ///
    /// # Examples
    /// ```
    /// merge(v: &Value, key: String, value: String);
    /// ```
    ///
    /// # Returns
    /// - `Value` new JSON object with added property
    pub fn merge(v: &Value, key: String, value: String) -> Value {
        match v {
            Value::Object(m) => {
                let mut m = m.clone();
                m.insert(key.clone(), Value::String(value.clone()));
                Value::Object(m)
            }
            v => v.clone()
        }
    }

}