use std::{collections::HashMap, error::Error, fs};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The filename to read from
    #[arg(short)]
    filename: String,
}

fn find_frequency_of_each_char(filename: String) -> Option<HashMap<char, u64>> {
    let mut occurrence_map: HashMap<char, u64> = HashMap::new();
    for c in filename.chars() {
        occurrence_map
            .entry(c)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    Some(occurrence_map)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Accept a filename as an input (return error if not found)
    // cargo run -- -f gutenberg.txt
    let args = Args::parse();

    // Read the file
    let file = fs::read_to_string(args.filename)?.parse::<String>()?;

    // determine the frequency of each character occurring within the text.
    let results = find_frequency_of_each_char(file).ok_or("No occurrences")?;
    for (k, v) in results {
        println!("{k} : {v}")
    }

    Ok(())
}
