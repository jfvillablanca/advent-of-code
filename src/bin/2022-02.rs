use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::{env, println};

const INPUT_FILENAME: &str = "2022-02-input.txt";

struct StrategyScores {
    strategy_one: i32,
    strategy_two: i32,
}

fn logic(buffered: BufReader<File>) {
    let score = buffered
        .lines()
        .filter_map(|unparsed_line| unparsed_line.ok())
        .fold(
            StrategyScores {
                strategy_one: 0,
                strategy_two: 0,
            },
            |mut acc, cur| {
                let mut collected_line = cur.split_whitespace();
                let opponent = collected_line.next().unwrap_or("");
                let yours = collected_line.next().unwrap_or("");

                let acc_strategy_one = match yours {
                    "X" => 1,
                    "Y" => 2,
                    _ => 3,
                } + if opponent == "A" {
                    // Rock
                    match yours {
                        "Y" => 6,
                        "X" => 3,
                        _ => 0,
                    } // [Paper, Rock, Scissors]
                } else if opponent == "B" {
                    // Paper
                    match yours {
                        "Z" => 6,
                        "Y" => 3,
                        _ => 0,
                    } // [Scissors, Paper, Rock]
                } else {
                    // Scissors
                    match yours {
                        "X" => 6,
                        "Z" => 3,
                        _ => 0,
                    } // [Rock, Scissors, Paper]
                };

                let acc_strategy_two = match yours {
                    "X" => 0,
                    "Y" => 3,
                    _ => 6,
                } + if opponent == "A" {
                    // Rock
                    match yours {
                        "Y" => 1,
                        "X" => 3,
                        _ => 2,
                    } // [Rock, Scissors, Paper]
                } else if opponent == "B" {
                    // Paper
                    match yours {
                        "Y" => 2,
                        "X" => 1,
                        _ => 3,
                    } // [Paper, Rock, Scissors]
                } else {
                    // Scissors
                    match yours {
                        "Y" => 3,
                        "X" => 2,
                        _ => 1,
                    } // [Scissors, Paper, Rock]
                };
                acc.strategy_one += acc_strategy_one;
                acc.strategy_two += acc_strategy_two;
                acc
            },
        );

    println!(
        "Part 1 | Total score based on strategy guide 1: {}",
        score.strategy_one
    );
    println!(
        "Part 2 | Total score based on strategy guide 2: {}",
        score.strategy_two
    );
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
