#[derive(Debug, Clone)]
struct Bingo {
  numbers: Vec<i32>,
  cards: Vec<Vec<Vec<(i32, bool)>>>,
}

fn read_file(file: &str) -> Bingo {
  // -> Bingo {
  let mut lines = std::fs::read_to_string(file)
    .unwrap()
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();
  let nums = lines
    .remove(0)
    .split(',')
    .map(|s| s.to_string().parse::<i32>().unwrap())
    .collect::<Vec<i32>>();
  lines.remove(0);

  let mut cards = Vec::new();
  let mut curr = Vec::<Vec<(i32, bool)>>::new();
  for line in lines.iter() {
    if line.is_empty() {
      cards.push(curr);
      curr = Vec::<Vec<(i32, bool)>>::new();
      continue;
    }
    curr.push(
      line
        .split_whitespace()
        .map(|s| (s.to_string().parse::<i32>().unwrap(), false))
        .collect::<Vec<(i32, bool)>>(),
    );
  }
  cards.push(curr);
  Bingo {
    numbers: nums,
    cards,
  }
}

fn run_card(card: &mut Vec<Vec<(i32, bool)>>, number: i32) {
  for rows in card.iter_mut() {
    for col in rows.iter_mut() {
      if col.0 == number {
        col.1 = true;
      }
    }
  }
}

fn sum_of_unmarked(card: &[Vec<(i32, bool)>]) -> i32 {
  let mut sum = 0;
  for row in card.iter() {
    for col in row.iter() {
      if !col.1 {
        sum += col.0;
      }
    }
  }
  sum
}

fn check_card(card: &[Vec<(i32, bool)>]) -> i32 {
  // check horizontal
  for row in card.iter() {
    let mut count = 0;
    for col in row.iter() {
      if col.1 {
        count += 1;
      }
    }
    if count == 5 {
      return sum_of_unmarked(card);
    }
  }

  // check vertical
  for i in 0..5 {
    let mut count = 0;
    for col in card.iter().take(5) {
      if col[i].1 {
        count += 1;
      }
    }
    if count == 5 {
      return sum_of_unmarked(card);
    }
  }
  0
}

fn ex1(input: &Bingo) {
  let mut copy = input.clone();
  for n in copy.numbers.iter() {
    for card in copy.cards.iter_mut() {
      run_card(card, *n);
      let score = check_card(card);
      if score != 0 {
        println!("{}", score * *n);
        return;
      }
    }
  }
}

fn ex2(input: &Bingo) {
  let mut copy = input.clone();
  let mut res = 0;
  for n in copy.numbers.iter() {
    let mut rem_indices = Vec::new();
    for (idx, card) in copy.cards.iter_mut().enumerate() {
      run_card(card, *n);
      let score = check_card(card);
      if score != 0 {
        res = score * *n;
        rem_indices.push(idx);
        continue;
      }
    }
    for idx in rem_indices.iter().rev() {
      copy.cards.remove(*idx);
    }
  }
  println!("{}", res);
}

fn main() {
  let input = read_file("data/input");
  ex1(&input);
  ex2(&input);
}
