use anyhow::{Context, Result};

use std::io::{self, Read};

use day8::Grid;

fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin().lock();
    stdin.read_to_string(&mut buffer).context("read stdin")?;

    let mut grid = Grid::default();
    for line in buffer.lines() {
        grid.add_row(line)?;
    }

    println!("Number of visible trees: {}", grid.count_visible());
    println!("Best scenic score: {}", grid.best_scenic_score());

    Ok(())
}
