use std::{
    backtrace::Backtrace,
    fs::File,
    io::{self, Read},
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
    if path.is_empty() {
        return Err(IoError::InvalidPath {
            path: path.to_string(),
            source: MyError {
                msg: "Path provided is empty!".to_string(),
                backtrace: Backtrace::capture(),
            },
        });
    } else if path.ends_with("/") {
        return Err(IoError::InvalidPath {
            path: path.to_string(),
            source: MyError {
                msg: "Path provided is not a file but a directory!".to_string(),
                backtrace: Backtrace::capture(),
            },
        });
    } else {
        let mut f = File::open(path)?;
        let mut buffer = [0; 10];
        // read up to 10 bytes
        let n = f.read(&mut buffer)?;

        println!("The bytes read fro file: {:?}", &buffer[..n]);
    }

    Ok(())
}
