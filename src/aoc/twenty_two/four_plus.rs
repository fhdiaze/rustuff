use std::{
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
            count += pair.fully_contained() as i32;
        }
    }

    println!("The number of fully contained assignments is: {}", count);
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

    fn fully_contained(&self) -> bool {
        let diff = self.first.diff(&self.second);
        let sum = diff.start - diff.end;

        diff.start.abs() + diff.end.abs() == sum.abs()
    }

    /// Are the assignments verlapped
    fn overlapped() -> bool {

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
}

fn read_input() -> Lines<BufReader<File>> {
    let file = File::open("./four.txt").unwrap();
    let buf = BufReader::new(file);

    buf.lines()
}
