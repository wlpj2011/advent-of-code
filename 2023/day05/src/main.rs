use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::ops::Add;
use tqdm::tqdm;

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

    fn convert_seeds_from_list(&mut self) {
        let old_seeds = self.seeds_to_plant.clone();
        self.seeds_to_plant = Vec::new();

        for pair in tqdm(old_seeds.windows(2).step_by(2)) {
            for i in 0..pair[1].val {
                self.seeds_to_plant.push(pair[0] + i);
            }
        }
    }

    fn get_location(&self, seed: Seed) -> Location {
        let mut seed_soil: Soil = Soil { val: seed.val };

        for (list_soil, list_seed, range) in self.map_seed_to_soil.clone() {
            if (seed >= list_seed) && (seed <= list_seed + (range - 1)) {
                seed_soil = list_soil + (seed.val - list_seed.val);
                break;
            }
        }

        let mut seed_fertilizer: Fertilizer = Fertilizer { val: seed_soil.val };
        for (list_fertilizer, list_soil, range) in self.map_soil_to_fertilizer.clone() {
            if (seed_soil >= list_soil) && (seed_soil <= list_soil + (range - 1)) {
                seed_fertilizer = list_fertilizer + (seed_soil.val - list_soil.val);
                break;
            }
        }

        let mut seed_water: Water = Water {
            val: seed_fertilizer.val,
        };
        for (list_water, list_fertilizer, range) in self.map_fertilizer_to_water.clone() {
            if (seed_fertilizer >= list_fertilizer)
                && (seed_fertilizer <= list_fertilizer + (range - 1))
            {
                seed_water = list_water + (seed_fertilizer.val - list_fertilizer.val);
                break;
            }
        }

        let mut seed_light: Light = Light {
            val: seed_water.val,
        };
        for (list_light, list_water, range) in self.map_water_to_light.clone() {
            if (seed_water >= list_water) && (seed_water <= list_water + (range - 1)) {
                seed_light = list_light + (seed_water.val - list_water.val);
                break;
            }
        }

        let mut seed_temperature: Temperature = Temperature {
            val: seed_light.val,
        };
        for (list_temperature, list_light, range) in self.map_light_to_temperature.clone() {
            if (seed_light >= list_light) && (seed_light <= list_light + (range - 1)) {
                seed_temperature = list_temperature + (seed_light.val - list_light.val);
                break;
            }
        }

        let mut seed_humidity: Humidity = Humidity {
            val: seed_temperature.val,
        };
        for (list_humidity, list_temperature, range) in self.map_temperature_to_humidity.clone() {
            if (seed_temperature >= list_temperature)
                && (seed_temperature <= list_temperature + (range - 1))
            {
                seed_humidity = list_humidity + (seed_temperature.val - list_temperature.val);
                break;
            }
        }

        let mut seed_location: Location = Location {
            val: seed_humidity.val,
        };
        for (list_location, list_humidity, range) in self.map_humidity_to_location.clone() {
            if (seed_humidity >= list_humidity) && (seed_humidity <= list_humidity + (range - 1)) {
                seed_location = list_location + (seed_humidity.val - list_humidity.val);
                break;
            }
        }
        seed_location
    }
}

fn solution_a(file: File) -> Result<u64> {
    let almanac = Almanac::from_file(file)?;
    let seeds = almanac.seeds_to_plant.clone();
    let mut min_location: Location = almanac.get_location(seeds[0]);

    for seed in tqdm(seeds) {
        let seed_location = almanac.get_location(seed);
        if seed_location < min_location {
            min_location = seed_location;
        }
    }

    Ok(min_location.val)
}

fn solution_b(file: File) -> Result<u64> {
    let mut almanac = Almanac::from_file(file)?;
    almanac.convert_seeds_from_list();
    let seeds = almanac.seeds_to_plant.clone();
    let mut min_location: Location = almanac.get_location(seeds[0]);

    for seed in tqdm(seeds) {
        let seed_location = almanac.get_location(seed);
        if seed_location < min_location {
            min_location = seed_location;
        }
    }

    Ok(min_location.val)
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

        println!("The lowest location value for a seed is {result}.");
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

    #[test]
    fn test_solution_b() -> Result<()> {
        assert_eq!(solution_b(File::open("test-input-05.txt")?)?, 46);
        Ok(())
    }
}
