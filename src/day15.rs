use std::collections::HashMap;

type Cell = (i64, Option<i64>);

fn read_file(file: &str) -> Vec<Vec<Cell>> {
  std::fs::read_to_string(file)
    .unwrap()
    .lines()
    .map(|s| {
      s.chars()
        .map(|n| (n.to_digit(10).unwrap() as i64, None))
        .collect::<Vec<Cell>>()
    })
    .collect()
}

fn ex1(input: &mut [Vec<Cell>]) {
  input[0][0].1 = Some(0);

  let mut candidates: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
  candidates.insert((0, 1), (0, 0));
  candidates.insert((1, 0), (0, 0));

  let mut visited = vec![vec![false; input[0].len()]; input.len()];
  visited[0][0] = true;

  while !visited[input.len() - 1][input[0].len() - 1] {
    let mut remove = None;
    let mut min = std::i64::MAX;
    for (&(y, x), &(from_y, from_x)) in candidates.iter() {
      if input[y][x].0 + input[from_y][from_x].1.unwrap() < min {
        min = input[y][x].0 + input[from_y][from_x].1.unwrap();
        remove = Some((y, x));
      }
    }
    let a = candidates.remove_entry(&remove.unwrap()).unwrap();
    input[a.0 .0][a.0 .1].1 =
      Some(input[a.0 .0][a.0 .1].0 + input[a.1 .0][a.1 .1].1.unwrap());
    visited[a.0 .0][a.0 .1] = true;
    if let Some(c) = a.0 .0.checked_sub(1) {
      if !visited[c][a.0 .1] && !candidates.contains_key(&(c, a.0 .1)) {
        candidates.insert((c, a.0 .1), (a.0 .0, a.0 .1));
      }
    }
    if let Some(c) = a.0 .1.checked_sub(1) {
      if !visited[a.0 .0][c] && !candidates.contains_key(&(a.0 .0, c)) {
        candidates.insert((a.0 .0, c), (a.0 .0, a.0 .1));
      }
    }
    if let Some(c) = visited.get(a.0 .0 + 1) {
      if !c[a.0 .1] && !candidates.contains_key(&(a.0 .0 + 1, a.0 .1)) {
        candidates.insert((a.0 .0 + 1, a.0 .1), (a.0 .0, a.0 .1));
      }
    }
    if let Some(&c) = visited[a.0 .0].get(a.0 .1 + 1) {
      if !c && !candidates.contains_key(&(a.0 .0, a.0 .1 + 1)) {
        candidates.insert((a.0 .0, a.0 .1 + 1), (a.0 .0, a.0 .1));
      }
    }
  }

  println!("{}", input[input.len() - 1][input[0].len() - 1].1.unwrap());
}

fn ex2(input: &[Vec<Cell>]) {
  let mut data = vec![vec![(0, None); input.len() * 5]; input.len() * 5];
  for (y, l) in data.iter_mut().enumerate() {
    for (x, c) in l.iter_mut().enumerate() {
      c.0 = (input[y % input.len()][x % input[0].len()].0
        + (((y / input.len()) + (x / input[0].len())) as i64)
        - 1)
        % 9
        + 1;
    }
  }
  ex1(&mut data);
}

pub fn run() {
  let input = read_file("data/input15");
  ex1(&mut input.clone());
  ex2(&input);
}
