use std::fs;
use regex::Regex;

struct Throw {
  red: u32,
  green: u32,
  blue: u32
}

struct Game {
  id: u32,
  throws: Vec<Throw>
}

fn parse_game(line: &str) -> Game {
  let re_game = Regex::new(r"Game (?<id>\d+): (?<cubes>.*)").unwrap();
  let re_throw = Regex::new(r"(?<num>\d+) (?<color>\w+)").unwrap();
  match re_game.captures(line) {
    Some(cap_game) => {
      let throws: Vec<&str> = (&cap_game["cubes"]).split("; ").collect();
      let mut game_throws: Vec<Throw> = Vec::new();
      for throw in throws {
        let colors: Vec<&str> = throw.split(", ").collect();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for color in colors {
          match re_throw.captures(color) {
            Some(cap_throw) => {
              if &cap_throw["color"] == "red" {
                red = (&cap_throw["num"]).parse().unwrap();
              } else if &cap_throw["color"] == "green" {
                green = (&cap_throw["num"]).parse().unwrap();
              } else if &cap_throw["color"] == "blue" {
                blue = (&cap_throw["num"]).parse().unwrap();
              }
            },
            None => panic!("Failed throw regex")
          };
        }
        game_throws.push(Throw {
          red: red,
          green: green,
          blue: blue
        });
      }
      return Game {
        id: (&cap_game["id"]).parse().unwrap(),
        throws: game_throws
      }
    },
    None => panic!("Failed game regex")
  };
}

fn prob_a(items: &Vec<&str>) -> u32 {
  let red_cubes = 12;
  let green_cubes = 13;
  let blue_cubes = 14;
  let mut sum = 0;
  
  for item in items {
    let mut is_game_possible = true;
    let game = parse_game(item);
    for throw in game.throws {
      if throw.red > red_cubes || throw.green > green_cubes || throw.blue > blue_cubes {
        is_game_possible = false;
        break;
      }
    }
    if is_game_possible {
      sum = sum + game.id;
      println!("Game {} is possible, sum {}", game.id, sum);
    }
  }
  return sum;
}

fn prob_b(items: &Vec<&str>) -> u32 {
  let mut sum = 0;
  
  for item in items {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;
    let game = parse_game(item);
    for throw in game.throws {
      if throw.red > min_red {
        min_red = throw.red;
      }
      if throw.green > min_green {
        min_green = throw.green;
      }
      if throw.blue > min_blue {
        min_blue = throw.blue;
      }
    }
    sum = sum + (min_red * min_green * min_blue);
  }
  return sum;
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let file: String = fs::read_to_string(&args[1]).unwrap();
  let lines: Vec<&str> = file.lines()
    .collect();

  let result = prob_b(&lines);

  println!("Result: {}", result);
}