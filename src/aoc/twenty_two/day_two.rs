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

enum Outcome {
  Lost,
  Draw,
  Won
}

struct Round {
    opponent: Shape,
    you: Shape,
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
    let file = File::open("./day_two_input.txt").unwrap();
    let buf = io::BufReader::new(file);

    buf.lines()
}

impl Round {
  fn from_string(string: String) -> Self {
    let chars: Vec<&str> = string.split(' ').collect();

    Round {
      opponent: Shape::from_char(chars[0]),
      you: Shape::from_char(chars[1])
    }
  }

  fn your_score(&self) -> i32 {
     let outcome = self.you.against(&self.opponent);

     self.you.to_int() + outcome.to_int()
  }

}

impl Shape {
  fn from_char(c: &str) -> Self {
    match c {
      "A" => Shape::Rock,
      "B" => Shape::Paper,
      "C" => Shape::Scissors,
      "X" => Shape::Rock,
      "Y" => Shape::Paper,
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

  fn against(&self, other: &Shape) -> Outcome {
    if self == other {
      return Outcome::Draw
    } else if *other == self.defeated_by() {
      return Outcome::Lost
    }

    Outcome::Won
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
}
