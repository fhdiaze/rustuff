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
    let mut fs = FileSystem::new();

    for line in input.flatten() {
        let expression = parser::parse(line).unwrap();
        match expression {
            Expression::Cmd(cmd) => {
                if let Command::Cd { name } = cmd {
                    fs.cd(&name);
                }
            }
            Expression::Out(out) => {
                _ = match out {
                    Output::Dir { name } => fs.add_child(Item::Directory(Directory::new(name))),
                    Output::File { name, size } => fs.add_child(Item::File(File { name, size })),
                }
            }
        }
    }

    println!("{:?}", fs);
}

#[derive(Debug)]
struct FileSystem {
    pwd: Rc<RefCell<Item>>,
    parent: Rc<RefCell<Item>>,
    root: Rc<RefCell<Item>>,
}

impl FileSystem {
    fn new() -> FileSystem {
        let root = Rc::new(RefCell::new(Item::Directory(Directory::new(
            "/".to_string(),
        ))));

        FileSystem {
            pwd: root.clone(),
            parent: root.clone(),
            root,
        }
    }

    fn get_child(&self, name: &str) -> Option<Rc<RefCell<Item>>> {
        self.pwd.borrow().get_child(name)
    }

    fn add_child(&mut self, item: Item) -> Rc<RefCell<Item>> {
        self.pwd.borrow_mut().add_child(item)
    }

    fn cd(&mut self, name: &str) {
        let new_pwd = match name {
            "/" => Some(self.root.clone()),
            ".." => Some(self.parent.clone()),
            _ => self.get_child(name),
        };
        match new_pwd {
            Some(dir) => {
                self.parent = self.pwd.clone();
                self.pwd = dir;
            }
            None => panic!("No such file or directory"),
        }
    }
}

#[derive(Debug)]
enum Item {
    File(File),
    Directory(Directory),
}

impl Item {
    fn name(&self) -> &str {
        match self {
            Item::File(f) => &f.name,
            Item::Directory(d) => &d.name,
        }
    }

    fn get_child(&self, name: &str) -> Option<Rc<RefCell<Item>>> {
        match self {
            Item::File(_) => None,
            Item::Directory(d) => d
                .children
                .iter()
                .find(|i| i.borrow().name() == name)
                .cloned(),
        }
    }

    fn add_child(&mut self, item: Item) -> Rc<RefCell<Item>> {
        match self {
            Item::File(_) => panic!("Cannot add child to file"),
            Item::Directory(d) => d.add_child(item),
        }
    }
}

#[derive(Debug)]
struct Directory {
    name: String,
    children: Vec<Rc<RefCell<Item>>>,
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
        self.children.push(item.clone());

        item
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
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
        Cd { name: String },
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
            .map(|path: String| Command::Cd { name: path })
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
