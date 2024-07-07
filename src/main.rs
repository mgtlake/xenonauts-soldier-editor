use clap::Parser;
use std::error::Error;
use std::fs;

mod save;
mod soldier;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the file to load
    path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let file = fs::read(args.path)?;
    println!("{:?}", save::parse_save(&file).unwrap().0);
    Result::Ok(())
}
