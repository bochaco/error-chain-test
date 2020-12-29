#![feature(backtrace)]

mod io;

use anyhow::Result;
use io::read_file;
use std::env;

fn main() -> Result<()> {
    println!("Let's read a JSON file first...");
    let mut args = env::args();
    args.next();
    read_file(args.next().as_deref())?;

    Ok(())
}
