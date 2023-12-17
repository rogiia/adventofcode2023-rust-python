use std::fs;
use std::time::{Instant};
use pathfinding::prelude::dijkstra;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Direction(i32, i32);

impl Direction {
  fn opposite(&self) -> Direction {
    Direction(self.0 * -1, self.1 * -1)
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position(usize, usize, Direction, usize);

impl Position {
  fn mv(&self, direction: Direction, map: &Vec<Vec<usize>>) -> Option<Position> {
    let d = direction.clone();
    if self.0 as i32 >= (d.0 * -1) && self.1 as i32 >= (d.1 * -1) &&
      self.0 as i32 + d.0 < map.len() as i32 && self.1 as i32 + d.1 < map[0].len() as i32 {
      let x = self.0 as i32 + d.0;
      let y = self.1 as i32 + d.1;
      return Some(Position(x as usize, y as usize, direction, if d == self.2 { self.3 + 1 } else { 1 }))
    }
    None
  }

  fn successors(&self, map: &Vec<Vec<usize>>, min: usize, max: usize) -> Vec<(Position, usize)> {
    let mut positions: Vec<Position> = Vec::new();
    let possible_directions = if self.3 >= min {
      vec![Direction(1, 0), Direction(-1, 0), Direction(0, 1), Direction(0, -1)]
    } else { vec![self.2.clone()] };
    for direction in possible_directions {
      if self.2 != direction.opposite() && !(self.2 == direction && self.3 >= max) {
        let new_pos = self.mv(direction, &map);
        match new_pos {
          Some(p) => {
            if !(p.0 == map.len() - 1 && p.1 == map[0].len() - 1 && (p.3 < min || p.3 > max)) {
              positions.push(p);
            }
          },
          None => {}
        }
      }
    }
    return positions
      .into_iter().map(|p| (p.clone(), map[p.0][p.1])).collect()
  }
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
  let mut map: Vec<Vec<usize>> = Vec::new();
  for line in input.lines().filter(|l| !l.is_empty() ) {
    let mut map_line: Vec<usize> = Vec::new();
    for c in line.chars() {
      map_line.push(c.to_digit(10).unwrap() as usize)
    }
    map.push(map_line);
  }
  map
}

fn path_contains_position(path: &Vec<Position>, position: (usize, usize)) -> bool {
  for pos in path {
    if pos.0 == position.0 && pos.1 == position.1 {
      return true;
    }
  }
  false
}

fn print_path(map: &Vec<Vec<usize>>, path: &Vec<Position>) {
  let mut i = 0;
  while i < map.len() {
    let mut j = 0;
    while j < map[i].len() {
      if path_contains_position(&path, (i, j)) {
        print!("#");
      } else {
        print!("{}", map[i][j]);
      }
      j += 1;
    }
    print!("\n");
    i += 1;
  }
}

fn find_distance_to(map: &Vec<Vec<usize>>, start: Position, min: usize, max: usize) -> usize {
  let result = dijkstra(&start, |p| p.successors(&map, min, max), |p| p.0 == map.len() - 1 && p.1 == map[0].len() - 1);
  match result {
    Some((path, heat_loss)) => {
      print_path(&map, &path);
      return heat_loss
    },
    None => panic!("No path found!")
  }
}

fn prob_a(map: &Vec<Vec<usize>>) -> usize {
  let start = Instant::now();
  let distance = find_distance_to(&map, Position(0, 0, Direction(0, 1), 0), 0, 3);
  println!("Parsed input in {:?}", start.elapsed());
  distance
}

fn prob_b(map: &Vec<Vec<usize>>) -> usize {
  let start = Instant::now();
  let distance = find_distance_to(&map, Position(0, 0, Direction(1, 0), 0), 4, 10);
  println!("Parsed input in {:?}", start.elapsed());
  distance
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