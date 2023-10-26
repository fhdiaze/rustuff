use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub fn main() {
    solve();
}

fn solve() {
    let input = read_input();

    for l in input.flatten() {
        print!("{:?}", l);
    }
}

fn read_input() -> Lines<BufReader<File>> {
    let file = File::open("src/aoc/twenty_two/eight.txt").expect("Could not read file");
    let reader = BufReader::new(file);

    reader.lines()
}
