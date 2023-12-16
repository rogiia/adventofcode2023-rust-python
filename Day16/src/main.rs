use std::fs;
use std::time::{Instant};
use std::collections::HashSet;
use rayon::prelude::*;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Tile {
  Empty,
  ForwardMirror,
  BackwardMirror,
  VerticalMirror,
  HorizontalMirror
}

fn move_dir(position: Position, direction: Direction) -> Option<Position> {
	let x = position.0 as i32;
	let y = position.1 as i32;
	if x + direction.0 < 0 || y + direction.1 < 0 {
		return None;
	}
	Some(((x + direction.0) as usize, (y + direction.1) as usize))
}

impl Tile {
	fn next_position(&self, current_position: Position, direction: Direction) -> Vec<Position> {
		match self {
			Tile::Empty => {
				match move_dir(current_position, direction) {
					Some(p) => return vec![p],
					None => return vec![]
				}
			},
			Tile::ForwardMirror => {
				match move_dir(current_position, (-direction.1, -direction.0)) {
					Some(p) => return vec![p],
					None => return vec![]
				}
			},
			Tile::BackwardMirror => {
				match move_dir(current_position, (direction.1, direction.0)) {
					Some(p) => return vec![p],
					None => return vec![]
				}
			},
			Tile::VerticalMirror => {
				match direction {
					(0, 1) | (0, -1) => {
						let mut result: Vec<Position> = Vec::new();
						match move_dir(current_position, (direction.1, direction.0)) {
							Some(p) => result.push(p),
							None => {}
						}
						match move_dir(current_position, (-direction.1, direction.0)) {
							Some(p) => result.push(p),
							None => {}
						}
						return result;
					},
					(1, 0) | (-1, 0) => {
						match move_dir(current_position, direction) {
							Some(p) => return vec![p],
							None => return vec![]
						}
					},
					_ => panic!("Invalid direction")
				}
			},
			Tile::HorizontalMirror => {
				match direction {
					(0, 1) | (0, -1) => {
						match move_dir(current_position, direction) {
							Some(p) => return vec![p],
							None => return vec![]
						}
					},
					(1, 0) | (-1, 0) => {
						let mut result: Vec<Position> = Vec::new();
						match move_dir(current_position, (direction.1, direction.0)) {
							Some(p) => result.push(p),
							None => {}
						}
						match move_dir(current_position, (direction.1, -direction.0)) {
							Some(p) => result.push(p),
							None => {}
						}
						return result;
					},
					_ => panic!("Invalid direction")
				}
			}
		}
	}
}

type Position = (usize, usize);
type Direction = (i32, i32);

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
  let mut map: Vec<Vec<Tile>> = Vec::new();
  for line in input.lines().filter(|l| !l.is_empty() ) {
    let mut map_line: Vec<Tile> = Vec::new();
    for c in line.chars() {
      match c {
        '.' => map_line.push(Tile::Empty),
        '/' => map_line.push(Tile::ForwardMirror),
        '\\' => map_line.push(Tile::BackwardMirror),
        '|' => map_line.push(Tile::VerticalMirror),
        '-' => map_line.push(Tile::HorizontalMirror),
        _ => panic!("Found invalid character {}", c)
      };
    }
    map.push(map_line);
  }
  map
}

fn visited_contains_position(pos: Position, visited: &Vec<(Position, Direction)>) -> bool {
	for v in visited {
		if v.0 == pos {
			return true;
		}
	}
	false
}

fn print_map(map: &Vec<Vec<Tile>>, visited: &Vec<(Position, Direction)>) {
	let mut i = 0;
	while i < map.len() {
		let mut j = 0;
		while j < map[i].len() {
			if visited_contains_position((i, j), &visited) {
				print!("#");
			} else {
				print!(".");
			}
			j += 1;
		}
		print!("\n");
		i += 1;
	}
}

fn beam(starting_position: Position, direction: Direction, map: &Vec<Vec<Tile>>, visited: &mut Vec<(Position, Direction)>) -> HashSet<Position> {
	let mut energized_tiles: HashSet<Position> = HashSet::new();
	energized_tiles.insert(starting_position);
	visited.push((starting_position, direction));
	let next_positions = map[starting_position.0][starting_position.1].next_position(starting_position, direction);
	for pos in next_positions {
		if pos.0 < map.len() && pos.1 < map[0].len() {
			let new_direction = (pos.0 as i32 - starting_position.0 as i32, pos.1 as i32 - starting_position.1 as i32);
			if !visited.contains(&(pos, new_direction)) {
				let new_tiles: HashSet<Position> = beam(pos, new_direction, &map, visited);
				energized_tiles = energized_tiles.union(&new_tiles).map(|p| p.clone()).collect();
			}
		}
	}

	energized_tiles
}

fn prob_a(map: &Vec<Vec<Tile>>) -> usize {
  let start = Instant::now();
	let mut visited: Vec<(Position, Direction)> = Vec::new();
	let energized_tiles = beam((0, 0), (0, 1), &map, &mut visited);
  print_map(&map, &visited);
  println!("Counted energized tiles in {:?}", start.elapsed());
  energized_tiles.len()
}

fn prob_b(map: &Vec<Vec<Tile>>) -> usize {
  let start = Instant::now();
	let mut starting_positions: Vec<(Position, Direction)> = Vec::new();
	// Up and down edges
	for n in 0..map.len() {
		starting_positions.push(((0, n), (1, 0)));
		starting_positions.push(((map.len() - 1, n), (-1, 0)));
	}
	// Left and right edges
	for n in 0..map[0].len() {
		starting_positions.push(((n, 0), (0, 1)));
		starting_positions.push(((n, map[0].len() - 1), (0, -1)));
	}

	let max = starting_positions.par_iter().map(|starting_position| {
    let mut visited: Vec<(Position, Direction)> = Vec::new();
		let energized_tiles = beam(starting_position.0, starting_position.1, &map, &mut visited);
		println!("Starting position: {:?}, tiles is {}", starting_position, energized_tiles.len());
		return energized_tiles.len();
  })
  .max().unwrap();
	
  println!("Counted energized tiles in {:?}", start.elapsed());
  max
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