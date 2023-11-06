use std::{collections::HashSet, fmt::Display};

use anyhow::{Error, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{char, i32, newline},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

const SOURCE: Point = Point(500, 0);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_pair(i32, char(','), i32), |(x, y)| Self(x, y))(input)
    }

    fn down(&self) -> Self {
        Self(self.0, self.1 + 1)
    }
    fn down_left(&self) -> Self {
        Self(self.0 - 1, self.1 + 1)
    }
    fn down_right(&self) -> Self {
        Self(self.0 + 1, self.1 + 1)
    }
}

pub struct Rock(pub Vec<Point>);

impl Rock {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_list1(tag(" -> "), Point::parse), |v| Self(v))(input)
    }
}

pub struct Simulation {
    rocks: Vec<Rock>,
    floor: Option<i32>,
    sand: HashSet<Point>,
    falling: HashSet<Point>,
}

impl Simulation {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_list1(newline, Rock::parse), |rocks| Self {
            rocks,
            floor: None,
            sand: HashSet::new(),
            falling: HashSet::new(),
        })(input)
    }

    pub fn set_floor(&mut self) {
        const OFFSET: i32 = 2;
        let mut max_y = i32::MIN;
        for rock in &self.rocks {
            for vertex in &rock.0 {
                max_y = std::cmp::max(max_y, vertex.1);
            }
        }
        self.floor = Some(max_y + OFFSET);
    }

    pub fn add_rock(&mut self, rock: Rock) {
        self.rocks.push(rock)
    }

    pub fn run_until_complete(&mut self) {
        loop {
            if self.step() == Outcome::SandSpawnBlocked {
                return;
            }
            if self.step() == Outcome::Movement && self.floor.is_none() {
                if self
                    .falling
                    .iter()
                    .fold(true, |acc, e| acc && self.will_fall_forever(e))
                {
                    // All blocks which are falling will continue falling forever
                    return;
                }
            }
        }
    }

    fn step(&mut self) -> Outcome {
        if self.falling.len() == 0 {
            if self.spawn_sand().is_err() {
                return Outcome::SandSpawnBlocked;
            }
        }
        self.apply_gravity()
    }

    fn apply_gravity(&mut self) -> Outcome {
        let mut outcome = Outcome::NoMovement;

        for point in &self.falling.clone() {
            if !self.down_blocked(&point) {
                self.falling.remove(&point);
                self.falling.insert(point.down());
                outcome = Outcome::Movement;
            } else if !self.down_left_blocked(&point) {
                self.falling.remove(&point);
                self.falling.insert(point.down_left());
                outcome = Outcome::Movement;
            } else if !self.down_right_blocked(&point) {
                self.falling.remove(&point);
                self.falling.insert(point.down_right());
                outcome = Outcome::Movement;
            } else {
                self.falling.remove(&point);
                self.sand.insert(*point);
            }
        }

        outcome
    }

    fn spawn_sand(&mut self) -> Result<()> {
        if self.sand.contains(&SOURCE) || !self.falling.insert(SOURCE) {
            return Err(Error::msg("sand already exists at spawn point"));
        }
        Ok(())
    }

    fn down_blocked(&self, current: &Point) -> bool {
        self.is_occupied(&current.down())
    }
    fn down_left_blocked(&self, current: &Point) -> bool {
        self.is_occupied(&current.down_left())
    }
    fn down_right_blocked(&self, current: &Point) -> bool {
        self.is_occupied(&current.down_right())
    }

    fn is_occupied(&self, point: &Point) -> bool {
        if self.sand.contains(point) || self.falling.contains(point) {
            return true;
        }

        if let Some(floor) = self.floor {
            if floor == point.1 {
                return true;
            }
        }

        for rock in &self.rocks {
            let mut previous_vertex = rock.0[0];
            for current_vertex in &rock.0 {
                // Assumes that rock lines cannot be diagonal
                let create_range = |a, b| {
                    if a <= b {
                        a..=b
                    } else {
                        b..=a
                    }
                };
                let x_range = create_range(previous_vertex.0, current_vertex.0);
                let y_range = create_range(previous_vertex.1, current_vertex.1);
                if x_range.contains(&point.0) && y_range.contains(&point.1) {
                    return true;
                }
                previous_vertex = *current_vertex;
            }
        }
        false
    }

    fn will_fall_forever(&self, point: &Point) -> bool {
        // y coordinate increases downwards
        let mut highest = 0;
        for rock in &self.rocks {
            for vertex in &rock.0 {
                highest = std::cmp::max(highest, vertex.1);
            }
        }
        return point.1 >= highest;
    }

    pub fn amount_of_sand_at_rest(&self) -> usize {
        self.sand.len()
    }
}

impl Display for Simulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut min_x = SOURCE.0;
        let mut min_y = SOURCE.1;
        let mut max_x = SOURCE.0;
        let mut max_y = SOURCE.1;

        for rock in &self.rocks {
            for vertex in &rock.0 {
                min_x = std::cmp::min(min_x, vertex.0);
                min_y = std::cmp::min(min_y, vertex.1);
                max_x = std::cmp::max(max_x, vertex.0);
                max_y = std::cmp::max(max_y, vertex.1);
            }
        }

        for sand in &self.sand {
            min_x = std::cmp::min(min_x, sand.0);
            min_y = std::cmp::min(min_y, sand.1);
            max_x = std::cmp::max(max_x, sand.0);
            max_y = std::cmp::max(max_y, sand.1);
        }

        for falling in &self.falling {
            min_x = std::cmp::min(min_x, falling.0);
            min_y = std::cmp::min(min_y, falling.1);
            max_x = std::cmp::max(max_x, falling.0);
            max_y = std::cmp::max(max_y, falling.1);
        }

        if let Some(floor) = self.floor {
            max_y = std::cmp::min(max_y, floor);
        }

        let mut output = String::new();

        const MARGIN: i32 = 3;
        for y in min_y - MARGIN..=max_y + MARGIN {
            for x in min_x - MARGIN..=max_x + MARGIN {
                if self.sand.contains(&Point(x, y)) {
                    output.push('o');
                } else if self.floor.is_some() && self.floor.unwrap() == y {
                    output.push('-');
                } else if self.falling.contains(&Point(x, y)) {
                    output.push('x');
                } else if self.is_occupied(&Point(x, y)) {
                    output.push('#');
                } else if SOURCE == Point(x, y) {
                    output.push('+');
                } else {
                    output.push('.');
                }
            }
            output.push('\n');
        }

        write!(f, "{}", output)
    }
}

#[derive(PartialEq, Eq)]
enum Outcome {
    Movement,
    NoMovement,
    SandSpawnBlocked,
}
