use clap::Parser;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use anyhow::Result;

/// Program to solve Day 1 of 2023 Advent of Code
#[derive(Parser, Debug)]
struct Args {
    /// File to run solution code on
    #[arg()]
    file : String,
}



fn main() -> Result<()>{
    let args = Args::parse();

    let file = File::open(args.file)?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    let len= reader.read_line(&mut line)?;
    println!("The first line is {len} bytes long");

    Ok(())
}
