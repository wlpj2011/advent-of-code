use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

/// Program to solve Day 6 of 2023 Advent of Code
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
      /// Run solution to part a of day 6.
      #[arg(short)]
      a: bool,
  
      /// Run solution to part b of day 6.
      #[arg(short)]
      b: bool,
}
#[derive(Debug, Clone, Copy)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn from_time(time: u64) -> Race {
        Race{time, distance: 0}
    }

    fn update_distance(&mut self, distance: u64) {
        self.distance = distance;
    }
}

fn solution_a(file: File) -> Result<u64> {
    let mut result: u64 = 1;

    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut races: Vec<Race> = Vec::new();
    while reader.read_line(&mut line)? != 0 {
        

        line.clear();
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
    
        println!("The total number of ways you can win is {result}.");
    }
    if args.group.b {
        let file = File::open(args.file.clone())?;
        let result = solution_b(file)?;
    
        println!("The total number of ways you can win is {result}.");
    }
    

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_solution_a() -> Result<()> {
        assert_eq!(solution_a(File::open("test-input-06.txt")?)?, 288);
        Ok(())
    }

    
}