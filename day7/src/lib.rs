use std::{collections::HashMap, path::PathBuf};

use anyhow::{Context, Error, Result};

#[derive(Debug)]
pub enum Node {
    File(usize),
    Directory(HashMap<String, Node>),
}

impl Node {
    pub fn cd(&mut self, name: &str) -> Result<&mut Self> {
        match self {
            Node::File(_) => {
                panic!("cannot cd from a file")
            }
            Node::Directory(children) => {
                children.get_mut(name).context("retrieving child directory")
            }
        }
    }

    pub fn add_child(&mut self, name: &str, node: Node) -> Result<()> {
        match self {
            Node::File(_) => Err(Error::msg("cannot add child to a file")),
            Node::Directory(children) => {
                children.insert(name.into(), node);
                Ok(())
            }
        }
    }

    pub fn directory() -> Self {
        Self::Directory(HashMap::new())
    }

    pub fn navigate_to(&mut self, path: &PathBuf) -> Result<&mut Self> {
        let mut current = self;
        for directory in path {
            if directory == "/" {
                continue;
            }
            current = current
                .cd(directory.to_str().context("directory to string")?)
                .context("changing into directory")?;
        }
        Ok(current)
    }

    pub fn size(&self) -> usize {
        match self {
            Node::File(size) => *size,
            Node::Directory(children) => children.values().fold(0, |acc, e| acc + e.size()),
        }
    }

    pub fn solve(&self) -> usize {
        let mut result = 0;
        if let Node::Directory(children) = self {
            let size = self.size();
            if size <= 100000 {
                result += size;
            }
            for child in children.values() {
                result += child.solve();
            }
        }
        result
    }

    pub fn smallest_dir_size_of_at_least(
        &self,
        minimum_size: usize,
        mut smallest: Option<usize>,
    ) -> Option<usize> {
        if let Node::Directory(children) = self {
            let size = self.size();
            if size >= minimum_size {
                match smallest {
                    Some(current_smallest) => {
                        if size < current_smallest {
                            smallest = Some(size)
                        }
                    }
                    None => smallest = Some(size),
                }
            }
            for child in children.values() {
                smallest = child.smallest_dir_size_of_at_least(minimum_size, smallest)
            }
        }
        smallest
    }
}
