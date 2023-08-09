use std::{
    cmp::{max, min},
    fs::File,
    io::{BufRead, BufReader, Lines},
};

fn main() {
    solve();
}

fn solve() {
    let input = read_input();
    let mut count = 0;

    for line in input {
        if let Ok(text) = line {
            let pair = Pair::parse(&text);
            count += pair.overlapped() as i32;
        }
    }

    println!("The number of overlapped assignments is: {}", count);
}

struct Pair {
    first: Range,
    second: Range,
}

impl Pair {
    fn parse(text: &str) -> Self {
        let assignments = text.split(",").collect::<Vec<&str>>();
        let first = Range::parse(assignments[0]);
        let second = Range::parse(assignments[1]);

        Pair { first, second }
    }

    /// Are the assignments overlapped
    fn overlapped(&self) -> bool {
        let union = Range {
            start: min(self.first.start, self.second.start),
            end: max(self.first.end, self.second.end),
        };
        let total = union.size();
        let sum = self.first.size() + self.second.size();

        total <= sum
    }
}

struct Range {
    start: isize,
    end: isize,
}

impl Range {
    fn parse(text: &str) -> Self {
        let ids = text.split("-").collect::<Vec<&str>>();
        let start = ids[0].parse::<isize>().unwrap();
        let end = ids[1].parse::<isize>().unwrap();

        Range { start, end }
    }

    fn diff(&self, other: &Range) -> Range {
        let start_diff = self.start - other.start;
        let end_diff = self.end - other.end;

        Range {
            start: start_diff,
            end: end_diff,
        }
    }

    fn size(&self) -> isize {
      self.end - self.start
    }
}

fn read_input() -> Lines<BufReader<File>> {
    let file = File::open("./four.txt").unwrap();
    let buf = BufReader::new(file);

    buf.lines()
}
