//use anyhow::Error;
use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
enum LogicError {
    #[error("String {0} contains no digits")]
    NoDigits(String),
}

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

fn find_first_digit(line: &str) -> Result<u64> {
    let result: char;
    let chars = line.chars();
    for char in chars {
        if char.is_ascii_digit() {
            result = char;
            return Ok(char::to_digit(result, 10).unwrap() as u64);
        }
    }
    Err(LogicError::NoDigits(line.to_string()).into())
}

fn find_last_digit(line: &str) -> Result<u64> {
    let mut result: char = ' ';
    let chars = line.chars();
    if chars.clone().count() > 0 {
        for char in chars {
            if char.is_ascii_digit() {
                result = char;
            }
        }
        return Ok(char::to_digit(result, 10).unwrap() as u64);
    }
    Err(LogicError::NoDigits(line.to_string()).into())
}

fn find_first_digit_w_text(line: &str) -> Result<u64> {
    Ok(0)
}

fn find_last_digit_w_text(line: &str) -> Result<u64> {
    Ok(0)
}

fn calibration_value(line: &str) -> Result<u64> {
    let result = 10 * find_first_digit(line)? + find_last_digit(line)?;

    Ok(result)
}

fn calibration_value_w_text(line: &str) -> Result<u64> {
    let result = 10 * find_first_digit_w_text(line)? + find_last_digit_w_text(line)?;

    Ok(result)
}

fn solution_a(file: File) -> Result<u64> {
    let mut result: u64 = 0;

    let mut reader = BufReader::new(file);
    let mut line = String::new();

    while reader.read_line(&mut line)? != 0 {
        result += calibration_value(&line)?;
        line.clear()
    }

    Ok(result)
}

fn solution_b(file: File) -> Result<u64> {
    let mut result: u64 = 0;

    let mut reader = BufReader::new(file);
    let mut line = String::new();

    while reader.read_line(&mut line)? != 0 {
        result += calibration_value_w_text(&line)?;
        line.clear()
    }

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
    fn test_solution_a() -> Result<()> {
        assert_eq!(solution_a(File::open("test-input-01a.txt")?)?, 142);
        Ok(())
    }

    #[test]
    fn test_solution_b() -> Result<()> {
        assert_eq!(solution_b(File::open("test-input-01b.txt")?)?, 281);
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

    #[test]
    fn test_calibration_w_text_1() -> Result<()> {
        assert_eq!(calibration_value("two1nine")?, 29);
        Ok(())
    }

    #[test]
    fn test_calibration_w_text_2() -> Result<()> {
        assert_eq!(calibration_value("eightwothree")?, 83);
        Ok(())
    }

    #[test]
    fn test_calibration_w_text_3() -> Result<()> {
        assert_eq!(calibration_value("abcone2threexyz")?, 13);
        Ok(())
    }

    #[test]
    fn test_calibration_w_text_4() -> Result<()> {
        assert_eq!(calibration_value("xtwone3four")?, 24);
        Ok(())
    }

    #[test]
    fn test_calibration_w_text_5() -> Result<()> {
        assert_eq!(calibration_value("4nineeightseven2")?, 42);
        Ok(())
    }

    #[test]
    fn test_calibration_w_text_6() -> Result<()> {
        assert_eq!(calibration_value("zoneight234")?, 14);
        Ok(())
    }

    #[test]
    fn test_calibration_w_text_7() -> Result<()> {
        assert_eq!(calibration_value("7pqrstsixteen")?, 76);
        Ok(())
    }

    #[test]
    fn test_find_first_digit_1() -> Result<()> {
        assert_eq!(find_first_digit("a1b2c3d4e5f")?, 1);
        Ok(())
    }

    #[test]
    fn test_find_first_digit_2() -> Result<()> {
        assert_eq!(find_first_digit("treb7uchet")?, 7);
        Ok(())
    }

    #[test]
    fn test_find_first_digit__w_text_1() -> Result<()> {
        assert_eq!(find_first_digit("two1nine")?, 2);
        Ok(())
    }

    #[test]
    fn test_find_first_digit_w_text_2() -> Result<()> {
        assert_eq!(find_first_digit("4nineeightseven2")?, 4);
        Ok(())
    }

    #[test]
    fn test_find_last_digit_1() -> Result<()> {
        assert_eq!(find_last_digit("a1b2c3d4e5f")?, 5);
        Ok(())
    }

    #[test]
    fn test_find_last_digit_2() -> Result<()> {
        assert_eq!(find_last_digit("treb7uchet")?, 7);
        Ok(())
    }

    #[test]
    fn test_find_last_digit_w_text_1() -> Result<()> {
        assert_eq!(find_last_digit("abcone2threexyz")?, 3);
        Ok(())
    }

    #[test]
    fn test_find_last_digit_w_text_2() -> Result<()> {
        assert_eq!(find_last_digit("zoneight234")?, 4);
        Ok(())
    }
}
