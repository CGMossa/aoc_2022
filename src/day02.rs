#[derive(Debug, Clone, Copy, PartialEq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Play {
    fn from(value: char) -> Self {
        match value {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("given char `{}` is not defined for `Play` variants", value),
        }
    }
}

#[derive(Debug, Default)]
struct Score(u32);

// can be done through `derive_more`.
impl std::iter::Sum for Score {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Score(0), std::ops::Add::add)
    }
}

impl std::ops::Add for Score {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl From<Play> for Score {
    fn from(value: Play) -> Self {
        match value {
            Play::Rock => Score(1),
            Play::Paper => Score(2),
            Play::Scissors => Score(3),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    pub fn new(oponent: Play, me: Play) -> Self {
        match (oponent, me) {
            (Play::Rock, Play::Paper) => Outcome::Win,
            (Play::Paper, Play::Rock) => Outcome::Loss,
            (Play::Scissors, Play::Rock) => Outcome::Win,
            (Play::Rock, Play::Scissors) => Outcome::Loss,
            (Play::Paper, Play::Scissors) => Outcome::Win,
            (Play::Scissors, Play::Paper) => Outcome::Loss,
            (Play::Rock, Play::Rock)
            | (Play::Paper, Play::Paper)
            | (Play::Scissors, Play::Scissors) => Outcome::Draw,
        }
    }
}

impl From<Outcome> for Score {
    fn from(value: Outcome) -> Self {
        match value {
            Outcome::Loss => Self(0),
            Outcome::Draw => Self(3),
            Outcome::Win => Self(6),
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_one_star() {
        let input = include_str!("../input/day02.txt");
        // let input = "A Y
        // B X
        // C Z";
        let answer = input
            .chars()
            .flat_map(|x| match x {
                ' ' => None,
                '\n' => None,
                x => Some(Play::from(x)),
            })
            .chunks(2)
            .into_iter()
            .map(|x| {
                let (oponent, me) = x.collect_tuple().unwrap();
                let outcome = Outcome::new(oponent, me);
                // dbg!(
                //     Score::from(me),
                //     Score::from(outcome),
                //     Score::from(me) + Score::from(outcome)
                // );
                Score::from(me) + Score::from(outcome)
            })
            .sum::<Score>();
        println!("Answer: {:?}", answer);
    }
}
