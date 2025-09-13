use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

/// Program to solve Day 5 of 2023 Advent of Code
#[derive(Parser, Debug)]
struct Args {
    /// File to run solution code on
    #[arg()]
    file: String,

    #[clap(flatten)]
    group: Group,
}

#[derive(Parser, Debug)]
#[group(required = true)]
struct Group {
    /// Run solution to part a of day 5.
    #[arg(short)]
    a: bool,

    /// Run solution to part b of day 5.
    #[arg(short)]
    b: bool,
}

#[derive(Debug)]
struct Seed {
    val: u64,
}

impl Seed {
    fn new_seed(val: u64) -> Seed {
        Seed { val }
    }
}

#[derive(Debug)]
struct Soil {
    val: u64,
}

impl Soil {
    fn new_soil(val: u64) -> Soil {
        Soil { val }
    }
}

#[derive(Debug)]
struct Fertilizer {
    val: u64,
}

impl Fertilizer {
    fn new_fertilizer(val: u64) -> Fertilizer {
        Fertilizer { val }
    }
}

#[derive(Debug)]
struct Water {
    val: u64,
}

impl Water {
    fn new_water(val: u64) -> Water {
        Water { val }
    }
}

#[derive(Debug)]
struct Light {
    val: u64,
}

impl Light {
    fn new_light(val: u64) -> Light {
        Light { val }
    }
}

#[derive(Debug)]
struct Temperature {
    val: u64,
}

impl Temperature {
    fn new_temperature(val: u64) -> Temperature {
        Temperature { val }
    }
}

#[derive(Debug)]
struct Humidity {
    val: u64,
}

impl Humidity {
    fn new_humidity(val: u64) -> Humidity {
        Humidity { val }
    }
}

#[derive(Debug)]
struct Location {
    val: u64,
}

impl Location {
    fn new_location(val: u64) -> Location {
        Location { val }
    }
}



fn solution_a(file: File) -> Result<u64> {
    let mut result: u64 = 0;

    let mut reader = BufReader::new(file);
    let mut line = String::new();
    while reader.read_line(&mut line)? != 0 {}
    Ok(result)
}

fn solution_b(file: File) -> Result<u64> {
    let mut result: u64 = 0;

    let mut reader = BufReader::new(file);
    let mut line = String::new();
    while reader.read_line(&mut line)? != 0 {}
    Ok(result)
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.group.a {
        let file = File::open(args.file.clone())?;
        let result = solution_a(file)?;

        println!("The lowest location value for a seed is {result}.");
    }
    if args.group.b {
        let file = File::open(args.file.clone())?;
        let result = solution_b(file)?;

        println!("The sum of the calibration values is {result}.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_solution_a() -> Result<()> {
        assert_eq!(solution_a(File::open("test-input-05.txt")?)?, 35);
        Ok(())
    }
}
