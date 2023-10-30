use std::collections::HashMap;
use std::io::{self, Read};
use std::path::PathBuf;

use anyhow::{Context, Result};

use day7::Node;

fn main() -> Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin().lock();
    stdin.read_to_string(&mut buffer).context("read stdin")?;

    let mut root = Node::Directory(HashMap::new());
    let mut current_path = PathBuf::from("/");
    let mut current_node = &mut root;

    for line in buffer.lines() {
        if line.starts_with("$ ") {
            let command = &line[2..];
            if command.starts_with("cd ") {
                match &command[3..] {
                    "/" => continue,
                    ".." => {
                        current_path.pop();
                        current_node = root
                            .navigate_to(&current_path)
                            .context("navigating to path")?;
                    }
                    path @ _ => {
                        current_path.push(path);
                        current_node = current_node.cd(path).context("changing directory")?;
                    }
                }
            }
        } else {
            match line.split_once(' ').context("splitting command output")? {
                ("dir", name @ _) => current_node
                    .add_child(name, Node::directory())
                    .context("adding child directory")?,
                (size @ _, name @ _) => {
                    let size = size.parse().context("parsing file size")?;
                    current_node
                        .add_child(name, Node::File(size))
                        .context("adding child file")?
                }
            };
        }
    }
    println!(
        "Total size of all directories of size at most 100000: {}",
        root.solve()
    );

    Ok(())
}
