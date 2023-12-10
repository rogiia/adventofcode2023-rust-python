use std::fs;
use std::collections::HashMap;

type Position = (usize, usize);

#[derive(PartialEq)]
enum Tile {
  Ground,
  Vertical,
  Horizontal,
  NtoE,
  NtoW,
  StoW,
  StoE,
  Starting
}

fn parse_input(lines: &Vec<&str>) -> (Position, Vec<Vec<Tile>>) {
  let mut starting_position: Option<Position> = None;
  let mut map: Vec<Vec<Tile>> = Vec::new();
  for (i, line) in lines.iter().enumerate() {
    let mut file: Vec<Tile> = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    for (j, tile) in chars.iter().enumerate() {
      match tile {
        '.' => file.push(Tile::Ground),
        '|' => file.push(Tile::Vertical),
        '-' => file.push(Tile::Horizontal),
        'L' => file.push(Tile::NtoE),
        'J' => file.push(Tile::NtoW),
        '7' => file.push(Tile::StoW),
        'F' => file.push(Tile::StoE),
        'S' => {
          starting_position = Some((i, j));
          file.push(Tile::Starting)
        },
        _ => panic!("Found invalid tile {}!", tile)
      }
    }
    map.push(file);
  }
  match starting_position {
    Some(p) => return (p, map),
    None => panic!("There is no starting position!")
  }  
}

fn find_starting_positions(starting_position: Position, map: &Vec<Vec<Tile>>) -> (Position, Position) {
  let mut found: Vec<Position> = Vec::new();
  let mut positions_to_look: Vec<(Position, Vec<Tile>)> = Vec::new();
  if starting_position.0 > 0 {
    positions_to_look.push(((starting_position.0 - 1, starting_position.1), vec![Tile::Vertical, Tile::StoE, Tile::StoW]));
  }
  if starting_position.1 > 0 {
    positions_to_look.push(((starting_position.0, starting_position.1 - 1), vec![Tile::Horizontal, Tile::StoE, Tile::NtoE]));
  }
  if starting_position.0 < map.len() - 1 {
    positions_to_look.push(((starting_position.0 + 1, starting_position.1), vec![Tile::Vertical, Tile::NtoE, Tile::NtoW]));
  }
  if starting_position.0 < map[0].len() - 1 {
    positions_to_look.push(((starting_position.0, starting_position.1 + 1), vec![Tile::Horizontal, Tile::StoW, Tile::NtoW]));
  }
  for position in positions_to_look {
    if position.1.contains(&map[position.0.0][position.0.1]) {
      found.push(position.0);
    }
  }

  (found[0], found[1])
}

fn find_next_position(current_position: Position, previous_position: Position, map: &Vec<Vec<Tile>>) -> Position {
  match map[current_position.0][current_position.1] {
    Tile::Vertical => {
      if current_position.0 > 0 && (current_position.0 - 1, current_position.1) != previous_position {
        return (current_position.0 - 1, current_position.1);
      } else {
        return (current_position.0 + 1, current_position.1);
      }
    },
    Tile::Horizontal => {
      if current_position.1 > 0 && (current_position.0, current_position.1 - 1) != previous_position {
        return (current_position.0, current_position.1 - 1);
      } else {
        return (current_position.0, current_position.1 + 1);
      }
    },
    Tile::NtoE => {
      if current_position.0 > 0 && (current_position.0 - 1, current_position.1) != previous_position {
        return (current_position.0 - 1, current_position.1);
      } else {
        return (current_position.0, current_position.1 + 1);
      }
    },
    Tile::NtoW => {
      if current_position.0 > 0 && (current_position.0 - 1, current_position.1) != previous_position {
        return (current_position.0 - 1, current_position.1);
      } else {
        return (current_position.0, current_position.1 - 1);
      }
    },
    Tile::StoW => {
      if current_position.0 < map.len() - 1 && (current_position.0 + 1, current_position.1) != previous_position {
        return (current_position.0 + 1, current_position.1);
      } else {
        return (current_position.0, current_position.1 - 1);
      }
    },
    Tile::StoE => {
      if current_position.0 < map.len() - 1 && (current_position.0 + 1, current_position.1) != previous_position {
        return (current_position.0 + 1, current_position.1);
      } else {
        return (current_position.0, current_position.1 + 1);
      }
    },
    _ => {
      panic!("Cannot find next position from {:?}", current_position);
    }
  }
}

fn print_distance_map(distances: &HashMap<Position, usize>, map: &Vec<Vec<Tile>>) {
  let mut i = 0;
  while i < map.len() {
    let mut j = 0;
    let mut line: Vec<String> = Vec::new();
    while j < map[i].len() {
      if map[i][j] == Tile::Starting {
        line.push("  0".to_owned());
      } else if distances.contains_key(&(i, j)) {
        line.push(format!("{: >3}", distances.get(&(i, j)).unwrap().to_string()));
      } else {
        line.push("  .".to_owned());
      }
      j = j + 1;
    }
    println!("{:?}", line.join(""));
    i = i + 1;
  }
}

fn prob_a(starting_position: Position, map: &Vec<Vec<Tile>>) -> usize {
  let mut current_distance: usize = 1;
  let mut distances: HashMap<Position, usize> = HashMap::new();
  let mut current_positions: (Position, Position) = find_starting_positions(starting_position, &map);
  let mut previous_positions: (Position, Position) = (starting_position, starting_position);
  while !distances.contains_key(&current_positions.0) && !distances.contains_key(&current_positions.1) {
    distances.insert(current_positions.0, current_distance);
    distances.insert(current_positions.1, current_distance);
    let new_previous_positions = current_positions;
    current_positions = (
      find_next_position(current_positions.0, previous_positions.0, &map),
      find_next_position(current_positions.1, previous_positions.1, &map)
    );
    previous_positions = new_previous_positions;
    current_distance = current_distance + 1;
  }
  current_distance - 1
}

fn prob_b(starting_position: Position, map: &Vec<Vec<Tile>>) -> usize {
  let mut current_distance: usize = 1;
  let mut distances: HashMap<Position, usize> = HashMap::new();
  let mut current_position: Position = find_starting_positions(starting_position, &map).0;
  let mut previous_position: Position = starting_position;
  while map[current_position.0][current_position.1] != Tile::Starting {
    distances.insert(current_position, current_distance);
    let new_previous_position = current_position;
    current_position = find_next_position(current_position, previous_position, &map);
    previous_position = new_previous_position;
    current_distance = current_distance + 1;
  }

  print_distance_map(&distances, &map);

  // Find inside and outside tiles
  let mut enclosed_tiles = 0;
  let mut i = 1;
  while i < map.len() - 1 {
    let mut j = 0;
    let mut inside: i32 = 0;
    let mut to_print: Vec<char> = Vec::new();
    while j < map[i].len() {
      //println!("Loop at {}, {}", i, j);
      let distance_1 = if map[i][j] == Tile::Starting { Some(&0) } else { distances.get(&(i, j)) };
      if distance_1 == None {
        if inside != 0 {
          //println!("Found enclosed tile at {}, {}; Inside value {}", i, j, inside);
          enclosed_tiles = enclosed_tiles + 1;
        }
      } else {
        let distance_2 = distances.get(&(i+1, j));
        if distance_1 != None && distance_2 != None {
          //println!("Distance 1: {}, Distance 2: {}", distance_1.unwrap(), distance_2.unwrap());
          let difference: i32 = distance_1.unwrap().clone() as i32 - distance_2.unwrap().clone() as i32;
          if difference == 1 || difference == -1 {
            inside = inside + difference;
            //println!("Inside value: {}", inside);
          }
        }
      }
      j = j + 1;
    }
    println!("{}", to_print.into_iter().collect::<String>());
    i = i + 1;
  }
  enclosed_tiles
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let file: String = fs::read_to_string(&args[1]).unwrap();
  let lines: Vec<&str> = file.lines()
    .collect();

  let (starting_position, map) = parse_input(&lines);
  let result = prob_b(starting_position, &map);

  println!("Result: {}", result);
}