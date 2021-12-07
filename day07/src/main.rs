fn read_file(file: &str) -> Vec<i32> {
  std::fs::read_to_string(file)
    .unwrap()
    .trim()
    .split(',')
    .map(|s| {
      s.to_string()
        .parse::<i32>()
        .expect("All lines should be numbers")
    })
    .collect()
}

fn ex1(input: &[i32]) {
  let max_destination = *input.iter().max().unwrap();
  let mut min_fuel = i32::max_value();
  for d in 1..max_destination {
    let mut fuel = 0;
    for crab in input {
      fuel += (crab - &d).abs();
    }
    if fuel < min_fuel {
      min_fuel = fuel;
    }
  }
  println!("{}", min_fuel);
}

fn real_crab_engineering(n: i32) -> i32 {
  (n * (n + 1)) / 2
}

fn ex2(input: &[i32]) {
  let max_destination = *input.iter().max().unwrap();
  let mut min_fuel = i32::max_value();
  for d in 1..max_destination {
    let mut fuel = 0;
    for crab in input {
      fuel += real_crab_engineering((crab - &d).abs());
    }
    if fuel < min_fuel {
      min_fuel = fuel;
    }
  }
  println!("{}", min_fuel);
}

fn main() {
  let input = read_file("data/input");
  ex1(&input);
  ex2(&input);
}
