use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    one_star();
    two_star();
}

fn two_star() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw";
    let input = include_str!("../../input/day03.txt");
    let inventory = input
        .lines()
        // remove whitespace from the lines
        .map(|x| x.chars().filter(|x| !x.is_whitespace()))
        .map(|x| x.collect::<Vec<_>>())
        // groups
        .collect_vec();
    let priority = inventory
        .chunks(3)
        .flat_map(|chunk| {
            let (x, y, z) = chunk.into_iter().collect_tuple().unwrap();
            x.into_iter()
                .filter(|x| y.contains(x) && z.contains(x))
                .next()
                .copied()
        })
        .map(|x| x as u8)
        .map(|x| x - if x >= 97 { 96 } else { 38 })
        .map(|x| -> u32 { x.try_into().unwrap() })
        // .inspect(|x| println!("{}", x))
        ;
    dbg!(priority.sum::<u32>());
}

fn one_star() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw";
    let input = include_str!("../../input/day03.txt");
    let rucksacks = input
        .lines()
        .map(|x| x.chars().filter(|x| !x.is_whitespace()).collect_vec())
        .flat_map(|rucksack| {
            let (first, second) = rucksack.split_at(rucksack.len() / 2);
            first
                .iter()
                .filter(|x| second.contains(x))
                .copied()
                .next()
        });
    let priority = rucksacks
        // .inspect(|x| println!("{}", x))
        // lower: 97..123 -> 1..27,  x |-> x - 96
        // upper: 65..91  -> 27..53, x |-> x - 38
        //
        .map(|x| x as u8)
        .map(|x| x - if x >= 97 { 96 } else { 38 })
        .map(|x| -> u32 { x.try_into().unwrap() })
        // .inspect(|x| println!("{}", x));
        ;
    dbg!(priority.sum::<u32>());
    // dbg!(rucksacks.clone().collect::<String>());
    // dbg!((priority).collect::<Vec<_>>());
    // .inspect(|x| println!("{}", x)).collect_vec();
}
