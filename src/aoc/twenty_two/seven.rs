use std::{
    cell::RefCell,
    fs,
    io::{BufRead, BufReader, Lines},
    rc::Rc,
};

use crate::aoc::twenty_two::seven::parser::{Command, Expression, Output};

pub fn main() {
    solve();
}

fn solve() {
    let input = read_input();

    for line in input.flatten() {
        let expression = parser::parse(line).unwrap();
        println!("{:?}", expression);
        match expression {
            Expression::Cmd(cmd) => match cmd {
                Command::Ls => println!("ls"),
                Command::Cd { path } => println!("cd {}", path),
            },
            Expression::Out(out) => match out {
                Output::Dir { name } => println!("out"),
                Output::File { name, size } => println!("out"),
            },
        }
    }
}

struct FileSystem {
    pwd: Option<Rc<RefCell<Directory>>>,
    root: Option<Rc<RefCell<Directory>>>,
}

enum Item {
    File(File),
    Directory(Directory),
}

struct Directory {
    name: String,
    children: Vec<Rc<RefCell<Item>>>,
}

struct File {
    name: String,
    size: usize,
}

impl FileSystem {
    fn new() -> FileSystem {
        FileSystem {
            root: None,
            pwd: None,
        }
    }

    fn get_child(&self, name: &str) -> Option<Rc<RefCell<Item>>> {
        self.pwd.and_then(|x| {
            x.borrow()
                .children
                .iter()
                .find(|i| match *i.borrow() {
                    Item::Directory(d) => d.name == name,
                    Item::File(f) => f.name == name,
                })
                .map(|&i| i.clone())
        })
    }

    fn add_child(&mut self, item: Item) -> Rc<RefCell<Item>> {
        self.pwd.take().unwrap().borrow_mut().add_child(item)
    }

    fn cd(&mut self, name: &str) {
        let child = match self.get_child(name) {
            Some(c) => c,
            None => {
                let c = Item::Directory(Directory::new(name.to_string()));
                self.add_child(c)
            }
        };

        self.pwd = Some(child);
    }
}

impl Directory {
    fn new(name: String) -> Directory {
        Directory {
            name,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, item: Item) -> Rc<RefCell<Item>> {
        let item = Rc::new(RefCell::new(item));
        self.children.push(item);

        item.clone()
    }
}

/// Grammar: https://bnfplayground.pauliankline.com/
/// <exp> ::= <cmd> | <out>
/// <cmd> ::= "$ " (<cd> | <ls>)
/// <cd> ::= "cd " <name>
/// <ls> ::= "ls"
/// <out> ::= <dir> | <file>
/// <dir> ::= "dir " <name>
/// <file> ::= <number> " " <file_name>
/// <file_name> ::= <name> ("." <name>)?
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

    fn out(scanner: &mut Peekable<Chars>) -> Option<Output> {
        dir(scanner).or(file(scanner))
    }

    fn dir(scanner: &mut Peekable<Chars>) -> Option<Output> {
        scanner
            .next_if_eq(&'d')
            .and_then(|_| scanner.next_if_eq(&'i'))
            .and_then(|_| scanner.next_if_eq(&'r'))
            .and_then(|_| scanner.next_if_eq(&' '))
            .map(|_| scanner.by_ref().take_while(|x| x.ne(&' ')).collect())
            .map(|name: String| Output::Dir { name })
    }

    fn file(scanner: &mut Peekable<Chars>) -> Option<Output> {
        number(scanner).and_then(|size| file_name(scanner).map(|name| Output::File { name, size }))
    }

    fn file_name(scanner: &mut Peekable<Chars>) -> Option<String> {
        name(scanner).map(|fname| {
            name(scanner)
                .map(|ext| format!("{}.{}", fname, ext))
                .unwrap_or(fname)
        })
    }

    fn name(scanner: &mut Peekable<Chars>) -> Option<String> {
        let n: String = scanner.by_ref().take_while(|x| x.is_alphabetic()).collect();

        match n.len() {
            0 => None,
            _ => Some(n),
        }
    }

    fn number(scanner: &mut Peekable<Chars>) -> Option<usize> {
        let n: String = scanner.by_ref().take_while(|x| x.is_numeric()).collect();

        match n.len() {
            0 => None,
            _ => Some(n.parse::<usize>().unwrap()),
        }
    }
}

fn read_input() -> Lines<BufReader<fs::File>> {
    let file = fs::File::open("./src/aoc/twenty_two/seven.txt").unwrap();
    let buf = BufReader::new(file);

    buf.lines()
}
