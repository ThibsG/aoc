use std::str::FromStr;

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
}
