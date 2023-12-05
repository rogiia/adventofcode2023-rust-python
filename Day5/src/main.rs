use std::fs;
use regex::Regex;
use rayon::prelude::*;

struct Map {
  source_range_start: usize,
  destination_range_start: usize,
  range_length: usize
}

struct EquivalenceMaps {
  seed_to_soil: Vec<Map>,
  soil_to_fertilizer: Vec<Map>,
  fertilizer_to_water: Vec<Map>,
  water_to_light: Vec<Map>,
  light_to_temperature: Vec<Map>,
  temperature_to_humidity: Vec<Map>,
  humidity_to_location: Vec<Map>
}

impl EquivalenceMaps {
  fn iter(&self) -> Vec<&Vec<Map>> {
    vec![&self.seed_to_soil, &self.soil_to_fertilizer, &self.fertilizer_to_water, &self.water_to_light, &self.light_to_temperature, &self.temperature_to_humidity, &self.humidity_to_location]
  }
}

fn parse_map(input: &str) -> Vec<Map> {
  let mut result = Vec::new();
  let maps: Vec<&str> = input.split("\n").collect();
  for map in maps {
    let parsed: Vec<usize> = map.split(" ")
      .map(|n| n.parse::<usize>().unwrap())
      .collect();
    if parsed.len() < 3 {
      panic!("Cannot parse map");
    }
    result.push(Map {
      source_range_start: parsed[1],
      destination_range_start: parsed[0],
      range_length: parsed[2]
    });
  }

  result
}

fn parse_input(file: &str) -> (Vec<usize>, EquivalenceMaps) {
  let re = Regex::new(r"seeds: (?<seeds>[\d\s]+)\n\nseed-to-soil map:\n(?<se2so>[\d\s\n]+)\n\nsoil-to-fertilizer map:\n(?<so2f>[\d\s\n]+)\n\nfertilizer-to-water map:\n(?<f2w>[\d\s\n]+)\n\nwater-to-light map:\n(?<w2l>[\d\s\n]+)\n\nlight-to-temperature map:\n(?<l2t>[\d\s\n]+)\n\ntemperature-to-humidity map:\n(?<t2h>[\d\s\n]+)\n\nhumidity-to-location map:\n(?<h2l>[\d\s\n]+)").unwrap();
  match re.captures(file) {
      Some(captures) => {
        let seeds: Vec<usize> = (&captures["seeds"])
          .split(" ")
          .map(|n| n.parse::<usize>().unwrap()).collect();
        return (seeds, EquivalenceMaps {
          seed_to_soil: parse_map(&captures["se2so"]),
          soil_to_fertilizer: parse_map(&captures["so2f"]),
          fertilizer_to_water: parse_map(&captures["f2w"]),
          water_to_light: parse_map(&captures["w2l"]),
          light_to_temperature: parse_map(&captures["l2t"]),
          temperature_to_humidity: parse_map(&captures["t2h"]),
          humidity_to_location: parse_map(&captures["h2l"])
        });
      },
      None => panic!("Could not parse card")
  }
}

fn find_location_from_seed(seed: usize, eq_maps: &EquivalenceMaps) -> usize {
  let mut value = seed;
  for eq in eq_maps.iter() {
    for map in eq {
      if value >= map.source_range_start && value < map.source_range_start + map.range_length {
        value = map.destination_range_start + (value - map.source_range_start);
        break;
      }
    }
  }

  value
}

fn find_location_from_seed_range(seed_range: (usize, usize), eq_maps: &EquivalenceMaps) -> usize {
  println!("Finding location from seed range {}[{}]", seed_range.0, seed_range.1);
  let mut location_values: Vec<usize> = Vec::new();
  let from = seed_range.0;
  let to = seed_range.0 + seed_range.1;
  for initial_value in from..to {
    println!("Finding location from seed value {}", initial_value);
    let mut value = initial_value;
    for eq in eq_maps.iter() {
      for map in eq {
        if value >= map.source_range_start && value < map.source_range_start + map.range_length {
          value = map.destination_range_start + (value - map.source_range_start);
          break;
        }
      }
    }
    location_values.push(value);
  }

  find_lowest_value(location_values)
}

fn find_location_from_seed_range_par(seed_range: (usize, usize), eq_maps: &EquivalenceMaps) -> usize {
  println!("Finding location from seed range {}[{}]", seed_range.0, seed_range.1);
  
  let from = seed_range.0;
  let to = seed_range.0 + seed_range.1;
  let location_values: Vec<usize> = (from..to).into_par_iter().map(|initial_value| {
    let mut value = initial_value;
    for eq in eq_maps.iter() {
      for map in eq {
        if value >= map.source_range_start && value < map.source_range_start + map.range_length {
          value = map.destination_range_start + (value - map.source_range_start);
          break;
        }
      }
    }
    value
  }).collect();

  find_lowest_value(location_values)
}

fn find_lowest_value(values: Vec<usize>) -> usize {
  let mut lowest = usize::MAX;
  for value in values {
    if value < lowest {
      lowest = value;
    }
  }

  lowest
}

fn get_seed_ranges(seeds: &Vec<usize>) -> Vec<(usize, usize)> {
  let mut seed_ranges: Vec<(usize, usize)> = Vec::new();
  let mut start_num: usize = 0;
  let mut is_range = false;
  for num in seeds {
    if is_range {
      seed_ranges.push((start_num, *num));
      is_range = false;
    } else {
      start_num = *num;
      is_range = true;
    }
  }

  seed_ranges
}

fn prob_a(seeds: Vec<usize>, eq_maps: &EquivalenceMaps) -> usize {
  let mut locations = Vec::new();
  for seed in seeds {
    locations.push(find_location_from_seed(seed, &eq_maps));
  }

  find_lowest_value(locations)
}

fn prob_b(seeds: Vec<usize>, eq_maps: &EquivalenceMaps) -> usize {
  let mut locations = Vec::new();
  let seed_ranges = get_seed_ranges(&seeds);
  for seed_range in seed_ranges {
    locations.push(find_location_from_seed_range_par(seed_range, &eq_maps));
  }

  find_lowest_value(locations)
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let binding = fs::read_to_string(&args[1]).unwrap();
  let file: &str = binding.as_str();

  let (seeds, eq_maps) = parse_input(file);
  let result = prob_b(seeds, &eq_maps);

  println!("Result: {}", result);
}