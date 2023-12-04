fn prob_a(items: &Vec<&str>) -> u32 {
  let mut sum: u32 = 0;
  for item in items {
    let mut numbers: (u32, u32) = (0, 0);
    let mut found: bool = false;
    for c in item.chars() {
      match c.to_digit(10) {
        Some(x) => {
          if !found {
            numbers.0 = x;
            numbers.1 = x;
            found = true
          } else {
            numbers.1 = x;
          }
        },
        None => {}
      }
    }
    sum = sum + (numbers.0 * 10 + numbers.1)
  }
  return sum;
}

fn prob_b(items: &Vec<&str>) -> u32 {
  let mut sum: u32 = 0;
  let numbers: Vec<(u32, &str)> = vec![(1, &"one"), (2, &"two"), (3, &"three"), (4, &"four"), (5, &"five"), (6, &"six"), (7, &"seven"), (8, &"eight"), (9, &"nine")];
  for item in items {
    let mut first: (u32, u32) = (0, u32::MAX);
    let mut last: (u32, u32) = (0, 0);
    for number in &numbers {
      match item.find(std::char::from_digit(number.0, 10).unwrap()) {
        Some(x) => {
          if x < (first.1 as usize) {
            first = (number.0, x as u32);
          }
        },
        None => {}
      }
      match item.rfind(std::char::from_digit(number.0, 10).unwrap()) {
        Some(x) => {
          if x >= (last.1 as usize) {
            last = (number.0, x as u32);
          }
        },
        None => {}
      }
      match item.find(number.1) {
        Some(x) => {
          if x < (first.1 as usize) {
            first = (number.0, x as u32);
          }
        }
        None => {}
      }
      match item.rfind(number.1) {
        Some(x) => {
          if x >= (last.1 as usize) {
            last = (number.0, x as u32);
          }
        },
        None => {}
      }
    }
    println!("{}, {}, {}", first.0, last.0, item);
    sum = sum + (first.0 * 10 + last.0)
  }
  return sum;
}

fn main() {
  let items: Vec<&str> = include_str!("./input.txt")
    .lines()
    .collect();

  let result = prob_b(&items);

  println!("Result: {}", result);
}