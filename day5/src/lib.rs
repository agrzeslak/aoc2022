use anyhow::{Context, Result};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Stacks(Vec<VecDeque<char>>);

impl Stacks {
    pub fn new() -> Self {
        Self(Vec::with_capacity(9))
    }

    pub fn add_crate(&mut self, column: usize, supply: char) {
        while self.0.len() <= column {
            self.0.push(VecDeque::new());
        }
        self.0
            .get_mut(column)
            .expect("ensured to have enough columns")
            .push_back(supply);
    }

    pub fn move_crate(&mut self, source: usize, destination: usize) -> Result<()> {
        self.move_crates(source, destination, 1)
    }

    pub fn move_crates(&mut self, source: usize, destination: usize, number: usize) -> Result<()> {
        // Source and destination are 1-based
        let source = source - 1;
        let destination = destination - 1;
        let mut popped = VecDeque::with_capacity(number);

        let source = self
            .0
            .get_mut(source)
            .context("source stack greater than the number of stacks")?;
        for _ in 0..number {
            popped.push_back(
                source
                    .pop_front()
                    .context("attempted to pop from empty stack")?,
            );
        }

        let destination = self
            .0
            .get_mut(destination)
            .context("destination stack greater than the number of stacks")?;
        for _ in 0..number {
            destination.push_front(
                popped
                    .pop_back()
                    .expect("guaranteed to have enough entries"),
            );
        }

        Ok(())
    }

    pub fn top_crates(&self) -> String {
        let mut top_crates = String::new();
        for stack in &self.0 {
            top_crates.push(*stack.get(0).unwrap_or(&' '));
        }
        top_crates
    }
}
