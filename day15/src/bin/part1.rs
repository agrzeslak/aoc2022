use std::io::{self, Read};

use anyhow::{Context, Result};
use day15::Area;
use nom::Finish;

const ROW: i32 = 2000000;

pub fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin().lock();
    stdin.read_to_string(&mut buffer).context("read stdin")?;

    let area = Area::parse(buffer.as_str()).finish().unwrap().1;
    let (low, high) = area
        .range_of_sensor_coverage_in_row(ROW)
        .expect("at least some positions should be impossible based on the input");
    let number_of_impossible_positions =
        (low..=high).count() - area.number_of_spots_taken_in_row_section(ROW, (low, high)) as usize;

    println!(
        "Number of impossible positions in row: {}",
        number_of_impossible_positions
    );
    Ok(())
}
