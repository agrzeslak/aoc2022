use anyhow::{Context, Result};
use std::io::{self, Read};

struct Elf {
    calories: u32,
}

fn main() -> Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle
        .read_to_string(&mut buffer)
        .context("reading stdin to end")?;

    let mut elves = Vec::new();
    let mut current_elf = Elf { calories: 0 };

    for line in buffer.split("\n") {
        if line.is_empty() {
            elves.push(current_elf);
            current_elf = Elf { calories: 0 };
            continue;
        }

        let calories = line.parse::<u32>().context("parsing calories as u32")?;
        current_elf.calories += calories;
    }

    let mut top_3_calories = Vec::with_capacity(3);
    for elf in elves {
        if top_3_calories.len() < 3 {
            top_3_calories.push(elf.calories);

            if top_3_calories.len() == 3 {
                top_3_calories.sort();
            }

            continue;
        }

        if elf.calories > top_3_calories[2] {
            top_3_calories[2] = elf.calories;
            top_3_calories.sort();
            top_3_calories.reverse();
        }
    }

    println!("Most calories carried by a single elf: {}", top_3_calories[0]);

    let total: u32 = top_3_calories.iter().sum();
    println!("Total calories carried by top 3 elves: {total}");
    Ok(())
}
