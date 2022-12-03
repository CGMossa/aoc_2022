use std::collections::HashSet;

use itertools::Itertools;

fn main() {
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
                //FIXME: find another way to remove duplicates
                .collect::<HashSet<_>>()
        });
    let priority = rucksacks
        // .inspect(|x| println!("{}", x))
        // 65..91 for upper letters and 97..123 for lower letters
        // ~>
        // 1..27  for lower letters and 27..53 for upper letters
        //
        // lower: 97..123 -> 1..27  x |-> x - 96
        // upper: 65..91  -> 27..53  x |-> x - 38
        //
        .map(|x| x as u8)
        .map(|x| x - if x >= 97 { 96 } else { 38 })
        .map(|x| -> u32 { x.try_into().unwrap() })
        .inspect(|x| println!("{}", x));
    // dbg!(rucksacks.clone().collect::<String>());
    // dbg!((priority).collect::<Vec<_>>());
    dbg!(priority.sum::<u32>());
    // .inspect(|x| println!("{}", x)).collect_vec();
}
