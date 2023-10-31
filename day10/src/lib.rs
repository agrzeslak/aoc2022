use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i32, space1},
    combinator::all_consuming,
    combinator::map,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
pub enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    pub fn cycles(&self) -> u32 {
        match self {
            Self::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }

    pub fn parse(i: &str) -> Result<Self> {
        all_consuming(alt((Self::noop_parser, Self::addx_parser)))(i)
            .map(|(_, instruction)| instruction)
            .map_err(|e| e.to_owned().into())
    }

    fn noop_parser(i: &str) -> IResult<&str, Self> {
        map(tag("noop"), |_| Self::Noop)(i)
    }

    fn addx_parser(i: &str) -> IResult<&str, Self> {
        map(separated_pair(tag("addx"), space1, i32), |(_, digit)| {
            Self::Addx(digit)
        })(i)
    }
}
