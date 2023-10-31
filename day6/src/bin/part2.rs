use std::{
    collections::VecDeque,
    io::{self, Read},
};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin().lock();
    stdin
        .read_to_string(&mut buffer)
        .context("reading stdin to string")?;
    let mut chars = VecDeque::with_capacity(4);

    'outer: for (i, char) in buffer.chars().enumerate() {
        if i < 14 {
            chars.push_back(char);
            continue;
        }

        chars.pop_front();
        chars.push_back(char);

        for (j, char) in chars.iter().enumerate() {
            // Only search past the index we are up to because otherwise we will be checking
            // permutations multiple times.
            for k in j + 1..chars.len() {
                if char == &chars[k] {
                    continue 'outer;
                }
            }
        }
        // +1 since the answer is 1-based
        println!("Marker appears after character {} arrives", i + 1);
        break 'outer;
    }

    Ok(())
}
