use std::io::{self, Read};

use anyhow::{Context, Result};
use day14::Simulation;
use nom::Finish;

pub fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin().lock();
    stdin.read_to_string(&mut buffer).context("read stdin")?;

    let mut simulation = Simulation::parse(buffer.as_str()).finish().unwrap().1;
    simulation.run_until_complete();

    println!("Units of sand at rest: {}", simulation.amount_of_sand_at_rest());
    Ok(())
}
