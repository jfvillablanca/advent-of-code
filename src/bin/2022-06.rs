use std::collections::HashSet;
use std::fs::File;
use std::hash;
use std::io::{BufRead, BufReader, Error};
use std::{env, println};

const INPUT_FILENAME: &str = "2022-06-input.txt";

fn logic(buffered: BufReader<File>) {
    let n_chars_to_start_marker = if let Some(Ok(line)) = buffered.lines().next() {
        let stream: Vec<(usize, char)> = line.chars().enumerate().collect();
        (
            get_start_index(&stream, 4).unwrap(),
            get_start_index(&stream, 14).unwrap(),
        )
    } else {
        (0, 0)
    };

    println!(
        "Part 1 | Number of chars until 4 distinct char slice: {}",
        n_chars_to_start_marker.0
    );
    println!(
        "Part 2 | Number of chars until 14 distinct char slice: {}",
        n_chars_to_start_marker.1
    );

    assert_eq!(n_chars_to_start_marker.0, 1544);
    assert_eq!(n_chars_to_start_marker.1, 2145);
}

fn get_start_index(stream: &Vec<(usize, char)>, marker_size: usize) -> Option<usize> {
    for (index, _) in stream {
        if let Some(slice) = stream.get(*index..index + marker_size) {
            let marker_candidate: Vec<char> = slice.iter().map(|(_, x)| *x).collect();
            if is_all_unique(&marker_candidate) {
                return Some(index + marker_size);
            }
        }
    }
    None
}

fn is_all_unique<T: Eq + hash::Hash>(check_vec: &[T]) -> bool {
    let hashed_vec: HashSet<&T> = check_vec.iter().collect();
    hashed_vec.len() == check_vec.len()
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
