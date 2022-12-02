use std::str::FromStr;

#[derive(Clone, Copy)]
enum Ending {
    Draw,
    Lose,
    Win,
}

impl FromStr for Ending {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => unimplemented!(),
        }
    }
}

/// Rock, Paper, Scissors
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl RPS {
    fn score(&self, opponent: RPS) -> u8 {
        let base_score = *self as u8;
        if *self == opponent {
            // draw
            return base_score + 3;
        }

        match (*self, opponent) {
            (Self::Rock, Self::Paper)
            | (Self::Scissors, Self::Rock)
            | (Self::Paper, Self::Scissors) => base_score, // lose
            (Self::Paper, Self::Rock)
            | (Self::Rock, Self::Scissors)
            | (Self::Scissors, Self::Paper) => base_score + 6, // win
            _ => panic!("should be scored"),
        }
    }

    fn score_with_ending(&self, ending: Ending) -> u8 {
        match (ending, *self) {
            (Ending::Draw, opponent) => self.score(opponent),
            (Ending::Lose, Self::Rock) => Self::Scissors.score(Self::Rock),
            (Ending::Lose, Self::Paper) => Self::Rock.score(Self::Paper),
            (Ending::Lose, Self::Scissors) => Self::Paper.score(Self::Scissors),
            (Ending::Win, Self::Scissors) => Self::Rock.score(Self::Scissors),
            (Ending::Win, Self::Rock) => Self::Paper.score(Self::Rock),
            (Ending::Win, Self::Paper) => Self::Scissors.score(Self::Paper),
        }
    }
}

impl FromStr for RPS {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => unimplemented!(),
        }
    }
}

pub fn main() {
    // First part
    let input = include_str!("../resources/02.txt");

    let score = input
        .lines()
        .map(|line| {
            let line = line.split(' ').collect::<Vec<&str>>();
            let (opponent, me) = (
                RPS::from_str(line[0]).unwrap(),
                RPS::from_str(line[1]).unwrap(),
            );
            me.score(opponent) as u32
        })
        .sum::<u32>();
    println!("Part 1 - Score: {score}");

    // Second part
    let score = input
        .lines()
        .map(|line| {
            let line = line.split(' ').collect::<Vec<&str>>();
            let (opponent, ending) = (
                RPS::from_str(line[0]).unwrap(),
                Ending::from_str(line[1]).unwrap(),
            );
            opponent.score_with_ending(ending) as u32
        })
        .sum::<u32>();

    println!("Part 2 - Score: {score}");
}
