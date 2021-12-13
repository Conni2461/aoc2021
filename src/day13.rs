#[derive(Debug)]
enum Dir {
  Y(i32),
  X(i32),
}

#[derive(Debug)]
struct Input {
  ins: Vec<(i32, i32)>,
  folds: Vec<Dir>,
  dimensions: (i32, i32),
}

fn read_file(file: &str) -> Input {
  let mut folds = Vec::new();
  let mut ins = Vec::new();
  let mut dimensions = (0, 0);
  for s in std::fs::read_to_string(file).unwrap().lines().into_iter() {
    if s.starts_with("f") {
      let (d, s) = s.split_once('=').unwrap();
      if d == "fold along x" {
        folds.push(Dir::X(s.parse().unwrap()));
      } else {
        folds.push(Dir::Y(s.parse().unwrap()));
      }
    } else if !s.is_empty() {
      let (y, x) = s.split_once(',').unwrap();
      let yy = y.parse::<i32>().unwrap();
      let xx = x.parse::<i32>().unwrap();
      if yy > dimensions.0 {
        dimensions.0 = yy;
      }
      if xx > dimensions.1 {
        dimensions.1 = xx;
      }
      ins.push((yy, xx));
    }
  }
  Input {
    ins,
    folds,
    dimensions,
  }
}

fn ex1(input: &Input) {
  let mut field = vec![
    vec![0; (input.dimensions.0 + 1) as usize];
    (input.dimensions.1 + 1) as usize
  ];
  for i in input.ins.iter() {
    field[i.1 as usize][i.0 as usize] = 1;
  }

  for fold in input.folds.iter() {
    match fold {
      Dir::Y(x) => {
        let mut o = 0;
        for i in ((*x as usize)..field.len()).rev() {
          for n in 0..field[i].len() {
            if field[i][n] == 1 {
              field[o][n] = 1;
            }
          }
          o += 1;
        }
        for i in ((*x as usize)..field.len()).rev() {
          field.remove(i);
        }
      }
      Dir::X(x) => {
        for row in field.iter_mut() {
          let mut o = 0;
          for i in ((*x as usize)..row.len()).rev() {
            if row[i] == 1 {
              row[o] = 1;
            }
            row.remove(i);
            o += 1;
          }
        }
      }
    }
  }

  let mut count = 0;
  for y in field.iter() {
    for x in y.iter() {
      if *x == 1 {
        count += 1;
      }
    }
  }
  println!("{:?}", field);
  println!("{}", count);
}

pub fn run() {
  let input = read_file("data/input13");
  ex1(&input);
}
