use std::fs;
use std::time::{Instant};
use std::collections::HashMap;
//use cached::proc_macro::cached;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Tile {
  Empty,
  RoundRock,
  CubeRock
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
  let mut map: Vec<Vec<Tile>> = Vec::new();
  for line in input.lines().filter(|l| !l.is_empty() ) {
    let mut map_line: Vec<Tile> = Vec::new();
    for c in line.chars() {
      match c {
        '.' => map_line.push(Tile::Empty),
        'O' => map_line.push(Tile::RoundRock),
        '#' => map_line.push(Tile::CubeRock),
        _ => panic!("Found invalid character {}", c)
      };
    }
    map.push(map_line);
  }
  map
}

fn tilt_north(map: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
  let mut new_map: Vec<Vec<Tile>> = map.clone();
  let mut has_moved = true;
  while has_moved {
    has_moved = false;
    let mut i = 1;
    while i < map.len() {
      let mut j = 0;
      while j < new_map[i].len() {
        if new_map[i][j] == Tile::RoundRock && new_map[i-1][j] == Tile::Empty {
          new_map[i-1][j] = Tile::RoundRock;
          new_map[i][j] = Tile::Empty;
          has_moved = true;
        }
        j += 1;
      }
      i += 1;
    }
  }
  new_map
}

//#[cached]
fn cycle(map: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
  let mut new_map: Vec<Vec<Tile>> = map.clone();
  // North
  let mut has_moved = true;
  while has_moved {
    has_moved = false;
    let mut i = 1;
    while i < map.len() {
      let mut j = 0;
      while j < new_map[i].len() {
        if new_map[i][j] == Tile::RoundRock && new_map[i-1][j] == Tile::Empty {
          new_map[i-1][j] = Tile::RoundRock;
          new_map[i][j] = Tile::Empty;
          has_moved = true;
        }
        j += 1;
      }
      i += 1;
    }
  }

  // West
  has_moved = true;
  while has_moved {
    has_moved = false;
    let mut j = 1;
    while j < map[0].len() {
      let mut i = 0;
      while i < new_map.len() {
        if new_map[i][j] == Tile::RoundRock && new_map[i][j-1] == Tile::Empty {
          new_map[i][j-1] = Tile::RoundRock;
          new_map[i][j] = Tile::Empty;
          has_moved = true;
        }
        i += 1;
      }
      j += 1;
    }
  }

  // South
  has_moved = true;
  while has_moved {
    has_moved = false;
    let mut i = map.len() - 2;
    loop {
      let mut j = 0;
      while j < new_map[i].len() {
        if new_map[i][j] == Tile::RoundRock && new_map[i+1][j] == Tile::Empty {
          new_map[i+1][j] = Tile::RoundRock;
          new_map[i][j] = Tile::Empty;
          has_moved = true;
        }
        j += 1;
      }
      if i == 0 {
        break;
      }
      i -= 1;
    }
  }

  // East
  has_moved = true;
  while has_moved {
    has_moved = false;
    let mut j = map[0].len() - 2;
    loop {
      let mut i = 0;
      while i < new_map.len() {
        if new_map[i][j] == Tile::RoundRock && new_map[i][j+1] == Tile::Empty {
          new_map[i][j+1] = Tile::RoundRock;
          new_map[i][j] = Tile::Empty;
          has_moved = true;
        }
        i += 1;
      }
      if j == 0 {
        break;
      }
      j -= 1;
    }
  }
  new_map
}

fn are_maps_equal(map1: &Vec<Vec<Tile>>, map2: &Vec<Vec<Tile>>) -> bool {
  if map1.len() != map2.len() || map1[0].len() != map2[0].len() {
    return false;
  }
  let mut i = 0;
  while i < map1.len() {
    let mut j = 0;
    while j < map1[i].len() {
      if map1[i][j] != map2[i][j] {
        return false;
      }
      j += 1;
    }
    i += 1;
  }
  true
}

fn prob_a(map: &Vec<Vec<Tile>>) -> usize {
  let mut start = Instant::now();
  let mut sum: usize = 0;
  let tilted = tilt_north(&map);
  println!("Tilted map in {:?}", start.elapsed());

  start = Instant::now();
  let mut idx = 0;
  let len = tilted.len();
  while idx < len {
    for tile in &tilted[idx] {
      if *tile == Tile::RoundRock {
        sum += len - idx;
      }
    }
    idx += 1;
  }
  println!("Counted rock weight in {:?}", start.elapsed());
  sum
}

fn prob_b(map: &Vec<Vec<Tile>>) -> usize {
  let mut start = Instant::now();
  //let mut sum: usize = 0;
  let cycles = 1000;
  let mut tilted = map.clone();
  let mut results: HashMap<usize, Vec<usize>> = HashMap::new();
  for n in 0..cycles {
    let mut start = Instant::now();
    tilted = cycle(&tilted);
    let mut idx = 0;
    let mut sum = 0;
    let len = tilted.len();
    while idx < len {
      for tile in &tilted[idx] {
        if *tile == Tile::RoundRock {
          sum += len - idx;
        }
      }
      idx += 1;
    }
    if results.contains_key(&sum) {
      results.get_mut(&sum).unwrap().push(n);
    } else {
      results.insert(sum, vec![n]);
    }
    println!("Finished cycle {} in {:?}, sum {}", n, start.elapsed(), sum);
  }
  println!("{:?}", results);
  
  println!("Tilted map in {:?}", start.elapsed());

  start = Instant::now();
  
  println!("Counted rock weight in {:?}", start.elapsed());
  0//sum
}

fn main() {
  let start = Instant::now();
  let args: Vec<String> = std::env::args().collect();
  let file: String = fs::read_to_string(&args[1]).unwrap();
  let map = parse_input(file.as_str());
  println!("Parsed input in {:?}", start.elapsed());
  let result = prob_b(&map);
  println!("Result: {}", result);
}