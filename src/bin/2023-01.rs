use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::{env, println};

const INPUT_FILENAME: &str = "2023-01-input.txt";

fn logic(buffered: BufReader<File>) {
    let sum_pure_digits: u32 = buffered
        .lines()
        .filter_map(|s| s.ok())
        .map(|line| {
            let first_digit = line
                .chars()
                .find(|c| c.is_digit(10))
                .unwrap_or('0')
                .to_digit(10)
                .unwrap();
            let last_digit = line
                .chars()
                .rev()
                .find(|c| c.is_digit(10))
                .unwrap_or('0')
                .to_digit(10)
                .unwrap();

            first_digit * 10 + last_digit
        })
        .sum();

    println!("Part 1 | Sum of combined numeric digits {sum_pure_digits}");
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
