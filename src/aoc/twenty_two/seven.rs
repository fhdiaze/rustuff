use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    str::CharIndices,
};

pub fn main() {
    solve();
}

fn solve() {
    let input = read_input();

    for line in input.flatten() {
        let token = parse_line(line);
    }
}

// Grammar: https://bnfplayground.pauliankline.com/
// <exp> ::= <cmd> | <out>
// <cmd> ::= "$ " (<cd> | <ls>)
// <cd> ::= "cd " <name>
// <ls> ::= "ls"
// <out> ::= "dir " <name> | <number> " " <file>
// <file> ::= <name> ("." <name>)?
// <number> ::= [0-9]+
// <name> ::= [a-z]+
struct Parser {}

impl Parser {
    fn parse(text: String) -> Option<Expression> {
        Some(Expression::Cmd(Command::Ls))
    }

    fn exp(&self) -> Option<Expression> {
        Some(Expression::Cmd(Command::Ls))
    }

    fn cmd(&self) -> Option<Command> {
        Some(Command::Ls)
    }
}

enum Expression {
    Cmd(Command),
    Out(Output),
}

enum Command {
    Ls,
    Cd { path: String },
}

enum Output {
    File { name: String, size: usize },
    Dir { name: String },
}

#[derive(Debug)]
enum Token {
    Dollar,
    Cd,
    Ls,
    Size(usize),
    Dir,
    Name(String),
}

struct Tokenizer<'a> {
    cursor: usize,
    chars: CharIndices<'a>,
}

impl Tokenizer<'a> {
    fn new(input: String) -> Self {
        Tokenizer {
            cursor: 0,
            chars: input.char_indices(),
        }
    }

    fn next(&self) -> Option<Token> {
        while let Some((pos, ch)) = self.chars.next() {
            match ch {
                '$' => {
                    return Some(Token::Dollar);
                }
                'c' => {
                    return Some(Token::Cd);
                }
                'l' => {
                    return Some(Token::Ls);
                }
                's' => {
                    return Some(Token::Size(parse_size()));
                }
                'd' => {
                    cursor += 1;
                    return Some(Token::Dir);
                }
                _ => {
                    cursor += 1;
                    return Some(Token::Name(parse_name()));
                }
            }
        }

        None
    }
}

fn read_input() -> Lines<BufReader<File>> {
    let file = File::open("./src/aoc/twenty_two/seven.txt").unwrap();
    let buf = BufReader::new(file);

    buf.lines()
}
