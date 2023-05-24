use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::{env, println};

const INPUT_FILENAME: &str = "2022-02-input.txt";

fn logic(buffered: BufReader<File>) {
    let score = buffered
        .lines()
        .filter_map(|unparsed_line| unparsed_line.ok())
        .fold(0, |mut acc, cur| {
            let collected_line = cur.split(" ").collect::<Vec<&str>>();
            let (opponent, yours) = (collected_line[0], collected_line[1]);

            acc += match yours {
                "X" => 1,
                "Y" => 2,
                _ => 3,
            };
            acc += if opponent == "A" {
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
                //

                match yours {
                    "X" => 6,
                    "Z" => 3,
                    _ => 0,
                } // [Rock, Scissors, Paper]
            };
            acc
        });

    println!("Part 1 | Total score based on strategy guide 1: {}", score);
    // println!("Part 2 | Total score based on strategy guide 2: {}");
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
