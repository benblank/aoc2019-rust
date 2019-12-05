use std::io::{self, BufRead};

fn calculate_fuel_cost(weight: i32) -> i32 {
  let fuel_cost = weight / 3 - 2;

  if fuel_cost > 0 {
    fuel_cost
  } else {
    0
  }
}

fn recursively_calculate_fuel_cost(weight: i32) -> i32 {
  let fuel_cost = calculate_fuel_cost(weight);

  if fuel_cost > 0 {
    fuel_cost + recursively_calculate_fuel_cost(fuel_cost)
  } else {
    fuel_cost
  }
}

pub fn part1() {
  let stdin = io::stdin();
  let handle = stdin.lock();

  println!("Total fuel requirements: {}", handle.lines().map(|line| {
    calculate_fuel_cost(line.unwrap().parse::<i32>().unwrap())
  }).sum::<i32>());
}

pub fn part2() {
  let stdin = io::stdin();
  let handle = stdin.lock();

  println!("Total fuel requirements: {}", handle.lines().map(|line| {
    recursively_calculate_fuel_cost(line.unwrap().parse::<i32>().unwrap())
  }).sum::<i32>());
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn calculate_fuel_cost_works() {
    assert_eq!(2, calculate_fuel_cost(12));
    assert_eq!(2, calculate_fuel_cost(14));
    assert_eq!(654, calculate_fuel_cost(1969));
    assert_eq!(33583, calculate_fuel_cost(100_756));
  }

  #[test]
  fn recursively_calculate_fuel_cost_works() {
    assert_eq!(2, recursively_calculate_fuel_cost(14));
    assert_eq!(966, recursively_calculate_fuel_cost(1969));
    assert_eq!(50346, recursively_calculate_fuel_cost(100_756));
  }
}
