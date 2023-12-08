use std::fs;
use regex::Regex;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use once_cell::sync::Lazy;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use num::integer::lcm;

#[derive(PartialEq)]
enum Direction {
  Left,
  Right
}

struct Node {
  id: String,
  left: String,
  right: String
}

impl Node {
  pub fn new(input: &str) -> Self {
    static NODE_RE: Lazy<regex::Regex> = Lazy::new(|| Regex::new(r"(?<id>\w{3}) = \((?<left>\w{3}), (?<right>\w{3})\)").unwrap());
    match NODE_RE.captures(input) {
      Some(captures) => {
        Node {
          id: (&captures["id"]).to_owned(),
          left: (&captures["left"]).to_owned(),
          right: (&captures["right"]).to_owned(),
        }
      },
      None => panic!("Could not parse node {}", input)
    }
  }
}

fn parse_input(file: &str) -> (Vec<Direction>, HashMap<String, Node>) {
  let start = Instant::now();
  let re = Regex::new(r"(?s)(?<directions>[LR]+)\n\n(?<nodes>.+)").unwrap();
  match re.captures(file) {
      Some(captures) => {
        let directions: Vec<Direction> = (&captures["directions"])
          .chars()
          .map(|d| if d == 'L' { Direction::Left } else { Direction::Right }).collect();
        let node_list = (&captures["nodes"])
          .split("\n");
        let mut node_map: HashMap<String, Node> = HashMap::new();
        for node_def in node_list {
          let start_n = Instant::now();
          let new_node = Node::new(node_def);
          node_map.insert(new_node.id.clone(), new_node);
        }

        return (directions, node_map);
      },
      None => panic!("Could not parse input file!")
  }
}

fn find_start_nodes(nodes: &HashMap<String, Node>) -> Vec<String> {
  nodes.keys()
    .filter(|key| key.ends_with("A"))
    .map(|key| key.clone() )
    .collect()
}

fn all_nodes_have_same_distances(current_nodes: &Vec<String>, distances: &HashMap<String, (usize, String)>) -> bool {
  let distance: usize;
  match distances.get(&current_nodes[0]) {
    Some(d) => {
      if d.0 == usize::MAX {
        return false;
      }
      distance = d.0.clone();
    },
    None => panic!("Could not find distance {}", current_nodes[0])
  }
  let mut idx = 1;
  while idx < current_nodes.len() {
    match distances.get(&current_nodes[idx]) {
      Some(d) => if d.0 == usize::MAX || d.0 != distance { return false },
      None => panic!("Could not find distance {}", current_nodes[idx])
    }
    idx = idx + 1;
  }

  true
}

fn find_distances_to_end(distances: &mut HashMap<String, (usize, String)>, nodes: &HashMap<String, Node>, directions: &Vec<Direction>) {
  let keys = distances.keys().clone();
  for (key, value) in distances.iter_mut() {
    let mut distance = 0;
    let mut current_node: String = key.clone();
    for direction in directions {
      match nodes.get(&current_node) {
        Some(n) => current_node = if *direction == Direction::Left { n.left.clone() } else { n.right.clone() },
        None => panic!("Could not find node {}", current_node)
      }
      distance = distance + 1;
      if current_node.ends_with("Z") {
        value.0 = distance;
      }
    }
    value.1 = current_node;
  }
}

fn do_iteration(current_nodes: &mut Vec<String>, nodes: &HashMap<String, Node>, directions: &Vec<Direction>) -> usize {
  let mut idx = 0;
  while idx < current_nodes.len() {
    let mut current_node: String = current_nodes[idx].clone();
    for direction in directions {
      match nodes.get(&current_node) {
        Some(n) => current_node = if *direction == Direction::Left { n.left.clone() } else { n.right.clone() },
        None => panic!("Could not find node {}", current_node)
      }
    }
    current_nodes[idx] = current_node;
    idx = idx + 1;
  }

  directions.len()
}

fn calc_cycle(node: &String, nodes: &HashMap<String, Node>, directions: &Vec<Direction>) -> usize {
  let mut distance = 0;
  let mut current_node: String = node.clone();
  let mut direction_idx = 0;
  while true {
    match nodes.get(&current_node) {
      Some(n) => current_node = if directions[direction_idx] == Direction::Left { n.left.clone() } else { n.right.clone() },
      None => panic!("Could not find node {}", current_node)
    }
    distance = distance + 1;
    if current_node.ends_with("Z") {
      return distance;
    }
    direction_idx = direction_idx + 1;
    if direction_idx >= directions.len() {
      direction_idx = 0;
    }
  }
  panic!("Cannot find cycle for node {}", current_node);
}

fn prob_a(directions: &Vec<Direction>, nodes: &HashMap<String, Node>) -> usize {
  let mut steps: usize = 0;
  let mut current_node: String = "AAA".to_owned();
  let mut current_direction_idx = 0;
  while current_node != "ZZZ" {
    //println!("Current node {}", current_node);
    match nodes.get(&current_node) {
      Some(n) => current_node = if directions[current_direction_idx] == Direction::Left { n.left.clone() } else { n.right.clone() },
      None => panic!("Could not find node {}", current_node)
    }
    steps = steps + 1;
    current_direction_idx = current_direction_idx + 1;
    if current_direction_idx >= directions.len() {
      current_direction_idx = 0;
    }
  }

  steps
}

fn prob_b(directions: &Vec<Direction>, nodes: &HashMap<String, Node>) -> usize {
  let start = Instant::now();
  let mut steps = 0;
  let mut current_nodes: Vec<String> = find_start_nodes(&nodes);
  let mut distances: HashMap<String, (usize, String)> = nodes.keys().map(|k| (k.clone(), (usize::MAX, "".to_owned()))).collect();
  find_distances_to_end(&mut distances, &nodes, &directions);
  println!("Time elapsed finding distances to end: {:?}", start.elapsed());
  
  while !all_nodes_have_same_distances(&current_nodes, &distances) {
    current_nodes = current_nodes.iter().map(|n| distances.get(n).unwrap().1.clone() ).collect();
    steps = steps + directions.len();
    println!("Steps: {}, Time: {:?}", steps, start.elapsed());
  }

  match distances.get(&current_nodes[0]) {
    Some(d) => return steps + d.0,
    None => panic!("Could not find distance {}", current_nodes[0])
  }
}

fn prob_b_2(directions: &Vec<Direction>, nodes: &HashMap<String, Node>) -> usize {
  let start = Instant::now();
  let mut steps = 0;
  let mut current_nodes: Vec<String> = find_start_nodes(&nodes);
  current_nodes.par_iter()
    .map(|n| calc_cycle(n, nodes, directions))
    .reduce(|| 1, |a, b| lcm(a, b))
}

fn main() {
  let start = Instant::now();
  let args: Vec<String> = std::env::args().collect();
  let binding = fs::read_to_string(&args[1]).unwrap();
  let file: &str = binding.as_str();

  let (directions, nodes) = parse_input(file);
  println!("Time elapsed parsing input is: {:?}", start.elapsed());
  let result = prob_b_2(&directions, &nodes);

  println!("Result: {}", result);
  println!("Time elapsed {:?}", start.elapsed());
}