use std::io::{self, Read};

use anyhow::{Context, Result};
use day16::Graph;
use nom::Finish;

fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin().lock();
    stdin.read_to_string(&mut buffer).context("read stdin")?;

    let mut graph = Graph::parse(&buffer).finish().unwrap().1;
    let most_pressure_released = graph.solve();

    println!("Most pressure released: {most_pressure_released}");
    Ok(())
}
