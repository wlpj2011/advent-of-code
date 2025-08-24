use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

/// Program to solve Day 1 of 2023 Advent of Code
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
      /// Run solution to part a of day 1.
      #[arg(short, default_value = "true")]
      a: bool,
  
      /// Run solution to part b of day 1.
      #[arg(short)]
      b: bool,
}

fn calibration_value(line: &str) -> Result<u64> {
    Ok(0)
}

fn solution_a(file: File) -> Result<u64> {
    let mut result: u64 = 0;

    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let len = reader.read_line(&mut line);

    result += calibration_value(&line)?;

    Ok(result)
}

fn solution_b(file: File) -> Result<u64> {
    let mut result: u64 = 0;

    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let len = reader.read_line(&mut line);

    result += calibration_value(&line)?;

    Ok(result)
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.group.a {
        let file = File::open(args.file.clone())?;
        let result = solution_a(file)?;
    
        println!("The sum of the calibration values is {result}.");
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
    fn test_solution() -> Result<()> {
        assert_eq!(solution_a(File::open("test-input-01.txt")?)?, 142);
        Ok(())
    }

    #[test]
    fn test_calibration_1() -> Result<()> {
        assert_eq!(calibration_value("1abc2")?, 12);
        Ok(())
    }

    #[test]
    fn test_calibration_2() -> Result<()> {
        assert_eq!(calibration_value("pqr3stu8vwx")?, 38);
        Ok(())
    }

    #[test]
    fn test_calibration_3() -> Result<()> {
        assert_eq!(calibration_value("a1b2c3d4e5f")?, 15);
        Ok(())
    }

    #[test]
    fn test_calibration_4() -> Result<()> {
        assert_eq!(calibration_value("treb7uchet")?, 77);
        Ok(())
    }
}
