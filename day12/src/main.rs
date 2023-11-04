use std::io::{self, Read};

use anyhow::{Result, Context};

use day12::{HeightMap, MultiStartDijkstra};

pub fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin().lock();
    stdin.read_to_string(&mut buffer).context("read stdin")?;

    let height_map = HeightMap::try_from(buffer.as_str()).context("parsing input")?;
    let mut multi_start_dijkstra = MultiStartDijkstra::from(&height_map);

    let shortest_distance = multi_start_dijkstra.solve().context("solving using Dijkstra's")?;
    println!("Shortest distance: {shortest_distance}");

    Ok(())
}
