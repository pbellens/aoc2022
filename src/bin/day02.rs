use std::{convert::TryFrom, str::FromStr};

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Loss,
    Draw, 
    Win,
}

impl Outcome {
    fn score(self) -> usize {
        return match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Hand {
    Rock, 
    Paper, 
    Scissors,
}

impl Hand {
    fn score(self) -> usize {
        return match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn outcome(self, yours: Hand) -> Outcome {
        match (self, yours) {
            (Hand::Rock, Hand::Rock) => Outcome::Draw,
            (Hand::Rock, Hand::Paper) => Outcome::Loss,
            (Hand::Rock, Hand::Scissors) => Outcome::Win,
            (Hand::Paper, Hand::Rock) => Outcome::Win,
            (Hand::Paper, Hand::Paper) => Outcome::Draw,
            (Hand::Paper, Hand::Scissors) => Outcome::Loss,
            (Hand::Scissors, Hand::Rock) => Outcome::Loss,
            (Hand::Scissors, Hand::Paper) => Outcome::Win,
            (Hand::Scissors, Hand::Scissors) => Outcome::Draw,
        }
    }

    fn counter(self, outcome: Outcome) -> Self {
        match (self, outcome) {
            (Hand::Rock, Outcome::Win) => Hand::Paper,
            (Hand::Paper, Outcome::Win) => Hand::Scissors,
            (Hand::Scissors, Outcome::Win) => Hand::Rock,
            (Hand::Rock, Outcome::Loss) => Hand::Scissors,
            (Hand::Paper, Outcome::Loss) => Hand::Rock,
            (Hand::Scissors, Outcome::Loss) => Hand::Paper,
            (rps, Outcome::Draw) => rps,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    yours: Hand,
    mine: Hand,
}

impl Round {
    fn score(self) -> usize {
        self.mine.score() + self.mine.outcome(self.yours).score()
    }
}

#[derive(Debug, Clone, Copy)]
struct FixedRound {
    yours: Hand,
    exp: Outcome,
}

impl TryFrom<char> for Hand {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Hand::Rock),
            'B' | 'Y' => Ok(Hand::Paper),
            'C' | 'Z' => Ok(Hand::Scissors),
            _ => Err(color_eyre::eyre::eyre!("Unknown hand {value:?}"))
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(color_eyre::eyre::eyre!("Unknown outcome {value:?}"))
        }
    }
}

impl FromStr for Round {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chs = s.chars();
        let (Some(y), Some(' '), Some(m), None) = (chs.next(), chs.next(), chs.next(), chs.next()) else {
            return Err(color_eyre::eyre::eyre!("Not a valid input line: {s:?}"))
        };

        Ok(Round {
            yours: y.try_into()?,
            mine: m.try_into()?,
        })
    }
}

impl FromStr for FixedRound {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chs = s.chars();
        let (Some(y), Some(' '), Some(m), None) = (chs.next(), chs.next(), chs.next(), chs.next()) else {
            return Err(color_eyre::eyre::eyre!("Not a valid input line: {s:?}"))
        };

        Ok(FixedRound {
            yours: y.try_into()?,
            exp: m.try_into()?,
        })
    }
}



fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let answer1 = itertools::process_results(
        include_str!("../../data/day02.input")
            .lines()
            .map(|l| l.parse::<Round>()) 
            .map(|r| r.map(|round| round.score())),
        |iter| iter.sum::<usize>());
    println!("Answer for part 1 is {:?}", answer1);

    let answer2 = itertools::process_results(
        include_str!("../../data/day02.input")
            .lines()
            .map(|l| l.parse::<FixedRound>()) 
            .map(|r| r.map(|f| {
                let hand = f.yours.counter(f.exp);
                hand.score() + f.exp.score()
            })),
        |iter| iter.sum::<usize>());
    println!("Answer for part 2 is {:?}", answer2);

    return Ok(())
}
