use std::fs;
use std::time::{Instant};
use std::collections::HashMap;

#[derive(Debug)]
struct InitSequence {
  steps: Vec<String>
}

#[derive(Debug)]
struct Lens {
  label: String,
  focal_length: usize
}

fn parse_input(input: &str) -> InitSequence {
  let mut steps: Vec<String> = Vec::new();
  for part in input.split(",") {
    steps.push(part.to_owned());
  }
  InitSequence {
    steps
  }
}

fn hash_algorithm(input: &str) -> usize {
  let mut sum = 0;
  for c in input.chars() {
    let ascii_val = c as usize;
    sum += ascii_val;
    sum *= 17;
    sum = sum % 256;
  }
  sum
}

fn get_focusing_power(boxes: &HashMap<usize, Vec<Lens>>) -> usize {
  let mut sum: usize = 0;
  for (id, lenses) in boxes.iter() {
    let box_number = id + 1;
    let mut idx = 0;
    while idx < lenses.len() {
      sum += box_number * (idx + 1) * lenses[idx].focal_length;
      idx += 1;
    }
  }
  sum
}

fn prob_a(init_sequence: &InitSequence) -> usize {
  let start = Instant::now();
  let mut sum: usize = 0;
  println!("{:?}", init_sequence);
  for step in &init_sequence.steps {
    sum += hash_algorithm(step.as_str());
  }
  println!("Calculated hash in {:?}", start.elapsed());
  sum
}

fn prob_b(init_sequence: &InitSequence) -> usize {
  let start = Instant::now();
  let mut boxes: HashMap<usize, Vec<Lens>> = HashMap::new();
  for step in &init_sequence.steps {
    let parts: Vec<&str> = step.split("=").collect();
    if parts.len() == 2 {
      let hash = hash_algorithm(parts[0]);
      if boxes.contains_key(&hash) {
        let lenses = boxes.get_mut(&hash).unwrap();
        let mut found = false;
        let mut idx = 0;
        while !found && idx < lenses.len() {
          if lenses[idx].label == parts[0].to_owned() {
            found = true;
            lenses[idx].focal_length = parts[1].parse::<usize>().unwrap();
          }
          idx += 1;
        }
        if !found {
          lenses.push(Lens {
            label: parts[0].to_owned(),
            focal_length: parts[1].parse::<usize>().unwrap()
          });
        }
        
      } else {
        boxes.insert(hash, vec![Lens {
          label: parts[0].to_owned(),
          focal_length: parts[1].parse::<usize>().unwrap()
        }]);
      }
    } else {
      let label: Vec<&str> = step.split("-").collect();
      let hash = hash_algorithm(label[0]);
      match boxes.get_mut(&hash) {
        Some(b) => b.retain(|value| value.label != label[0]),
        None => {}
      };
    }
  }
  //println!("{:?}", boxes);
  println!("Inserted lenses in {:?}", start.elapsed());
  get_focusing_power(&boxes)
}

fn main() {
  let start = Instant::now();
  let args: Vec<String> = std::env::args().collect();
  let file: String = fs::read_to_string(&args[1]).unwrap();
  let init_sequence = parse_input(file.as_str());
  println!("Parsed input in {:?}", start.elapsed());
  let result = prob_b(&init_sequence);
  println!("Result: {}", result);
}