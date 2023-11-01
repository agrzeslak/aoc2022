use std::io::{self, Read};

use anyhow::{Context, Result};

use day10::{Instruction, Pixel, Screen};

const WIDTH: usize = 40;
const HEIGHT: usize = 6;

fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin().lock();
    stdin.read_to_string(&mut buffer).context("read stdin")?;

    let instructions = buffer
        .lines()
        .map(Instruction::parse)
        .collect::<Result<Vec<_>, _>>()
        .context("parsing instruction")?;

    let mut screen = Screen::new(WIDTH, HEIGHT);

    let mut x = 1;
    let mut cycle = 0;
    for instruction in instructions {
        for _ in 0..instruction.cycles() {
            let x_to_draw = cycle % WIDTH as i32;
            if (x_to_draw - x).abs() <= 1 {
                // Cast to usize should never panic because the x and y coordinates we are passing
                // in should always be positive.
                screen
                    .set(x_to_draw as usize, cycle as usize / WIDTH, &Pixel::Lit)
                    .context("setting pixel")?;
            }
            cycle += 1;
        }
        match instruction {
            Instruction::Noop => (),
            Instruction::Addx(i) => x += i,
        }
    }

    println!("{screen}");

    Ok(())
}
