use std::collections::HashMap;

fn read_file(file: &str) -> Vec<String> {
  std::fs::read_to_string(file)
    .unwrap()
    .lines()
    .map(|s| s.to_string())
    .collect()
}

fn step(
  s: &HashMap<(char, char), usize>,
  rules: &HashMap<(char, char), char>,
) -> HashMap<(char, char), usize> {
  let mut ret = HashMap::default();
  for (rule, ins) in rules {
    if let Some(count) = s.get(rule) {
      *ret.entry((rule.0, *ins)).or_default() += count;
      *ret.entry((*ins, rule.1)).or_default() += count;
    }
  }
  ret
}

fn answer(
  s: &HashMap<(char, char), usize>,
  (first, last): (char, char),
) -> usize {
  let mut hist = make_hist(s);
  *hist.entry(first).or_default() += 1;
  *hist.entry(last).or_default() += 1;
  // Each char is counted twice - two different pairs cantain it
  (hist.values().max().unwrap() - hist.values().min().unwrap()) / 2
}

fn make_hist(s: &HashMap<(char, char), usize>) -> HashMap<char, usize> {
  let mut hist: HashMap<char, usize> = HashMap::default();
  for ((a, b), count) in s {
    *hist.entry(*a).or_default() += count;
    *hist.entry(*b).or_default() += count;
  }
  hist
}

pub fn run() {
  let input = read_file("data/input14");
  let rules = input[2..]
    .iter()
    .map(|s| {
      let mut s = s.split(" -> ");
      let from: Vec<_> = s.next().unwrap().chars().take(2).collect();
      let to = s.next().unwrap();
      ((from[0], from[1]), to.chars().next().unwrap())
    })
    .collect();

  let chars: Vec<_> = input[0].chars().collect();
  let mut template = HashMap::default();
  for w in chars.windows(2) {
    *template.entry((w[0], w[1])).or_default() += 1;
  }
  let edges = (*chars.first().unwrap(), *chars.last().unwrap());

  let mut part1 = 0;
  for s in 1..=40 {
    template = step(&template, &rules);
    if s == 10 {
      part1 = answer(&template, edges);
    }
  }
  let part2 = answer(&template, edges);

  println!("{}", part1);
  println!("{}", part2);
}
