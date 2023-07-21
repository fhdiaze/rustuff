use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<isize>()
        .unwrap();
    let s = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();
    let mut transformers: Vec<Location> = Vec::with_capacity((s + 1) as usize);
    let mut commands: Vec<Command> = Vec::with_capacity((s + 1) as usize);
    transformers.push(Location::new(0, 0));
    commands.push(Command::new(Location::new(0, 0), n));

    for i in 0..s as usize {
        let cmd_input: Vec<isize> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<isize>().unwrap())
            .collect();
        let cmd = Command::new(
            Location::new(cmd_input[0] - 1, cmd_input[1] - 1),
            cmd_input[2],
        );
        transformers.push(cmd.apply(&transformers[i]));
        commands.push(cmd);
    }

    let knights_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap();

    let mut knights: Vec<isize> = Vec::with_capacity(knights_count);

    for _ in 0..knights_count {
        let knights_item = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<isize>()
            .unwrap();
        knights.push(knights_item);
    }

    let result = king_richard_knights(n, &commands, &transformers, &knights);

    for l in result.iter() {
        write!(&mut fptr, "{}", l.row + 1).ok();
        write!(&mut fptr, " ").ok();
        write!(&mut fptr, "{}", l.column + 1).ok();
        writeln!(&mut fptr).ok();
    }
    writeln!(&mut fptr).ok();
}

#[derive(Clone)]
struct Location {
    row: isize,
    column: isize,
}

impl Location {
    fn new(row: isize, column: isize) -> Self {
        Location { row, column }
    }

    fn smaller(&self, other: &Self) -> bool {
        self.row <= other.row && self.column <= other.column
    }

    fn bigger(&self, other: &Self) -> bool {
        self.row >= other.row && self.column >= other.column
    }

    fn from_index(index: &isize, n: isize) -> Self {
        Self::new(index / n, index % n)
    }

    fn add(&self, other: &Self) -> Self {
        Self::new(self.row + other.row, self.column + other.column)
    }

    fn rotate(&self) -> Self {
        Self::new(self.column, -self.row)
    }

    fn rotate_many(&self, k: usize) -> Location {
        let pos = k % 4;

        match pos {
            0 => self.clone(),
            1 => Self::new(self.column, -self.row),
            2 => Self::new(-self.row, -self.column),
            _ => Self::new(-self.column, self.row)
        }
    }

    fn transform(&self, k: usize, transformer: &Location) -> Self {
        let rotated = self.rotate_many(k);

        rotated.add(transformer)
    }
}

struct Command {
    first: Location,
    last: Location,
}

impl Command {
    fn new(first: Location, side: isize) -> Self {
        let last = Location::new(first.row + side, first.column + side);
        Command {
            first,
            last,
        }
    }

    fn contains(&self, location: &Location) -> bool {
        self.first.smaller(location) && self.last.bigger(location)
    }

    fn apply(&self, location: &Location) -> Location {
        let t = location.rotate();
        let transformer = Location::new(
            self.first.row - self.first.column,
            self.last.column + self.first.row,
        );

        t.add(&transformer)
    }
}

/*
 * Complete the 'kingRichardKnights' function below.
 *
 * The function is expected to return a 2D_INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER n The side of the knights square
 *  2. Vec<Vec<i32>> cmds The commands
 *  3. INTEGER_ARRAY knights
 */
fn king_richard_knights(
    n: isize,
    cmds: &Vec<Command>,
    transformers: &Vec<Location>,
    knights: &[isize],
) -> Vec<Location> {
    let mut response = Vec::with_capacity(knights.len());

    for k in knights.iter() {
        let initial_position = Location::from_index(k, n);
        let inner_most_cmd = find_inner_most_command(cmds, transformers, &initial_position);

        println!("knight: {} -> command: {}", k, inner_most_cmd);

        let new_position =
            initial_position.transform(inner_most_cmd, &transformers[inner_most_cmd]);

        response.push(new_position);
    }

    response
}

fn find_inner_most_command(
    cmds: &Vec<Command>,
    transformers: &Vec<Location>,
    knight_location: &Location,
) -> usize {
    let mut l: usize = 0;
    let mut r: usize = cmds.len() as usize;
    while r - l > 1 {
        let m: usize = (l + r) / 2;
        let rotated = knight_location.transform(m, &transformers[m]);
        if cmds[m].contains(&rotated) {
            l = m;
        } else {
            r = m;
        }
    }

    l
}

