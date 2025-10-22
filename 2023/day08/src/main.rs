use anyhow::Result;
use clap::Parser;
use rust_tools::math;
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
    cur_locations: Vec<String>,
}

impl Map {
    fn from_file_a(file: File) -> Result<Map> {
        let mut reader = BufReader::new(file);
        let mut line = String::new();

        let mut new_map = Map {
            directions: "".to_string(),
            map: HashMap::new(),
            cur_locations: Vec::new(),
        };

        let mut done_first: bool = false;

        while reader.read_line(&mut line)? != 0 {
            if !done_first {
                new_map.directions = line.clone().strip_suffix("\n").unwrap().to_owned();
                done_first = true;
            } else {
                let line_parts: Vec<_> = line.split("=").collect();
                if line_parts.len() == 1 {
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
        new_map.cur_locations.push("AAA".to_owned());
        Ok(new_map)
    }

    fn from_file_b(file: File) -> Result<Map> {
        let mut reader = BufReader::new(file);
        let mut line = String::new();

        let mut new_map = Map {
            directions: "".to_string(),
            map: HashMap::new(),
            cur_locations: Vec::new(),
        };

        let mut done_first: bool = false;

        while reader.read_line(&mut line)? != 0 {
            if !done_first {
                new_map.directions = line.clone().strip_suffix("\n").unwrap().to_owned();
                done_first = true;
            } else {
                let line_parts: Vec<_> = line.split("=").collect();
                if line_parts.len() == 1 {
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
        for (location, (_, _)) in new_map.map.clone() {
            if location.chars().nth(location.len() - 1).unwrap() == 'A' {
                new_map.cur_locations.push(location.clone());
            }
        }
        Ok(new_map)
    }

    fn take_step(&mut self, step_num: u64, index_num: usize) {
        for (path_num, cur_location) in self.cur_locations.clone().iter().enumerate() {
            if index_num == path_num {
                //cur_location.chars().nth(cur_location.len() - 1).unwrap() != 'Z' {
                let cur_decision = self.map.get(cur_location).unwrap().clone();
                let direction_index = (step_num % (self.directions.len() as u64)) as usize;
                let cur_direction = self.directions.chars().nth(direction_index).unwrap();
                let mut cur_direction_num: usize = 0;
                if cur_direction == 'R' {
                    cur_direction_num = 1;
                }
                if cur_direction_num == 0 {
                    self.cur_locations[path_num] = cur_decision.0;
                } else if cur_direction_num == 1 {
                    self.cur_locations[path_num] = cur_decision.1;
                }
            }
        }
    }
}

fn solution_a(file: File) -> Result<u64> {
    let mut result: u64 = 0;

    let mut map = Map::from_file_a(file)?;
    while map.cur_locations[0] != "ZZZ".to_owned() {
        map.take_step(result, 0);
        result += 1;
    }

    Ok(result)
}

fn solution_b(file: File) -> Result<u64> {
    let mut results: Vec<u64> = Vec::new();

    let mut map = Map::from_file_b(file)?;
    for (location_num, location) in map.cur_locations.clone().iter().enumerate() {
        results.push(0);
        while map.cur_locations[location_num]
            .chars()
            .nth(location.len() - 1)
            .unwrap()
            != 'Z'
        {
            let cur_result = results.pop().unwrap();
            map.take_step(cur_result, location_num);
            results.push(cur_result + 1);
        }
    }
    let mut result = results.pop().unwrap();
    while results.len() >= 1 {
        result = math::lcm(result, results.pop().unwrap());
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

        println!("The number of steps to reach __Z in all worlds simultaneously is {result}.");
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
    fn test_solution_b() -> Result<()> {
        assert_eq!(solution_b(File::open("test-input-08-3.txt")?)?, 6);
        Ok(())
    }
}
