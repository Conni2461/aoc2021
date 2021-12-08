use itertools::Itertools;

fn read_file(file: &str) -> Vec<(Vec<String>, Vec<String>)> {
  std::fs::read_to_string(file)
    .unwrap()
    .lines()
    .map(|s| {
      let mut split = s.split(" | ");
      (
        split
          .next()
          .unwrap()
          .split(' ')
          .map(|t| t.to_string())
          .collect(),
        split
          .next()
          .unwrap()
          .split(' ')
          .map(|t| t.to_string())
          .collect(),
      )
    })
    .collect()
}

fn ex1(input: &[(Vec<String>, Vec<String>)]) {
  // 1 -> 2
  // 4 -> 4
  // 7 -> 3
  // 8 -> 7
  let mut res = 0;
  for line in input {
    for word in line.1.iter() {
      let len = word.len();
      if len == 2 || len == 3 || len == 4 || len == 7 {
        res += 1;
      }
    }
  }
  println!("{}", res);
}

fn is_good_permutation(lhs: &[String], abcdefg: &[char]) -> bool {
  let a = abcdefg[0];
  let b = abcdefg[1];
  let c = abcdefg[2];
  let d = abcdefg[3];
  let e = abcdefg[4];
  let f = abcdefg[5];
  let g = abcdefg[6];
  for l in lhs {
    match l.len() {
      2 => {
        if !(l.contains(c) && l.contains(f)) {
          return false;
        }
      }
      3 => {
        if !(l.contains(a) && l.contains(c) && l.contains(f)) {
          return false;
        }
      }
      4 => {
        if !(l.contains(b) && l.contains(c) && l.contains(d) && l.contains(f)) {
          return false;
        }
      }
      5 => {
        if !(l.contains(a)
          && l.contains(c)
          && l.contains(d)
          && l.contains(e)
          && l.contains(g))
          && !(l.contains(a)
            && l.contains(c)
            && l.contains(d)
            && l.contains(f)
            && l.contains(g))
          && !(l.contains(a)
            && l.contains(b)
            && l.contains(d)
            && l.contains(f)
            && l.contains(g))
        {
          return false;
        }
      }
      6 => {
        if !(l.contains(a)
          && l.contains(b)
          && l.contains(c)
          && l.contains(e)
          && l.contains(f)
          && l.contains(g))
          && !(l.contains(a)
            && l.contains(b)
            && l.contains(d)
            && l.contains(e)
            && l.contains(f)
            && l.contains(g))
          && !(l.contains(a)
            && l.contains(b)
            && l.contains(c)
            && l.contains(d)
            && l.contains(f)
            && l.contains(g))
        {
          return false;
        }
      }
      7 => (),
      _ => {
        panic!("Invalid length")
      }
    }
  }

  true
}

fn ex2(input: &[(Vec<String>, Vec<String>)]) {
  let mut res = 0;
  for line in input {
    let sigs = "abcdefg"
      .chars()
      .permutations(7)
      .find(|p| is_good_permutation(&line.0, p))
      .unwrap();
    let mut str = String::new();
    for word in line.1.iter() {
      match word.len() {
        2 => str.push('1'),
        3 => str.push('7'),
        4 => str.push('4'),
        5 => {
          let e = sigs[4];
          let c = sigs[2];
          if word.contains(e) {
            str.push('2');
          } else if word.contains(c) {
            str.push('3');
          } else {
            str.push('5');
          }
        }
        6 => {
          let d = sigs[3];
          let e = sigs[4];
          if word.contains(d) {
            if word.contains(e) {
              str.push('6');
            } else {
              str.push('9');
            }
          } else {
            str.push('0');
          }
        }
        7 => str.push('8'),
        _ => panic!("Invalid length word string"),
      }
    }
    res += str.parse::<i64>().unwrap();
  }
  println!("{}", res);
}

pub fn run() {
  let input = read_file("data/input08");
  ex1(&input);
  ex2(&input);
}
