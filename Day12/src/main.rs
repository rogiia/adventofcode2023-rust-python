use std::fs;
use std::time::{Duration, Instant};
use rayon::prelude::*;

#[derive(Debug, PartialEq, Clone)]
enum SpringCondition {
  Operational,
  Damaged,
  Unknown
}

#[derive(Debug)]
struct SpringRow {
  springs: Vec<SpringCondition>,
  condition_record: Vec<usize>
}

impl SpringRow {
  fn is_valid(&self) -> bool {
    let unknown_idx: usize = match self.springs.iter().position(|s| *s == SpringCondition::Unknown) {
      Some(x) => x,
      None => self.springs.len()
    };
    if unknown_idx == 0 {
      return true;
    }
    let mut groups: Vec<usize> = vec![0];
    let mut idx = 0;
    while idx < unknown_idx {
      let last = groups.len() - 1;
      if self.springs[idx] == SpringCondition::Damaged {
        groups[last] = groups[last] + 1;
      } else if groups[last] > 0 {
        groups.push(0);
      }
      idx += 1;
    }
    if groups[groups.len() - 1] == 0 {
      groups.truncate(groups.len() - 1);
    }
    idx = 0;
    let can_be_truncated = unknown_idx < self.springs.len() && self.springs[unknown_idx - 1] != SpringCondition::Operational;
    while idx < groups.len() {
      if self.condition_record.len() <= idx {
        return false;
      } else if can_be_truncated && idx == groups.len() - 1 {
        if groups[idx] > self.condition_record[idx] {
          return false;
        }
      } else if groups[idx] != self.condition_record[idx] {
        return false;
      }
      idx += 1;
    }
    if unknown_idx == self.springs.len() && groups.len() != self.condition_record.len() {
      return false;
    }
    true
  }
}

fn parse_input(lines: &Vec<&str>) -> Vec<SpringRow> {
  let mut rows: Vec<SpringRow> = Vec::new();
  for line in lines {
    let parts: Vec<&str> = line.split(" ").collect();
    if parts.len() != 2 {
      panic!("Invalid line: {}", line);
    }
    let springs: Vec<SpringCondition> = parts[0].chars().map(|c| if c == '.' { return SpringCondition::Operational } else if c == '#' { return SpringCondition::Damaged } else { return SpringCondition::Unknown } ).collect();
    let condition_record: Vec<usize> = parts[1].split(",").map(|n| n.parse::<usize>().unwrap() ).collect();
    rows.push(SpringRow {
      springs: springs,
      condition_record: condition_record
    });
  }
  rows
}

fn count_valid_arrengements_rec(row: &SpringRow, position: usize) -> usize {
  //println!("[POS] {}", position);
  if position == row.springs.len() {
    //println!("[LAST] {:?} is valid {}", row.springs, row.is_valid());
    return if row.is_valid() { 1 } else { 0 };
  }
  if row.springs[position] == SpringCondition::Unknown {
    let mut sum = 0;
    let mut n_row_1 = SpringRow {
      springs: row.springs.clone(),
      condition_record: row.condition_record.clone()
    };
    n_row_1.springs[position] = SpringCondition::Operational;
    let mut n_row_2 = SpringRow {
      springs: row.springs.clone(),
      condition_record: row.condition_record.clone()
    };
    n_row_2.springs[position] = SpringCondition::Damaged;
    //println!("[ROW] {:?} is valid {}", n_row_1.springs, n_row_1.is_valid());
    //println!("[ROW] {:?} is valid {}", n_row_2.springs, n_row_2.is_valid());
    if n_row_1.is_valid() {
      sum += count_valid_arrengements_rec(&n_row_1, position + 1);
    }
    if n_row_2.is_valid() {
      sum += count_valid_arrengements_rec(&n_row_2, position + 1);
    }
    return sum;
  } else {
    return count_valid_arrengements_rec(&row, position + 1);
  }
}

fn unfold(rows: &Vec<SpringRow>) -> Vec<SpringRow> {
  let num_folds = 5;
  let unfolded: Vec<SpringRow> = rows.iter().map(|row| {
    let mut springs: Vec<SpringCondition> = Vec::new();
    let mut condition_record: Vec<usize> = Vec::new();
    for i in 1..(num_folds+1) {
      springs.extend(row.springs.clone());
      condition_record.extend(row.condition_record.clone());
      if i != num_folds {
        springs.push(SpringCondition::Unknown);
      }
    }
    return SpringRow {
      springs: springs,
      condition_record: condition_record
    }
  }).collect();

  unfolded
}

fn prob_a(rows: &Vec<SpringRow>) -> usize {
  let mut sum = 0;
  let mut i = 1;
  let len = rows.len();
  for row in rows {
    let start = Instant::now();
    let valid_arrengements = count_valid_arrengements_rec(&row, 0);
    println!("Row {}/{} has {} valid arrengements ({:?})", i, len, valid_arrengements, start.elapsed());
    sum += valid_arrengements;
    i += 1;
  }

  sum
}

fn prob_b(rows: &Vec<SpringRow>) -> usize {
  let len = rows.len();
  let sum = rows.par_iter().map(|row| {
    let start = Instant::now();
    let valid_arrengements = count_valid_arrengements_rec(&row, 0);
    println!("Row {}/{} has {} valid arrengements ({:?})", 0, len, valid_arrengements, start.elapsed());
    return valid_arrengements;
  })
  .sum();

  sum
}

fn main() {
  let mut start = Instant::now();
  let args: Vec<String> = std::env::args().collect();
  let file: String = fs::read_to_string(&args[1]).unwrap();
  let lines: Vec<&str> = file.lines()
    .collect();

  let rows = parse_input(&lines);
  println!("Parsed input in {:?}", start.elapsed());
  start = Instant::now();
  let unfolded = unfold(&rows);
  println!("Unfolded rows in {:?}", start.elapsed());
  let result_a = prob_b(&unfolded);
  println!("Result: {}", result_a);
}