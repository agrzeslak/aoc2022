use anyhow::Result;
use itertools::Itertools;

#[derive(Debug)]
pub struct Rucksack(Vec<Item>, Vec<Item>);

impl Rucksack {
    pub fn common_items_between_compartments(&self) -> Vec<Item> {
        self.0
            .clone()
            .into_iter()
            .filter(|&item| self.1.contains(&item))
            .unique()
            .collect()
    }

    pub fn items(&self) -> Vec<Item> {
        let mut all_items = self.0.clone();
        all_items.append(&mut self.1.clone());
        all_items
    }

    pub fn intersect(&self, other: Vec<Item>) -> Vec<Item> {
        self.items()
            .into_iter()
            .filter(|item| other.contains(item)).collect()
    }
}

impl TryFrom<&str> for Rucksack {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() % 2 != 0 {
            return Err(anyhow::Error::msg(
                "rucksack must contain an even amount of items",
            ));
        }

        let mut compartment_a = Vec::with_capacity(value.len() / 2);
        let mut compartment_b = Vec::with_capacity(value.len() / 2);

        let (str_a, str_b) = value.split_at(value.len() / 2);
        for c in str_a.chars() {
            compartment_a.push(Item::try_from(c)?);
        }
        for c in str_b.chars() {
            compartment_b.push(Item::try_from(c)?);
        }

        Ok(Rucksack(compartment_a, compartment_b))
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Item(char);

impl TryFrom<char> for Item {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a'..='z' | 'A'..='Z' => Ok(Self(value)),
            _ => Err(anyhow::Error::msg("items can only be a-z or A-Z")),
        }
    }
}

impl Item {
    // a-z = 1-26
    // A-Z = 27-52
    pub fn priority(&self) -> u32 {
        match self.0 {
            'a'..='z' => self.0 as u32 - 96,
            'A'..='Z' => self.0 as u32 - 38,
            _ => unreachable!("items can only be a-z or A-Z"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priority() {
        assert_eq!(1, Item::try_from('a').unwrap().priority());
        assert_eq!(26, Item::try_from('z').unwrap().priority());
        assert_eq!(27, Item::try_from('A').unwrap().priority());
        assert_eq!(52, Item::try_from('Z').unwrap().priority());
    }

    #[test]
    fn rucksack() {
        let rucksack = Rucksack::try_from("vJrwpWtwJgWrhcsFMMfFFhFp").unwrap();
        let common_items = rucksack.common_items_between_compartments();
        assert_eq!(1, common_items.len());
        assert_eq!('p', common_items[0].0);

        let rucksack = Rucksack::try_from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL").unwrap();
        let common_items = rucksack.common_items_between_compartments();
        assert_eq!(1, common_items.len());
        assert_eq!('L', common_items[0].0);

        let rucksack = Rucksack::try_from("PmmdzqPrVvPwwTWBwg").unwrap();
        let common_items = rucksack.common_items_between_compartments();
        assert_eq!(1, common_items.len());
        assert_eq!('P', common_items[0].0);

        let rucksack = Rucksack::try_from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn").unwrap();
        let common_items = rucksack.common_items_between_compartments();
        assert_eq!(1, common_items.len());
        assert_eq!('v', common_items[0].0);

        let rucksack = Rucksack::try_from("ttgJtRGJQctTZtZT").unwrap();
        let common_items = rucksack.common_items_between_compartments();
        assert_eq!(1, common_items.len());
        assert_eq!('t', common_items[0].0);

        let rucksack = Rucksack::try_from("CrZsJsPPZsGzwwsLwLmpwMDw").unwrap();
        let common_items = rucksack.common_items_between_compartments();
        assert_eq!(1, common_items.len());
        assert_eq!('s', common_items[0].0);
    }
}
