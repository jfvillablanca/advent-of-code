use std::fs::File;
use std::io::{BufReader, Error};
use std::{env, println};

const INPUT_FILENAME: &str = "2022-01-input.txt";

fn logic(buffered: BufReader<File>) {
    println!("{:?}", buffered)
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
