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
      let tokens = parser::parse_line(&text);

      println!("{:?}", tokens);
    }
  }
}


mod parser {
  use regex::Regex;

  #[derive(Debug)]
  pub enum Token {
    Mark(char),
    Blank,
    Digit(char),
    Move {
      quantity: usize,
      from: usize,
      to: usize,
    }
  }

  pub fn parse_line(text: &str) -> Vec<Token> {

    match text.chars().next() {
      Some(' ') | Some('[') => parse_marks(text),
      Some('m') => vec![parse_move(text)],
      _ => vec![],
    }
  }

  pub fn parse_move(text: &str) -> Token {
    let regex = Regex::new(r"move (?<q>\d*) from (?<f>\d*) to (?<t>\d*)").unwrap();
    let caps = regex.captures(text).unwrap();
    let q = caps["q"].parse::<usize>().unwrap();
    let f = caps["f"].parse::<usize>().unwrap();
    let t = caps["t"].parse::<usize>().unwrap();

    Token::Move {
      quantity: q,
      from: f,
      to: t
    }
  }

  pub fn parse_marks(text: &str) -> Vec<Token> {
    let mut ti = 1;
    let mut tokens: Vec<Token> = Vec::with_capacity(10);

    for (i, c) in text.chars().enumerate() {
      if i == ti {
        let token = match c {
          _ if c.is_whitespace() => Token::Blank,
          m if c.is_alphabetic() => Token::Mark(m),
          _ => Token::Digit(c)
        };

        tokens.push(token);

        ti += 4;
      }
    }

    tokens
  }
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

  fn load(&mut self, stack: usize, mark: char) {
    self.stacks[stack].push(mark);
  }

  fn run(&mut self, step: Step) {
    let marks = self.stacks[step.from].pop(step.quantity);
    self.stacks[step.to].push_many(marks);
  }
}

struct Stack {
  marks: Vec<char>
}

impl Stack {
  fn push(&mut self, mark: char) {
    self.marks.push(mark);
  }

  fn push_many(&mut self, mut marks: Vec<char>) {
    self.marks.append(&mut marks);
  }

  fn pop(&mut self, quantity: usize) -> Vec<char> {
    let mut marks = Vec::with_capacity(quantity);

    for _ in 0 .. quantity {
      marks.push(self.marks.pop().unwrap());
    }

    marks
  }
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
