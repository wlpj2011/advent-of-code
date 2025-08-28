use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

/// Program to solve Day 2 of 2023 Advent of Code
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

#[derive(Debug, Clone, Copy, PartialEq)]
struct Turn {
    red: u64,
    green: u64,
    blue: u64,
}

impl Turn {
    fn from_string(turn_str: &str) -> Result<Turn> {
        todo!();
    }

    fn is_valid(&self, bounds: (u64, u64, u64)) -> bool {
        (self.red <= bounds.0) && (self.green <= bounds.0) && (self.blue <= bounds.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Game {
    index: u64,
    turns: Vec<Turn>,
    bounds: (u64, u64, u64),
}

impl Game {
    fn from_string(game: &str, bounds: (u64, u64, u64)) -> Result<Game> {
        let game_parts: Vec<_> = game.split(":").collect();
        let mut turns: Vec<Turn> = Vec::new();
        let mut index: u64 = 0;
        for (i, part) in game_parts.iter().enumerate() {
            if i == 0 {
                index = part.split(" ").collect::<Vec<&str>>()[1]
                    .parse()
                    .expect("Every string representing a game must have a game number.");
            } else {
                let turns_str = part.split(";").collect::<Vec<&str>>();
                for turn_str in turns_str.iter() {
                    turns.push(Turn::from_string(turn_str)?);
                }
            }
        }
        Ok(Game {
            index,
            turns,
            bounds,
        })
    }

    fn is_valid(&self) -> bool {
        self.turns.iter().all(|x| x.is_valid(self.bounds))
    }
}

fn solution_a(file: File) -> Result<u64> {
    let mut result: u64 = 0;

    let mut reader = BufReader::new(file);
    let mut line = String::new();
    while reader.read_line(&mut line)? != 0 {
        let game = Game::from_string(&line, (12, 13, 14))?;
        if game.is_valid() {
            result += game.index;
        }
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
        assert_eq!(solution_a(File::open("test-input-02.txt")?)?, 8);
        Ok(())
    }

    #[test]
    fn test_turn_from_string_1() -> Result<()> {
        assert_eq!(
            Turn::from_string("4 red, 2 green, 3 blue,")?,
            Turn {
                red: 4,
                green: 2,
                blue: 3,
            }
        );
        Ok(())
    }

    #[test]
    fn test_turn_from_string_2() -> Result<()> {
        assert_eq!(
            Turn::from_string("3 blue, 4 red")?,
            Turn {
                red: 4,
                green: 0,
                blue: 3,
            }
        );
        Ok(())
    }

    #[test]
    fn test_turn_from_string_3() -> Result<()> {
        assert_eq!(
            Turn::from_string("1 blue")?,
            Turn {
                red: 0,
                green: 0,
                blue: 1,
            }
        );
        Ok(())
    }

    #[test]
    fn test_turn_from_string_4() -> Result<()> {
        assert_eq!(
            Turn::from_string("1 red, 2 green")?,
            Turn {
                red: 1,
                green: 2,
                blue: 0,
            }
        );
        Ok(())
    }

    #[test]
    fn test_game_from_string_1() -> Result<()> {
        let mut result_turns = Vec::new();
        result_turns.push(Turn {
            red: 4,
            green: 0,
            blue: 3,
        });
        result_turns.push(Turn {
            red: 1,
            green: 2,
            blue: 6,
        });
        result_turns.push(Turn {
            red: 0,
            green: 2,
            blue: 0,
        });
        assert_eq!(
            Game::from_string(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
                (0, 0, 0)
            )?,
            Game {
                index: 1,
                turns: result_turns,
                bounds: (0, 0, 0),
            }
        );
        Ok(())
    }

    #[test]
    fn test_game_from_string_2() -> Result<()> {
        let mut result_turns = Vec::new();
        result_turns.push(Turn {
            red: 0,
            green: 2,
            blue: 1,
        });
        result_turns.push(Turn {
            red: 1,
            green: 3,
            blue: 4,
        });
        result_turns.push(Turn {
            red: 0,
            green: 1,
            blue: 1,
        });
        assert_eq!(
            Game::from_string(
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
                (0, 0, 0)
            )?,
            Game {
                index: 2,
                turns: result_turns,
                bounds: (0, 0, 0),
            }
        );
        Ok(())
    }

    #[test]
    fn test_game_from_string_3() -> Result<()> {
        let mut result_turns = Vec::new();
        result_turns.push(Turn {
            red: 20,
            green: 8,
            blue: 6,
        });
        result_turns.push(Turn {
            red: 4,
            green: 13,
            blue: 5,
        });
        result_turns.push(Turn {
            red: 1,
            green: 5,
            blue: 0,
        });
        assert_eq!(
            Game::from_string(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                (0, 0, 0)
            )?,
            Game {
                index: 3,
                turns: result_turns,
                bounds: (0, 0, 0),
            }
        );
        Ok(())
    }

    #[test]
    fn test_game_from_string_4() -> Result<()> {
        let mut result_turns = Vec::new();
        result_turns.push(Turn {
            red: 3,
            green: 1,
            blue: 6,
        });
        result_turns.push(Turn {
            red: 6,
            green: 3,
            blue: 0,
        });
        result_turns.push(Turn {
            red: 14,
            green: 3,
            blue: 15,
        });
        assert_eq!(
            Game::from_string(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                (0, 0, 0)
            )?,
            Game {
                index: 4,
                turns: result_turns,
                bounds: (0, 0, 0),
            }
        );
        Ok(())
    }

    #[test]
    fn test_game_from_string_5() -> Result<()> {
        let mut result_turns = Vec::new();
        result_turns.push(Turn {
            red: 6,
            green: 3,
            blue: 1,
        });
        result_turns.push(Turn {
            red: 1,
            green: 2,
            blue: 2,
        });
        assert_eq!(
            Game::from_string(
                "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
                (0, 0, 0)
            )?,
            Game {
                index: 5,
                turns: result_turns,
                bounds: (0, 0, 0),
            }
        );
        Ok(())
    }

    #[test]
    fn test_game_is_valid_1() -> Result<()> {
        let mut result_turns = Vec::new();
        result_turns.push(Turn {
            red: 4,
            green: 0,
            blue: 3,
        });
        result_turns.push(Turn {
            red: 1,
            green: 2,
            blue: 6,
        });
        result_turns.push(Turn {
            red: 0,
            green: 2,
            blue: 0,
        });
        assert_eq!(
            Game {
                index: 1,
                turns: result_turns,
                bounds: (12, 13, 14),
            }
            .is_valid(),
            true
        );
        Ok(())
    }

    #[test]
    fn test_game_is_valid_2() -> Result<()> {
        let mut result_turns = Vec::new();
        result_turns.push(Turn {
            red: 0,
            green: 2,
            blue: 1,
        });
        result_turns.push(Turn {
            red: 1,
            green: 3,
            blue: 4,
        });
        result_turns.push(Turn {
            red: 0,
            green: 1,
            blue: 1,
        });
        assert_eq!(
            Game {
                index: 2,
                turns: result_turns,
                bounds: (12, 13, 15),
            }
            .is_valid(),
            true
        );
        Ok(())
    }

    #[test]
    fn test_game_is_valid_3() -> Result<()> {
        let mut result_turns = Vec::new();
        result_turns.push(Turn {
            red: 20,
            green: 8,
            blue: 6,
        });
        result_turns.push(Turn {
            red: 4,
            green: 13,
            blue: 5,
        });
        result_turns.push(Turn {
            red: 1,
            green: 5,
            blue: 0,
        });
        assert_eq!(
            Game {
                index: 3,
                turns: result_turns,
                bounds: (12, 13, 14),
            }
            .is_valid(),
            false
        );
        Ok(())
    }

    #[test]
    fn test_game_is_valid_4() -> Result<()> {
        let mut result_turns = Vec::new();
        result_turns.push(Turn {
            red: 3,
            green: 1,
            blue: 6,
        });
        result_turns.push(Turn {
            red: 6,
            green: 3,
            blue: 0,
        });
        result_turns.push(Turn {
            red: 14,
            green: 3,
            blue: 15,
        });
        assert_eq!(
            Game {
                index: 4,
                turns: result_turns,
                bounds: (12, 13, 14),
            }
            .is_valid(),
            false
        );
        Ok(())
    }

    #[test]
    fn test_game_is_valid_5() -> Result<()> {
        let mut result_turns = Vec::new();
        result_turns.push(Turn {
            red: 6,
            green: 3,
            blue: 1,
        });
        result_turns.push(Turn {
            red: 1,
            green: 2,
            blue: 2,
        });
        assert_eq!(
            Game {
                index: 5,
                turns: result_turns,
                bounds: (12, 13, 14),
            }
            .is_valid(),
            true
        );
        Ok(())
    }
}
