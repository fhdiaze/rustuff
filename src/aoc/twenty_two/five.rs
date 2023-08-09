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

fn read_input() -> Lines<BufReader<File>> {
  let file = File::open("five.txt").unwrap();
  let buf = BufReader::new(file);

  buf.lines()
}
