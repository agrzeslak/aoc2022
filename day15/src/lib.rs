use std::{fmt::Display, ops::Add, cmp};

use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point(i32, i32);

impl Point {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                preceded(tag("x="), i32),
                tag(", "),
                preceded(tag("y="), i32),
            ),
            |(x, y)| Self(x, y),
        )(input)
    }

    pub fn manhattan_distance(&self, rhs: &Self) -> i32 {
        (self.0 - rhs.0).abs() + (self.1 - rhs.1).abs()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

pub struct Sensor {
    location: Point,
    closest_beacon: Point,
}

impl Sensor {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                preceded(tag("Sensor at "), Point::parse),
                preceded(tag(": closest beacon is at "), Point::parse),
            )),
            |(location, closest_beacon)| Self {
                location,
                closest_beacon,
            },
        )(input)
    }
}

pub struct Area {
    sensors: Vec<Sensor>,
}

impl Area {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_list1(newline, Sensor::parse), |sensors| Self {
            sensors,
        })(input)
    }

    pub fn range_of_sensor_coverage_in_row(&self, y: i32) -> Option<(i32, i32)> {
        let mut coverage = None;
        for sensor in &self.sensors {
            let sensor_covered_distance =
                sensor.location.manhattan_distance(&sensor.closest_beacon);
            let distance_to_row = sensor
                .location
                .manhattan_distance(&Point(sensor.location.0, y));
            if distance_to_row > sensor_covered_distance {
                // Row is not covered by this sensor
                continue;
            }
            let coverage_past_row = sensor_covered_distance - distance_to_row;
            let (new_low, new_high) = (
                sensor.location.0 - coverage_past_row,
                sensor.location.0 + coverage_past_row,
            );
            match coverage {
                Some((low, high)) => {
                    coverage = Some((cmp::min(low, new_low), cmp::max(high, new_high)))
                }
                None => coverage = Some((new_low, new_high)),
            }
        }
        coverage
    }

    pub fn number_of_spots_taken_in_row_section(&self, y: i32, range: (i32, i32)) -> u32 {
        let mut count = 0;
        'outer: for x in range.0..=range.1 {
            for sensor in &self.sensors {
                if sensor.location == Point(x, y) || sensor.closest_beacon == Point(x, y) {
                    count += 1;
                    continue 'outer;
                }
            }
        }
        count
    }
}
