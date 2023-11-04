use anyhow::{Context, Result};
use indicatif::ProgressIterator;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, newline, space0, space1, u64};
use nom::combinator::map;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};
use nom::IResult;
use num::BigUint;

use std::cmp::Reverse;
use std::collections::VecDeque;

const ROUNDS: u32 = 10000;
const TOP_NUMBER_OF_MONKEYS: usize = 2;
const WORRY_DIVISOR: u64 = 1;

#[derive(Debug)]
pub struct Scenario {
    monkeys: Vec<Monkey>,
}

impl Scenario {
    pub fn throw_item(&mut self, item: BigUint, monkey: usize) -> Result<()> {
        self.monkeys
            .get_mut(monkey)
            .context("get monkey to throw to")?
            .give_item(item);
        Ok(())
    }

    pub fn monkey_business(&mut self) -> BigUint {
        let divisor_product = self.monkeys.iter().map(|m| m.test.divisible_by).product::<u64>();
        for _ in (0..ROUNDS).progress() {
            for i in 0..self.monkeys.len() {
                let mut monkey_clone = self.monkeys[i].clone();
                let inspections = monkey_clone.items.len();

                while let Some(mut item) = monkey_clone.next_item() {
                    item %= divisor_product;
                    let (item, target) = monkey_clone.decide(item);
                    self.throw_item(item, target as usize)
                        .expect("assuming valid input, the target should always exist");
                }

                self.monkeys[i].items.clear();
                self.monkeys[i].inspections += inspections as u64;
            }
        }

        let mut inspections = self
            .monkeys
            .clone()
            .into_iter()
            .map(|monkey| monkey.inspections)
            .collect::<Vec<_>>();
        inspections.sort_by_key(|i| Reverse(i.clone()));
        dbg!(&inspections);
        inspections.iter().take(TOP_NUMBER_OF_MONKEYS).product()
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_list1(newline, Monkey::parse), |monkeys| Self {
            monkeys,
        })(input)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Monkey {
    items: VecDeque<BigUint>,
    operation: Operation,
    test: Test,
    inspections: BigUint,
}

impl Monkey {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tuple((tag("Monkey"), space1, digit1, char(':'), newline))(input)?;
        let (input, items) = delimited(
            tuple((space0, tag("Starting items:"))),
            separated_list0(char(','), preceded(space1, u64)),
            newline,
        )(input)?;
        let (input, operation) = terminated(Operation::parse, newline)(input)?;
        let (input, test) = terminated(Test::parse, newline)(input)?;
        Ok((
            input,
            Self {
                items: items.into_iter().map(|i| i.into()).collect(),
                operation,
                test,
                inspections: 0u64.into(),
            },
        ))
    }

    fn next_item(&mut self) -> Option<BigUint> {
        self.items.pop_front()
    }

    fn give_item(&mut self, item: BigUint) {
        self.items.push_back(item)
    }

    pub fn decide(&mut self, mut worry_level: BigUint) -> (BigUint, usize) {
        self.inspections += BigUint::from(1u64);
        let value = match self.operation.operand {
            Operand::Value(value) => value.into(),
            Operand::Old => worry_level.clone(),
        };
        match self.operation.operator {
            Operator::Add => worry_level += value,
            Operator::Multiply => worry_level *= value,
        }
        worry_level /= WORRY_DIVISOR;
        let target_monkey = match &worry_level % self.test.divisible_by == 0u32.into() {
            true => self.test.pass,
            false => self.test.fail,
        };
        (worry_level, target_monkey)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Operation {
    operator: Operator,
    operand: Operand,
}

impl Operation {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        preceded(
            tuple((space1, tag("Operation: new = old "))),
            map(
                separated_pair(Operator::parse, space0, Operand::parse),
                |(operator, operand)| Operation { operator, operand },
            ),
        )(input)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operator {
    Add,
    Multiply,
}

impl Operator {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(char('*'), |_| Operator::Multiply),
            map(char('+'), |_| Operator::Add),
        ))(input)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operand {
    Value(u64),
    Old,
}

impl Operand {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(u64, |value| Operand::Value(value.into())),
            map(tag("old"), |_| Operand::Old),
        ))(input)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Test {
    divisible_by: u64,
    pass: usize,
    fail: usize,
}

impl Test {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, divisible_by) =
            delimited(tuple((space1, tag("Test: divisible by "))), u64, newline)(input)?;
        let (input, pass) = delimited(
            tuple((space0, tag("If true: throw to monkey "))),
            u64,
            newline,
        )(input)?;
        let (input, fail) =
            preceded(tuple((space0, tag("If false: throw to monkey "))), u64)(input)?;
        Ok((
            input,
            Self {
                divisible_by: divisible_by.into(),
                pass: pass as usize,
                fail: fail as usize,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let input = r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";
        let (remainder, monkey) = Monkey::parse(input).unwrap();
        assert!(remainder.is_empty());
        assert_eq!(
            monkey,
            Monkey {
                inspections: 0u64.into(),
                items: VecDeque::from([79u64.into(), 98u64.into()]),
                operation: Operation {
                    operator: Operator::Multiply,
                    operand: Operand::Value(19)
                },
                test: Test {
                    divisible_by: 23u32.into(),
                    pass: 2,
                    fail: 3
                }
            }
        );
    }
}
