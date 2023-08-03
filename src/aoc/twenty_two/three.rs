use std::{fs::File, io::{BufRead, BufReader, Lines}};

fn main() {
  solve();
}

fn solve() {
  let lines = read_lines();

  for l in lines {
    if let Ok(t) = l {
      println!("{}", t);
    }
  }
}

fn read_lines() -> Lines<BufReader<File>> {
  let file = File::open("three.txt").unwrap();
  let buf = BufReader::new(file);

  buf.lines()
}
