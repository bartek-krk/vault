#[derive(Debug)]
pub enum ErrorCode {
    FileDoesNotExist,
    BrokenFile,
    InvalidJsonFormat,
    JsonPropertyNotFound,
    UnsupportedOperation,
    NonexistentArgument
}

impl ErrorCode {
    pub fn get_code(&self) -> String {
        match *self {
            ErrorCode::FileDoesNotExist => String::from("File does not exist"),
            ErrorCode::BrokenFile => String::from("The file seemed to be broken while attempting to read it"),
            ErrorCode::InvalidJsonFormat => String::from("JSON file formatted incorrectly"),
            ErrorCode::JsonPropertyNotFound => String::from("JSON property not found in object"),
            ErrorCode::UnsupportedOperation => String::from("Given operation is not supported"),
            ErrorCode::NonexistentArgument => String::from("Required argument was not provided")
        }
    }
}