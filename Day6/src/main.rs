use std::fs;
use regex::Regex;

struct Race {
  time: usize,
  record: usize
}

fn parse_input(file: &str) -> Vec<Race> {
  let re = Regex::new(r"Time:\s+(?<time>[\d\s]+)\nDistance:\s+(?<distance>[\d\s]+)").unwrap();
  match re.captures(file) {
      Some(captures) => {
        let times: Vec<usize> = (&captures["time"])
          .split(" ")
          .filter(|s| !s.is_empty())
          .map(|n| n.parse::<usize>().unwrap()).collect();
        let distances: Vec<usize> = (&captures["distance"])
          .split(" ")
          .filter(|s| !s.is_empty())
          .map(|n| n.parse::<usize>().unwrap()).collect();
        if times.len() != distances.len() {
          panic!("Inconsistent number of races!");
        }
        let mut races: Vec<Race> = Vec::new();
        let mut idx = 0;
        while idx < times.len() {
          races.push(Race {
            time: times[idx],
            record: distances[idx]
          });
          idx = idx + 1;
        }

        return races;
      },
      None => panic!("Could not parse races")
  }
}

fn find_min_max_hold_times(race: &Race) -> (usize, usize) {
  let a: f32 = (race.time.pow(2) - 4 * race.record) as f32;
  if a < 0.0 {
    panic!("No solution for race {} {}", race.time, race.record);
  }
  let root: f32 = a.sqrt();
  let first_solution = ((race.time as f32 * -1.0) + root) / (-2.0);
  let second_solution = ((race.time as f32 * -1.0) - root) / (-2.0);
  let first_solution_rounded = if first_solution == first_solution.ceil() { first_solution + 1.0 } else { first_solution.ceil() };
  let second_solution_rounded = if second_solution == second_solution.floor() { second_solution - 1.0 } else { second_solution.floor() };

  (first_solution_rounded as usize, second_solution_rounded as usize)
}

fn prob_a(races: &Vec<Race>) -> usize {
  let mut total: usize = 1;
  for race in races {
    let hold_times = find_min_max_hold_times(race);
    println!("Hold times {:?}", hold_times);
    total = total * (hold_times.1 - hold_times.0 + 1);
    println!("Total {}", total);
  }

  total
}

fn prob_b(races: &Vec<Race>) -> usize {
  let mut time_str: String = "".to_owned();
  let mut distance_str: String = "".to_owned();
  for race in races {
    time_str.push_str(race.time.to_string().as_str());
    distance_str.push_str(race.record.to_string().as_str());
  }
  let actual_race = Race {
    time: time_str.parse::<usize>().unwrap(),
    record: distance_str.parse::<usize>().unwrap()
  };
  println!("Actual race: {} {}", actual_race.time, actual_race.record);
  let hold_times = find_min_max_hold_times(&actual_race);
  println!("Hold times {:?}", hold_times);
  hold_times.1 - hold_times.0 - 1
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let binding = fs::read_to_string(&args[1]).unwrap();
  let file: &str = binding.as_str();

  let races: Vec<Race> = parse_input(file);
  let result = prob_b(&races);

  println!("Result: {}", result);
}