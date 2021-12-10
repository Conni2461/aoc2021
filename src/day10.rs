lazy_static::lazy_static! {
  static ref OPENING_BRACKETS_MAP: std::collections::HashMap<char, char> = {
    let mut m = std::collections::HashMap::new();
    m.insert('[', ']');
    m.insert('{', '}');
    m.insert('(', ')');
    m.insert('<', '>');
    m
  };
  static ref CLOSING_BRACKETS_MAP: std::collections::HashMap<char, char> = {
    let mut m = std::collections::HashMap::new();
    m.insert(']', '[');
    m.insert('}', '{');
    m.insert(')', '(');
    m.insert('>', '<');
    m
  };
}

fn read_file(file: &str) -> Vec<String> {
  std::fs::read_to_string(file)
    .unwrap()
    .lines()
    .map(|s| s.to_string())
    .collect()
}

fn find_broken_syntax(line: &str) -> Option<char> {
  let mut stack: Vec<char> = Vec::with_capacity(line.len());
  for c in line.chars() {
    if !CLOSING_BRACKETS_MAP.contains_key(&c) {
      stack.push(c);
      continue;
    }
    if stack.is_empty() {
      return Some(c);
    }
    let opening = CLOSING_BRACKETS_MAP.get(&c).unwrap();
    if stack.last().unwrap() == opening {
      stack.pop();
    } else {
      return Some(c);
    }
  }
  None
}

fn find_missing_syntax(line: &str) -> Option<String> {
  let mut stack: Vec<char> = Vec::with_capacity(line.len());
  for c in line.chars() {
    if !CLOSING_BRACKETS_MAP.contains_key(&c) {
      stack.push(c);
      continue;
    }
    if stack.is_empty() {
      return None;
    }
    let opening = CLOSING_BRACKETS_MAP.get(&c).unwrap();
    if stack.last().unwrap() == opening {
      stack.pop();
    } else {
      return None;
    }
  }
  let mut res = String::new();
  while !stack.is_empty() {
    res.push(*OPENING_BRACKETS_MAP.get(&stack.pop().unwrap()).unwrap());
  }
  Some(res)
}

fn ex1(input: &[String]) {
  let mut res = 0;
  for line in input {
    if let Some(s) = find_broken_syntax(line) {
      res += match s {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("how did we get here"),
      }
    }
  }
  println!("{}", res);
}

fn ex2(input: &[String]) {
  let mut points = Vec::new();
  for line in input {
    if let Some(s) = find_missing_syntax(line) {
      let mut res: u64 = 0;
      for c in s.chars() {
        res *= 5;
        res += match c {
          ')' => 1,
          ']' => 2,
          '}' => 3,
          '>' => 4,
          _ => panic!("how did we get here"),
        };
      }
      points.push(res);
    }
  }
  points.sort_unstable();
  println!("{}", points[(points.len() - 1) / 2]);
}

pub fn run() {
  let input = read_file("data/input10");
  ex1(&input);
  ex2(&input);
}
