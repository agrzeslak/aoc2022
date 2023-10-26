use anyhow::{Context, Result};
use std::io::{self, Read};

#[derive(Clone, Copy, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn score(&self) -> u32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

impl TryFrom<char> for Choice {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => Err(anyhow::Error::msg("invalid choice character")),
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Lose => 0,
            Self::Draw => 3,
        }
    }
}

impl Outcome {
    fn choice_for_outcome(&self, their_choice: &Choice) -> Choice {
        use Choice::*;
        match (self, their_choice) {
            (Self::Win, Rock) => Paper,
            (Self::Win, Paper) => Scissors,
            (Self::Win, Scissors) => Rock,
            (Self::Lose, Rock) => Scissors,
            (Self::Lose, Paper) => Rock,
            (Self::Lose, Scissors) => Paper,
            (Self::Draw, _) => *their_choice,
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Self::Lose),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => Err(anyhow::Error::msg("invalid outcome character")),
        }
    }
}

struct Round {
    pub our_choice: Choice,
    pub their_choice: Choice,
}

impl Round {
    fn play(&self) -> Outcome {
        use Choice::*;

        match (&self.our_choice, &self.their_choice) {
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Outcome::Win,
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Outcome::Lose,
            _ => Outcome::Draw,
        }
    }
    fn score(&self) -> u32 {
        self.our_choice.score() + self.play().score()
    }
}

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut buffer)
        .context("reading stdin")?;

    let mut total_score = 0;

    for line in buffer.lines() {
        let (their_choice, our_choice) = line
            .split_once(' ')
            .context("splitting string to get choices")?;
        let their_choice = Choice::try_from(
            their_choice
                .chars()
                .next()
                .context("their choice string to char")?,
        )
        .context("parsing their choice from char")?;
        let our_choice = Choice::try_from(
            our_choice
                .chars()
                .next()
                .context("our choice string to char")?,
        )
        .context("parsing our choice from char")?;
        let round = Round {
            our_choice,
            their_choice,
        };
        total_score += round.score();
    }
    println!("Total score (part 1): {total_score}");

    total_score = 0;
    for line in buffer.lines() {
        let (their_choice, outcome) = line
            .split_once(' ')
            .context("splitting string to get their choice and outcome")?;
        let their_choice = Choice::try_from(
            their_choice
                .chars()
                .next()
                .context("their choice string to char")?,
        )
        .context("parsing their choice from char")?;
        let outcome = Outcome::try_from(outcome.chars().next().context("outcome string to char")?)
            .context("parsing outcome from char")?;
        let our_choice = outcome.choice_for_outcome(&their_choice);
        let round = Round {
            our_choice,
            their_choice,
        };
        total_score += round.score();
    }
    println!("Total score (part 2): {total_score}");

    Ok(())
}
