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
}

fn one_star(input: &str) -> usize {
    let height_map: HashMap<(usize, usize), _> = input
        .lines()
        .enumerate()
        .flat_map(|x|
            // beware: this assumes input would be numbers as ascii
            repeat(x.0).zip(x.1.bytes().map(|x| x - b'0').enumerate()))
        .map(|x| -> ((usize, usize), u8) { ((x.0, x.1 .0), (x.1).1) })
        .collect();

    let max_elements = height_map
        .keys()
        .fold((0, 0), |acc, x| (acc.0.max(x.0), acc.1.max(x.1)));
    let (rows, cols) = max_elements;
    let (rows, cols) = (rows + 1, cols + 1);

    let mut visible: HashSet<(usize, usize)> = Default::default();

    // determine if each cell is visible
    for id in (0..rows).cartesian_product(0..cols) {
        let xs = repeat(id.0);
        let ys = repeat(id.1);
        let directions = vec![
            (0..id.0).into_iter().zip(ys.clone()).collect_vec(), // up
            (id.0 + 1..cols).into_iter().zip(ys).collect_vec(),  // down
            xs.clone().zip((0..id.1).into_iter()).collect_vec(), // left
            xs.zip((id.1 + 1..rows).into_iter()).collect_vec(),  // right
        ];

        let value = height_map[&id];
        'dir: for direction in directions {
            let is_visible =
            //an empty direction is true
            direction.into_iter().all(|x| {
                    let x_value = height_map[&x];
                    value > x_value
                });
            if is_visible {
                // dbg!(id, value);
                visible.insert(id);
                break 'dir;
            }
        }
    }

    visible.len()
}
