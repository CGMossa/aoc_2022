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

#[derive(Debug)]
enum Rope {
    Head,
    Tail,
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
}

fn one_star(input: &str) -> usize {
    let instructions = input
        .lines()
        .map(|x| {
            let (direction, steps) = x.split(' ').collect_tuple().unwrap();
            let steps = steps.parse().unwrap();
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

    let start = (0, 0);
    let mut head: Position = start.into();
    let mut tail: Position = start.into();
    let mut visited: HashSet<(isize, isize)> = Default::default();

    println!("== Initial State ==");
    display_state(5, 6, start.into(), head.clone(), tail.clone());
    println!();
    for inst in instructions {
        let (direction, steps) = inst;
        println!("== {direction} {steps} ==");
        head.apply_instruction_once(direction.clone());
        display_state(5, 6, start.into(), head.clone(), tail.clone());
        println!();
        for step in 0..steps {
            if step == steps - 1 {
                continue;
            }
            head.apply_instruction_once(direction.clone());
            let angle = f64::atan2(
                (head.inner.1 - tail.inner.1) as _,
                (head.inner.0 - tail.inner.0) as _,
            );
            println!("angle = {:?}", angle);
            display_state(5, 6, start.into(), head.clone(), tail.clone());
            let delta_x = angle.cos();
            let delta_y = angle.sin();
            let delta = (
                if delta_x.abs() <= 0.1 {
                    0
                } else {
                    delta_x.signum() as _
                },
                if delta_y.abs() <= 0.1 {
                    0
                } else {
                    delta_y.signum() as _
                },
            );
            println!("delta = {:?}", delta);
            assert!(!((delta.0 == 0) && (delta.1 == 0)));
            tail.inner.0 += delta.0;
            tail.inner.1 += delta.1;

            println!();
            visited.insert(tail.inner);
        }
    }

    visited.len()
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
