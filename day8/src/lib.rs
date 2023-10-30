use anyhow::{Context, Error, Result};
use std::cmp;

pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Default)]
pub struct Grid(Vec<Vec<u32>>);

impl Grid {
    pub fn add_row(&mut self, row: &str) -> Result<()> {
        let current_width = self.width();
        if current_width > 0 && current_width != row.len() {
            return Err(Error::msg(format!(
                "row to be added is of width {}, while existing grid is of width {}",
                row.len(),
                current_width
            )));
        }

        self.0.push(
            row.chars()
                .map(|c| {
                    c.to_digit(10)
                        .context("converting tree height char to digit")
                })
                .collect::<Result<Vec<u32>, Error>>()?,
        );
        Ok(())
    }

    pub fn get(&self, x: usize, y: usize) -> Result<&u32> {
        self.0
            .get(y)
            .context("getting row")?
            .get(x)
            .context("getting tree from row")
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn width(&self) -> usize {
        match self.0.get(0) {
            Some(row) => row.len(),
            None => 0,
        }
    }

    pub fn is_visible(&self, x: usize, y: usize) -> Result<bool> {
        Ok(self.is_visible_from(x, y, &Direction::Top)?
            || self.is_visible_from(x, y, &Direction::Right)?
            || self.is_visible_from(x, y, &Direction::Bottom)?
            || self.is_visible_from(x, y, &Direction::Left)?)
    }

    pub fn is_visible_from(&self, x: usize, y: usize, direction: &Direction) -> Result<bool> {
        let target_height = self.get(x, y).context("get target tree height")?;
        match direction {
            Direction::Top => {
                for i in 0..y {
                    if self
                        .get(x, i)
                        .context("get tree height, checking visibility from top")?
                        >= target_height
                    {
                        return Ok(false);
                    }
                }
            }
            Direction::Right => {
                for i in x + 1..self.width() {
                    if self
                        .get(i, y)
                        .context("get tree height, checking visibility from right")?
                        >= target_height
                    {
                        return Ok(false);
                    }
                }
            }
            Direction::Bottom => {
                for i in y + 1..self.height() {
                    if self
                        .get(x, i)
                        .context("get tree height, checking visibility from bottom")?
                        >= target_height
                    {
                        return Ok(false);
                    }
                }
            }
            Direction::Left => {
                for i in 0..x {
                    if self
                        .get(i, y)
                        .context("get tree height, checking visibility from left")?
                        >= target_height
                    {
                        return Ok(false);
                    }
                }
            }
        };
        Ok(true)
    }

    pub fn count_visible(&self) -> usize {
        let on_edge = self.width() * 2 + (self.height() - 2) * 2;
        let mut count = 0;
        // Trees on edges are always visible and have already been counted
        for y in 1..self.height() - 1 {
            for x in 1..self.width() - 1 {
                if self
                    .is_visible(x, y)
                    .expect("always within grid boundaries")
                {
                    count += 1;
                }
            }
        }
        on_edge + count
    }

    pub fn best_scenic_score(&self) -> u32 {
        let mut best = 0;
        // Trees on edges have a scenic score of 0, so skip them
        for y in 1..self.height() - 1 {
            for x in 1..self.width() - 1 {
                best = cmp::max(
                    self.scenic_score(x, y)
                        .expect("always within grid boundaries"),
                    best,
                );
            }
        }
        best
    }

    pub fn scenic_score(&self, x: usize, y: usize) -> Result<u32> {
        Ok(self.viewing_distance(x, y, &Direction::Top)?
            * self.viewing_distance(x, y, &Direction::Right)?
            * self.viewing_distance(x, y, &Direction::Bottom)?
            * self.viewing_distance(x, y, &Direction::Left)?)
    }

    pub fn viewing_distance(&self, x: usize, y: usize, direction: &Direction) -> Result<u32> {
        let target_height = self.get(x, y).context("get target tree height")?;
        let mut distance = 0;
        match direction {
            Direction::Top => {
                for i in (0..y).rev() {
                    distance += 1;
                    if self
                        .get(x, i)
                        .context("get tree height, checking top viewing distance")?
                        >= target_height
                    {
                        break;
                    }
                }
            }
            Direction::Right => {
                for i in x + 1..self.width() {
                    distance += 1;
                    if self
                        .get(i, y)
                        .context("get tree height, checking top viewing distance")?
                        >= target_height
                    {
                        break;
                    }
                }
            }
            Direction::Bottom => {
                for i in y + 1..self.height() {
                    distance += 1;
                    if self
                        .get(x, i)
                        .context("get tree height, checking top viewing distance")?
                        >= target_height
                    {
                        break;
                    }
                }
            }
            Direction::Left => {
                for i in (0..x).rev() {
                    distance += 1;
                    if self
                        .get(i, y)
                        .context("get tree height, checking top viewing distance")?
                        >= target_height
                    {
                        break;
                    }
                }
            }
        }
        Ok(distance)
    }

    pub fn print_chosen_row_column(&self, x: usize, y: usize) {
        for i in 0..self.height() {
            for j in 0..self.width() {
                if j == x || i == y {
                    print!("{}", self.get(j, i).unwrap());
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}
