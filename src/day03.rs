fn read_file(file: &str) -> Vec<Vec<char>> {
  std::fs::read_to_string(file)
    .unwrap()
    .lines()
    .map(|s| s.to_string().chars().collect::<Vec<char>>())
    .collect()
}

fn gamma_rate(input: &Vec<Vec<char>>) -> String {
  let len = input[0].len();
  let rows = input.len() as i32;
  let mut counter: Vec<i32> = vec![0; len];
  for row in input {
    for (i, c) in row.iter().enumerate() {
      if *c == '1' {
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
  let mut res = String::with_capacity(bin.len());
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

fn gen_rating(
  input: &Vec<Vec<char>>,
  f: impl Fn(Vec<usize>, Vec<usize>) -> Vec<usize>,
) -> String {
  let mut copy = input.clone();
  let mut column: usize = 0;
  while copy.len() != 1 {
    let mut zeros = Vec::<usize>::with_capacity(copy.len());
    let mut ones = Vec::<usize>::with_capacity(copy.len());
    for (i, row) in copy.iter().enumerate() {
      if row[column] == '0' {
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

  copy[0].iter().collect()
}

fn oxygen(input: &Vec<Vec<char>>) -> String {
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

fn co2(input: &Vec<Vec<char>>) -> String {
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

fn ex1(input: &Vec<Vec<char>>) {
  let gamma = gamma_rate(input);
  let epsilon = invert_binary(&gamma);
  println!(
    "{}",
    binary_to_decimal(&gamma) * binary_to_decimal(&epsilon)
  );
}

fn ex2(input: &Vec<Vec<char>>) {
  let o = oxygen(input);
  let c = co2(input);
  println!("{}", binary_to_decimal(&o) * binary_to_decimal(&c));
}

pub fn run() {
  let input = read_file("data/input03");
  ex1(&input);
  ex2(&input);
}
