/*
Implement a function that reads integers from a file, line by line.
The file may have some lines that are not integers.
The function has to handle errors correctly.
*/

use std::fs;
use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, long, default_value = "input.txt", help = "File path to read integers from.")]
    input_path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let path = args.input_path;
    let input_file = fs::read_to_string(&path)
        .with_context(|| format!("Could not read file from path '{}'.", &path))?;

    let integers:Vec<i32> = input_file.lines().filter_map(|l| l.parse().ok()).collect();

    for integer in integers{
        println!("{}", integer);
    }

    Ok(())
}
