use anyhow::Result;
use clap::Parser;
use core::num;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

/// Program to solve Day 4 of 2023 Advent of Code
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
      /// Run solution to part a of day 4.
      #[arg(short)]
      a: bool,
  
      /// Run solution to part b of day 4.
      #[arg(short)]
      b: bool,
}

#[derive(Debug)]
struct Card {
    _card_number: u64,
    winning_numbers: Vec<u64>,
    card_numbers: Vec<u64>,
}

impl Card {
    fn from_str(line: String) -> Result<Card> {
        let mut new_card: Card  = Card{_card_number: 0, winning_numbers: Vec::new(), card_numbers: Vec::new() };
        let line_parts : Vec<_> = line.split(':').collect();
        let card_num = line_parts[0].split(' ').collect::<Vec<_>>().pop().unwrap();
        new_card._card_number = card_num.parse::<u64>()?;
        let line_parts: Vec<_> = line_parts[1].split("|").collect();
        for num in line_parts[0].split_whitespace() {
            if num.is_empty() {
                continue
            } else {
                new_card.winning_numbers.push(num.parse()?);
            }
        }
        for num in line_parts[1].split_whitespace() {
            if num.is_empty() {
                continue
            } else {
                new_card.card_numbers.push(num.parse()?);
            }
        }
        Ok(new_card)
    }

    fn score(self) -> u64 {
        let mut num_matches: u64 = 0;
        for num in self.card_numbers {
            if self.winning_numbers.contains(&num) {
                num_matches += 1;
            }
        }
        if num_matches == 0 {
            0
        } else {
            return 1 << (num_matches - 1);
        }
    }
}

fn solution_a(file: File) -> Result<u64> {
    let mut result: u64 = 0;

    let mut reader = BufReader::new(file);
    let mut line = String::new();
    while reader.read_line(&mut line)? != 0 {
        result += Card::from_str(line.clone())?.score();
        line.clear();
    }
    Ok(result)
}

fn solution_b(file: File) -> Result<u64> {
    let mut result: u64 = 0;

    let mut reader = BufReader::new(file);
    let mut line = String::new();
    while reader.read_line(&mut line)? != 0 {
        
    }
    Ok(result)
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.group.a {
        let file = File::open(args.file.clone())?;
        let result = solution_a(file)?;
    
        println!("The scratchcards are worth {result} points.");
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
        assert_eq!(solution_a(File::open("test-input-04.txt")?)?, 13);
        Ok(())
    }

    
}