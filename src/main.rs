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
    let save = save::parse_save(&file).unwrap().1;
    println!("File length {}", file.len());
    println!("Before soldiers length {}", save.before_soldiers.len());
    println!("After soldiers length {}", save.after_soldiers.len());
    // println!("{:x?}", save.after_soldiers);
    Result::Ok(())
}
