use anyhow::{Context, Result};
use day3::Rucksack;
use itertools::Itertools;
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin().lock();
    stdin.read_to_string(&mut buffer).context("reading stdin")?;

    let mut total_priority = 0;
    let mut intersection = Vec::with_capacity(3);
    let mut count = 0;
    for line in buffer.lines() {
        let rucksack = Rucksack::try_from(line).context("rucksack from input")?;
        if intersection.is_empty() {
            intersection = rucksack.items();
        } else {
            intersection = rucksack.intersect(intersection);
        }
        count += 1;

        if count == 3 {
            total_priority += intersection
                .iter()
                .unique()
                .fold(0, |acc, e| acc + e.priority());
            intersection.clear();
            count = 0;
        }
    }

    println!("Total priority: {total_priority}");

    Ok(())
}
