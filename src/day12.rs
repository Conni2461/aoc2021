use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Graph {
  adj_list: HashMap<String, Vec<String>>,
}

impl Graph {
  fn new() -> Self {
    Self {
      adj_list: HashMap::new(),
    }
  }

  fn add_edge(&mut self, src: &str, dst: &str) {
    self
      .adj_list
      .entry(src.to_string())
      .or_insert_with(Vec::new)
      .push(dst.to_string());
  }

  fn add(&mut self, b: &str, a: &str) {
    self.add_edge(a, b);
    self.add_edge(b, a);
  }

  fn adj(&self, node: &str) -> impl Iterator<Item = &String> {
    self.adj_list[node].iter()
  }
}

fn read_file(file: &str) -> Graph {
  let mut graph = Graph::new();
  for line in std::fs::read_to_string(file).unwrap().lines().into_iter() {
    let (src, dest) = line.split_once('-').unwrap();
    graph.add(src, dest);
  }
  graph
}

fn walk(graph: &Graph, node: String, mut seen: HashSet<String>) -> usize {
  if seen.contains(&node) {
    return 0;
  }
  if node == "end" {
    return 1;
  }
  if node == node.to_lowercase() {
    seen.insert(node.to_string());
  }

  graph
    .adj(&node)
    .map(|node| walk(graph, node.to_string(), seen.clone()))
    .sum()
}

fn walk2(
  graph: &Graph,
  node: String,
  mut seen: HashSet<String>,
  mut double: bool,
) -> usize {
  if seen.contains(&node) {
    if double {
      return 0;
    } else {
      double = true;
    }
  }
  if node == "end" {
    return 1;
  }
  if node == node.to_lowercase() {
    seen.insert(node.to_string());
  }

  graph
    .adj(&node)
    .filter(|&node| node != "start")
    .map(|node| walk2(graph, node.to_string(), seen.clone(), double))
    .sum()
}

fn ex1(graph: &Graph) -> usize {
  walk(graph, "start".to_string(), HashSet::new())
}

fn ex2(graph: &Graph) -> usize {
  walk2(graph, "start".to_string(), HashSet::new(), false)
}

pub fn run() {
  let input = read_file("data/input12");
  println!("{}", ex1(&input));
  println!("{}", ex2(&input));
}
