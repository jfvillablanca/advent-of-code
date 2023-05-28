use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::{env, println, vec};

const INPUT_FILENAME: &str = "2022-05-input.txt";

fn logic(buffered: BufReader<File>) {
    let mut stack_initial_state = vec![];
    let mut stack_instruction_set = vec![];
    let mut current_block = &mut stack_initial_state;

    for line in buffered.lines().filter_map(|line| line.ok()) {
        if line.trim().is_empty() {
            current_block = &mut stack_instruction_set;
        } else {
            current_block.push(line)
        }
    }

    stack_initial_state.iter().for_each(|level| {
        let parsed = get_crate_at_level(level);
        println!("{:?}", parsed);
    });

    stack_instruction_set.iter().for_each(|line| {
        let parsed = get_instruction(line);
        println!("{:?}", parsed);
    });

    // println!("Part 1 | Crates on top: {:?}", crates_on_top);
}

fn get_crate_at_level(level: &String) -> String {
    level
        .chars()
        .enumerate()
        .filter_map(|(index, crate_name)| {
            if index % 2 != 0 && (index - 1) % 4 == 0 {
                Some(crate_name)
            } else {
                None
            }
        })
        .collect()
}

fn get_instruction(instruction: &String) -> (i32, i32, i32) {
    let inst: Vec<i32> = instruction
        .split(' ')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();

    (inst[0], inst[1], inst[2])
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
