use itertools::Itertools;

fn main() {
    assert_eq!(one_star("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    assert_eq!(one_star("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(one_star("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(one_star("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(one_star("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    let input = include_str!("../../input/day_06.txt");
    println!("Answer: {}", one_star(input));

    assert_eq!(two_star("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    assert_eq!(two_star("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    assert_eq!(two_star("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    assert_eq!(two_star("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    assert_eq!(two_star("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    println!("Answer: {}", two_star(input));
}

fn one_star(input: &str) -> usize {
    4 + input
        .bytes()
        .tuple_windows()
        .find_position(|(a, b, c, d)| {
            !([b, c, d].contains(&a)
                || [a, c, d].contains(&b)
                || [a, b, d].contains(&c)
                || [a, b, c].contains(&d))
        })
        .unwrap()
        .0
}

fn two_star(input: &str) -> usize {
    14 + input
        .as_bytes()
        .windows(14)
        .find_position(|x| {
            for id in 0..x.len() {
                if x[id + 1..].contains(&x[id]) {
                    return false;
                }
            }
            true
        })
        .unwrap()
        .0
}
