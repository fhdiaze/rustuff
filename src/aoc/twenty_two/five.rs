use relative_path::RelativePath;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use self::parser::Token;

pub fn main() {
    solve();
}

fn solve() {
    let input = read_input();
    let mut ship = Ship::new();

    for text in input.flatten() {
        let tokens = parser::parse_line(&text);
        ship.run(tokens);
    }

    println!("{:?}", ship.top().iter().collect::<String>());
}

mod parser {
    use regex::Regex;

    #[derive(Debug)]
    pub enum Token {
        Mark(char),
        Blank,
        Digit(char),
        Move {
            quantity: usize,
            from: usize,
            to: usize,
        },
    }

    pub fn parse_line(text: &str) -> Vec<Token> {
        match text.chars().next() {
            Some(' ') | Some('[') => parse_marks(text),
            Some('m') => vec![parse_move(text)],
            _ => vec![],
        }
    }

    pub fn parse_move(text: &str) -> Token {
        let regex = Regex::new(r"move (?<q>\d*) from (?<f>\d*) to (?<t>\d*)").unwrap();
        let caps = regex.captures(text).unwrap();
        let q = caps["q"].parse::<usize>().unwrap();
        let f = caps["f"].parse::<usize>().unwrap();
        let t = caps["t"].parse::<usize>().unwrap();

        Token::Move {
            quantity: q,
            from: f,
            to: t,
        }
    }

    pub fn parse_marks(text: &str) -> Vec<Token> {
        let mut ti = 1;
        let mut tokens: Vec<Token> = Vec::with_capacity(10);

        for (i, c) in text.chars().enumerate() {
            if i == ti {
                let token = match c {
                    _ if c.is_whitespace() => Token::Blank,
                    m if c.is_alphabetic() => Token::Mark(m),
                    _ => Token::Digit(c),
                };

                tokens.push(token);

                ti += 4;
            }
        }

        tokens
    }
}

#[derive(Debug)]
struct Ship {
    stacks: Vec<Stack>,
}

impl Ship {
    fn new() -> Self {
        Ship {
            stacks: Vec::with_capacity(10),
        }
    }

    fn run(&mut self, tokens: Vec<Token>) {
        for (index, token) in tokens.iter().enumerate() {
            match token {
                Token::Mark(_) => self.run_load(index, token),
                Token::Move { .. } => self.run_move(token),
                _ => (),
            }
        }
    }

    fn run_load(&mut self, stack: usize, token: &Token) {
        if let Token::Mark(m) = token {
            self.try_add_stack(stack);
            self.stacks[stack].insert(*m);
        }
    }

    fn try_add_stack(&mut self, s: usize) {
        while s >= self.stacks.len() {
            self.stacks.push(Stack::new());
        }
    }

    fn run_move(&mut self, m: &Token) {
        if let Token::Move { quantity, from, to } = m {
            let marks = self.stacks[from - 1].pop(*quantity);
            self.stacks[to - 1].push_many(marks);
        }
    }

    fn top(&self) -> Vec<char> {
        self.stacks.iter().map(|s| s.top()).collect()
    }
}

#[derive(Debug)]
struct Stack {
    marks: Vec<char>,
}

impl Stack {
    fn new() -> Self {
        Stack { marks: vec![] }
    }
    fn insert(&mut self, mark: char) {
        self.marks.insert(0, mark);
    }

    fn push(&mut self, mark: char) {
        self.marks.push(mark);
    }

    fn push_many(&mut self, mut marks: Vec<char>) {
        self.marks.append(&mut marks);
    }

    fn pop(&mut self, quantity: usize) -> Vec<char> {
        let mut marks = Vec::with_capacity(quantity);

        for _ in 0..quantity {
            marks.push(self.marks.pop().unwrap());
        }

        marks
    }

    fn top(&self) -> char {
        self.marks.last().copied().unwrap_or(' ')
    }
}

fn read_input() -> Lines<BufReader<File>> {
    let relative = RelativePath::new("./src/aoc/twenty_two/five.txt");
    let file = File::open(relative.to_path("./")).unwrap();
    let buf = BufReader::new(file);

    buf.lines()
}
