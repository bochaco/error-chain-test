use std::{
    backtrace::Backtrace,
    fs::File,
    io::{self, Read},
    path::Path,
    result::Result,
};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("This is MyError description: {msg}")]
pub struct MyError {
    msg: String,
    backtrace: Backtrace, // automatically detected
}

#[derive(Error, Debug)]
pub enum IoError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Invalid path provided: '{path}'")]
    InvalidPath {
        path: String,
        #[source]
        source: MyError,
    },
}

pub fn read_file(path: Option<&str>) -> Result<(), IoError> {
    let path = path.unwrap_or("default.json");
    println!("Reading file: {:?}", path);
    let path = Path::new(path);

    if !path.is_file() {
        Err(IoError::InvalidPath {
            path: path.display().to_string(),
            source: MyError {
                msg: "Path provided is not a file path or it doesn't exist!".to_string(),
                backtrace: Backtrace::capture(),
            },
        })
    } else {
        if let Some(ext) = path.extension() {
            let ext = ext.to_str().ok_or(IoError::InvalidPath {
                path: path.display().to_string(),
                source: MyError {
                    msg: "Failed to obtained file extension string!".to_string(),

                    backtrace: Backtrace::capture(),
                },
            })?;

            if ext == "json" {
                let mut f = File::open(path)?;
                let mut buffer = [0; 10];
                // read up to 10 bytes
                let n = f.read(&mut buffer)?;

                println!("The bytes read from file: {:?}", &buffer[..n]);
                Ok(())
            } else {
                Err(IoError::InvalidPath {
                    path: path.display().to_string(),
                    source: MyError {
                        msg: format!("Path provided is not with .json extension: {}", ext),

                        backtrace: Backtrace::capture(),
                    },
                })
            }
        } else {
            Err(IoError::InvalidPath {
                path: path.display().to_string(),
                source: MyError {
                    msg: "Path provided should contain a file with extension!".to_string(),
                    backtrace: Backtrace::capture(),
                },
            })
        }
    }
}
