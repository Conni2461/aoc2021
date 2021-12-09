fn read_file(file: &str) -> Vec<Vec<i32>> {
  std::fs::read_to_string(file)
    .unwrap()
    .lines()
    .map(|s| {
      s.chars()
        .map(|n| n.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>()
    })
    .collect()
}

fn ex1(input: &[Vec<i32>]) {
  let res = input
    .iter()
    .enumerate()
    .flat_map(|(y, line)| {
      line.iter().enumerate().filter_map(move |(x, &height)| {
        if (y == 0 || height < input[y - 1][x])
          && (y == input.len() - 1 || height < input[y + 1][x])
          && (x == 0 || height < input[y][x - 1])
          && (x == line.len() - 1 || height < input[y][x + 1])
        {
          Some(height + 1)
        } else {
          None
        }
      })
    })
    .sum::<i32>();
  println!("{}", res);
}

fn ex2(input: &[Vec<i32>]) {
  let mut used: Vec<Vec<bool>> =
    input.iter().map(|line| vec![false; line.len()]).collect();
  let mut basins = Vec::new();
  let mut queue = Vec::new();

  for (y, line) in input.iter().enumerate() {
    for (x, &height) in line.iter().enumerate() {
      if height == 9 || used[y][x] {
        continue;
      }

      queue.push((y, x));
      let mut size = 0;

      while let Some((y, x)) = queue.pop() {
        if used[y as usize][x as usize] {
          continue;
        }

        size += 1;
        used[y][x] = true;

        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
          let y = y as isize + dy;
          let x = x as isize + dx;

          if y >= 0
            && x >= 0
            && (y as usize) < input.len()
            && (x as usize) < line.len()
            && input[y as usize][x as usize] != 9
          {
            queue.push((y as usize, x as usize));
          }
        }
      }

      basins.push(size);
    }
  }

  basins.sort();
  println!("{}", basins.iter().rev().take(3).product::<usize>());
}

pub fn run() {
  let input = read_file("data/input09");
  ex1(&input);
  ex2(&input);
}
