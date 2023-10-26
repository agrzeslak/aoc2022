use anyhow::{Context, Result};
use day5::Stacks;
use std::io::{self, Read};

enum State {
    ParsingCrates,
    ParsingMoves,
}

fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin().lock();
    stdin
        .read_to_string(&mut buffer)
        .context("reading stdin to string")?;
    let mut state = State::ParsingCrates;
    let mut stacks = Stacks::new();

    for line in buffer.split('\n') {
        if line.is_empty() {
            state = State::ParsingMoves;
            continue;
        }
        match &state {
            State::ParsingCrates => {
                // Line with column numbers is the only one starting with whitespace and is ignored
                if line.starts_with(' ') {
                    continue;
                }

                let mut column = 0;
                for (index, supply) in line.chars().enumerate() {
                    if index == 0 || (index - 1) % 4 != 0 {
                        // Starting from the 2nd index, every 4th is a supply
                        continue;
                    }

                    if supply != ' ' {
                        stacks.add_crate(column, supply)
                    }

                    column += 1
                }
            }
            State::ParsingMoves => {
                let line = line.replace("move ", "");
                let (number_to_move, rest) = line
                    .split_once(' ')
                    .context("splitting number to move from rest of line")?;
                let number_to_move = number_to_move
                    .parse()
                    .context("parsing number of crates to move")?;
                let rest = rest.replace("from ", "");
                let (source, rest) = rest
                    .split_once(" ")
                    .context("splitting source from rest of line")?;
                let source = source.parse().context("parsing source")?;
                let destination = rest
                    .replace("to ", "")
                    .parse()
                    .context("parsing destination")?;
                stacks
                    .move_crates(source, destination, number_to_move)
                    .context("moving crate")?;
            }
        }
    }
    println!("Top crates in each stack: {}", stacks.top_crates());
    Ok(())
}
