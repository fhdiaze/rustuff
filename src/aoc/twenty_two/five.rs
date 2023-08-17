use std::fs::File;
use std::io::{BufReader, BufRead, Lines};
use relative_path::RelativePath;

pub fn main() {
  solve();
}

fn solve() {
  let input = read_input();

  for l in input {
    if let Ok(text) = l {
      parser::parse_line(&text);
    }
  }
}


mod parser {
  use regex::Regex;

  pub fn parse() {

  }

  enum Token {
    Mark(char),
    Blank,
    Splitter
  }

  pub fn parse_line(text: &str) {
    println!("Line={}", text);
    let re = Regex::new(r"(?<b> {3})|\[(?<m>.)\]|(?<s> )").unwrap();
    for m in re.find_iter(text) {

      println!("m={:?}", m.as_str());
    }
  }
}

fn process_line() {

}

struct Ship {
  stacks: Vec<Stack>
}

impl Ship {
  fn new() -> Self {
    Ship {
      stacks: Vec::with_capacity(10)
    }
  }
}

struct Stack {
  crates: Vec<char>
}

struct Step {
  quantity: usize,
  from: usize,
  to: usize
}

fn read_input() -> Lines<BufReader<File>> {
  let relative = RelativePath::new("./src/aoc/twenty_two/five.txt");
  let file = File::open(relative.to_path("./")).unwrap();
  let buf = BufReader::new(file);

  buf.lines()
}
