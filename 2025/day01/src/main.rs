use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

/// Program to solve Day 1 of 2025 Advent of Code
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

#[derive(Debug)]
struct Lock {
    size: i32,
    current_val: i32,
}

impl Lock {
    fn new(size: i32, current_val: i32) -> Self{
        Lock {size, current_val}
    }

    fn rotate(&mut self, rot: Rotation) -> bool {
        self.current_val = (self.current_val + (rot.direction * rot.distance)).rem_euclid(self.size);
        self.current_val == 0
    }
    
    fn rotate_count(&mut self, rot: Rotation) -> i32 {
        let mut count = 0;
        for _ in 0..rot.distance {
            self.current_val = (self.current_val + (rot.direction)).rem_euclid(self.size);
            if self.current_val == 0{
                count += 1;
            }
        }
        count
    }
}

#[derive(Debug)]
struct Rotation {
    direction: i32,
    distance: i32,
}

impl Rotation {
    fn new_from_str(string: String) -> Result<Rotation> {
        let mut direction = 0;
        let distance: i32;
        let dir = string.chars().next().unwrap();
        if dir == 'R' {
            direction = 1;
        } else if dir == 'L' {
            direction = -1;
        }
        let mut chars = string.chars();
        chars.next();
        let string = chars.as_str();
        distance = string.trim().parse()?;
        Ok(Rotation {direction, distance})
    }
}




fn solution_a(file: File) -> Result<i64> {
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut result = 0;
    let mut lock = Lock::new(100, 50);
    while reader.read_line(&mut line)? != 0 {
        let rot = Rotation::new_from_str(line.clone())?;
        if lock.rotate(rot){
            result += 1;
        }
        line.clear();
    }

    Ok(result)
}

fn solution_b(file: File) -> Result<i64> {
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut result = 0;
    let mut lock = Lock::new(100, 50);
    while reader.read_line(&mut line)? != 0 {
        let rot = Rotation::new_from_str(line.clone())?;
        result += lock.rotate_count(rot);
        line.clear();
    }

    Ok(result.into())
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.group.a {
        let file = File::open(args.file.clone())?;
        let result = solution_a(file)?;

        println!("The password for the door is {result}.");
    }
    if args.group.b {
        let file = File::open(args.file.clone())?;
        let result = solution_b(file)?;

        println!("The password for the door is actually {result}.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_solution_a() -> Result<()> {
        assert_eq!(solution_a(File::open("test-input-01.txt")?)?, 3);
        Ok(())
    }

    #[test]
    fn test_solution_b() -> Result<()> {
        assert_eq!(solution_b(File::open("test-input-01.txt")?)?, 6);
        Ok(())
    }

}
