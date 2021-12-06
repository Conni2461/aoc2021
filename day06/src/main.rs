// BAD BRUTE FORCE
fn read_file(file: &str) -> Vec<i32> {
  std::fs::read_to_string(file)
    .unwrap()
    .trim_end()
    .split(',')
    .map(|s| {
      s.to_string()
        .parse::<i32>()
        .expect("All lines should be numbers")
    })
    .collect()
}

fn ex1() {
  let mut input = read_file("data/input");
  for _ in 0..80 {
    let len = input.len();
    for i in 0..len {
      if input[i] == 0 {
        input.push(8);
        input[i] = 7;
      }
      input[i] -= 1;
    }
  }
  println!("res: {}", input.len());
}

// SOMEWHAT SMART VERSION
type Fish = [u128; 9];
fn parse(fname: &str) -> Fish {
  std::fs::read_to_string(fname)
    .unwrap()
    .split(',')
    .into_iter()
    .map(|s| s.trim().parse::<usize>().unwrap())
    .fold([0u128; 9], |mut acc, val| {
      acc[val] += 1;
      acc
    })
}

fn step_day(f: Fish) -> Fish {
  [f[1], f[2], f[3], f[4], f[5], f[6], f[7] + f[0], f[8], f[0]]
}

fn step_days(days: usize, fish: Fish) -> Fish {
  (0..days).fold(fish, |acc, _| step_day(acc))
}

fn ex2(fish: Fish) {
  println!("res: {}", step_days(256, fish).iter().sum::<u128>());
}

fn main() {
  ex1();
  ex2(parse("data/input"));
}
