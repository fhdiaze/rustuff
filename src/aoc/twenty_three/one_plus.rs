use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    iter::Peekable,
    str::Chars,
};

pub fn solve() {
    let lines = read_input();
    let mut sum = 0;

    for l in lines.flatten() {
        let mut digits = parse(l).into_iter();
        println!("{:?}", digits);
        let first = digits.next().unwrap();
        let last = digits.last().unwrap_or(first);
        let value: usize = format!("{}{}", first, last).parse().unwrap();
        dbg!(value);

        sum += value;
    }

    println!("Sum of all of the calibration values: {}", sum);
}

/// Grammar: https://bnfplayground.pauliankline.com/
/// <line> ::= (<digit> | <maybe_spell_digit>)+
/// <digit> ::= [0-9]
/// <maybe_spell_digit> ::= "o" ("n""e")? | t(wo | hree)? | f(our | ive) | tc
/// <tc> ::= [a-z]
fn parse(line: String) -> Vec<u8> {
    let mut chars = line.chars().peekable();
    let mut digits = vec![];

    while chars.peek().is_some() {
        if let Some(digit) = md(&mut chars) {
            digits.push(digit);
        }
    }

    digits
}

fn md(scanner: &mut Peekable<Chars>) -> Option<u8> {
    digit(scanner).or_else(|| mbsd(scanner))
}

fn digit(scanner: &mut Peekable<Chars>) -> Option<u8> {
    scanner
        .next_if(|c| c.is_ascii_digit())
        .map(|c| c.to_string().parse().unwrap())
}

fn mbsd(scanner: &mut Peekable<Chars>) -> Option<u8> {
    if let Some(c) = scanner.next_if(|c| c.is_ascii_alphabetic()) {
        return match c {
            'o' => so(scanner),
            't' => st(scanner),
            'f' => sf(scanner),
            's' => ss(scanner),
            'e' => se(scanner),
            'n' => sn(scanner),
            _ => None,
        };
    }

    None
}

fn so(scanner: &mut Peekable<Chars>) -> Option<u8> {
    if scanner.next_if_eq(&'n').is_some() {
        if scanner.next_if_eq(&'e').is_some() {
            return Some(1);
        } else {
            return sn(scanner);
        }
    };

    None
}

fn st(scanner: &mut Peekable<Chars>) -> Option<u8> {
    if scanner.next_if_eq(&'w').is_some() {
        if scanner.next_if_eq(&'o').is_some() {
            return Some(2);
        } else {
            return sn(scanner);
        }
    } else if scanner.next_if_eq(&'h').is_some()
        && scanner.next_if_eq(&'r').is_some()
        && scanner.next_if_eq(&'e').is_some()
    {
        if scanner.next_if_eq(&'e').is_some() {
            return Some(3);
        } else {
            return se(scanner);
        }
    };

    None
}

fn sf(scanner: &mut Peekable<Chars>) -> Option<u8> {
    if scanner.next_if_eq(&'o').is_some() {
        if scanner.next_if_eq(&'u').is_some() && scanner.next_if_eq(&'r').is_some() {
            return Some(4);
        } else {
            return so(scanner);
        }
    } else if scanner.next_if_eq(&'i').is_some()
        && scanner.next_if_eq(&'v').is_some()
        && scanner.next_if_eq(&'e').is_some()
    {
        return Some(5);
    };

    None
}

fn ss(scanner: &mut Peekable<Chars>) -> Option<u8> {
    if scanner.next_if_eq(&'i').is_some() && scanner.next_if_eq(&'x').is_some() {
        return Some(6);
    } else if scanner.next_if_eq(&'e').is_some() {
        if scanner.next_if_eq(&'v').is_some() {
            if scanner.next_if_eq(&'e').is_some() {
                if scanner.next_if_eq(&'n').is_some() {
                    return Some(7);
                } else {
                    return se(scanner);
                }
            }
        } else {
            return se(scanner);
        }
    }

    None
}

fn se(scanner: &mut Peekable<Chars>) -> Option<u8> {
    scanner
        .next_if_eq(&'i')
        .and_then(|_| scanner.next_if_eq(&'g'))
        .and_then(|_| scanner.next_if_eq(&'h'))
        .and_then(|_| scanner.next_if_eq(&'t'))
        .map(|_| 8)
}

fn sn(scanner: &mut Peekable<Chars>) -> Option<u8> {
    if scanner.next_if_eq(&'i').is_some() && scanner.next_if_eq(&'n').is_some() {
        if scanner.next_if_eq(&'e').is_some() {
            return Some(9);
        } else {
            return sn(scanner);
        }
    };

    None
}

fn read_input() -> Lines<BufReader<File>> {
    let file = File::open("src/aoc/twenty_three/one.txt").unwrap();
    let buffer = BufReader::new(file);

    buffer.lines()
}
