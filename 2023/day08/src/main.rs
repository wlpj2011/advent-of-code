use anyhow::Result;
use clap::Parser;
use itertools::Itertools;
use std::collections::HashMap;
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
    map: HashMap<String, (String, String)>,
}

impl Map {
    fn from_file(file: File) -> Result<Map> {
        let mut reader = BufReader::new(file);
        let mut line = String::new();

        let mut new_map = Map {
            directions: "".to_string(),
            map: HashMap::new(),
        };

        let mut done_first: bool = false;

        while reader.read_line(&mut line)? != 0 {
            if !done_first {
                new_map.directions = line.clone().strip_suffix("\n").unwrap().to_owned();
                done_first = true;
            } else {
                let line_parts: Vec<_> = line.split("=").collect();
                if line_parts.len() == 1{
                    continue;
                }
                let first_part = line_parts[0].trim().to_owned();
                let second_part = line_parts[1].trim();
                let line_parts: Vec<_> = second_part.split(",").collect();
                let first_dir = line_parts[0][1..].to_owned();
                let second_dir = line_parts[1][1..line_parts[1].len() - 1].to_owned();
                new_map.map.insert(first_part, (first_dir, second_dir));
            }
            line.clear();
        }

        Ok(new_map)
    }
}

fn solution_a(file: File) -> Result<u64> {
    let mut result: u64 = 0;

    let map = Map::from_file(file)?;
    let mut cur_location = "AAA".to_owned();
    while cur_location != "ZZZ".to_owned() {
        let cur_decision = map.map.get(&cur_location).unwrap().clone();
        let direction_index = (result % (map.directions.len() as u64)) as usize;
        let cur_direction = map.directions.chars().nth(direction_index).unwrap();
        let mut cur_direction_num: usize = 0;
        if cur_direction == 'R' {
            cur_direction_num = 1;
        }
        if cur_direction_num == 0 {
            cur_location = cur_decision.0;
        } else if cur_direction_num == 1 {
            cur_location = cur_decision.1;
        }
        result += 1;
    }

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
