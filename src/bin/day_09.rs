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
    #[display(fmt = "→")]
    Right,
    #[display(fmt = "↗")]
    UpRight,
    #[display(fmt = "↑")]
    Up,
    #[display(fmt = "↖")]
    UpLeft,
    #[display(fmt = "←")]
    Left,
    #[display(fmt = "↙")]
    DownLeft,
    #[display(fmt = "↓")]
    Down,
    #[display(fmt = "↘")]
    DownRight,
}

#[derive(Debug, Clone, Copy, derive_more::Display)]
#[display(fmt = "{:?}", "inner")]
struct Position {
    inner: (isize, isize),
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self { inner: (x, y) }
    }

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
}

fn one_star(input: &str) -> usize {
    let instructions = input
        .lines()
        .map(|x| {
            let (direction, steps) = x.split_once(' ').unwrap();
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

    let start = Position::new(0, 0);
    let mut head: Position = start;
    let mut tail: Position = start;
    let mut visited: HashSet<(isize, isize)> = Default::default();
    visited.insert(tail.inner);

    // println!("== Initial State ==");
    // display_state(5, 6, start.clone(), head.clone(), tail.clone());
    // println!();

    for (direction, steps) in instructions {
        // println!("== {direction} {steps} ==");
        for _step in 0..steps {
            head.apply_instruction_once(direction.clone());
            let is_adjacent = [-1, 0, 1]
                .iter()
                .cartesian_product([-1, 0, 1].iter())
                .any(|x| {
                    (tail.inner.0 + x.0 == head.inner.0) && (tail.inner.1 + x.1 == head.inner.1)
                });
            if is_adjacent {
                // // println!("== {direction} {steps} ==");
                // println!("skipping");
                // display_state(5, 6, start.clone(), head.clone(), tail.clone());
                // println!();
                continue;
            }
            let direct = (head.inner.0 - tail.inner.0, head.inner.1 - tail.inner.1);
            let delta = (direct.0.signum(), direct.1.signum());
            tail.inner.0 += delta.0;
            tail.inner.1 += delta.1;
            visited.insert(tail.inner);

            // check if the head and tail are adjacent after the update
            assert!([-1, 0, 1]
                .iter()
                .cartesian_product([-1, 0, 1].iter())
                .any(|x| {
                    (tail.inner.0 + x.0 == head.inner.0) && (tail.inner.1 + x.1 == head.inner.1)
                }));
        }
        // // println!("== {direction} {steps} ==");
        // display_state(5, 6, start.clone(), head.clone(), tail.clone());
        // println!();
    }

    // display_visited(5, 6, start, visited.iter().collect_vec().as_slice());

    // dbg!(
    //     visited.iter().max_by_key(|x| -x.0),
    //     visited.iter().max_by_key(|x| x.0),
    //     visited.iter().max_by_key(|x| -x.1),
    //     visited.iter().max_by_key(|x| x.1)
    // );

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
    dbg!(rows * cols);
    let position_to_index = |position: Position| {
        let (col, row) = (position.inner.0 as usize, position.inner.1 as usize);
        dbg!(col, row);
        dbg!((rows - 1 - row) * cols + col);
        (rows - 1 - row) * cols + col
    };
    println!("head: {}, tail: {}", head.clone(), tail.clone());
    state[position_to_index(start)] = 's';
    state[position_to_index(tail)] = 'T';
    state[position_to_index(head)] = 'H';
    let state_display: String = state.chunks(cols).map(|x| x.iter().join("")).join("\n");
    println!("{:}", state_display);
}
