use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::{env, println};

const INPUT_FILENAME: &str = "2023-01-input.txt";

// Puzzle link:
// https://adventofcode.com/2023/day/1

fn logic(buffered: BufReader<File>) {
    let lines: Vec<String> = buffered.lines().filter_map(|s| s.ok()).collect();

    let (sum_only_numeric, sum_with_spelled_out): (u32, u32) =
        lines.iter().fold((0, 0), |acc, line| {
            let first = line
                .chars()
                .find(|c| c.is_ascii_digit())
                .unwrap_or('0')
                .to_digit(10)
                .unwrap();
            let last = line
                .chars()
                .rev()
                .find(|c| c.is_ascii_digit())
                .unwrap_or('0')
                .to_digit(10)
                .unwrap();
            let numeric = first * 10 + last;

            let substituted_line = substitute_digit(line);
            let first = substituted_line
                .chars()
                .find(|c| c.is_ascii_digit())
                .unwrap_or('0')
                .to_digit(10)
                .unwrap();
            let last = substituted_line
                .chars()
                .rev()
                .find(|c| c.is_ascii_digit())
                .unwrap_or('0')
                .to_digit(10)
                .unwrap();
            let spelled_out = first * 10 + last;

            (acc.0 + numeric, acc.1 + spelled_out)
        });

    println!("Part 1 | Sum of combined numeric digits {sum_only_numeric}");
    println!("Part 2 | Sum of combined numeric or spelled out digits {sum_with_spelled_out}");

    assert_eq!(sum_only_numeric, 55621);
    assert_eq!(sum_with_spelled_out, 53592);
}

fn substitute_digit(line: &String) -> String {
    let digits = vec![
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3ree"),
        ("four", "f4ur"),
        ("five", "f5ve"),
        ("six", "s6x"),
        ("seven", "s7ven"),
        ("eight", "e8ght"),
        ("nine", "n9ne"),
    ];
    let mut substituted = String::from(line);
    for (digit, sub) in digits {
        substituted = substituted.replace(digit, sub)
    }

    substituted
}

fn main() -> Result<(), Error> {
    let current_dir = env::current_dir()?;
    let shared_path = current_dir.join("../shared");
    let input_path = shared_path.join(INPUT_FILENAME);
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
    logic(buffered);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substitute_digit() {
        assert_eq!(
            substitute_digit(&String::from("eightwothree")),
            String::from("e8ght2ot3ree")
        )
    }
}
