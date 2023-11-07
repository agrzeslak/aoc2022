use std::{
    collections::HashSet,
    io::{self, Read},
};

use anyhow::{Context, Result};
use day15::{Area, Point};
use nom::Finish;

const ROW: i32 = 2000000;

pub fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin().lock();
    stdin.read_to_string(&mut buffer).context("read stdin")?;

    let area = Area::parse(buffer.as_str()).finish().unwrap().1;

    let mut points_covered_in_row = HashSet::new();
    for range in &area.ranges_covered_in_row(ROW) {
        for point in range.0..=range.1 {
            if !area.point_contains_item(&Point(point, ROW)) {
                points_covered_in_row.insert(point);
            }
        }
    }

    println!(
        "Number of impossible positions in row: {}",
        points_covered_in_row.len()
    );
    Ok(())
}
