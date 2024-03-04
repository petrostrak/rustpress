use std::{error::Error, fs};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The filename to read from
    #[arg(short)]
    filename: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Accept a filename as an input (return error if not found)
    // cargo run -- -f gutenberg.txt
    let args = Args::parse();

    // Read the file
    let file = fs::read_to_string(args.filename)?.parse::<String>()?;

    // determine the frequency of each character occurring within the text.

    Ok(())
}
