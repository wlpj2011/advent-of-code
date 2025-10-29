use anyhow::Result;
use clap::Parser;
use itertools::Itertools;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

/// Program to solve Day 9 of 2023 Advent of Code
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

#[derive(Debug, Clone)]
struct History {
    vals: Vec<i64>,
    diffs: Vec<Vec<i64>>,
    kth_diff: Option<i64>,
    k: Option<u64>,
}

impl History {
    fn from_string(string: String) -> Result<History> {
        let mut vals: Vec<_> = Vec::new();
        for num in string.split(" ").collect::<Vec<_>>() {
            vals.push(num.trim().parse::<i64>()?)
        }
        Ok(History {
            vals,
            diffs: Vec::new(),
            kth_diff: None,
            k: None,
        })
    }

    fn find_diff(&mut self) {
        loop {
            if self.diffs.is_empty() {
                let mut next_diffs: Vec<_> = Vec::new();
                for (first, second) in self.vals.iter().tuple_windows() {
                    next_diffs.push(second - first);
                }
                self.diffs.push(next_diffs);
            } else {
                let mut next_diffs: Vec<_> = Vec::new();
                for (first, second) in self.diffs.last().unwrap().iter().tuple_windows() {
                    next_diffs.push(second - first);
                }
                self.diffs.push(next_diffs);
            }
            if self.diffs.last().unwrap().iter().all(|x| *x == 0) {
                self.k = Some((self.diffs.len() - 1) as u64);
                self.kth_diff = Some(*self.diffs[(self.k.unwrap() - 1) as usize].last().unwrap());
                break;
            }
        }
    }

    fn compute_next(&mut self) -> i64 {
        for i in (0..(self.diffs.len() - 1)).rev() {
            let next_diff = *self.diffs[i].last().unwrap() + self.diffs[i + 1].last().unwrap();
            self.diffs[i].push(next_diff);
        }
        let new_val = (*self.diffs.first().unwrap()).last().unwrap() + self.vals.last().unwrap();
        self.vals.push(new_val);
        new_val
    }
}

fn solution_a(file: File) -> Result<i64> {
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut result = 0;

    while reader.read_line(&mut line)? != 0 {
        let mut hist = History::from_string(line.clone())?;
        hist.find_diff();
        result += hist.compute_next();
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

        println!("The sum of the next values for each history is {result}.");
    }
    if args.group.b {
        let file = File::open(args.file.clone())?;
        let result = solution_b(file)?;

        println!("The sum of the next values for each history is {result}.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_solution_a() -> Result<()> {
        assert_eq!(solution_a(File::open("test-input-09.txt")?)?, 114);
        Ok(())
    }

    #[test]
    fn test_solution_b() -> Result<()> {
        assert_eq!(solution_b(File::open("test-input-09.txt")?)?, 0);
        Ok(())
    }
}
