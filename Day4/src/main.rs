use std::fs;
use regex::Regex;

struct Scratchcard {
  id: usize,
  winning_numbers: Vec<usize>,
  own_numbers: Vec<usize>
}

impl Scratchcard {
  fn sort_numbers(&mut self) {
    self.winning_numbers.sort();
    self.own_numbers.sort();
  }

  fn get_winning_numbers(&self) -> Vec<usize> {
    let mut winning_nums: Vec<usize> = Vec::new();
    for num in &self.own_numbers {
      if (&self).winning_numbers.contains(&num) {
        winning_nums.push(*num);
      }
    }
    return winning_nums;
  }
}

fn parse_input(lines: Vec<&str>) -> Vec<Scratchcard> {
  let mut cards: Vec<Scratchcard> = Vec::new();
  let re = Regex::new(r"Card\s+(?<id>\d+): (?<winning>[\d\s]*) \| (?<own>[\d\s]*)").unwrap();
  for line in lines {
    match re.captures(line) {
      Some(captures) => {
        let winning_num: Vec<&str> = (&captures)["winning"].split(" ").filter(|s| !s.is_empty()).collect();
        let own_num: Vec<&str> = (&captures)["own"].split(" ").filter(|s| !s.is_empty()).collect();
        cards.push(Scratchcard {
          id: (&captures)["id"].parse::<usize>().unwrap(),
          winning_numbers: winning_num.iter().map(|n| n.parse::<usize>().unwrap()).collect(),
          own_numbers: own_num.iter().map(|n| n.parse::<usize>().unwrap()).collect()
        });
      },
      None => panic!("Could not parse card")
    }
  }

  cards
}

fn prob_a(cards: &mut Vec<Scratchcard>) -> usize {
  let mut sum: usize = 0;
  for card in cards {
    card.sort_numbers();
    let winning_nums = card.get_winning_numbers();
    let base: usize = 2;
    if winning_nums.len() > 0 {
      sum = sum + base.pow((winning_nums.len() - 1).try_into().unwrap());
    }
  }

  sum
}

fn prob_b(cards: &mut Vec<Scratchcard>) -> usize {
  let mut sum: usize = 0;
  let mut instances: Vec<usize> = vec![1; cards.len()];
  for (idx, card) in cards.iter().enumerate() {
    let winning_nums = card.get_winning_numbers();
    let mut j = idx + 1;
    while j <= idx + winning_nums.len() {
      instances[j] = instances[j] + instances[idx];
      j = j + 1;
    }
  }

  for instance in instances {
    sum = sum + instance;
  }

  sum
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let file: String = fs::read_to_string(&args[1]).unwrap();
  let lines: Vec<&str> = file.lines()
    .collect();

  let mut cards = parse_input(lines);
  let result = prob_b(&mut cards);

  println!("Result: {}", result);
}