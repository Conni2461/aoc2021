#[derive(Debug, Clone)]
struct Vec2 {
  x: i32,
  y: i32,
}

#[derive(Debug, Clone)]
struct Line {
  lhs: Vec2,
  rhs: Vec2,
}

fn split_vec2(s: String) -> Vec2 {
  let r = s
    .split(',')
    .map(|c| c.to_string().parse::<i32>().unwrap())
    .collect::<Vec<i32>>();
  Vec2 { x: r[0], y: r[1] }
}

fn read_file(file: &str) -> Vec<Line> {
  // -> Bingo {
  std::fs::read_to_string(file)
    .unwrap()
    .lines()
    .map(|s| {
      let split = s
        .to_string()
        .split(" -> ")
        .map(|c| c.to_string())
        .collect::<Vec<String>>();
      let lhs = split_vec2(split[0].clone());
      let rhs = split_vec2(split[1].clone());
      Line { lhs, rhs }
    })
    .collect::<Vec<Line>>()
}

fn ex1(input: &[Line]) {
  let mut width = 0;
  let mut height = 0;
  for line in input.iter() {
    if line.lhs.x > width {
      width = line.lhs.x;
    }
    if line.rhs.x > width {
      width = line.rhs.x;
    }

    if line.lhs.y > height {
      height = line.lhs.y;
    }
    if line.rhs.y > height {
      height = line.rhs.y;
    }
  }

  let mut map: Vec<Vec<i32>> = Vec::with_capacity(height as usize);
  for _ in 0..height + 1 {
    map.push(vec![0; (width + 1) as usize]);
  }

  for line in input.iter() {
    if line.lhs.x == line.rhs.x {
      let x = line.lhs.x as usize;
      let (lower, upper) = if line.lhs.y < line.rhs.y {
        (line.lhs.y as usize, (line.rhs.y + 1) as usize)
      } else {
        (line.rhs.y as usize, (line.lhs.y + 1) as usize)
      };

      for y in lower..upper {
        map[x][y] += 1;
      }
      continue;
    } else if line.lhs.y == line.rhs.y {
      let y = line.lhs.y as usize;
      let (lower, upper) = if line.lhs.x < line.rhs.x {
        (line.lhs.x as usize, (line.rhs.x + 1) as usize)
      } else {
        (line.rhs.x as usize, (line.lhs.x + 1) as usize)
      };

      for line in map.iter_mut().take(upper).skip(lower) {
        line[y] += 1;
      }
      continue;
    } else { // ex1 is without this else
      let (dir_x, mut distance) = if line.lhs.x < line.rhs.x {
        (1, line.rhs.x - line.lhs.x)
      } else {
        (-1, line.lhs.x - line.rhs.x)
      };

      let dir_y = if line.lhs.y < line.rhs.y { 1 } else { -1 };

      let mut x = line.lhs.x as usize;
      let mut y = line.lhs.y as usize;
      while distance >= 0 {
        map[x][y] += 1;
        x = ((x as i32) + dir_x) as usize;
        y = ((y as i32) + dir_y) as usize;
        distance -= 1;
      }
    }
  }

  let mut count = 0;
  for row in map {
    for x in row {
      if x >= 2 {
        count += 1;
      }
    }
  }
  println!("{}", count);
}

fn main() {
  let input = read_file("data/input");
  ex1(&input);
}
