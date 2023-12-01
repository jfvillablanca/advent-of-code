use core::panic;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::{dbg, env, fmt, println};

const INPUT_FILENAME: &str = "2022-09-sample.txt";

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
struct Knot {
    x: i32,
    y: i32,
}

impl fmt::Display for Knot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

impl Knot {
    // fn calc_next_tail(&self, current_head: &Knot, direction: &Direction) -> Knot {
    //     let next_head = &current_head.calc_next_head(direction);
    //     match self.calc_relative_direction(next_head) {
    //         HeadRelativeToTail::Top => Knot {
    //             x: next_head.x,
    //             y: next_head.y - 1,
    //         },
    //         HeadRelativeToTail::Left => Knot {
    //             x: next_head.x + 1,
    //             y: next_head.y,
    //         },
    //         HeadRelativeToTail::Right => Knot {
    //             x: next_head.x - 1,
    //             y: next_head.y,
    //         },
    //         HeadRelativeToTail::Bottom => Knot {
    //             x: next_head.x,
    //             y: next_head.y + 1,
    //         },
    //         HeadRelativeToTail::DiagonalOrOverlap => {
    //             if self.calc_manhattan_distance(next_head) <= 2 {
    //                 Knot {
    //                     x: self.x,
    //                     y: self.y,
    //                 }
    //             } else {
    //                 Knot {
    //                     x: current_head.x,
    //                     y: current_head.y,
    //                 }
    //             }
    //         }
    //     }
    // }

    fn calc_manhattan_distance(&self, section: &Knot) -> i32 {
        (self.x - section.x).abs() + (self.y - section.y).abs()
    }

    fn calc_relative_direction(&self, head: &Knot) -> HeadRelativeToTail {
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

    fn calc_next_head(&self, current_tail: &Knot, direction: &Direction) -> Knot {
        let future_self = match direction {
            Direction::Up(_) => Knot {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left(_) => Knot {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Down(_) => Knot {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Right(_) => Knot {
                x: self.x + 1,
                y: self.y,
            },
        };

        let next_head = current_tail.clone();

        match current_tail.calc_relative_direction(&future_self) {
            HeadRelativeToTail::Top => Knot {
                x: next_head.x,
                y: next_head.y - 1,
            },
            HeadRelativeToTail::Left => Knot {
                x: next_head.x + 1,
                y: next_head.y,
            },
            HeadRelativeToTail::Right => Knot {
                x: next_head.x - 1,
                y: next_head.y,
            },
            HeadRelativeToTail::Bottom => Knot {
                x: next_head.x,
                y: next_head.y + 1,
            },
            HeadRelativeToTail::DiagonalOrOverlap => {
                if current_tail.calc_manhattan_distance(&future_self) <= 2 {
                    Knot {
                        x: current_tail.x,
                        y: current_tail.y,
                    }
                } else {
                    Knot {
                        x: self.x,
                        y: self.y,
                    }
                }
            }
        }
    }
}

fn logic(buffered: BufReader<File>) {
    let two_knot_rope = vec![Knot { x: 0, y: 0 }; 2];

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

    let cells_visited: HashSet<Knot> = vector_arr
        .iter()
        .flat_map(|direction| match direction {
            Direction::Up(steps) => std::iter::repeat(Direction::Up(1)).take(*steps as usize),
            Direction::Left(steps) => std::iter::repeat(Direction::Left(1)).take(*steps as usize),
            Direction::Down(steps) => std::iter::repeat(Direction::Down(1)).take(*steps as usize),
            Direction::Right(steps) => std::iter::repeat(Direction::Right(1)).take(*steps as usize),
        })
        .fold((two_knot_rope, HashSet::new()), |acc, direction| {
            let (rope_vec, mut cells_visited) = acc;
            let mut rope_vec_iter = rope_vec.iter().cloned();
            let initial_head = rope_vec_iter.next().unwrap();

            let new_rope_vec =
                rope_vec_iter.fold(vec![initial_head], |mut accumulator, current_tail| {
                    let current_head = accumulator.last().unwrap();
                    let next_head = current_head.calc_next_head(&current_tail, &direction);
                    // let next_tail = current_tail.calc_next_tail(&current_head, &direction);
                    // dbg!(&current_tail, &current_head, &next_head);
                    // println!(
                    //     "cur_t: {:?} cur_h: {:?} n_h: {:?} n_t: {:?}",
                    //     &current_tail, &current_head, &next_head, &next_tail
                    // );
                    // println!("cur_h: {:?}, n_t: {:?}", &current_head, &next_tail);

                    // accumulator.push(next_tail.clone());
                    accumulator.push(next_head.clone());
                    accumulator
                });
            dbg!(&new_rope_vec);

            cells_visited.insert(new_rope_vec.last().unwrap().clone());
            (new_rope_vec, cells_visited)
        })
        .1
        .into_iter()
        .collect();

    println!(
        "Part 1 | cells visited by the tail of a 2-knot rope: {}",
        cells_visited.len()
    );

    // assert_eq!(cells_visited.len(), 5683);
    assert_eq!(cells_visited.len(), 36);
}

// fn process(current_section: &RopeSection, direction: &Direction) -> RopeSection {
//     let RopeSection {
//         head: current_head,
//         tail: current_tail,
//     } = current_section;

//     let next_head = current_head.calc_next_coordinates(&direction);
//     let next_tail = current_tail.calc_tail_next(&current_head, &next_head);

//     RopeSection {
//         head: next_head,
//         tail: next_tail,
//     }
// }

fn main() -> Result<(), Error> {
    let current_dir = env::current_dir()?;
    let shared_path = current_dir.join("../shared");
    let input_path = shared_path.join(INPUT_FILENAME);
    let input = File::open(input_path)?;
    let buffered = BufReader::new(input);
    logic(buffered);

    Ok(())
}
