use core::panic;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::{env, fmt, println};

const INPUT_FILENAME: &str = "2022-09-input.txt";

#[derive(Debug, Clone)]
enum Direction {
    Up(i32),
    Left(i32),
    Down(i32),
    Right(i32),
}

#[derive(Debug)]
enum HeadRelativeToTail {
    Top,
    Left,
    Bottom,
    Right,
    DiagonalOrOverlap,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct RopeSection {
    x: i32,
    y: i32,
}

impl fmt::Display for RopeSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

impl RopeSection {
    fn calc_tail_next(&self, current_head: &RopeSection, next_head: &RopeSection) -> RopeSection {
        match self.calc_relative_direction(next_head) {
            HeadRelativeToTail::Top => RopeSection {
                x: next_head.x,
                y: next_head.y - 1,
            },
            HeadRelativeToTail::Left => RopeSection {
                x: next_head.x + 1,
                y: next_head.y,
            },
            HeadRelativeToTail::Right => RopeSection {
                x: next_head.x - 1,
                y: next_head.y,
            },
            HeadRelativeToTail::Bottom => RopeSection {
                x: next_head.x,
                y: next_head.y + 1,
            },
            HeadRelativeToTail::DiagonalOrOverlap => {
                if self.calc_manhattan_distance(next_head) <= 2 {
                    RopeSection {
                        x: self.x,
                        y: self.y,
                    }
                } else {
                    RopeSection {
                        x: current_head.x,
                        y: current_head.y,
                    }
                }
            }
        }
    }

    fn calc_manhattan_distance(&self, section: &RopeSection) -> i32 {
        (self.x - section.x).abs() + (self.y - section.y).abs()
    }

    fn calc_relative_direction(&self, head: &RopeSection) -> HeadRelativeToTail {
        let delta_y = head.y - self.y;
        let delta_x = head.x - self.x;

        if delta_y > 0 && delta_x == 0 {
            HeadRelativeToTail::Top
        } else if delta_y == 0 && delta_x < 0 {
            HeadRelativeToTail::Left
        } else if delta_y == 0 && delta_x > 0 {
            HeadRelativeToTail::Right
        } else if delta_y < 0 && delta_x == 0 {
            HeadRelativeToTail::Bottom
        } else {
            HeadRelativeToTail::DiagonalOrOverlap
        }
    }

    fn calc_next_coordinates(&self, direction: &Direction) -> RopeSection {
        match direction {
            Direction::Up(dist) => RopeSection {
                x: self.x,
                y: self.y + dist,
            },
            Direction::Left(dist) => RopeSection {
                x: self.x - dist,
                y: self.y,
            },
            Direction::Down(dist) => RopeSection {
                x: self.x,
                y: self.y - dist,
            },
            Direction::Right(dist) => RopeSection {
                x: self.x + dist,
                y: self.y,
            },
        }
    }
}

fn logic(buffered: BufReader<File>) {
    let initial_head = RopeSection { x: 0, y: 0 };
    let initial_tail = RopeSection { x: 0, y: 0 };

    let vector_arr: Vec<Direction> = buffered
        .lines()
        .filter_map(|unparsed_line| unparsed_line.ok())
        .map(|x| {
            let mut line = x.split_whitespace();
            let direction_string = line.next().unwrap_or("");
            let distance = line.next().unwrap_or("").parse::<i32>().unwrap_or_default();
            match direction_string {
                "U" => Direction::Up(distance),
                "L" => Direction::Left(distance),
                "D" => Direction::Down(distance),
                "R" => Direction::Right(distance),
                _ => panic!("Parse error: make sure input file follows the puzzle format"),
            }
        })
        .collect();

    let cells_visited: HashSet<RopeSection> = vector_arr
        .iter()
        .flat_map(|direction| match direction {
            Direction::Up(steps) => std::iter::repeat(Direction::Up(1)).take(*steps as usize),
            Direction::Left(steps) => std::iter::repeat(Direction::Left(1)).take(*steps as usize),
            Direction::Down(steps) => std::iter::repeat(Direction::Down(1)).take(*steps as usize),
            Direction::Right(steps) => std::iter::repeat(Direction::Right(1)).take(*steps as usize),
        })
        .fold(
            (initial_head, initial_tail, HashSet::new()),
            |acc, direction| {
                let (current_head, current_tail, mut cells_visited) = acc;
                let next_head = current_head.calc_next_coordinates(&direction);
                let next_tail = current_tail.calc_tail_next(&current_head, &next_head);
                cells_visited.insert(next_tail.clone());
                (next_head, next_tail, cells_visited)
            },
        )
        .2
        .into_iter()
        .collect();

    println!(
        "Part 1 | cells visited by the tail of a 2-knot rope: {}",
        cells_visited.len()
    );

    assert_eq!(cells_visited.len(), 5683);
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
