use std::{fs::File, io::{BufRead, BufReader, Lines}};

fn main() {
  solve();
}

fn solve() {
  let lines = read_lines();
  let mut sum = 0;
  let mut group_set = set::FULL;

  for (index, l) in lines.enumerate() {
    if let Ok(t) = l {
      let current_set = set::from_items(t);
      group_set = set::intersect(&group_set, &current_set);

      if index % 3 == 2 {
        sum += set::sum_priorities(group_set);
        group_set = set::FULL;
      }
    }
  }

  println!("Sum of priorities: {}", sum);
}

mod item {
  pub fn priority(item: char) -> usize {
    let code = item as usize;

    if item.is_uppercase() {
      return code - 38;
    }

    code - 96
  }
}

mod set {
  use super::item;

  pub type Set = [bool; 52];
  pub const FULL: Set = [true; 52];

  pub fn intersect(one: &Set, other: &Set) -> Set {
    let mut intersection: Set = [false; 52];

    for i in 0..52 {
      intersection[i] = one[i] && other[i];
    }

    intersection
  }

  pub fn sum_priorities(set: Set) -> usize {
    let mut sum = 0;

    for (index, is_contained) in set.iter().enumerate() {
      sum += *is_contained as usize * ( index + 1);
    }

    sum
  }

  pub fn from_items(items: String) -> Set {
    let mut set = [false; 52];

    for i in items.chars() {
      let priority = item::priority(i);
      set[priority-1] = true;
    }

    set
  }
}

fn read_lines() -> Lines<BufReader<File>> {
  let file = File::open("three_plus.txt").unwrap();
  let buf = BufReader::new(file);

  buf.lines()
}

