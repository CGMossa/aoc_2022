#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_star() {
        let input = include_str!("../input/day01.txt");

        let mut current = 0;
        //FIXME: actually maximum should not be zero.. but it works anyways..
        let mut maximum: u32 = 0;
        input.lines().for_each(|x| {
            match x {
                "" => {
                    // next inventory
                    assert_ne!(current, 0, "current should not be zero ever");
                    maximum = maximum.max(current);
                    current = 0;
                }
                x => {
                    let x: u32 = x.parse().unwrap();
                    current += x;
                }
            }
        });
        println!("Maximum: {}", maximum);
    }
}
