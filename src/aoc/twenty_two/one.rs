use std::{io::{self, BufRead}, fs::File};

fn main() {
    let max_calories = solve();

    println!("{}", max_calories);
}

pub fn solve() -> i32 {
    let lines = read_lines();
    let mut max_calories = -1;
    let mut elf_calories = 0;

    for l in lines {
        if let Ok(t) = l {
            if t.is_empty() {
                if elf_calories > max_calories {
                    max_calories = elf_calories;
                } 

                elf_calories = 0;
            } else {
                elf_calories = elf_calories + t.parse::<i32>().unwrap();
            }
        }
    }

    return max_calories; 
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let file = File::open("./day_one_input.txt").unwrap();
    let buf = io::BufReader::new(file);
    
    return buf.lines();
}
