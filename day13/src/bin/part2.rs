use std::io::{self, Read};

use anyhow::{Context, Result};
use nom::{character::complete::newline, multi::separated_list1, Finish, combinator::all_consuming};

use day13::{Pair, Entry};

fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin().lock();
    stdin.read_to_string(&mut buffer).context("read stdin")?;

    let mut pairs = all_consuming(separated_list1(newline, Pair::parse))(&buffer).finish().unwrap().1;
    let divider_packets = all_consuming(Pair::parse)("[[2]]\n[[6]]\n").finish().unwrap().1;
    pairs.push(divider_packets);

    let mut entries = Vec::with_capacity(pairs.len() * 2);
    for Pair(lhs, rhs) in pairs {
        entries.push(lhs);
        entries.push(rhs);
    }
    entries.sort();

    let mut decoder_key = None;
    for (index, entry) in entries.iter().enumerate() {
        let Entry::List(list) = entry else {
            continue;
        };

        if list.len() != 1 {
            continue;
        }

        let Entry::List(list) = &list[0] else {
            continue;
        };

        if list.len() != 1 {
            continue;
        }

        let Entry::Value(value) = list[0] else {
            continue;
        };

        if value != 2 && value != 6 {
            continue;
        }

        match decoder_key {
            Some(current) => decoder_key = Some(current * (index + 1)),
            None => decoder_key = Some(index + 1),
        }
    }

    println!("Decoder key: {}", decoder_key.unwrap());

    Ok(())
}
