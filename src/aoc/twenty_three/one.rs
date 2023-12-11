use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub fn solve() {
    let lines = read_input();
    let mut sum = 0;

    for l in lines.flatten() {
        let mut digits = l.chars().filter(|c| c.is_ascii_digit());
        let first = digits.next().unwrap();
        let last = digits.last().unwrap_or(first);
        let value: usize = format!("{}{}", first, last).parse().unwrap();

        sum += value;
    }

    println!("Sum of all of the calibration values: {}", sum);
}

fn read_input() -> Lines<BufReader<File>> {
    let file = File::open("src/aoc/twenty_three/one.txt").unwrap();
    let buffer = BufReader::new(file);

    buffer.lines()
}
