use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::{env, println};

const INPUT_FILENAME: &str = "2022-08-input.txt";

fn logic(buffered: BufReader<File>) {
    let tree_grid: Vec<Vec<u32>> = buffered
        .lines()
        .filter_map(|unparsed_tree_row| unparsed_tree_row.ok())
        .map(|tree_row| {
            tree_row
                .chars()
                .map(|tree| tree.to_digit(10).unwrap_or_default())
                .collect()
        })
        .collect();

    println!(
        "Part 1 | Trees visible outside the grid: {}",
        count_trees_on_border(&tree_grid) + count_trees_within_border(&tree_grid)
    );
}

fn count_trees_on_border<T>(tree_grid: &Vec<Vec<T>>) -> usize {
    2 * tree_grid.len() + 2 * (tree_grid[0].len() - 2)
}

fn count_trees_within_border(tree_grid: &Vec<Vec<u32>>) -> usize {
    let mut counter: usize = 0;
    for (row, tree_row) in tree_grid.iter().enumerate() {
        for (column, current_tree) in tree_grid[row].iter().enumerate() {
            let is_in_border = row == 0
                || row == tree_grid.len() - 1
                || column == 0
                || column == tree_row.len() - 1;
            if !is_in_border {
                let tree_column = get_column(tree_grid, column);
                let (up, down) = get_split(&tree_column, row);
                let max_up = up.iter().max();
                let max_down = down.iter().max();

                let (left, right) = get_split(&tree_row, column);
                let max_left = left.iter().max();
                let max_right = right.iter().max();

                let is_tree_visible = match (max_up, max_down, max_left, max_right) {
                    (Some(max_up), Some(max_down), Some(max_left), Some(max_right)) => {
                        current_tree > max_up
                            || current_tree > max_down
                            || current_tree > max_left
                            || current_tree > max_right
                    }
                    _ => false,
                };
                if is_tree_visible {
                    counter += 1;
                }
            }
        }
    }
    counter
}

fn get_column<T: Copy>(grid: &Vec<Vec<T>>, col: usize) -> Vec<T> {
    grid.iter().map(|row| row[col]).collect()
}

fn get_split<T: Copy>(vector: &Vec<T>, index: usize) -> (Vec<T>, Vec<T>) {
    let (a, b) = vector.split_at(index);
    let a: Vec<T> = a.iter().map(|x| *x).collect();
    let b: Vec<T> = b.iter().skip(1).map(|x| *x).collect();
    (a, b)
}

fn main() -> Result<(), Error> {
    let current_dir = env::current_dir()?;
    let shared_path = current_dir.join("shared");
    let input_path = shared_path.join(INPUT_FILENAME);
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
    logic(buffered);

    Ok(())
}
