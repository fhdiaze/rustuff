use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    solve();
}

#[derive(PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

struct Round {
    opponent: Shape,
    you: Shape,
}

fn solve() {
    let lines = read_file();

    for l in lines {
        if let Ok(t) = l {
            let r = parse_line(t);
        }
    }
}

fn parse_line(line: String) -> Round {
    let chars: Vec<&str> = line.split(' ').collect();
    Round {
        opponent: char_to_shape(chars[0]),
        you: char_to_shape(chars[1]),
    }
}

fn char_to_shape(c: &str) -> Shape {
  match c {
    "A" => Shape::Rock,
    "B" => Shape::Paper,
    "C" => Shape::Scissors,
    "X" => Shape::Rock,
    "Y" => Shape::Paper,
    _ => Shape::Scissors,
  }
}

fn your_score(round: Round) -> {
  let round.you.defeated_by()
}

fn read_file() -> io::Lines<io::BufReader<File>> {
    let file = File::open("./day_two_input.txt").unwrap();
    let buf = io::BufReader::new(file);

    buf.lines()
}

impl Shape {
  fn defeated_by(&self) -> Shape {
    match self {
      Shape::Rock => Shape::Paper,
      Shape::Paper => Shape::Scissors,
      Shape::Scissors => Shape::Rock
    }
  }
}
