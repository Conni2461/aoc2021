#[derive(Debug)]
enum Dir {
  Forward,
  Down,
  Up,
}

fn read_file(file: &str) -> Vec<(Dir, i32)> {
  std::fs::read_to_string(file)
    .unwrap()
    .lines()
    .map(|s| {
      let mut i = s.split_whitespace();
      let direction = i.next().unwrap();
      let count = i.next().unwrap();
      let dir = if direction == "forward" {
        Dir::Forward
      } else if direction == "down" {
        Dir::Down
      } else if direction == "up" {
        Dir::Up
      } else {
        panic!("My input is wrong")
      };
      (
        dir,
        count
          .to_string()
          .parse::<i32>()
          .expect("All lines should be numbers"),
      )
    })
    .collect()
}

fn ex1(input: &Vec<(Dir, i32)>) {
  let mut horizontal = 0;
  let mut depth = 0;
  for e in input.iter() {
    match e.0 {
      Dir::Forward => horizontal += e.1,
      Dir::Down => depth += e.1,
      Dir::Up => depth -= e.1,
    }
  }
  println!("res: {}", horizontal * depth);
}

fn ex2(input: &Vec<(Dir, i32)>) {
  let mut horizontal = 0;
  let mut depth = 0;
  let mut aim = 0;
  for e in input.iter() {
    match e.0 {
      Dir::Forward => {
        horizontal += e.1;
        depth += aim * e.1;
      }
      Dir::Down => aim += e.1,
      Dir::Up => aim -= e.1,
    }
  }
  println!("res: {}", horizontal * depth);
}

pub fn run() {
  let input = read_file("data/input02");
  ex1(&input);
  ex2(&input);
}
