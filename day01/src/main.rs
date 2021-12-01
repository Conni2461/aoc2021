fn read_file(file: &str) -> Vec<i32> {
  std::fs::read_to_string(file)
    .unwrap()
    .lines()
    .map(|s| {
      s.to_string()
        .parse::<i32>()
        .expect("All lines should be numbers")
    })
    .collect()
}

fn ex1(input: &Vec<i32>) {
  let mut count = 0;
  let mut prev = -1;
  for n in input {
    if prev == -1 {
      prev = *n;
      continue;
    }
    if prev < *n {
      count += 1;
    }
    prev = *n;
  }
  println!("count: {}", count);
}

fn ex2(input: &Vec<i32>) {
  let mut count = 0;
  let mut prev = -1;
  for i in 0..input.len() - 2 {
    let sum = input[i] + input[i + 1] + input[i + 2];
    if prev == -1 {
      prev = sum;
      continue;
    }
    if prev < sum {
      count += 1;
    }
    prev = sum;
  }
  println!("count: {}", count);
}

fn main() {
  let input = read_file("data/input");
  ex1(&input);
  ex2(&input);
}
