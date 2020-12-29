use super::io::{read_file, IoError};
use std::result::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("Failed to read file")]
    ReadError(#[from] IoError),
}

pub fn read_data(path: Option<&str>) -> Result<(), DataError> {
    read_file(path)?;
    Ok(())
}
