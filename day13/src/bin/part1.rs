use std::io::{self, Read};

use anyhow::{Context, Result};
use nom::{character::complete::newline, multi::separated_list1, Finish, combinator::all_consuming};

use day13::{Pair, Outcome};

fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin().lock();
    stdin.read_to_string(&mut buffer).context("read stdin")?;

    let pairs = all_consuming(separated_list1(newline, Pair::parse))(&buffer).finish().unwrap().1;
    let mut sum_of_indicies = 0;

    for (index, pair) in pairs.iter().enumerate() {
        if pair.0.check_ordering(&pair.1) == Outcome::CorrectOrder {
            sum_of_indicies += index + 1;
        }
    }

    println!("Sum of indicies: {sum_of_indicies}");

    Ok(())
}
