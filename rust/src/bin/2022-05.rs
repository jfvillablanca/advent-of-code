use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::{assert_eq, env, println, vec};

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
    stack_initial_state.pop();

    let mut stack_state_9000: Vec<Vec<char>> =
        vec![Vec::new(); get_crate_at_level(&stack_initial_state[0]).len()];

    let mut stack_state_9001: Vec<Vec<char>> =
        vec![Vec::new(); get_crate_at_level(&stack_initial_state[0]).len()];

    for level in stack_initial_state.iter().rev() {
        get_crate_at_level(level)
            .chars()
            .enumerate()
            .for_each(|(index, crate_name)| {
                if !crate_name.is_whitespace() {
                    stack_state_9000[index].push(crate_name);
                    stack_state_9001[index].push(crate_name);
                }
            });
    }

    stack_instruction_set.iter().for_each(|line| {
        move_crates_one_at_a_time(&mut stack_state_9000, get_instruction(line));
        move_crates_n_at_a_time(&mut stack_state_9001, get_instruction(line));
    });

    println!(
        "Part 1 | Crates on top when moved one at a time: {:?}",
        get_crates_on_top(&stack_state_9000)
    );
    println!(
        "Part 2 | Crates on top when moved N at a time: {:?}",
        get_crates_on_top(&stack_state_9001)
    );
    assert_eq!(get_crates_on_top(&stack_state_9000), "TLFGBZHCN");
    assert_eq!(get_crates_on_top(&stack_state_9001), "QRQFHFWCL");
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

fn get_instruction(instruction: &String) -> (usize, usize, usize) {
    let inst: Vec<usize> = instruction
        .split(' ')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();
    (inst[0], inst[1] - 1, inst[2] - 1)
}

fn move_crates_one_at_a_time(
    crate_state: &mut Vec<Vec<char>>,
    (count, src, dest): (usize, usize, usize),
) {
    for _ in 0..count {
        if let Some(crate_name) = crate_state[src].pop() {
            crate_state[dest].push(crate_name);
        } else {
            break;
        }
    }
}

fn move_crates_n_at_a_time(
    crate_state: &mut Vec<Vec<char>>,
    (count, src, dest): (usize, usize, usize),
) {
    let len = crate_state[src].len();
    if count <= len {
        let crates_to_move = crate_state[src].split_off(len - count);
        crate_state[dest].extend_from_slice(&crates_to_move);
    }
}

fn get_crates_on_top(crate_state: &Vec<Vec<char>>) -> String {
    crate_state
        .iter()
        .filter_map(|stack| stack.last().cloned())
        .collect()
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
