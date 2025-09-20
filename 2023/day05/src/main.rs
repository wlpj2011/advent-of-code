use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::ops::Add;

/// Program to solve Day 5 of 2023 Advent of Code
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
    /// Run solution to part a of day 5.
    #[arg(short)]
    a: bool,

    /// Run solution to part b of day 5.
    #[arg(short)]
    b: bool,
}

enum GardenType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct Seed {
    val: u64,
}

impl Add<u64> for Seed {
    type Output = Seed;

    fn add(self, other: u64) -> Self::Output {
        Seed {
            val: self.val + other,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct Soil {
    val: u64,
}

impl Add<u64> for Soil {
    type Output = Soil;

    fn add(self, other: u64) -> Self::Output {
        Soil {
            val: self.val + other,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct Fertilizer {
    val: u64,
}

impl Add<u64> for Fertilizer {
    type Output = Fertilizer;

    fn add(self, other: u64) -> Self::Output {
        Fertilizer {
            val: self.val + other,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct Water {
    val: u64,
}

impl Add<u64> for Water {
    type Output = Water;

    fn add(self, other: u64) -> Self::Output {
        Water {
            val: self.val + other,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct Light {
    val: u64,
}

impl Add<u64> for Light {
    type Output = Light;

    fn add(self, other: u64) -> Self::Output {
        Light {
            val: self.val + other,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct Temperature {
    val: u64,
}

impl Add<u64> for Temperature {
    type Output = Temperature;

    fn add(self, other: u64) -> Self::Output {
        Temperature {
            val: self.val + other,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct Humidity {
    val: u64,
}

impl Add<u64> for Humidity {
    type Output = Humidity;

    fn add(self, other: u64) -> Self::Output {
        Humidity {
            val: self.val + other,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct Location {
    val: u64,
}

impl Add<u64> for Location {
    type Output = Location;

    fn add(self, other: u64) -> Self::Output {
        Location {
            val: self.val + other,
        }
    }
}

#[derive(Debug, Clone)]
struct Almanac {
    seeds_to_plant: Vec<Seed>,
    map_seed_to_soil: Vec<(Soil, Seed, u64)>,
    map_soil_to_fertilizer: Vec<(Fertilizer, Soil, u64)>,
    map_fertilizer_to_water: Vec<(Water, Fertilizer, u64)>,
    map_water_to_light: Vec<(Light, Water, u64)>,
    map_light_to_temperature: Vec<(Temperature, Light, u64)>,
    map_temperature_to_humidity: Vec<(Humidity, Temperature, u64)>,
    map_humidity_to_location: Vec<(Location, Humidity, u64)>,
}

impl Almanac {
    fn from_file(file: File) -> Result<Almanac> {
        let mut result_almanac = Almanac {
            seeds_to_plant: Vec::new(),
            map_seed_to_soil: Vec::new(),
            map_soil_to_fertilizer: Vec::new(),
            map_fertilizer_to_water: Vec::new(),
            map_water_to_light: Vec::new(),
            map_light_to_temperature: Vec::new(),
            map_temperature_to_humidity: Vec::new(),
            map_humidity_to_location: Vec::new(),
        };

        let mut reader = BufReader::new(file);
        let mut line = String::new();

        let mut current_type = GardenType::Seed;

        while reader.read_line(&mut line)? != 0 {
            if line.contains(":") {
                if line.starts_with("seeds") {
                    line = line.get(7..).unwrap().to_string();
                    let parts: Vec<_> = line.split(" ").collect();
                    for part in parts {
                        result_almanac.seeds_to_plant.push(Seed {
                            val: part.trim().parse()?,
                        });
                    }
                } else if line.starts_with("seed-") {
                    current_type = GardenType::Soil;
                } else if line.starts_with("soil-") {
                    current_type = GardenType::Fertilizer;
                } else if line.starts_with("fertilizer-") {
                    current_type = GardenType::Water;
                } else if line.starts_with("water-") {
                    current_type = GardenType::Light;
                } else if line.starts_with("light-") {
                    current_type = GardenType::Temperature;
                } else if line.starts_with("temperature-") {
                    current_type = GardenType::Humidity;
                } else if line.starts_with("humidity-") {
                    current_type = GardenType::Location;
                }
            } else {
                let parts: Vec<_> = line.split(" ").collect();
                if parts.len() == 1 {
                    line.clear();
                    continue;
                }
                match current_type {
                    GardenType::Soil => {
                        result_almanac.map_seed_to_soil.push((
                            Soil {
                                val: parts[0].parse()?,
                            },
                            Seed {
                                val: parts[1].parse()?,
                            },
                            parts[2].trim().parse()?,
                        ));
                    }
                    GardenType::Fertilizer => {
                        result_almanac.map_soil_to_fertilizer.push((
                            Fertilizer {
                                val: parts[0].parse()?,
                            },
                            Soil {
                                val: parts[1].parse()?,
                            },
                            parts[2].trim().parse()?,
                        ));
                    }
                    GardenType::Water => {
                        result_almanac.map_fertilizer_to_water.push((
                            Water {
                                val: parts[0].parse()?,
                            },
                            Fertilizer {
                                val: parts[1].parse()?,
                            },
                            parts[2].trim().parse()?,
                        ));
                    }
                    GardenType::Light => {
                        result_almanac.map_water_to_light.push((
                            Light {
                                val: parts[0].parse()?,
                            },
                            Water {
                                val: parts[1].parse()?,
                            },
                            parts[2].trim().parse()?,
                        ));
                    }
                    GardenType::Temperature => {
                        result_almanac.map_light_to_temperature.push((
                            Temperature {
                                val: parts[0].parse()?,
                            },
                            Light {
                                val: parts[1].parse()?,
                            },
                            parts[2].trim().parse()?,
                        ));
                    }
                    GardenType::Humidity => {
                        result_almanac.map_temperature_to_humidity.push((
                            Humidity {
                                val: parts[0].parse()?,
                            },
                            Temperature {
                                val: parts[1].parse()?,
                            },
                            parts[2].trim().parse()?,
                        ));
                    }
                    GardenType::Location => {
                        result_almanac.map_humidity_to_location.push((
                            Location {
                                val: parts[0].parse()?,
                            },
                            Humidity {
                                val: parts[1].parse()?,
                            },
                            parts[2].trim().parse()?,
                        ));
                    }
                    _ => line.clear(),
                }
            }

            line.clear();
        }

        Ok(result_almanac)
    }

    fn get_location(seed: Seed) -> Location {
        todo!()
    }
}

fn solution_a(file: File) -> Result<u64> {
    let mut result: u64 = 0;

    let almanac = Almanac::from_file(file)?;

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

        println!("The lowest location value for a seed is {result}.");
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
        assert_eq!(solution_a(File::open("test-input-05.txt")?)?, 35);
        Ok(())
    }
}
