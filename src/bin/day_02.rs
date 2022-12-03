use anyhow::{anyhow, Error, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<char> for Play {
    type Error = Error;
    fn try_from(value: char) -> Result<Self> {
        match value {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => Err(anyhow!(
                "given char `{}` is not defined for `Play` variants",
                value
            )),
        }
    }
}

#[derive(Debug, Default)]
struct Score(u32);

// can be done through `derive_more`.
impl std::iter::Sum for Score {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Score::default(), std::ops::Add::add)
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
            (Play::Rock, Play::Paper)
            | (Play::Scissors, Play::Rock)
            | (Play::Paper, Play::Scissors) => Outcome::Win,
            (Play::Paper, Play::Rock)
            | (Play::Rock, Play::Scissors)
            | (Play::Scissors, Play::Paper) => Outcome::Loss,
            (Play::Rock, Play::Rock)
            | (Play::Paper, Play::Paper)
            | (Play::Scissors, Play::Scissors) => Outcome::Draw,
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = Error;
    fn try_from(value: char) -> Result<Self> {
        match value {
            'X' => Ok(Self::Loss),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => Err(anyhow!(
                "given char `{}` is not defined for `Play` variants",
                value
            )),
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

/// Returns the `Play` that the player is supposed to play.
fn play_from_outcome(oponent: Play, outcome: Outcome) -> Play {
    use Play::*;
    match (oponent, outcome) {
        (Rock, Outcome::Loss) => Scissors,
        (Paper, Outcome::Win) => Scissors,
        (Scissors, Outcome::Draw) => Scissors,
        (Rock, Outcome::Win) => Paper,
        (Scissors, Outcome::Loss) => Paper,
        (Paper, Outcome::Draw) => Paper,
        (Paper, Outcome::Loss) => Rock,
        (Scissors, Outcome::Win) => Rock,
        (Rock, Outcome::Draw) => Rock,
        //FIXME: better way to phrase this?
    }
}

mod solutions {
    use super::*;
    use itertools::Itertools;

    pub(crate) fn one_star() {
        let input = include_str!("../../input/day02.txt");
        // let input = "A Y
        // B X
        // C Z";
        let answer = input
            .chars()
            .flat_map(|x| match x {
                ' ' => None,
                '\n' => None,
                x => Some(Play::try_from(x).unwrap()),
            })
            .chunks(2)
            .into_iter()
            .map(|x| {
                let (oponent, me) = x.collect_tuple().unwrap();
                let outcome = Outcome::new(oponent, me);
                Score::from(me) + Score::from(outcome)
            })
            .sum::<Score>();
        println!("Answer: {:?}", answer);
    }

    pub(crate) fn two_star() {
        let input = include_str!("../../input/day02.txt");
        // let input = "A Y
        // B X
        // C Z";
        let answer = input
            .chars()
            .filter(|x| !x.is_whitespace())
            .chunks(2)
            .into_iter()
            .map(|x| {
                let (oponent, outcome) = x.collect_tuple().unwrap();
                let (oponent, outcome): (Play, Outcome) =
                    (oponent.try_into().unwrap(), outcome.try_into().unwrap());
                let me = play_from_outcome(oponent, outcome);
                Score::from(me) + Score::from(outcome)
            })
            .sum::<Score>();
        println!("Answer: {:?}", answer);
    }
}

fn main() {
    solutions::one_star();
    solutions::two_star();
}
