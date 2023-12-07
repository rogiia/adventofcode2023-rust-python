use std::fs;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Card {
  label: char,
  value: u8
}

impl Card {
  pub fn new(label: char, j_as_joker: bool) -> Self {
    let value: u8;
    match label.to_digit(10) {
      Some(n) => { value = n as u8 },
      None => {
        if label == 'T' {
          value = 10;
        } else if label == 'J' {
          if j_as_joker {
            value = 1;
          } else {
            value = 11;
          }
        } else if label == 'Q' {
          value = 12;
        } else if label == 'K' {
          value = 13;
        } else if label == 'A' {
          value = 14;
        } else {
          panic!("Unexpected card {}!", label);
        }
      }
    }
    Self {
      label: label,
      value: value
    }
  }
}

impl PartialOrd for Card {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(self.cmp(other))
  }

  fn lt(&self, other: &Self) -> bool {
    self.value < other.value
  }

  fn le(&self, other: &Self) -> bool {
    self.value <= other.value
  }

  fn gt(&self, other: &Self) -> bool {
    self.value > other.value
  }

  fn ge(&self, other: &Self) -> bool {
    self.value >= other.value
  }
}

impl Ord for Card {
  fn cmp(&self, other: &Self) -> Ordering {
      self.value.cmp(&other.value)
  }
}

impl PartialEq for Card {
  fn eq(&self, other: &Self) -> bool {
      self.value == other.value
  }
}
impl Eq for Card {}

#[derive(Debug)]
struct Hand {
  cards: Vec<Card>,
  bid: usize,
  j_as_joker: bool
}

impl Hand {
  fn get_type(&self) -> u8 {
    let mut groups: HashMap<char, u8> = HashMap::new();
    let cards = self.cards.clone();
    let mut jokers = 0;
    for card in &cards {
      if self.j_as_joker && card.label == 'J' {
        jokers = jokers + 1;
      } else {
        if groups.contains_key(&card.label) {
          match groups.get(&card.label) {
            Some(x) => { groups.insert(card.label, x + 1); },
            None => {}
          };
        } else {
          groups.insert(card.label, 1);
        }
      }
    }
    let mut kinds: Vec<u8> = groups.into_values().collect();
    kinds.sort();
    kinds.reverse();
    if kinds.len() < 1 {
      if self.j_as_joker && jokers == 5 {
        return 6 // Five of a kind
      } else {
        panic!("Hand has not enough kinds of cards to get a type");
      }
    }
    if self.j_as_joker {
      kinds[0] = kinds[0] + jokers;
    }
    if kinds[0] == 5 {
      6 // Five of a kind
    } else if kinds[0] == 4 {
      5 // Four of a kind
    } else if kinds[0] == 3 && kinds[1] == 2 {
      4 // Full house
    } else if kinds[0] == 3 {
      3 // Three of a kind
    } else if kinds[0] == 2 && kinds[1] == 2 {
      2 // Two pair
    } else if kinds[0] == 2 {
      1 // One pair
    } else {
      0 // High card
    }
  }
}


impl PartialOrd for Hand {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(self.cmp(other))
  }
}

impl Ord for Hand {
  fn cmp(&self, other_hand: &Self) -> Ordering {
    let own_type = self.get_type();
    let other_type = other_hand.get_type();
    if own_type > other_type {
      return Ordering::Greater;
    } else if own_type < other_type {
      return Ordering::Less;
    } else {
      let mut idx = 0;
      while idx < self.cards.len() {
        if self.cards[idx] > other_hand.cards[idx] {
          return Ordering::Greater;
        } else if self.cards[idx] < other_hand.cards[idx] {
          return Ordering::Less;
        }
        idx = idx + 1;
      }
    }
    Ordering::Equal
  }
}
impl PartialEq for Hand {
  fn eq(&self, other: &Self) -> bool {
      self.cmp(other) == Ordering::Equal
  }
}
impl Eq for Hand {}

fn parse_input(lines: Vec<&str>, j_as_joker: bool) -> Vec<Hand> {
  let mut hands: Vec<Hand> = Vec::new();
  for line in lines {
    let parts: Vec<&str> = line.split(" ").collect();
    if parts.len() != 2 {
      panic!("Hand {} does not have two components as expected!", line);
    }
    hands.push(Hand {
      cards: parts[0].chars().map(|c| Card::new(c, j_as_joker)).collect(),
      bid: parts[1].parse::<usize>().unwrap(),
      j_as_joker: j_as_joker
    });
  }

  hands
}

fn solve(hands: &Vec<Hand>) -> usize {
  let mut total: usize = 0;
  let mut rank: usize = 1;
  for hand in hands {
    total = total + hand.bid * rank;
    rank = rank + 1;
  }
  total
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let file: String = fs::read_to_string(&args[1]).unwrap();
  let lines: Vec<&str> = file.lines()
    .collect();

  let mut hands = parse_input(lines, true);
  hands.sort();
  let result = solve(&hands);

  println!("Result: {}", result);
}