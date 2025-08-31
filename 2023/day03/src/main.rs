use anyhow::Result;
use clap::Parser;
use clap::builder::Str;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

/// Program to solve Day 3 of 2023 Advent of Code
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
    #[arg(short)]
    a: bool,

    /// Run solution to part b of day 1.
    #[arg(short)]
    b: bool,
}

const SYMBOLS: &str = "!@#$%^&*()_-+=|\\<>,?/;:~`";

#[derive(Debug, Clone)]
struct Context {
    line1: String,
    line2: String,
    line3: String,
}

impl Context {
    fn from_str(line: &str) -> Context {
        Context {
            line1: "".to_string(),
            line2: "".to_string(),
            line3: line.to_string(),
        }
    }

    fn update_context(&mut self, new_line: &str) {
        self.line1 = self.line2.clone();
        self.line2 = self.line3.clone();
        self.line3 = new_line.to_string();
    }

    fn find_parts(&self) -> Vec<u64> {
        let parts = Vec::new();

        parts
    }
}
fn solution_a(file: File) -> Result<u64> {
    let mut result: u64 = 0;

    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut context: Context = Context::from_str(&line);
    while reader.read_line(&mut line)? != 0 {
        context.update_context(&line);
        let parts = context.find_parts();
        for part in parts {
            result += part;
        }
        dbg!(&context);
        line.clear();
    }
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

        println!("The sum of the engine part numbers is {result}.");
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
        assert_eq!(solution_a(File::open("test-input-03.txt")?)?, 4361);
        Ok(())
    }
}
