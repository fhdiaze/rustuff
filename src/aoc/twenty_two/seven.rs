use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub fn main() {
    solve();
}

fn solve() {
    let input = read_input();

    for line in input.flatten() {
        let token = parser::parse(line);
        println!("{:?}", token);
    }
}

/// Grammar: https://bnfplayground.pauliankline.com/
/// <exp> ::= <cmd> | <out>
/// <cmd> ::= "$ " (<cd> | <ls>)
/// <cd> ::= "cd " <name>
/// <ls> ::= "ls"
/// <out> ::= <dir> | <file>
/// <dir> ::= "dir " <name>
/// <file> ::= <number> " " <fname>
/// <fname> ::= <name> ("." <name>)?
/// <number> ::= [0-9]+
/// <name> ::= [a-z]+
mod parser {
    use std::{iter::Peekable, str::Chars};

    #[derive(Debug)]
    pub enum Expression {
        Cmd(Command),
        Out(Output),
    }

    #[derive(Debug)]
    pub enum Command {
        Ls,
        Cd { path: String },
    }

    #[derive(Debug)]
    pub enum Output {
        File { name: String, size: usize },
        Dir { name: String },
    }

    pub fn parse(text: String) -> Option<Expression> {
        let mut chars = text.chars().peekable();

        exp(&mut chars)
    }

    fn exp(scanner: &mut Peekable<Chars>) -> Option<Expression> {
        cmd(scanner)
            .map(Expression::Cmd)
            .or(out(scanner).map(Expression::Out))
    }

    fn cmd(scanner: &mut Peekable<Chars>) -> Option<Command> {
        scanner
            .next_if_eq(&'$')
            .and_then(|_| scanner.next_if_eq(&' '))
            .and_then(|_| cd(scanner))
            .or(ls(scanner))
    }

    fn ls(scanner: &mut Peekable<Chars>) -> Option<Command> {
        scanner
            .next_if_eq(&'l')
            .and_then(|_| scanner.next_if_eq(&'s'))
            .map(|_| Command::Ls)
    }

    fn cd(scanner: &mut Peekable<Chars>) -> Option<Command> {
        scanner
            .next_if_eq(&'c')
            .and_then(|_| scanner.next_if_eq(&'d'))
            .and_then(|_| scanner.next_if_eq(&' '))
            .map(|_| scanner.by_ref().take_while(|x| x.ne(&' ')).collect())
            .map(|path: String| Command::Cd { path })
    }

    fn out(scanner: &Peekable<Chars>) -> Option<Output> {
        dir(scanner).or(file(scanner))
    }

    fn dir(scanner: &Peekable<Chars>) -> Option<Output> {
        Some(Output::Dir {
            name: String::from(""),
        })
    }

    fn file(scanner: &Peekable<Chars>) -> Option<Output> {
        Some(Output::File {
            name: String::from(""),
            size: 0,
        })
    }
}

fn read_input() -> Lines<BufReader<File>> {
    let file = File::open("./src/aoc/twenty_two/seven.txt").unwrap();
    let buf = BufReader::new(file);

    buf.lines()
}
