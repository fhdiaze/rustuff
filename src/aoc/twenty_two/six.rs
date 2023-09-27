use std::{fs::File, io::BufReader};

fn main() {
  solve();
}

fn solve() {
  let input = read_input();
}

fn read_input() -> String {
  let file = File::open("input.txt").unwrap();
  let buf = BufReader::new(file);
  buf.;
}
