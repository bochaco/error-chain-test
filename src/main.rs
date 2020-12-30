#![feature(backtrace)]

mod data;
mod io;

use anyhow::{anyhow, Result};
use data::read_data;
use std::{env, error::Error};

fn main() -> Result<()> {
    println!("Let's read a JSON file first...");
    let mut args = env::args();
    args.next();
    let filepath = args.next();
    let just_test_generic_error = args.next().is_some();
    match read_data(filepath.as_deref(), just_test_generic_error) {
        Err(err) => {
            println!("TOP LEVEL ERROR: {}", err);
            let mut curr_err = err.source();
            let mut index = 0;
            println!("-----------------");
            while let Some(src) = curr_err {
                println!("SOURCE #{}: {}", index, src);
                println!("SOURCE #{} BACKTRACE: {:#?}", index, src.backtrace());
                curr_err = src.source();
                index += 1;
                println!("-----------------");
            }
            // TODO: review why this cannot be done, a conversion is missing
            // due to the SourceIsGeneric(#[from] Box<dyn std::error::Error>) definition
            // Err(anyhow!(err))
            Err(anyhow!(err.to_string()))
        }
        Ok((name, phone)) => {
            println!("Please call {} at the number {}", name, phone);
            Ok(())
        }
    }
}
