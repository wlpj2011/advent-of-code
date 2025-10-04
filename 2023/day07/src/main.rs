use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

/// Program to solve Day 7 of 2023 Advent of Code
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
      /// Run solution to part a of day 7.
      #[arg(short)]
      a: bool,
  
      /// Run solution to part b of day 7.
      #[arg(short)]
      b: bool,
}


fn solution_a(file: File) -> Result<u64> {
    let mut result: u64 = 0;

    let mut reader = BufReader::new(file);
    let mut line = String::new();
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
    
        println!("The total winnings of your hands is {result}.");
    }
    if args.group.b {
        let file = File::open(args.file.clone())?;
        let result = solution_b(file)?;
    
        println!("The total winnings of your hands is {result}.");
    }
    

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_solution_a() -> Result<()> {
        assert_eq!(solution_a(File::open("test-input-07.txt")?)?, 6440);
        Ok(())
    }

    #[test]
    fn test_solution_b() -> Result<()> {
        assert_eq!(solution_b(File::open("test-input-07.txt")?)?, 0);
        Ok(())
    }

    
}