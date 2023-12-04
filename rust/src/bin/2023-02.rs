use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::{env, println};

use regex::Regex;

const INPUT_FILENAME: &str = "2023-02-input.txt";

// Puzzle link:
// https://adventofcode.com/2023/day/2

struct BagOfCubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl BagOfCubes {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn logic(buffered: BufReader<File>) {
    let lines: Vec<String> = buffered.lines().filter_map(|s| s.ok()).collect();
    let re_red = Regex::new(r"\b(\d+) red\b").unwrap();
    let re_green = Regex::new(r"\b(\d+) green\b").unwrap();
    let re_blue = Regex::new(r"\b(\d+) blue\b").unwrap();

    let reference_bag = BagOfCubes {
        red: 12,
        green: 13,
        blue: 14,
    };

    let (sum_of_game_ids, sum_of_set_powers): (usize, u32) =
        lines
            .iter()
            .enumerate()
            .fold((0, 0), |acc, (game_id, game)| {
                let current_game_bag = BagOfCubes {
                    red: re_red
                        .captures_iter(game)
                        .map(|cap| cap[1].parse::<u32>().unwrap_or(0))
                        .max()
                        .unwrap_or(0),
                    green: re_green
                        .captures_iter(game)
                        .map(|cap| cap[1].parse::<u32>().unwrap_or(0))
                        .max()
                        .unwrap_or(0),
                    blue: re_blue
                        .captures_iter(game)
                        .map(|cap| cap[1].parse::<u32>().unwrap_or(0))
                        .max()
                        .unwrap_or(0),
                };

                if current_game_bag.red <= reference_bag.red
                    && current_game_bag.green <= reference_bag.green
                    && current_game_bag.blue <= reference_bag.blue
                {
                    (acc.0 + (game_id + 1), acc.1 + current_game_bag.power())
                } else {
                    (acc.0, acc.1 + current_game_bag.power())
                }
            });
    println!("Part 1 | Sum of valid game ids {sum_of_game_ids}");
    println!("Part 2 | Sum of power of sets {sum_of_set_powers}");

    assert_eq!(sum_of_game_ids, 2416);
    assert_eq!(sum_of_set_powers, 63307);
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
