use std::iter::once;

use itertools::Itertools;

fn main() {
    let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    // println!("Answer: {}", one_star(input));
    assert_eq!(one_star(input), 13140);
    let input = include_str!("../../input/day_10.txt");
    println!("Answer: {}", one_star(input));
}

fn one_star(input: &str) -> isize {
    let cycles = input.lines().flat_map(|x| {
        let x = x.split(" ").collect_vec();
        match x.as_slice() {
            ["addx", v] => {
                let v: isize = v.parse().unwrap();
                vec![None, Some(v)]
            }
            ["noop"] => {
                vec![None]
            }
            _ => panic!("failed to process instructions"),
        }
    });
    cycles
        .scan((1, 1), |state, val| {
            if let Some(val) = val {
                *state = (state.0 + 1, state.1 + val);
            } else {
                *state = (state.0 + 1, state.1);
            }
            Some(*state)
        })
        .filter(|(i, _v)| [20, 60, 100, 140, 180, 220].contains(i))
        .inspect(|x| println!("{x:?} => {}", x.0 * x.1))
        .map(|(i, v)| i * v)
        // .inspect(|x| println!("{x:?}"))
        .sum()
}
