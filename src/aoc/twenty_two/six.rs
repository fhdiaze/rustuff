use std::{fs::File, io::BufRead, io::BufReader};

pub fn main() {
    solve();
}

fn solve() {
    let input = read_input();
    let mut start = 0;
    let mut i = 1;

    while i < input.len() && i - start <= 3 {
        let current = input[i];
        let mut j = i - 1;

        loop {
            if current == input[j] {
                start = j + 1;
            }

            if j <= start {
                break;
            } else {
                j -= 1;
            }
        }

        i += 1;
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
