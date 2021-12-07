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
  let mut prev = input[0];
  for n in input.iter().skip(1) {
    if prev < *n {
      count += 1;
    }
    prev = *n;
  }
  println!("count: {}", count);
}

fn ex2(input: &Vec<i32>) {
  let mut count = 0;
  let mut prev = input[0] + input[1] + input[2];
  for i in 1..input.len() - 2 {
    let sum = input[i] + input[i + 1] + input[i + 2];
    if prev < sum {
      count += 1;
    }
    prev = sum;
  }
  println!("count: {}", count);
}

pub fn run() {
  let input = read_file("data/input01");
  ex1(&input);
  ex2(&input);
}
