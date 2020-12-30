use super::io::{read_file, IoError};
use serde_json::Value;
use std::{backtrace::Backtrace, result::Result, str};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("Failed to read config file")]
    ReadConfigError { source: FileReadError },
    #[error("Failed to read data file")]
    ReadDataError { source: FileReadError },
    #[error("Config entry for default file path not found. Missing 'default' attribute")]
    ConfigInfoNotFound,
    #[error("Contact information not found: {0}")]
    ContactInfoNotFound(String),
}

#[derive(Error, Debug)]
pub enum FileReadError {
    #[error("Failed to read file")]
    FileError {
        #[from]
        source: IoError,
        backtrace: Backtrace,
    },
    #[error("File's content doesnt' contain valid UTF8 bytes")]
    Utf8Error {
        #[from]
        source: str::Utf8Error,
        backtrace: Backtrace,
    },
    #[error("Couldn't parse file's content")]
    ParseError {
        #[from]
        source: serde_json::Error,
        backtrace: Backtrace,
    },
}

// read name and phone number from file
pub fn read_data(path: Option<&str>) -> Result<(String, String), DataError> {
    let file_path = match path {
        Some(p) => p.to_string(),
        None => {
            let value =
                read_config_file().map_err(|source| DataError::ReadConfigError { source })?;
            let default = value.get("default").ok_or(DataError::ConfigInfoNotFound)?;
            println!("Using default file path: {}", default);
            default.to_string()
        }
    };

    let value = read_data_file(&file_path).map_err(|source| DataError::ReadDataError { source })?;
    let name = value
        .get("name")
        .ok_or(DataError::ContactInfoNotFound("name".to_string()))?;

    let phones = value
        .get("phones")
        .ok_or(DataError::ContactInfoNotFound("phones".to_string()))?;

    Ok((name.to_string(), phones[0].to_string()))
}

// private function to read default file path from config file
fn read_config_file() -> Result<Value, FileReadError> {
    println!("Reading config file...");
    let data = read_file("config.json")?;
    let config_str = str::from_utf8(&data)?;
    /*let config_str = r#"
    {
        "default": "myfile.json"
    }"#;*/
    println!("Config read: {}", config_str);
    let v: Value = serde_json::from_str(config_str)?;

    Ok(v)
}

// private function to read data from file path
fn read_data_file(file_path: &str) -> Result<Value, FileReadError> {
    println!("Reading data file...");
    let data = read_file(file_path)?;

    let data_str = str::from_utf8(&data)?;
    /*let data_str = r#"
    {
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;*/
    println!("Data read: {}", data_str);
    let v: Value = serde_json::from_str(data_str)?;

    Ok(v)
}
