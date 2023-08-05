use std::{fs::File, io::{BufRead, BufReader, Lines}};

fn main() {
  solve();
}

fn solve() {
  let lines = read_lines();
  let mut sum = 0;

  for l in lines {
    if let Ok(t) = l {
      let rucksack = Rucksack::new(t);
      sum += rucksack.common();
    }
  }

  println!("Sum of priorities: {}", sum);
}

fn read_lines() -> Lines<BufReader<File>> {
  let file = File::open("three.txt").unwrap();
  let buf = BufReader::new(file);

  buf.lines()
}

struct Item { }

struct Rucksack {
  items: String
}

impl Rucksack {
  fn new(items: String) -> Self {
    Rucksack { items }
  }

  fn common(&self) -> usize {
    let mut chk = [0; 52];

    for (index, item) in self.items.chars().enumerate() {
      let priority = Item::priority(item);

      if index < self.items.len()/2 {
        chk[priority-1] = 1;
      } else if chk[priority-1] != 0 {
        return priority;
      }
    }

    0
  }
}

impl Item {
  fn priority(item: char) -> usize {
    let code = item as usize;

    if item.is_uppercase() {
      return code - 38;
    }

    code - 96
  }
}
