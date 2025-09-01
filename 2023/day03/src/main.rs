use anyhow::Result;
use clap::Parser;
//use clap::builder::Str;
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
const DIRECTIONS: [(i64, i64); 8] = [
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

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

    fn find_parts(&self) -> Result<Vec<u64>> {
        let mut parts = Vec::new();
        let mut part = 0;
        let mut end_current_part: usize = 0;
        if !self.line2.is_empty() {
            for (i, char) in self.line2.chars().enumerate() {
                if i < end_current_part {
                    continue;
                } else if char.is_ascii_digit() {
                    for dir in DIRECTIONS {
                        if (((dir.0 + i as i64) > 0)
                            && (dir.0 + i as i64) < self.line2.len() as i64)
                            && ((dir.1 == -1
                                && !self.line1.is_empty()
                                && SYMBOLS.contains(
                                    self.line1
                                        .get(
                                            ((dir.0 + i as i64) as usize)
                                                ..=((dir.0 + i as i64) as usize),
                                        )
                                        .unwrap(),
                                ))
                                || (dir.1 == 0
                                    && SYMBOLS.contains(
                                        self.line2
                                            .get(
                                                ((dir.0 + i as i64) as usize)
                                                    ..=((dir.0 + i as i64) as usize),
                                            )
                                            .unwrap(),
                                    ))
                                || (dir.1 == 1
                                    && !self.line3.is_empty()
                                    && SYMBOLS.contains(
                                        self.line3
                                            .get(
                                                ((dir.0 + i as i64) as usize)
                                                    ..=((dir.0 + i as i64) as usize),
                                            )
                                            .unwrap(),
                                    )))
                        {
                            (part, end_current_part) = self.extract_part(i)?;
                            parts.push(part);
                        }
                    }
                }
            }
        }
        Ok(parts)
    }

    fn extract_part(&self, idx: usize) -> Result<(u64, usize)> {
        // Returns the part number and the index of the final character of the part number
        let mut found_part = false;
        let mut part_str = "".to_string();
        let mut end = 0;
        for (i, char) in self.line2.chars().enumerate() {
            if i == idx {
                found_part = true;
            }
            if char.is_ascii_digit() {
                part_str.push(char);
            } else if found_part {
                end = i;
                break;
            } else {
                part_str.clear();
            }
        }
        Ok((part_str.parse()?, end))
    }
}
fn solution_a(file: File) -> Result<u64> {
    let mut result: u64 = 0;

    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut context: Context = Context::from_str(&line);
    while reader.read_line(&mut line)? != 0 {
        context.update_context(&line);
        let parts = context.find_parts()?;
        for part in parts {
            result += part;
        }
        line.clear();
    }
    context.update_context("");
    let parts = context.find_parts()?;
    for part in parts {
        result += part;
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
