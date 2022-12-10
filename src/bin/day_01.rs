mod solutions {
    pub(crate) fn one_star() {
        let input = include_str!("../../input/day01.txt");

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
        // ANSWER: 71471
    }

    /// Doesn't work as it is maintaining a list of top three throughout, which
    /// doesn't work. You need to sort everybody.
    #[allow(dead_code)]
    pub(crate) fn two_star() {
        let input = include_str!("../../input/day01.txt");
        let mut current = 0;
        //FIXME: actually maximum should not be zero.. but it works anyways..
        let mut maximum = [0_u32; 3];
        input.lines().for_each(|inventory_entry| {
            match inventory_entry {
                "" => {
                    // next inventory
                    assert_ne!(current, 0, "current should not be zero ever");
                    // maximum = maximum.map(|x| x.max(current));
                    for ele in &mut maximum {
                        if current > *ele {
                            *ele = current;
                            break;
                        }
                    }
                    current = 0;
                }
                x => {
                    let x: u32 = x.parse().unwrap();
                    current += x;
                }
            }
        });
        println!("Top 3: {:?}", maximum);
        println!("Sum: {:?}", maximum.into_iter().sum::<u32>());
        // ANSWER: 71471
    }

    pub(crate) fn two_star_slow() {
        let input = include_str!("../../input/day01.txt");
        let mut all_calories = input.lines().fold(Vec::new(), |mut acc, x| {
            match x {
                "" => acc.push(0),
                x => {
                    let x: u32 = x.parse().unwrap();
                    if let Some(last) = acc.last_mut() {
                        *last += x;
                    }
                }
            }
            acc
        });
        dbg!(&all_calories[0..10]);
        all_calories.sort();
        all_calories.reverse();
        let answer: u32 = all_calories[0..3].iter().sum();
        println!("Top 3: {:?}", &all_calories[0..3]);
        println!("Answer: {}", answer);
    }
}

fn main() {
    solutions::one_star();
    solutions::two_star_slow();
}
