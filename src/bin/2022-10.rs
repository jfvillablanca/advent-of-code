use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::{env, println, vec};

const INPUT_FILENAME: &str = "2022-10-input.txt";

fn logic(buffered: BufReader<File>) {
    let start_cycle = 20;
    let cycle_interval = 40;

    let vector_arr: Vec<_> = buffered
        .lines()
        .filter_map(|unparsed_line| unparsed_line.ok())
        .flat_map(|x| {
            let mut line = x.split_whitespace();
            let instruction = line.next().unwrap_or("");
            match instruction {
                "noop" => vec![0],
                "addx" => {
                    let value = line.next().unwrap_or("").parse::<i32>().unwrap_or_default();
                    vec![0, value]
                }
                _ => panic!("Parse error: make sure input file follows the puzzle format"),
            }
        })
        .collect();

    let signal_strength = vector_arr
        .into_iter()
        .enumerate()
        .fold(
            (1, 0),
            |(base_accumulator, multiplied_accumulator), (cycle, signal)| {
                let adjusted_cycle = cycle + 1;

                let updated_base_accumulator = base_accumulator + signal;
                let updated_multiplied_accumulator = if adjusted_cycle == start_cycle
                    || (adjusted_cycle > start_cycle
                        && (adjusted_cycle - start_cycle) % cycle_interval == 0)
                {
                    (adjusted_cycle as i32 * base_accumulator) + multiplied_accumulator
                } else {
                    multiplied_accumulator
                };

                (updated_base_accumulator, updated_multiplied_accumulator)
            },
        )
        .1;
    println!("{:#?}", signal_strength)
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
