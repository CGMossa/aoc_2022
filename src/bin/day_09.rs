use std::collections::HashSet;
#[allow(unused_imports)]
use std::f64::consts::PI;

use itertools::Itertools;

fn main() {
    assert_eq!(
        one_star(
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
        ),
        13
    );
    let input = include_str!("../../input/day_09.txt");
    println!("Answer: {}", one_star(input));
}

#[derive(Debug, Clone, derive_more::Display)]
enum Direction {
    #[display(fmt = "R")]
    Right,
    #[display(fmt = "UR")]
    UpRight,
    #[display(fmt = "U")]
    Up,
    #[display(fmt = "UL")]
    UpLeft,
    #[display(fmt = "L")]
    Left,
    #[display(fmt = "DL")]
    DownLeft,
    #[display(fmt = "D")]
    Down,
    #[display(fmt = "DR")]
    DownRight,
}

#[derive(Debug, Clone, derive_more::From, derive_more::Display)]
#[display(fmt = "{:?}", "inner")]
struct Position {
    inner: (isize, isize),
}

impl Position {
    fn apply_instruction_once(&mut self, direction: Direction) {
        match direction {
            Direction::Right => {
                // right
                self.inner.0 += 1;
            }
            Direction::UpRight => {
                // up-right
                self.inner.0 += 1;
                self.inner.1 += 1;
            }
            Direction::Up => {
                // up
                self.inner.1 += 1;
            }
            Direction::UpLeft => {
                // up-left
                self.inner.0 -= 1;
                self.inner.1 += 1;
            }
            Direction::Left => {
                // left
                self.inner.0 -= 1;
            }
            Direction::DownLeft => {
                // down-left
                self.inner.0 -= 1;
                self.inner.1 -= 1;
            }
            Direction::Down => {
                // down
                self.inner.1 -= 1;
            }
            Direction::DownRight => {
                // down-right
                self.inner.0 += 1;
                self.inner.1 -= 1;
            }
        }
    }
    fn apply_instruction(&mut self, direction: Direction, steps: isize) {
        match direction {
            Direction::Right => {
                // right
                self.inner.0 += steps;
            }
            Direction::UpRight => {
                // up-right
                self.inner.0 += steps;
                self.inner.1 += steps;
            }
            Direction::Up => {
                // up
                self.inner.1 += steps;
            }
            Direction::UpLeft => {
                // up-left
                self.inner.0 -= steps;
                self.inner.1 += steps;
            }
            Direction::Left => {
                // left
                self.inner.0 -= steps;
            }
            Direction::DownLeft => {
                // down-left
                self.inner.0 -= steps;
                self.inner.1 -= steps;
            }
            Direction::Down => {
                // down
                self.inner.1 -= steps;
            }
            Direction::DownRight => {
                // down-right
                self.inner.0 += steps;
                self.inner.1 -= steps;
            }
        }
    }
}

fn one_star(input: &str) -> usize {
    let instructions = input
        .lines()
        .map(|x| {
            let (direction, steps) = x.split(' ').collect_tuple().unwrap();
            let steps: isize = steps.parse().unwrap();
            let direction: Direction = match direction {
                "U" => Direction::Up,
                "L" => Direction::Left,
                "D" => Direction::Down,
                "R" => Direction::Right,
                _ => panic!("provided instruction `{}` doesn't exist", direction),
            };
            (direction, steps)
        })
        .collect_vec();
    // There are parallel directions used..

    let start: Position = (0, 0).into();
    let mut head: Position = start.clone().into();
    let mut tail: Position = start.clone().into();
    let mut visited: HashSet<(isize, isize)> = Default::default();
    visited.insert(tail.inner);

    // println!("== Initial State ==");
    // display_state(5, 6, start.into(), head.clone(), tail.clone());
    // println!();

    let mut previous_direction = instructions[0].0.clone();
    for inst in instructions {
        let (direction, steps) = inst;

        head.apply_instruction(direction.clone(), steps);
        // assert_ne!(steps - 1, 0);
        let is_close = [-1, 0, 1]
            .iter()
            .cartesian_product([-1, 0, 1].iter())
            .any(|x| (tail.inner.0 + x.0 == head.inner.0) && (tail.inner.1 + x.1 == head.inner.1));
        if is_close {
            // println!("== {direction} {steps} ==");
            // display_state(5, 6, start.into(), head.clone(), tail.clone());
            // println!();

            previous_direction = direction.clone();
            continue;
        }
        use Direction::*;
        let diagonal = match (previous_direction, direction.clone()) {
            (Up, Right) | (Right, Up) => Some(UpRight),
            (Down, Left) | (Left, Down) => Some(DownLeft),
            (Up, Left) | (Left, Up) => Some(UpLeft),
            (Down, Right) | (Right, Down) => Some(DownRight),
            _ => None,
        };
        if let Some(diagonal_direction) = diagonal {
            // println!("diagonal: {}", diagonal_direction);
            tail.apply_instruction_once(diagonal_direction);
            visited.insert(tail.inner);
        }
        let steps = match direction {
            Up | Down => (head.inner.1 - tail.inner.1).abs(),
            Left | Right => (head.inner.0 - tail.inner.0).abs(),
            _ => unreachable!(),
        };
        // tail.apply_instruction(direction.clone(), steps - 1);
        for _step in 0..steps - 1 {
            tail.apply_instruction_once(direction.clone());
            visited.insert(tail.inner);
        }

        // println!("== {direction} {steps} ==");
        // display_state(5, 6, start.into(), head.clone(), tail.clone());
        // println!();

        previous_direction = direction.clone();
    }

    // display_visited(5, 6, start, visited.iter().collect_vec().as_slice());

    visited.len()
}

fn display_visited(rows: usize, cols: usize, start: Position, visited: &[&(isize, isize)]) {
    let mut state = vec!['.'; rows * cols];
    let position_to_index = |position: &(isize, isize)| {
        let (col, row) = (position.0 as usize, position.1 as usize);
        (rows - 1 - row) * cols + col
    };
    for cell in visited {
        state[position_to_index(cell)] = '#';
    }
    state[position_to_index(&start.inner)] = 's';
    let state_display: String = state.chunks(cols).map(|x| x.iter().join("")).join("\n");
    println!("{:}", state_display);
}

fn display_state(rows: usize, cols: usize, start: Position, head: Position, tail: Position) {
    let mut state = vec!['.'; rows * cols];
    let position_to_index = |position: Position| {
        let (col, row) = (position.inner.0 as usize, position.inner.1 as usize);
        (rows - 1 - row) * cols + col
    };
    println!("head: {}, tail: {}", head.clone(), tail.clone());
    state[position_to_index(start)] = 's';
    state[position_to_index(tail)] = 'T';
    state[position_to_index(head)] = 'H';
    let state_display: String = state.chunks(cols).map(|x| x.iter().join("")).join("\n");
    println!("{:}", state_display);
}
