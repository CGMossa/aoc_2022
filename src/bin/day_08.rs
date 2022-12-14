use std::{
    collections::{HashMap, HashSet},
    iter::repeat,
};

use itertools::Itertools;

fn main() {
    assert_eq!(
        one_star(
            "30373
25512
65332
33549
35390"
        ),
        21
    );
    let input = include_str!("../../input/day_08.txt");
    println!("Answer: {}", one_star(input));
    //     println!(
    //         "Answer (example): {}",
    //         two_star(
    //             "30373
    // 25512
    // 65332
    // 33549
    // 35390"
    //         )
    //     );
    println!("Answer: {}", two_star(input));
}

// TODO: The one star solution is not as efficient as it could be.
// It should be possible to do a single pass in the directions available, to
// figure out which trees are visible wrt. to these directions.

fn one_star(input: &str) -> usize {
    let height_map: HashMap<(usize, usize), _> = input
        .lines()
        .enumerate()
        // beware: this assumes input would be numbers as ascii
        .flat_map(|x| repeat(x.0).zip(x.1.bytes().map(|x| x - b'0').enumerate()))
        .map(|x| -> ((usize, usize), u8) { ((x.0, x.1 .0), (x.1).1) })
        .collect();

    let max_elements = height_map
        .keys()
        .fold((0, 0), |acc, x| (acc.0.max(x.0), acc.1.max(x.1)));
    let (rows, cols) = max_elements;
    let (rows, cols) = (rows + 1, cols + 1);

    // determine if each cell is visible
    let visible: HashSet<(usize, usize)> = (0..rows)
        .cartesian_product(0..cols)
        .flat_map(|id| {
            let xs = repeat(id.0);
            let ys = repeat(id.1);
            let directions = vec![
                (0..id.0).into_iter().rev().zip(ys.clone()).collect_vec(), // up
                xs.clone().zip((0..id.1).into_iter().rev()).collect_vec(), // left
                (id.0 + 1..cols).into_iter().zip(ys).collect_vec(),        // down
                xs.zip((id.1 + 1..rows).into_iter()).collect_vec(),        // right
            ];

            let value = height_map[&id];
            directions
                .into_iter()
                .filter_map(|direction| {
                    // an empty direction is true
                    let visible = direction.iter().all(|x| {
                        let x_value: u8 = height_map[x];
                        value > x_value
                    });
                    visible.then_some(id)
                })
                .next()
        })
        .collect();

    visible.len()
}

fn two_star(input: &str) -> usize {
    let height_map: HashMap<(usize, usize), _> = input
        .lines()
        .enumerate()
        // beware: this assumes input would be numbers as ascii
        .flat_map(|x| repeat(x.0).zip(x.1.bytes().map(|x| x - b'0').enumerate()))
        .map(|x| -> ((usize, usize), u8) { ((x.0, x.1 .0), (x.1).1) })
        .collect();

    let max_elements = height_map
        .keys()
        .fold((0, 0), |acc, x| (acc.0.max(x.0), acc.1.max(x.1)));
    let (rows, cols) = max_elements;
    let (rows, cols) = (rows + 1, cols + 1);

    // determine if each cell is visible
    (0..rows)
        .cartesian_product(0..cols)
        .map(|id| {
            let xs = repeat(id.0);
            let ys = repeat(id.1);
            let directions = vec![
                (0..id.0).into_iter().rev().zip(ys.clone()).collect_vec(), // up
                xs.clone().zip((0..id.1).into_iter().rev()).collect_vec(), // left
                (id.0 + 1..cols).into_iter().zip(ys).collect_vec(),        // down
                xs.zip((id.1 + 1..rows).into_iter()).collect_vec(),        // right
            ];

            let value = height_map[&id];
            directions
                .into_iter()
                // count visible trees in the different directions
                .map(|x| {
                    let mut acc = 0;
                    for x in x {
                        let new_value = height_map[&x];
                        acc += 1;
                        if value <= new_value {
                            break;
                        }
                    }
                    acc
                })
                .product()
        })
        .max()
        .unwrap()
}
