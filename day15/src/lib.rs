use std::{fmt::Display, ops::Add};

use nom::{
    bytes::complete::tag,
    character::complete::{i64, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

const MAX_COORDINATE: i64 = 4000000;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point(pub i64, pub i64);

impl Point {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                preceded(tag("x="), i64),
                tag(", "),
                preceded(tag("y="), i64),
            ),
            |(x, y)| Self(x, y),
        )(input)
    }

    pub fn manhattan_distance(&self, rhs: &Self) -> i64 {
        (self.0 - rhs.0).abs() + (self.1 - rhs.1).abs()
    }

    pub fn tuning_frequency(&self) -> i64 {
        self.0 * 4000000 + self.1
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

    pub fn ranges_covered_in_row(&self, y: i64) -> Vec<(i64, i64)> {
        let mut coverage = Vec::new();
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
            let covered_range = (
                sensor.location.0 - coverage_past_row,
                sensor.location.0 + coverage_past_row,
            );
            coverage.push(covered_range);
        }
        coverage
    }

    pub fn point_contains_item(&self, point: &Point) -> bool {
        for sensor in &self.sensors {
            if sensor.location == *point || sensor.closest_beacon == *point {
                return true;
            }
        }
        false
    }

    pub fn number_of_spots_taken_in_row_section(&self, y: i64, range: (i64, i64)) -> u64 {
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

    pub fn is_point_covered(&self, point: &Point) -> bool {
        let mut is_covered = false;
        for sensor in &self.sensors {
            let sensor_covered_distance =
                sensor.location.manhattan_distance(&sensor.closest_beacon);
            let distance = sensor.location.manhattan_distance(point);
            if distance <= sensor_covered_distance {
                is_covered = true;
            }
        }
        is_covered
    }

    pub fn locate_distress_beacon(&self) -> Option<Point> {
        for y in 0..=MAX_COORDINATE {
            let row_coverage = self.ranges_covered_in_row(y);
            let mut x = 0;
            loop {
                if x > MAX_COORDINATE {
                    break;
                }
                if let Some((_, high)) = row_coverage
                    .iter()
                    .find(|(low, high)| (low..=high).contains(&&x))
                {
                    x = high + 1;
                    continue;
                }
                let point = Point(x, y);
                if !self.is_point_covered(&point) {
                    return Some(point);
                }
                x += 1;
            }
        }
        None
    }
}
