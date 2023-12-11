use std::{
    cmp::{max, Ordering},
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub fn main() {
    solve();
}

pub struct Tree {
    height: u8,
    up: usize,
    left: usize,
    right: usize,
    down: usize,
}

impl Tree {
    fn new(height: u8, up: usize, left: usize, right: usize, down: usize) -> Tree {
        Tree {
            height,
            up,
            left,
            right,
            down,
        }
    }

    fn scenic_score(&self) -> usize {
        self.left * self.up * self.right * self.down
    }

    fn correct_right_and_down(&mut self, row: usize, column: usize, rows: usize, columns: usize) {
        if column != columns - 1 && self.right == 0 {
            self.right = columns - column - 1;
        }

        if row != rows - 1 && self.down == 0 {
            self.down = rows - row - 1;
        }
    }
}

fn solve() {
    let input = read_input();
    let mut grid: Vec<Vec<Tree>> = vec![];

    for (row, l) in input.flatten().enumerate() {
        grid.push(vec![]);

        for c in l.chars() {
            let height = c.to_string().parse::<u8>().unwrap();
            let mut tree = Tree::new(height, 0, 0, 0, 0);

            update_row(&mut grid, &mut tree);
            update_column(&mut grid, &mut tree);
            grid[row].push(tree);
        }
    }

    print!("Max scenic score: {}", highest_scenic_score(grid));
}

fn highest_scenic_score(grid: Vec<Vec<Tree>>) -> usize {
    let rows = grid.len();
    let columns = grid[0].len();

    grid.into_iter()
        .flatten()
        .enumerate()
        .map(|mut t| {
            t.1.correct_right_and_down(t.0 / columns, t.0 % columns, rows, columns);
            t.1.scenic_score()
        })
        .max()
        .unwrap()
}

fn update_row(grid: &mut [Vec<Tree>], current_tree: &mut Tree) {
    let row = grid.len() - 1;
    let column = grid[row].len();
    let mut c = column;

    while c > 0 {
        let mut t = &mut grid[row][c - 1];
        current_tree.left = max(1, current_tree.left);

        match current_tree.height.cmp(&t.height) {
            Ordering::Greater => {
                current_tree.left += t.left;
                if t.right == 0 {
                    t.right = column - c + 1;
                }
                c -= max(t.left, 1);
            }
            Ordering::Equal => {
                if t.right == 0 {
                    t.right = column - c + 1;
                }
                break;
            }
            Ordering::Less => {
                break;
            }
        }
    }
}

fn update_column(grid: &mut [Vec<Tree>], current_tree: &mut Tree) {
    let row = grid.len() - 1;
    let column = grid[row].len();
    let mut r = row;

    while r > 0 {
        let mut t = &mut grid[r - 1][column];
        current_tree.up = max(1, current_tree.up);

        match current_tree.height.cmp(&t.height) {
            Ordering::Greater => {
                current_tree.up += t.up;
                if t.down == 0 {
                    t.down = row - r + 1;
                }
                r -= max(t.up, 1);
            }
            Ordering::Equal => {
                if t.down == 0 {
                    t.down = row - r + 1;
                }
                break;
            }
            Ordering::Less => {
                break;
            }
        }
    }
}

fn read_input() -> Lines<BufReader<File>> {
    let file = File::open("src/aoc/twenty_two/eight.txt").expect("Could not read file");
    let reader = BufReader::new(file);

    reader.lines()
}
