use anyhow::{Context, Error, Result};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

const INFINITY: u32 = u32::MAX;

#[derive(Clone)]
pub struct HeightMap {
    grid: Vec<Vec<Rc<RefCell<Node>>>>,
    starts: Vec<Position>,
    end: Position,
}

impl TryFrom<&str> for HeightMap {
    type Error = Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut starts = Vec::new();
        let mut end = None;
        let mut grid = Vec::new();

        if input.len() == 0 {
            return Err(Error::msg("empty input"));
        }

        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::with_capacity(line.len());
            for (x, char) in line.chars().enumerate() {
                let height = match char {
                    'S' => {
                        starts.push(Position(x, y));
                        0
                    }
                    'E' => {
                        end = Some(Position(x, y));
                        25
                    }
                    char @ 'a'..='z' => {
                        let height = char as u32 - 97;
                        if height == 0 {
                            starts.push(Position(x, y));
                        }
                        height
                    }
                    _ => return Err(Error::msg("invalid node height character")),
                };
                let node = Node {
                    position: Position(x, y),
                    height,
                    tentative_distance: INFINITY,
                    neighbours: Vec::new(),
                    previous: None,
                };
                row.push(Rc::new(RefCell::new(node)));
            }
            grid.push(row);
        }

        let grid_clone = grid.clone();
        for (y, row) in grid_clone.iter().enumerate() {
            for (x, node) in row.iter().enumerate() {
                if y > 0 {
                    if let Some(row_above) = grid.get(y - 1) {
                        if let Some(node_above) = row_above.get(x) {
                            if node_above.borrow().height <= node.borrow().height + 1 {
                                node.borrow_mut().neighbours.push(node_above.clone());
                            }
                        }
                    }
                }
                if x > 0 {
                    if let Some(row) = grid.get(y) {
                        if let Some(node_before) = row.get(x - 1) {
                            if node_before.borrow().height <= node.borrow().height + 1 {
                                node.borrow_mut().neighbours.push(node_before.clone());
                            }
                        }
                    }
                }
                if let Some(row) = grid.get(y) {
                    if let Some(node_after) = row.get(x + 1) {
                        if node_after.borrow().height <= node.borrow().height + 1 {
                            node.borrow_mut().neighbours.push(node_after.clone());
                        }
                    }
                }
                if let Some(row_below) = grid.get(y + 1) {
                    if let Some(node_below) = row_below.get(x) {
                        if node_below.borrow().height <= node.borrow().height + 1 {
                            node.borrow_mut().neighbours.push(node_below.clone());
                        }
                    }
                }
            }
        }

        Ok(Self {
            grid,
            starts,
            end: end.context("no end node was identified")?,
        })
    }
}

#[derive(Clone)]
pub struct Node {
    position: Position,
    height: u32,
    tentative_distance: u32,
    neighbours: Vec<Rc<RefCell<Node>>>,
    previous: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn distance_from_start(&self) -> Option<u32> {
        Some(
            self.previous
                .clone()?
                .borrow()
                .distance_from_start_recurse(1),
        )
    }

    fn distance_from_start_recurse(&self, distance_so_far: u32) -> u32 {
        let Some(previous) = self.previous.clone() else {
            return distance_so_far;
        };
        let distance = previous
            .borrow()
            .distance_from_start_recurse(distance_so_far + 1);
        distance
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position(usize, usize);

#[derive(Clone)]
pub struct Dijkstra {
    start: Position,
    end: Position,
    current: Position,
    all_nodes: HashMap<Position, Rc<RefCell<Node>>>,
    unvisited_nodes: HashMap<Position, Rc<RefCell<Node>>>,
}

impl Dijkstra {
    pub fn solve(&mut self) -> Option<u32> {
        self.unvisited_nodes
            .get_mut(&self.current)
            .unwrap()
            .borrow_mut()
            .tentative_distance = 0;

        loop {
            match self.step() {
                State::Continue => (),
                State::Found => {
                    let end_node = self.all_nodes.get(&self.end).expect("end node must exist");
                    return Some(
                        end_node
                            .borrow()
                            .distance_from_start()
                            .expect("a path from the start must be found at this point"),
                    );
                }
                State::NoPossiblePath => return None,
            }
        }
    }

    fn step(&mut self) -> State {
        self.current = self.lowest_distance_node();
        self.mark_as_visited(self.current)
            .expect("nodes we try to move should always exist");
        let current_node = self
            .all_nodes
            .get(&self.current)
            .expect("current node must exist");

        for neighbour in &current_node.borrow().neighbours {
            if neighbour.borrow().tentative_distance > current_node.borrow().tentative_distance + 1
            {
                neighbour.borrow_mut().tentative_distance =
                    current_node.borrow().tentative_distance + 1;
                neighbour.borrow_mut().previous = Some(current_node.clone());
            }
        }

        if current_node.borrow().position == self.end {
            return State::Found;
        }

        for node in self.unvisited_nodes.values() {
            if node.borrow().tentative_distance != INFINITY {
                return State::Continue;
            }
        }

        State::NoPossiblePath
    }

    fn lowest_distance_node(&self) -> Position {
        let mut lowest_distance = INFINITY;
        let mut position = None;
        for node in self.unvisited_nodes.values() {
            let node = node.borrow();
            if node.tentative_distance < lowest_distance {
                lowest_distance = node.tentative_distance;
                position = Some(node.position);
            }
        }
        position.expect("the algorithm should end before there are no more unvisited nodes")
    }

    fn mark_as_visited(&mut self, position: Position) -> Result<()> {
        self.unvisited_nodes
            .remove(&position)
            .context("removing unvisited node")
            .map(|_| Ok(()))?
    }
}

impl From<&HeightMap> for MultiStartDijkstra {
    fn from(height_map: &HeightMap) -> Self {
        let mut dijkstras = Vec::new();
        for start in &height_map.starts {
            let h = height_map.clone();
            let unvisited_nodes: HashMap<_, Rc<RefCell<_>>> = h
                .grid
                .iter()
                .flatten()
                .map(|node| (node.borrow().position, node.clone()))
                .collect();
            dijkstras.push(Dijkstra {
                start: *start,
                end: h.end,
                current: *start,
                all_nodes: unvisited_nodes.clone(),
                unvisited_nodes: unvisited_nodes.clone(),
            });
        }

        Self(dijkstras)
    }
}

pub struct MultiStartDijkstra(Vec<Dijkstra>);

impl MultiStartDijkstra {
    pub fn solve(&mut self) -> Option<u32> {
        let mut shortest_distance = None;
        for dijkstra in &mut self.0.clone() {
            self.reset_nodes();
            let solution = dijkstra.solve();
            if let Some(distance) = solution {
                if shortest_distance.is_none() || shortest_distance.unwrap() > distance {
                    shortest_distance = Some(distance);
                }
            }
        }
        shortest_distance
    }

    fn reset_nodes(&mut self) {
        for node in self.0[0].all_nodes.values() {
            node.borrow_mut().tentative_distance = INFINITY;
            node.borrow_mut().previous = None;
        }
    }
}

pub enum State {
    Continue,
    Found,
    NoPossiblePath,
}
