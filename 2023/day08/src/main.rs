use anyhow::Result;
use clap::Parser;
use itertools::Itertools;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

/// Program to solve Day 8 of 2023 Advent of Code
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
    /// Run solution to part a of day 8.
    #[arg(short)]
    a: bool,

    /// Run solution to part b of day 8.
    #[arg(short)]
    b: bool,
}

#[derive(Debug, Clone)]
struct Map {
    directions: String,
    base: Box<Node>,
    current: Box<Node>,
}

impl Map {
    fn from_file(file: File) -> Result<Map> {
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        let mut base = Node {
            name: "AAA".to_string(),
            left: None,
            right: None,
        };
        let box_base = Box::new(base);
        let mut new_map = Map {
            directions: "".to_string(),
            base: box_base,
            current: box_base,
        };

        while reader.read_line(&mut line)? != 0 {
            line.clear();
        }

        Ok(new_map)
    }
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn solution_a(file: File) -> Result<u64> {
    let mut result: u64 = 0;

    Ok(result)
}

fn solution_b(file: File) -> Result<u64> {
    let mut result: u64 = 0;

    let mut reader = BufReader::new(file);
    let mut line = String::new();

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

        println!("The number of steps to reach ZZZ is {result}.");
    }
    if args.group.b {
        let file = File::open(args.file.clone())?;
        let result = solution_b(file)?;

        println!("The number of steps to reach ZZZ is {result}.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};

    #[test]
    fn test_solution_a_1() -> Result<()> {
        assert_eq!(solution_a(File::open("test-input-08-1.txt")?)?, 2);
        Ok(())
    }

    #[test]
    fn test_solution_a_2() -> Result<()> {
        assert_eq!(solution_a(File::open("test-input-08-2.txt")?)?, 6);
        Ok(())
    }

    #[test]
    fn test_solution_b_1() -> Result<()> {
        assert_eq!(solution_b(File::open("test-input-08-1.txt")?)?, 0);
        Ok(())
    }

    #[test]
    fn test_solution_b_2() -> Result<()> {
        assert_eq!(solution_b(File::open("test-input-08-2.txt")?)?, 0);
        Ok(())
    }
}
