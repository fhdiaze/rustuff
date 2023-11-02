use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub fn main() {
    solve();
}

fn solve() {
    let input = read_input();
    let mut columns_max: Vec<Option<u8>> = vec![];
    let mut grid: Vec<Vec<u8>> = vec![];

    for (row, l) in input.flatten().enumerate() {
        let mut row_max: Option<u8> = None;
        grid.push(vec![]);

        for (column, c) in l.char_indices() {
            let height: u8 = c.to_string().parse::<u8>().unwrap();
            let mut status: u8 = 20;
            add_column_max(&mut columns_max, column);

            if row_max.map_or(true, |m| height > m) {
                row_max = Some(height);
                status = 30;
            }

            if columns_max[column].map_or(true, |m| height > m) {
                status = 30;
                columns_max[column] = Some(height);
            }

            grid[row].push(status + height);

            update_row(&mut grid, row, height);
            update_column(&mut grid, column, height);
        }
    }

    print!("Visible trees: {}", count_visible(grid));
}

fn count_visible(grid: Vec<Vec<u8>>) -> usize {
    grid.into_iter().flatten().filter(|&h| h >= 10).count()
}

fn update_column(grid: &mut [Vec<u8>], column: usize, height: u8) {
    let mut row = grid.len() - 1;

    while row > 0 && height >= grid[row - 1][column] % 10 {
        if grid[row - 1][column] / 10 == 1 {
            grid[row - 1][column] -= 10;
        }
        row -= 1;
    }
}

fn update_row(grid: &mut [Vec<u8>], row: usize, height: u8) {
    let mut column = grid[row].len() - 1;

    while column > 0 && height >= grid[row][column - 1] % 10 {
        if grid[row][column - 1] / 10 == 2 {
            grid[row][column - 1] -= 10;
        }
        column -= 1;
    }
}

fn add_column_max(columns_max: &mut Vec<Option<u8>>, column: usize) {
    if columns_max.get(column).is_none() {
        columns_max.push(None);
    }
}

fn read_input() -> Lines<BufReader<File>> {
    let file = File::open("src/aoc/twenty_two/eight.txt").expect("Could not read file");
    let reader = BufReader::new(file);

    reader.lines()
}
