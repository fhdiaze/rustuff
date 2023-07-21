pub fn solve() -> i32 {
    d;
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let file = File::open(Path::from_string("./src/aoc22/day_one_input.txt")).unwrap();
    let buf = io::BufReader::new(file);
    
    return buf.lines();
}
