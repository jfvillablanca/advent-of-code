use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::{env, println};

const INPUT_FILENAME: &str = "2022-01-input.txt";

fn logic(buffered: BufReader<File>) {
    let mut output_arr: Vec<i32> = buffered
        .lines()
        .filter_map(|unparsed_number| unparsed_number.ok())
        .fold(Vec::new(), |mut acc: Vec<i32>, cur| {
            if cur.is_empty() {
                acc.push(0);
            } else {
                if let Some(last) = acc.last_mut() {
                    *last += if let Ok(parsed_number) = cur.parse::<i32>() {
                        parsed_number
                    } else {
                        0
                    };
                }
            }
            acc
        });
    // output_arr.sort(); // sort in ascending order
    output_arr.sort_by(|a, b| b.cmp(a));

    // println!("Part 1 | Highest total num: {}", output_arr.last().unwrap_or(&0)); // if sorted in ascending order
    println!("Part 1 | Highest total num: {}", output_arr[0]);
    println!(
        "Part 2 | Total sum of top 3: {}",
        // output_arr[output_arr.len() - 3..] // if sorted in ascending order
        output_arr[..3]
            .iter()
            .map(|&x| x)
            .reduce(|acc, cur| acc + cur)
            .unwrap_or(0)
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
