use std::{
    fs::File,
    io::BufRead,
    io::{BufReader, Error, Lines},
    iter::FlatMap,
    str::Chars,
};

pub fn main() {
    solve();
}

fn solve() {
    let input = read_input();
    let mut i = 3;

    while i < input.len() {
        match input[i] {
            x if x == input[i - 1] => i += 3,
            y if y == input[i - 2] => i += 2,
            z if z == input[i - 3] => i += 1,
            _ => {
                break;
            }
        }
    }

    println!("Marker found at: {}", i);
}

fn read_input() -> Vec<char> {
    let file = File::open("./src/aoc/twenty_two/six.txt").unwrap();
    let buf = BufReader::new(file);

    buf.lines()
        .flat_map(|l| l.expect("Lines failed").chars().collect::<Vec<char>>())
        .collect::<Vec<char>>()
}
