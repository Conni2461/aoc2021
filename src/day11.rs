fn read_file(file: &str) -> Vec<Vec<i32>> {
  std::fs::read_to_string(file)
    .unwrap()
    .lines()
    .map(|s| {
      s.to_string()
        .chars()
        .map(|n| n.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>()
    })
    .collect()
}

fn has_flash(input: &[Vec<i32>]) -> bool {
  for r in input.iter() {
    for o in r.iter() {
      if *o > 9 && *o != 100 {
        return true;
      }
    }
  }
  false
}

fn step(input: &mut [Vec<i32>]) -> i32 {
  for r in input.iter_mut() {
    for o in r.iter_mut() {
      *o += 1;
    }
  }

  let mut counts = 0;
  while has_flash(input) {
    for x in 0..input.len() {
      for y in 0..input[x].len() {
        if input[x][y] <= 9 || input[x][y] == 100 {
          continue;
        }
        // We flashed
        counts += 1;
        // So we dont flash again
        input[x][y] = 100;
        // flashing neighbors
        for dx in [-1, 0, 1] {
          for dy in [-1, 0, 1] {
            if dx == 0 && dy == 0 {
              continue;
            }
            let new_x = x as i32 - dx;
            let new_y = y as i32 - dy;
            if new_x < 0
              || new_y < 0
              || new_x as usize >= input.len()
              || new_y as usize >= input[y].len()
            {
              continue;
            }
            if input[new_x as usize][new_y as usize] != 100 {
              input[new_x as usize][new_y as usize] += 1;
            }
          }
        }
      }
    }
  }

  for r in input.iter_mut() {
    for o in r.iter_mut() {
      if *o == 100 {
        *o = 0;
      }
    }
  }
  counts
}

fn ex1(input: &mut [Vec<i32>]) {
  let mut counts = 0;
  for _ in 0..100 {
    counts += step(input);
  }
  println!("{}", counts);
}

fn ex2(input: &mut [Vec<i32>]) {
  let mut idx = 1;
  loop {
    let res = step(input);
    if res == 100 {
      break;
    }
    idx += 1;
  }
  println!("{}", idx);
}

pub fn run() {
  ex1(&mut read_file("data/input11"));
  ex2(&mut read_file("data/input11"));
}
