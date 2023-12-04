use std::fs;

struct Map {
    grid: Vec<Vec<char>>
}

impl Map {
  fn get_full_number(&self, x: usize, y: usize) -> (usize, usize) {
    let mut digits: Vec<usize> = Vec::new();
    let mut length = 0;
    let mut number: usize = 0;
    let mut multiplier = 1;
    let mut current_y = y.clone();
    if !(&self.grid[x][y].is_numeric()) {
      panic!("Trying to get full number in a non numeric position");
    }
    while self.is_in_bounds(x, current_y) && (&self).grid[x][current_y].is_numeric() {
      length = length + 1;
      digits.push((&self).grid[x][current_y].to_digit(10).unwrap().try_into().unwrap());
      current_y = current_y + 1;
    }
    while digits.len() > 0 {
      number = number + digits.pop().unwrap() * multiplier;
      multiplier = multiplier * 10;
    }
    (number, length)
  }

  fn is_in_bounds(&self, x: usize, y: usize) -> bool {
    x >= 0 && x < (&self).grid.len() && y >= 0 && y < (&self).grid.len()
  }

  fn is_symbol(&self, x: usize, y: usize) -> bool {
    (&self).grid[x][y] != '.' && !(&self).grid[x][y].is_numeric()
  }
  fn is_part_number(&self, x: usize, y: usize) -> bool {
    let mut found = false;
    let mut current_y = y.clone();

    while !found && self.is_in_bounds(x, current_y) && (&self).grid[x][current_y].is_numeric() {
      if x > 0 && current_y > 0 && self.is_symbol(x - 1, current_y - 1) {
        found = true;
      } else if current_y > 0 && self.is_symbol(x, current_y - 1) {
        found = true;
      } else if current_y > 0 && self.is_in_bounds(x + 1, current_y - 1) && self.is_symbol(x + 1, current_y - 1) {
        found = true;
      } else if x > 0 && self.is_in_bounds(x - 1, current_y) && self.is_symbol(x - 1, current_y) {
        found = true;
      } else if self.is_in_bounds(x + 1, current_y) && self.is_symbol(x + 1, current_y) {
        found = true;
      } else if x > 0 && self.is_in_bounds(x - 1, current_y + 1) && self.is_symbol(x - 1, current_y + 1) {
        found = true;
      } else if self.is_in_bounds(x, current_y + 1) && self.is_symbol(x, current_y + 1) {
        found = true;
      } else if self.is_in_bounds(x + 1, current_y + 1) && self.is_symbol(x + 1, current_y + 1) {
        found = true;
      }

      current_y = current_y + 1;
    }

    found
  }

  fn find_next_number(&self, currentX: usize, currentY: usize) -> (bool, usize, usize) {
    let mut nextX = currentX.clone();
    let mut nextY = currentY.clone();
    let mut done = false;
    if (&self).grid[nextX][nextY].is_numeric() {
      return (done, nextX, nextY);
    }
    while !done && !(&self).grid[nextX][nextY].is_numeric() {
      nextY = nextY + 1;
      if nextY >= (&self).grid[nextX].len() {
        nextY = 0;
        nextX = nextX + 1;
        if nextX >= (&self).grid.len() {
          done = true;
          nextX = 0;
        }
      }
    }
    (done, nextX, nextY)
  }
}

fn parse_input(lines: Vec<&str>) -> Map {
  let mut grid: Vec<Vec<char>> = Vec::new();
  for line in lines {
    grid.push(line.chars().collect());
  }
  Map {
    grid: grid
  }
}

fn prob_a(map: Map) -> usize {
  let mut sum = 0;
  let mut currentX = 0;
  let mut currentY = 0;
  let mut done = false;
  while !done {
    (done, currentX, currentY) = map.find_next_number(currentX, currentY);
    if !done {
      let number = map.get_full_number(currentX, currentY);
      if map.is_part_number(currentX, currentY) {
        println!("Found number {}", number.0);
        sum = sum + number.0;
      }
      currentY = currentY + number.1;
      if !map.is_in_bounds(currentX, currentY) {
        currentY = 0;
        currentX = currentX + 1;
        if !map.is_in_bounds(currentX, currentY) {
          done = true;
        }
      }
    }
    
  }

  sum
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let file: String = fs::read_to_string(&args[1]).unwrap();
  let lines: Vec<&str> = file.lines()
    .collect();

  let map = parse_input(lines);
  let result = prob_a(map);

  println!("Result: {}", result);
}