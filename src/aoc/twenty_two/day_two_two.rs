use std::{
    fs::File,
    io::{self, BufRead},
};


fn main() {
    solve();
}

#[derive(PartialEq, Eq, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
  Lost,
  Draw,
  Won
}

struct Round {
    opponent: Shape,
    outcome: Outcome,
}

fn solve() {
    let lines = read_file();
    let mut score = 0;

    for l in lines {
        if let Ok(t) = l {
            score += Round::from_string(t).your_score();
        }
    }

    println!("{}", score);
}


fn read_file() -> io::Lines<io::BufReader<File>> {
    let file = File::open("./day_two_two_input.txt").unwrap();
    let buf = io::BufReader::new(file);

    buf.lines()
}

impl Round {
  fn from_string(string: String) -> Self {
    let chars: Vec<&str> = string.split(' ').collect();

    Round {
      opponent: Shape::from_char(chars[0]),
      outcome: Outcome::from_char(chars[1])
    }
  }

  fn your_score(&self) -> i32 {
    let you = self.outcome.what_to_play(&self.opponent);

    you.to_int() + self.outcome.to_int()
  }
}

impl Shape {
  fn from_char(c: &str) -> Self {
    match c {
      "A" => Shape::Rock,
      "B" => Shape::Paper,
      _ => Shape::Scissors,
    }
  }

  fn defeated_by(&self) -> Self {
    match self {
      Shape::Rock => Shape::Paper,
      Shape::Paper => Shape::Scissors,
      Shape::Scissors => Shape::Rock
    }
  }

  fn beats_to(&self) -> Self {
    match self {
      Shape::Rock => Shape::Scissors,
      Shape::Paper => Shape::Rock,
      Shape::Scissors => Shape::Paper
    }
  }

  fn to_int(&self) -> i32 {
    match self {
      Self::Rock => 1,
      Self::Paper => 2,
      Self::Scissors => 3
    }
  }
}

impl Outcome {
  fn to_int(&self) -> i32 {
    match self {
      Self::Lost => 0,
      Self::Draw => 3,
      Self::Won => 6
    }
  }

  fn from_char(c: &str) -> Self {
    match c {
      "X" => Outcome::Lost,
      "Y" => Outcome::Draw,
      _ => Outcome::Won,
    }
  }

  fn what_to_play(&self, opponent: &Shape) -> Shape {
    match self {
      Outcome::Draw => opponent.clone(),
      Outcome::Lost => opponent.beats_to(),
      Outcome::Won => opponent.defeated_by()
    }
  }
}
