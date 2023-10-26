use anyhow::{Context, Result};
use day3::Rucksack;
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin().lock();
    stdin.read_to_string(&mut buffer).context("reading stdin")?;

    let mut total_priority = 0;
    for line in buffer.lines() {
        let rucksack_common_items = Rucksack::try_from(line)
            .context("creating rucksack from input string")?
            .common_items_between_compartments();
        total_priority += rucksack_common_items
            .iter()
            .fold(0, |acc, item| acc + item.priority());
    }

    println!("Total priority: {total_priority}");

    Ok(())
}
