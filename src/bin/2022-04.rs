use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::{env, println};

const INPUT_FILENAME: &str = "2022-04-input.txt";

struct OverlapSum {
    strict_subset: i32,
    overlap: i32,
}

fn logic(buffered: BufReader<File>) {
    let sum: OverlapSum = buffered
        .lines()
        .filter_map(|unparsed_number| unparsed_number.ok())
        .fold(
            OverlapSum {
                strict_subset: 0,
                overlap: 0,
            },
            |mut sum, current_line| {
                let (first_elf, second_elf) = splitter(&current_line, ',');
                let (first_elf_1, first_elf_2) = splitter(&first_elf, '-');
                let (first_elf_1, first_elf_2): (i32, i32) =
                    (first_elf_1.parse().unwrap(), first_elf_2.parse().unwrap());
                let (second_elf_1, second_elf_2) = splitter(&second_elf, '-');
                let (second_elf_1, second_elf_2): (i32, i32) =
                    (second_elf_1.parse().unwrap(), second_elf_2.parse().unwrap());

                let is_second_elf_subset =
                    first_elf_1 <= second_elf_1 && first_elf_2 >= second_elf_2;
                let is_first_elf_subset =
                    second_elf_1 <= first_elf_1 && second_elf_2 >= first_elf_2;

                if is_second_elf_subset || is_first_elf_subset {
                    sum.strict_subset += 1;
                };

                let is_not_overlapping_shift =
                    first_elf_2 < second_elf_1 || second_elf_2 < first_elf_1;
                if !is_not_overlapping_shift {
                    sum.overlap += 1;
                }
                sum
            },
        );

    println!(
        "Part 1 | Number of pairs with strict subsets: {}",
        sum.strict_subset
    );
    println!("Part 2 | Number of pairs with overlaps: {}", sum.overlap);

    assert_eq!(sum.strict_subset, 496);
    assert_eq!(sum.overlap, 847);
}

fn splitter(a: &String, delimiter: char) -> (String, String) {
    let mut split_iter = a.split(delimiter);
    let first = split_iter.next().unwrap();
    let second = split_iter.next().unwrap();
    (first.to_string(), second.to_string())
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
