use std::fs;

type Position = (usize, usize);
type Galaxy = (usize, Position);

fn parse_input(lines: &Vec<&str>) -> Vec<Vec<usize>> {
  let mut galaxy_number = 1;
  let mut map: Vec<Vec<usize>> = Vec::new();
  for line in lines {
    let mut file: Vec<usize> = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    for c in chars {
      match c {
        '.' => file.push(0),
        '#' => {
          file.push(galaxy_number);
          galaxy_number = galaxy_number + 1;
        },
        _ => {
          panic!("Invalid character found: {}", c);
        }
      }
    }
    map.push(file);
  }
  map
}

fn print_map(map: &Vec<Vec<usize>>) {
  for line in map {
    let str_line: Vec<String> = line.iter().map(|n| n.to_string()).collect();
    let to_print: String = str_line.join("");
    println!("{}", to_print);
  }
}

fn find_empty_rows_and_columns(map: &Vec<Vec<usize>>) -> (Vec<usize>, Vec<usize>) {
  let mut empty_rows: Vec<usize> = Vec::new();
  let mut empty_columns: Vec<usize> = Vec::new();
  // Rows
  for (n, line) in map.iter().enumerate() {
    let mut is_empty = true;
    let mut idx = 0;
    while is_empty && idx < line.len() {
      if line[idx] > 0 {
        is_empty = false;
      }
      idx += 1;
    }
    if is_empty {
      empty_rows.push(n);
    }
  }

  // Columns
  let mut j = 0;
  while j < map[0].len() {
    let mut is_empty = true;
    let mut i = 0;
    while is_empty && i < map.len() {
      if map[i][j] > 0 {
        is_empty = false;
      }
      i += 1;
    }
    if is_empty {
      empty_columns.push(j);
    }
    j += 1;
  }

  (empty_rows, empty_columns)
}

fn expand_map(map: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
  let mut expanded_map: Vec<Vec<usize>> = Vec::new();
  let (empty_rows, empty_columns) = find_empty_rows_and_columns(&map);
  println!("Found empty rows: {:?}", empty_rows);
  println!("Found empty columns: {:?}", empty_columns);

  let mut i = 0;
  while i < map.len() {
    let mut j = 0;
    let mut line: Vec<usize> = Vec::new();
    while j < map[i].len() {
      line.push(map[i][j]);
      if empty_columns.contains(&j) {
        line.push(map[i][j]);
      }
      j += 1;
    }
    expanded_map.push(line.clone());
    if empty_rows.contains(&i) {
      expanded_map.push(line.clone());
    }
    i += 1;
  }
  expanded_map
}

fn find_galaxies(map: &Vec<Vec<usize>>) -> Vec<Galaxy> {
  let mut galaxies: Vec<Galaxy> = Vec::new();
  let mut i = 0;
  while i < map.len() {
    let mut j = 0;
    while j < map[i].len() {
      if map[i][j] > 0 {
        galaxies.push((map[i][j], (i, j)));
      }
      j += 1;
    }
    i += 1;
  }
  galaxies
}

fn find_shortest_path(a: &Galaxy, b: &Galaxy) -> usize {
  (if a.1.0 >= b.1.0 { a.1.0 - b.1.0 } else { b.1.0 - a.1.0 })
    + (if a.1.1 >= b.1.1 { a.1.1 - b.1.1 } else { b.1.1 - a.1.1 })
}

fn number_in_between(a: usize, b: usize, n: usize) -> bool {
  if a == b {
    false
  } else if a > b {
    n < a && n > b
  } else {
    n > a && n < b
  }
}

fn find_num_of_empty_lines_in_path(
  a: &Galaxy, b: &Galaxy,
  empty_rows: &Vec<usize>, empty_columns: &Vec<usize>) -> usize {
    let num_empty_rows: usize = empty_rows.iter().fold(0, |acc, row| if number_in_between(a.1.0, b.1.0, *row) { acc + 1 } else { acc });
    let num_empty_columns: usize = empty_columns.iter().fold(0, |acc, col| if number_in_between(a.1.1, b.1.1, *col) { acc + 1 } else { acc });
    return num_empty_rows + num_empty_columns;
}

fn prob_a(map: &Vec<Vec<usize>>) -> usize {
  let mut sum = 0;
  let galaxies = find_galaxies(&map);
  let mut pairs: usize = 0;
  let mut idx = 0;
  while idx < galaxies.len() {
    let mut j = idx + 1;
    while j < galaxies.len() {
      pairs += 1;
      sum += find_shortest_path(&galaxies[idx], &galaxies[j]);
      j += 1;
    }
    idx += 1;
  }
  println!("Found {} pairs of galaxies", pairs);

  sum
}

fn prob_b(map: &Vec<Vec<usize>>) -> usize {
  let mut sum = 0;
  let (empty_rows, empty_columns) = find_empty_rows_and_columns(&map);
  let galaxies = find_galaxies(&map);
  let mut pairs: usize = 0;
  let mut idx = 0;
  while idx < galaxies.len() {
    let mut j = idx + 1;
    while j < galaxies.len() {
      pairs += 1;
      sum += find_shortest_path(&galaxies[idx], &galaxies[j]);
      sum += find_num_of_empty_lines_in_path(&galaxies[idx], &galaxies[j], &empty_rows, &empty_columns) * (1000000 - 1);
      j += 1;
    }
    idx += 1;
  }
  println!("Found {} pairs of galaxies", pairs);

  sum
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let file: String = fs::read_to_string(&args[1]).unwrap();
  let lines: Vec<&str> = file.lines()
    .collect();

  let map = parse_input(&lines);
  let expanded_map = expand_map(&map);
  let result_a = prob_a(&expanded_map);
  println!("Result: {}", result_a);
  let result_b = prob_b(&map);
  println!("Result: {}", result_b);
}