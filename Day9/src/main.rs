use std::fs;
use std::collections::VecDeque;

#[derive(Debug)]
struct History {
  readings: Vec<VecDeque<i32>>
}

impl History {
  pub fn new(readings: Vec<i32>) -> Self {
    let mut own_readings: Vec<VecDeque<i32>> = Vec::new();
    let mut idx = 0;
    own_readings.push(VecDeque::from(readings));
    while !all_readings_are_zero(&(own_readings[idx])) {
      own_readings.push(calc_differences(&(own_readings[idx])));
      idx = idx + 1;
    }
    Self {
      readings: own_readings
    }
  }

  fn get_next_value(&mut self) -> i32 {
    if self.readings.len() == 0 {
      panic!("Cannot get next value! There are no readings in history!");
    }
    let mut idx = self.readings.len() - 1;
    self.readings[idx].push_back(0);
    while idx > 0 {
      let last_upper_reading = self.readings[idx - 1].back().unwrap().clone();
      let last_current_reading = self.readings[idx].back().unwrap().clone();
      self.readings[idx - 1].push_back(last_upper_reading + last_current_reading);
      idx = idx - 1;
    }

    *self.readings[0].back().unwrap()
  }

  fn get_previous_value(&mut self) -> i32 {
    if self.readings.len() == 0 {
      panic!("Cannot get next value! There are no readings in history!");
    }
    let mut idx = self.readings.len() - 1;
    self.readings[idx].push_front(0);
    while idx > 0 {
      let first_upper_reading = self.readings[idx - 1].front().unwrap().clone();
      let first_current_reading = self.readings[idx].front().unwrap().clone();
      self.readings[idx - 1].push_front(first_upper_reading - first_current_reading);
      idx = idx - 1;
    }

    *self.readings[0].front().unwrap()
  }
}

fn calc_differences(readings: &VecDeque<i32>) -> VecDeque<i32> {
  let mut idx = 1;
  let mut differences: VecDeque<i32> = VecDeque::new();
  while idx < readings.len() {
    differences.push_back(readings[idx] - readings[idx - 1]);
    idx = idx + 1;
  }
  differences
}

fn all_readings_are_zero(readings: &VecDeque<i32>) -> bool {
  for reading in readings {
    if *reading != 0 {
      return false;
    }
  }
  true
}

fn parse_input(lines: Vec<&str>) -> Vec<Vec<i32>> {
  let mut histories = Vec::new();
  for line in lines {
    let readings: Vec<i32> = line.split(" ").map(|r| r.parse::<i32>().unwrap() ).collect();
    histories.push(readings);
  }

  histories
}

fn prob_a(hist_readings: Vec<Vec<i32>>) -> i32 {
  let mut sum: i32 = 0;
  let mut histories: Vec<History> = Vec::new();
  for readings in hist_readings {
    histories.push(History::new(readings));
  }
  for history in &mut histories {
    sum = sum + history.get_next_value();
  }
  sum
}

fn prob_b(hist_readings: Vec<Vec<i32>>) -> i32 {
  let mut sum: i32 = 0;
  let mut histories: Vec<History> = Vec::new();
  for readings in hist_readings {
    histories.push(History::new(readings));
  }
  for history in &mut histories {
    sum = sum + history.get_previous_value();
  }
  sum
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let file: String = fs::read_to_string(&args[1]).unwrap();
  let lines: Vec<&str> = file.lines()
    .collect();

  let histories = parse_input(lines);
  let result = prob_b(histories);

  println!("Result: {}", result);
}