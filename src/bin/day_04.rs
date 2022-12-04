use itertools::Itertools;

fn main() {
    one_star();
    two_star();
}

fn two_star() {
    let input = include_str!("../../input/day04.txt");
    let answer = input
        .lines()
        .map(|xy| -> (u32, u32, u32, u32) {
            xy.split(',')
                .map(|xy| xy.split('-').map(|x| x.parse::<u32>().unwrap()))
                .flatten()
                .collect_tuple()
                .unwrap()
        })
        // .inspect(|xy| println!("{:?}", xy))
        .fold(0, |acc, abxy| {
            let (a, b, x, y) = abxy;
            /* a-b <= x-y */
            /* x-y <= a-b */
            // if (x <= a && b <= y) || (a <= x && y <= b)
            if ((x <= a) & (a <= y))
                | ((x <= b) & (b <= y))
                | ((a <= x) & (x <= b))
                | ((a <= y) & (y <= b))
            {
                acc + 1
            } else {
                acc
            }
        });
    // .collect_vec();
    println!("Answer: {}", answer);
}

fn one_star() {
    let input = include_str!("../../input/day04.txt");
    let answer = input
        .lines()
        .map(|xy| -> (u32, u32, u32, u32) {
            xy.split(',')
                .map(|xy| xy.split('-').map(|x| x.parse::<u32>().unwrap()))
                .flatten()
                .collect_tuple()
                .unwrap()
        })
        // .inspect(|xy| println!("{:?}", xy))
        .fold(0, |acc, abxy| {
            let (a, b, x, y) = abxy;
            /* a-b <= x-y */
            /* x-y <= a-b */
            if (x <= a && b <= y) || (a <= x && y <= b) {
                acc + 1
            } else {
                acc
            }
        });
    // .collect_vec();
    println!("Answer: {}", answer);
}
