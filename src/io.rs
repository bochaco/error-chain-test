use bytes::Bytes;
use std::{
    backtrace::Backtrace,
    fs::File,
    io::{self, Read},
    path::Path,
    result::Result,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InvalidPathError {
    #[error("Path provided should contain a file with extension: {0}")]
    NoFileExtension(String),
    #[error("Path provided is not with .json extension: {0}")]
    InvalidFileExtension(String),
    #[error("Failed to obtain file extension string: {0}")]
    FileExtFormat(String),
    #[error("Path provided is not a file path or it doesn't exist: {0}")]
    NotFile(String),
}

#[derive(Error, Debug)]
pub enum IoError {
    #[error("IO error")]
    Io {
        #[from]
        source: io::Error,
        backtrace: Backtrace, // automatically detected
    },
    #[error("Invalid path provided")]
    InvalidPath {
        #[from]
        source: InvalidPathError,
        backtrace: Backtrace, // automatically detected
    },
}

pub fn read_file(path: &str) -> Result<Bytes, IoError> {
    println!("Reading file: {:?}", path);
    let path = Path::new(path);

    if !path.is_file() {
        Err(InvalidPathError::NotFile(path.display().to_string()).into())
    } else {
        if let Some(ext) = path.extension() {
            let ext = ext
                .to_str()
                .ok_or(InvalidPathError::FileExtFormat(path.display().to_string()))?;

            let json_ext = "json".to_string();
            if ext != json_ext {
                Err(InvalidPathError::InvalidFileExtension(json_ext).into())
            } else {
                let mut f = File::open(path)?;
                let mut buffer = [0; 1024];
                let n = f.read(&mut buffer)?;

                println!("Read {} bytes from file", n);
                Ok(Bytes::copy_from_slice(&buffer))
            }
        } else {
            Err(InvalidPathError::NoFileExtension(path.display().to_string()).into())
        }
    }
}
