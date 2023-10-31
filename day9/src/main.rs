use anyhow::{Context, Result};

use std::io::{self, Read};

use day9::{Puzzle, Motion};

const ROPE_LENGTH: usize = 10;

fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin().lock();
    stdin.read_to_string(&mut buffer).context("read stdin")?;

    let mut puzzle = Puzzle::new(ROPE_LENGTH).context("creating puzzle")?;
    for line in buffer.lines() {
        let motion = Motion::try_from(line).context("parsing line as motion")?;
        puzzle.apply_motion(&motion);
    }

    println!("Tail has visited {} unique locations", puzzle.num_unique_tail_locations());

    Ok(())
}
