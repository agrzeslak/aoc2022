use anyhow::{Context, Error, Result};

use std::collections::HashSet;

pub struct Knot {
    pub position: (i32, i32),
    pub next: Option<Box<Knot>>,
}

impl Knot {
    pub fn new() -> Self {
        Self {
            position: (0, 0),
            next: None,
        }
    }

    pub fn propagate(&mut self) {
        if self.next.is_none() {
            return;
        }

        if self.touching(self.next.as_ref().expect("guaranteed to be Some")) {
            return;
        }

        let mut next = self.next.as_mut().expect("guaranteed to be Some");

        let x_delta = self.position.0 - next.position.0;
        let y_delta = self.position.1 - next.position.1;

        let move_from_delta = |i| -> i32 {
            if i >= 2 {
                return 1;
            } else if i <= -2 {
                return -1;
            }
            i
        };

        let x_move = move_from_delta(x_delta);
        let y_move = move_from_delta(y_delta);

        next.position.0 += x_move;
        next.position.1 += y_move;

        next.propagate();
    }

    fn touching(&self, other: &Knot) -> bool {
        let x_delta = (self.position.0 - other.position.0).abs();
        let y_delta = (self.position.1 - other.position.1).abs();
        x_delta < 2 && y_delta < 2
    }

    pub fn tail(&self) -> &Knot {
        let mut tail = self;
        while let Some(knot) = &tail.next {
            tail = knot
        }
        tail
    }
}

pub struct Puzzle {
    pub rope: Knot,
    tail_visited: HashSet<(i32, i32)>,
}

impl Puzzle {
    pub fn new(rope_length: usize) -> Result<Self> {
        if rope_length < 1 {
            return Err(Error::msg("cannot create a rope of length < 1"));
        }
        let mut puzzle = Self {
            rope: Knot::new(),
            tail_visited: HashSet::from([(0, 0)]),
        };
        let mut knot = &mut puzzle.rope;
        for _ in 1..rope_length {
            knot.next = Some(Box::new(Knot::new()));
            knot = knot.next.as_mut().unwrap();
        }
        Ok(puzzle)
    }
    pub fn apply_motion(&mut self, motion: &Motion) {
        for _ in 0..motion.count {
            match motion.direction {
                Direction::Up => self.rope.position.1 += 1,
                Direction::Right => self.rope.position.0 += 1,
                Direction::Down => self.rope.position.1 -= 1,
                Direction::Left => self.rope.position.0 -= 1,
            }
            self.rope.propagate();
            let tail = self.rope.tail();
            self.tail_visited.insert((tail.position.0, tail.position.1));
        }
    }

    pub fn num_unique_tail_locations(&self) -> usize {
        self.tail_visited.len()
    }
}

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub struct Motion {
    direction: Direction,
    count: u32,
}

impl TryFrom<&str> for Motion {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (direction, count) = value.split_once(' ').context("splitting motion str")?;
        let count = count
            .parse()
            .context("parsing number of time to apply motion")?;
        match direction {
            "U" => Ok(Motion {
                direction: Direction::Up,
                count,
            }),
            "R" => Ok(Motion {
                direction: Direction::Right,
                count,
            }),
            "D" => Ok(Motion {
                direction: Direction::Down,
                count,
            }),
            "L" => Ok(Motion {
                direction: Direction::Left,
                count,
            }),
            direction @ _ => Err(Error::msg(format!("{direction} is not a valid direction"))),
        }
    }
}
