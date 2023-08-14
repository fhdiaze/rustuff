use std::fs::File;
use std::io::{BufReader, BufRead, Lines};

fn main() {
  solve();
}

fn solve() {
  let input = read_input();

  for l in input {
    if let Ok(text) = l {
      println!("{}", text);
    }
  }
}

mod parser {
  pub fn parse() {

  }

  pub fn parse_line(text: &str) {
    let re = Regex::new(r" *\[.\] *");
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
  let file = File::open("five.txt").unwrap();
  let buf = BufReader::new(file);

  buf.lines()
}
