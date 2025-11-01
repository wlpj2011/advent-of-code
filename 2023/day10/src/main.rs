use anyhow::Result;
use clap::Parser;
use itertools::Itertools;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

/// Program to solve Day 10 of 2023 Advent of Code
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
    /// Run solution to part a.
    #[arg(short)]
    a: bool,

    /// Run solution to part b.
    #[arg(short)]
    b: bool,
}





fn solution_a(file: File) -> Result<i64> {
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut result = 0;

    while reader.read_line(&mut line)? != 0 {

        line.clear();
    }

    Ok(result)
}

fn solution_b(file: File) -> Result<i64> {
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut result = 0;

    while reader.read_line(&mut line)? != 0 {
        line.clear();
    }

    Ok(result)
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.group.a {
        let file = File::open(args.file.clone())?;
        let result = solution_a(file)?;

        println!("The furthest point from the starting position is {result} steps along the loop.");
    }
    if args.group.b {
        let file = File::open(args.file.clone())?;
        let result = solution_b(file)?;

        println!("The furthest point from the starting position is {result} steps along the loop.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_solution_a_1() -> Result<()> {
        assert_eq!(solution_a(File::open("test-input-10-1.txt")?)?, 4);
        Ok(())
    }

    #[test]
    fn test_solution_a_2() -> Result<()> {
        assert_eq!(solution_a(File::open("test-input-10-2.txt")?)?, 8);
        Ok(())
    }

    #[test]
    fn test_solution_b_1() -> Result<()> {
        assert_eq!(solution_b(File::open("test-input-10-1.txt")?)?, 0);
        Ok(())
    }

    #[test]
    fn test_solution_b_2() -> Result<()> {
        assert_eq!(solution_b(File::open("test-input-10-2.txt")?)?, 0);
        Ok(())
    }
}
