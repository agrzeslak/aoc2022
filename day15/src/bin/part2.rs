use std::io::{self, Read};

use anyhow::{Context, Result};
use day15::Area;
use nom::Finish;

pub fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin().lock();
    stdin.read_to_string(&mut buffer).context("read stdin")?;

    let area = Area::parse(buffer.as_str()).finish().unwrap().1;
    let tuning_frequency = area
        .locate_distress_beacon()
        .expect("distress beacon must exist")
        .tuning_frequency();

    println!("Tuning frequency of distress beacon: {}", tuning_frequency);
    Ok(())
}
