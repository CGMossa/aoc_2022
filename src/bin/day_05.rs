use std::collections::VecDeque;

use itertools::Itertools;

#[derive(Debug)]
struct Stacks {
    inner: Vec<VecDeque<char>>,
}

impl Stacks {
    fn new(stacks: usize) -> Self {
        Self {
            inner: vec![VecDeque::default(); stacks],
        }
    }
}

#[derive(Debug)]
struct MoveInstruction {
    count: usize,
    from: usize,
    to: usize,
}

fn main() {
    one_star();
    two_star();
}

fn one_star() {
    #[allow(unused_variables)]
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    let input = include_str!("../../input/day_05.txt");

    // three spaces for each position, and separated by one space.
    let first_line_length = input.chars().position(|x| x == '\n').unwrap();
    let stacks = first_line_length / 4 + if first_line_length % 4 == 0 { 0 } else { 1 };
    // dbg!(first_line_length, stacks);

    let (initial_state_input, instructions_input) = input.split("\n\n").collect_tuple().unwrap();
    // parse initial state of stacks
    let mut initial_state: Stacks = Stacks::new(stacks);
    initial_state_input.lines().for_each(|line| {
        for (id, c) in line.char_indices() {
            if !c.is_ascii_uppercase() {
                continue;
            }
            let id = id / 4;
            initial_state.inner[id].push_back(c);
        }
    });
    // println!("{:#?}", initial_state);

    // parse instructions
    let instructions = instructions_input.lines().map(|x| {
        let x = x.replace("move ", "");
        let (count, rest) = x.split(" from ").collect_tuple().unwrap();
        let count = count.parse().unwrap();
        let (from, to) = rest.split(" to ").collect_tuple().unwrap();
        let from: usize = from.parse().unwrap();
        let to: usize = to.parse().unwrap();

        let from = from - 1;
        let to = to - 1;

        MoveInstruction { count, from, to }
    });

    let mut current_stacks = initial_state;
    for instr in instructions {
        let MoveInstruction { count, from, to } = instr;
        for _move in 0..count {
            let ele = current_stacks.inner[from]
                .pop_front()
                .expect("no more object in the stack to move");
            current_stacks.inner[to].push_front(ele);
        }
    }
    // println!("{:#?}", current_stacks);
    let answer = current_stacks
        .inner
        .into_iter()
        .flat_map(|mut x| x.pop_front())
        .join("");
    println!("Answer: {}", answer);
}

fn two_star() {
    #[allow(unused_variables)]
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    let input = include_str!("../../input/day_05.txt");

    // three spaces for each position, and separated by one space.
    let first_line_length = input.chars().position(|x| x == '\n').unwrap();
    let stacks = first_line_length / 4 + if first_line_length % 4 == 0 { 0 } else { 1 };
    // dbg!(first_line_length, stacks);

    let (initial_state_input, instructions_input) = input.split("\n\n").collect_tuple().unwrap();
    // parse initial state of stacks
    let mut initial_state: Stacks = Stacks::new(stacks);
    initial_state_input.lines().for_each(|line| {
        for (id, c) in line.char_indices() {
            if !c.is_ascii_uppercase() {
                continue;
            }
            let id = id / 4;
            initial_state.inner[id].push_back(c);
        }
    });
    // println!("Initial state:\n\t{:#?}", initial_state);

    // parse instructions
    let instructions = instructions_input.lines().map(|x| {
        let x = x.replace("move ", "");
        let (count, rest) = x.split(" from ").collect_tuple().unwrap();
        let count = count.parse().unwrap();
        let (from, to) = rest.split(" to ").collect_tuple().unwrap();
        let from: usize = from.parse().unwrap();
        let to: usize = to.parse().unwrap();

        let from = from - 1;
        let to = to - 1;

        MoveInstruction { count, from, to }
    });

    let mut current_stacks = initial_state;
    for instr in instructions {
        let MoveInstruction { count, from, to } = instr;
        let elements = current_stacks.inner[from].drain(0..count).collect_vec();
        elements
            .into_iter()
            .rev()
            .for_each(|x| current_stacks.inner[to].push_front(x));
    }
    // println!("Last state:\n\t{:#?}", current_stacks);
    let answer = current_stacks
        .inner
        .into_iter()
        .flat_map(|mut x| x.pop_front())
        .join("");
    println!("Answer: {}", answer);
}
