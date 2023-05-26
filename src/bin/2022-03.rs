use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::{env, println};

const INPUT_FILENAME: &str = "2022-03-input.txt";

struct PrioritySum {
    priority_by_compartment: i32,
    // priority_by_elf_group: i32,
}

fn logic(buffered: BufReader<File>) {
    let sum = buffered
        .lines()
        .filter_map(|unparsed_line| unparsed_line.ok())
        .enumerate()
        .fold(
            PrioritySum {
                priority_by_compartment: 0,
                // priority_by_elf_group: 0,
            },
            |mut acc, (_index, current_line)| {
                let (compartment_one, compartment_two) =
                    current_line.split_at(current_line.len() / 2);
                acc.priority_by_compartment +=
                    map_char_to_value(find_common_elements(&[compartment_one, compartment_two])[0]);
                acc
            },
        );

    assert_eq!(sum.priority_by_compartment, 7553);
    // assert_eq!(sum.priority_by_elf_group, 2758);

    println!(
        "Part 1 | Sum of priorities between rucksack compartments: {}",
        sum.priority_by_compartment
    );
    // println!(
    //     "Part 2 | Sum of priorities between elf rucksack groups of 3: {}",
    //     sum.priority_by_elf_group
    // );
}

fn map_char_to_value(c: char) -> i32 {
    let code = c as u32;
    if code >= 'a' as u32 && code <= 'z' as u32 {
        (code - 96) as i32
    } else if code >= 'A' as u32 && code <= 'Z' as u32 {
        (code - 38) as i32
    } else {
        0
    }
}

fn find_common_elements(strings: &[&str]) -> Vec<char> {
    if strings.is_empty() {
        return Vec::new();
    }
    let mut common_elements: HashSet<char> = strings[0].chars().collect();
    for i in 1..strings.len() {
        common_elements = common_elements
            .intersection(&strings[i].chars().collect::<HashSet<char>>())
            .cloned()
            .collect();
    }
    common_elements.into_iter().collect()
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
