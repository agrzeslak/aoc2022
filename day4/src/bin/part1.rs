use anyhow::{Context, Result};
use day4::Assignment;
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin().lock();
    stdin
        .read_to_string(&mut buffer)
        .context("reading stdin to string")?;
    let mut overlaps = 0;
    for line in buffer.lines() {
        let (a, b) = line
            .split_once(',')
            .context("splitting into two section ID ranges")?;
        let assignment_a = Assignment::try_from(
            a.split_once('-')
                .context("splitting into low and high IDs")?,
        )
        .context("creating assignment from ranges")?;
        let assignment_b = Assignment::try_from(
            b.split_once('-')
                .context("splitting into low and high IDs")?,
        )
        .context("creating assignment from ranges")?;
        if assignment_a.fully_overlaps(&assignment_b) {
            overlaps += 1;
        }
    }
    println!("Number of fully overlapping assignments: {overlaps}");
    Ok(())
}
