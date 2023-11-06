use std::io::{self, Read};

use anyhow::{Context, Result};
use nom::{
    character::complete::newline, combinator::all_consuming, multi::separated_list1, Finish,
};

use day13::{Entry, Pair};

fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin().lock();
    stdin.read_to_string(&mut buffer).context("read stdin")?;

    let pairs = all_consuming(separated_list1(newline, Pair::parse))(&buffer)
        .finish()
        .unwrap()
        .1;

    let mut entries = Vec::with_capacity(pairs.len() * 2);
    for Pair(lhs, rhs) in pairs {
        entries.push(lhs);
        entries.push(rhs);
    }
    entries.append(&mut Entry::divider_packets());
    entries.sort();

    let mut decoder_key = None;
    for (index, entry) in entries.iter().enumerate() {
        if !entry.is_divider_packet() {
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
