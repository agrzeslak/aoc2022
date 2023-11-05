use std::cmp::Ordering;

use nom::{
    branch::alt,
    character::complete::{char, newline, u32},
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, terminated, tuple},
    IResult,
};

pub struct Pair(pub Entry, pub Entry);

impl Pair {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                terminated(Entry::parse, newline),
                terminated(Entry::parse, newline),
            )),
            |(a, b)| Self(a, b),
        )(input)
    }
}

#[derive(Clone, Debug)]
pub enum Entry {
    List(Vec<Entry>),
    Value(u32),
}

impl Entry {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(u32, |n| Self::Value(n)),
            map(
                delimited(
                    char('['),
                    separated_list0(char(','), Self::parse),
                    char(']'),
                ),
                |vec| Self::List(vec),
            ),
        ))(input)
    }

    pub fn check_ordering(&self, rhs: &Self) -> Outcome {
        use Entry::*;
        match (self, rhs) {
            (List(lhs), List(rhs)) => {
                for (lhs, rhs) in lhs.iter().zip(rhs.iter()) {
                    match lhs.check_ordering(rhs) {
                        Outcome::CorrectOrder => return Outcome::CorrectOrder,
                        Outcome::IncorrectOrder => return Outcome::IncorrectOrder,
                        Outcome::Inconclusive => (),
                    }
                }
                match lhs.len().cmp(&rhs.len()) {
                    Ordering::Less => Outcome::CorrectOrder,
                    Ordering::Equal => Outcome::Inconclusive,
                    Ordering::Greater => Outcome::IncorrectOrder,
                }
            }
            (List(lhs), Value(rhs)) => {
                List(lhs.to_vec()).check_ordering(&List(vec![Value(*rhs)]))
            }
            (Value(lhs), List(rhs)) => {
                List(vec![Value(*lhs)]).check_ordering(&List(rhs.clone()))
            }
            (Value(lhs), Value(rhs)) => {
                match lhs.cmp(rhs) {
                    Ordering::Less => Outcome::CorrectOrder,
                    Ordering::Equal => Outcome::Inconclusive,
                    Ordering::Greater => Outcome::IncorrectOrder,
                }
            }
        }
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.check_ordering(other) {
            Outcome::CorrectOrder => Ordering::Less,
            Outcome::IncorrectOrder => Ordering::Greater,
            Outcome::Inconclusive => Ordering::Equal,
        }
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Entry {}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.check_ordering(other) == Outcome::Inconclusive
    }
}

#[derive(PartialEq, Eq)]
pub enum Outcome {
    CorrectOrder,
    IncorrectOrder,
    Inconclusive,
}
