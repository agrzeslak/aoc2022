use std::io::{self, Read};

use anyhow::{Context, Result};

use day10::Instruction;

const NUM_SAMPLES: usize = 6;

fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin().lock();
    stdin.read_to_string(&mut buffer).context("read stdin")?;

    let instructions = buffer
        .lines()
        .map(Instruction::parse)
        .collect::<Result<Vec<_>, _>>()
        .context("parsing instruction")?;

    let mut x = 1;
    let mut cycle = 0;
    let mut samples = Vec::with_capacity(NUM_SAMPLES);
    for instruction in instructions {
        for _ in 0..instruction.cycles() {
            cycle += 1;
            if (cycle - 20) % 40 == 0 {
                samples.push(x * cycle);
            }
        }
        match instruction {
            Instruction::Noop => (),
            Instruction::Addx(i) => x += i,
        }
    }

    println!("Sum of signal strengths: {}", samples.iter().sum::<i32>());
    Ok(())
}
