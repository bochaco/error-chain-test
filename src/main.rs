#![feature(backtrace)]

mod data;
mod io;

use anyhow::{bail, Result};
use data::read_data;
use std::{env, error::Error};

fn main() -> Result<()> {
    println!("Let's read a JSON file first...");
    let mut args = env::args();
    args.next();
    match read_data(args.next().as_deref()) {
        Err(err) => {
            println!("-----------------");
            println!("BACKTRACE: {:?}", err.backtrace());
            println!("ERR: {:?}", err);
            println!("ERR DISPLAY: {}", err);
            let mut curr_err = err.source();
            let mut index = 0;
            while let Some(src) = curr_err {
                println!("SOURCE {}: {}", index, src);
                curr_err = src.source();
                index += 1;
            }
            println!("-----------------");
            bail!(err)
        }
        Ok((name, phone)) => {
            println!("Please call {} at the number {}", name, phone);
            Ok(())
        }
    }
}
