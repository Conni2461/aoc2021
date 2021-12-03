fn read_file(file: &str) -> Vec<String> {
  std::fs::read_to_string(file)
    .unwrap()
    .lines()
    .map(|s| s.to_string())
    .collect()
}

fn gamma_rate(input: &Vec<String>) -> String {
  let len = input[0].len();
  let rows = input.len() as i32;
  let mut counter: Vec<i32> = vec![0; len];
  for row in input {
    for (i, c) in row.chars().enumerate() {
      if c == '1' {
        counter[i] += 1;
      }
    }
  }
  let mut res = String::new();
  for count in counter {
    if count > (rows - count) {
      res.push('1');
    } else {
      res.push('0');
    }
  }
  res
}

fn invert_binary(bin: &str) -> String {
  let mut res = String::new();
  for c in bin.chars() {
    if c == '1' {
      res.push('0');
    } else {
      res.push('1');
    }
  }
  res
}

fn binary_to_decimal(bin: &str) -> i32 {
  i32::from_str_radix(&bin, 2).unwrap()
}

fn ex1(input: &Vec<String>) {
  let gamma = gamma_rate(input);
  let epsilon = invert_binary(&gamma);
  println!(
    "{}",
    binary_to_decimal(&gamma) * binary_to_decimal(&epsilon)
  );
}

fn gen_rating(
  input: &Vec<String>,
  f: impl Fn(Vec<usize>, Vec<usize>) -> Vec<usize>,
) -> String {
  let mut copy = input.clone();
  let mut column: usize = 0;
  while copy.len() != 1 {
    let mut zeros = Vec::<usize>::with_capacity(copy.len());
    let mut ones = Vec::<usize>::with_capacity(copy.len());
    for (i, row) in copy.iter().enumerate() {
      if row.chars().nth(column).unwrap() == '0' {
        zeros.push(i);
      } else {
        ones.push(i);
      }
    }
    let rem = f(zeros, ones);
    for idx in rem.iter().rev() {
      copy.remove(*idx);
    }

    column += 1;
  }

  copy[0].clone()
}

fn oxygen(input: &Vec<String>) -> String {
  gen_rating(input, |z, o| {
    if z.len() > o.len() {
      o
    } else if z.len() < o.len() {
      z
    } else {
      z
    }
  })
}

fn co2(input: &Vec<String>) -> String {
  gen_rating(input, |z, o| {
    if z.len() > o.len() {
      z
    } else if z.len() < o.len() {
      o
    } else {
      o
    }
  })
}

fn ex2(input: &Vec<String>) {
  let o = oxygen(input);
  let c = co2(input);
  println!("{}", binary_to_decimal(&o) * binary_to_decimal(&c));
}

fn main() {
  let input = read_file("data/input");
  ex1(&input);
  ex2(&input);
}
