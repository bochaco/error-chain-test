#![feature(backtrace)]

mod data;
mod io;

use anyhow::Result;
use data::read_data;
use std::env;

fn main() -> Result<()> {
    println!("Let's read a JSON file first...");
    let mut args = env::args();
    args.next();
    read_data(args.next().as_deref())?;

    Ok(())
}
