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

fn ex(days: usize, fish: Fish) {
  println!("res: {}", step_days(days, fish).iter().sum::<u128>());
}

pub fn run() {
  ex(80, parse("data/input06"));
  ex(256, parse("data/input06"));
}
