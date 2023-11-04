use std::io::{self, Read};

use anyhow::{Context, Result};
use day11::Scenario;
use nom::{combinator::all_consuming, Finish};

pub fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin().lock();
    stdin.read_to_string(&mut buffer).context("read stdin")?;

    let mut scenario = all_consuming(Scenario::parse)(&buffer).finish().unwrap().1;
    let monkey_business = scenario.monkey_business();
    println!("Monkey business: {monkey_business}");

    Ok(())
}
