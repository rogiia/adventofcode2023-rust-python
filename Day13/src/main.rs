use std::fs;
use std::time::{Instant};

#[derive(PartialEq)]
enum Tile {
    Ash,
    Rock
}
type Map = Vec<Vec<Tile>>;

fn parse_input(input: &str) -> Vec<Map> {
  let mut maps: Vec<Map> = Vec::new();
  for m in input.split("\n\n") {
    let mut map: Map = Vec::new();
    for line in m.lines() {
      let mut map_line: Vec<Tile> = Vec::new();
      for c in line.chars() {
        match c {
          '.' => map_line.push(Tile::Ash),
          '#' => map_line.push(Tile::Rock),
          _ => panic!("Found invalid character {}", c)
        };
      }
      map.push(map_line);
    }
    maps.push(map);
  }
  maps
}

fn is_horizontally_simetric(map: &Map, idx: usize, smudges: usize) -> bool {
  let mut i1 = idx;
  let mut i2 = idx + 1;
  let mut remaining_smudges = smudges;
  while i2 < map.len() {
    let mut j = 0;
    while j < map[i1].len() {
      if map[i1][j] != map[i2][j] {
        if remaining_smudges == 1 {
          remaining_smudges = 0;
        } else {
          return false;
        }
      }
      j += 1;
    }
    if i1 == 0 {
      break;
    }
    i1 -= 1;
    i2 += 1;
  }

  remaining_smudges == 0
}

fn is_vertically_simetric(map: &Map, idx: usize, smudges: usize) -> bool {
  let mut j1 = idx;
  let mut j2 = idx + 1;
  let mut remaining_smudges = smudges;
  while j2 < map[0].len() {
    let mut i = 0;
    while i < map.len() {
      if map[i][j1] != map[i][j2] {
        if remaining_smudges == 1 {
          remaining_smudges = 0;
        } else {
          return false;
        }
      }
      i += 1;
    }
    if j1 == 0 {
      break;
    }
    j1 -= 1;
    j2 += 1;
  }

  remaining_smudges == 0
}

fn solve(maps: &Vec<Map>) -> usize {
  let mut sum: usize = 0;
  for map in maps {
    let start = Instant::now();
    let mut idx = 0;
    let mut found = false;
    while !found && idx < map[0].len() - 1 {
      if is_vertically_simetric(&map, idx, 1) {
        found = true;
        sum += idx + 1;
      } else {
        idx += 1;
      }
    }
    idx = 0;
    while !found && idx < map.len() - 1 {
      if is_horizontally_simetric(&map, idx, 1) {
        found = true;
        sum += 100 * (idx + 1);
      } else {
        idx += 1;
      }
    }
    
    println!("Solved map in {:?}, sum is {}", start.elapsed(), sum);
  }
  sum
}

fn main() {
  let start = Instant::now();
  let args: Vec<String> = std::env::args().collect();
  let file: String = fs::read_to_string(&args[1]).unwrap();
  let maps = parse_input(file.as_str());
  println!("Parsed input in {:?}", start.elapsed());
  let result = solve(&maps);
  println!("Result: {}", result);
}