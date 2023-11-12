use std::{
    cmp,
    collections::{hash_map::Entry, HashMap},
    fmt::Display,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

const TOTAL_MINUTES: u64 = 30;

#[derive(Clone, Debug)]
pub struct Valve {
    name: String,
    flow_rate: u64,
    neighbours: Vec<String>,
}

impl Valve {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                preceded(tag("Valve "), alpha1),
                preceded(tag(" has flow rate="), nom::character::complete::u64),
                preceded(
                    alt((
                        tag("; tunnels lead to valves "),
                        tag("; tunnel leads to valve "),
                    )),
                    separated_list1(tag(", "), alpha1),
                ),
            )),
            |(name, flow_rate, neighbours): (&str, u64, Vec<&str>)| Self {
                name: name.to_string(),
                flow_rate,
                neighbours: neighbours.into_iter().map(String::from).collect(),
            },
        )(input)
    }
}

pub struct Graph {
    valves: HashMap<String, Valve>,
}

impl Graph {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_list1(newline, Valve::parse), |vec| Self {
            valves: vec
                .into_iter()
                .map(|valve| (valve.name.clone(), valve))
                .collect(),
        })(input)
    }

    pub fn solve_recurse(&mut self, mut state: State) -> u64 {
        if state.time_limit_reached() {
            return state.total_released;
        }
        if state.all_valves_open(self) {
            loop {
                let (outcome, new_state) = state.do_nothing();
                if let MoveOutcome::TimeLimitReached = outcome {
                    return new_state.total_released;
                }
                state = new_state;
            }
        }
        let paths = self.shortest_paths(&state.location);
        let mut choices: Vec<(u64, Path)> = paths
            .into_iter()
            .map(|(_, path)| (path.expected_benefit(self, &state), path))
            .collect();
        choices.sort_by(|a, b| b.0.cmp(&a.0));
        let top_n_choices = 10;
        choices
            .into_iter()
            .take(top_n_choices)
            .map(|(_, path)| self.solve_recurse(state.clone().apply(&path, self)))
            .fold(0, |acc, e| cmp::max(acc, e))
    }

    pub fn solve(&mut self) -> u64 {
        let state = State::new();
        self.solve_recurse(state)
    }

    pub fn shortest_paths(&self, from: &str) -> HashMap<String, Path> {
        let mut paths: HashMap<String, Path> = HashMap::new();
        paths.insert(from.into(), Path::default());
        let mut queue = paths.clone();
        while !queue.is_empty() {
            let mut next: HashMap<String, Path> = HashMap::new();
            for (from, path) in &queue {
                for neighbour in &self.valves[&from.to_string()].neighbours {
                    let mut path = path.clone();
                    path.push(neighbour.into());
                    if let Entry::Vacant(v) = paths.entry(neighbour.into()) {
                        v.insert(path.clone());
                        next.insert(neighbour.into(), path);
                    }
                }
            }
            queue = next;
        }
        paths
    }
}

#[derive(Clone, Debug, Default)]
pub struct Path(pub Vec<String>);

impl Path {
    pub fn push(&mut self, valve: String) {
        self.0.push(valve);
    }

    pub fn ends_at<'a>(&'a self, state: &'a State) -> &'a str {
        self.0.last().unwrap_or(&state.location)
    }

    pub fn cost(&self) -> u64 {
        let moves = self.0.len() as u64;
        let turn_to_open = 1;
        moves + turn_to_open
    }

    pub fn expected_benefit(&self, graph: &Graph, state: &State) -> u64 {
        let minutes_remaining = TOTAL_MINUTES - state.minutes_elapsed;
        if minutes_remaining <= self.cost() {
            return 0;
        }
        let productive_minutes = minutes_remaining - self.cost();
        if !state.is_valve_open(self.ends_at(state)) {
            graph.valves[self.ends_at(state)].flow_rate * productive_minutes
        } else {
            0
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.clone().join(" -> "))
    }
}

impl From<Vec<String>> for Path {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
    location: String,
    previous_locations: Vec<String>,
    total_flow_rate: u64,
    total_released: u64,
    minutes_elapsed: u64,
    open_valves: Vec<String>,
}

impl State {
    fn new() -> Self {
        Self {
            location: "AA".into(),
            previous_locations: Vec::new(),
            total_flow_rate: 0,
            total_released: 0,
            minutes_elapsed: 0,
            open_valves: Vec::new(),
        }
    }

    fn tick(&mut self) -> bool {
        self.minutes_elapsed += 1;
        self.total_released += self.total_flow_rate;
        self.minutes_elapsed < TOTAL_MINUTES
    }

    fn open_valve(mut self, graph: &Graph) -> (MoveOutcome, Self) {
        if !self.tick() {
            return (MoveOutcome::TimeLimitReached, self);
        }
        if self.is_valve_open(&self.location) {
            return (MoveOutcome::Impossible, self);
        }
        self.open_valves.push(self.location.clone());
        self.total_flow_rate += graph.valves[&self.location].flow_rate;
        (MoveOutcome::Complete, self)
    }

    fn move_to(mut self, move_to: &str, graph: &Graph) -> (MoveOutcome, Self) {
        if !self.tick() {
            return (MoveOutcome::TimeLimitReached, self);
        }
        if !graph.valves[&self.location]
            .neighbours
            .contains(&move_to.to_string())
        {
            return (MoveOutcome::Impossible, self);
        }
        self.previous_locations.push(self.location);
        self.location = move_to.to_string();
        (MoveOutcome::Complete, self)
    }

    fn do_nothing(mut self) -> (MoveOutcome, Self) {
        if !self.tick() {
            return (MoveOutcome::TimeLimitReached, self);
        }
        (MoveOutcome::Complete, self)
    }

    fn all_valves_open(&self, graph: &Graph) -> bool {
        self.open_valves.len() == graph.valves.len()
    }

    fn is_valve_open(&self, valve: &str) -> bool {
        self.open_valves.contains(&valve.to_owned())
    }

    fn apply(self, path: &Path, graph: &Graph) -> State {
        if path.0.len() == 0 {
            return self.do_nothing().1;
        }
        let mut state = self;
        for move_to in &path.0 {
            let (outcome, new_state) = state.move_to(move_to, graph);
            match outcome {
                MoveOutcome::Complete => (),
                MoveOutcome::Impossible => panic!("attempted impossible move from path"),
                MoveOutcome::TimeLimitReached => return new_state,
            }
            state = new_state;
        }
        let (_outcome, state) = state.open_valve(graph);
        state
    }

    fn time_limit_reached(&self) -> bool {
        self.minutes_elapsed >= TOTAL_MINUTES
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MoveOutcome {
    Complete,
    Impossible,
    TimeLimitReached,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_part1_example_graph() -> Graph {
        Graph::parse(
            r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II",
        )
        .unwrap()
        .1
    }

    #[test]
    fn part1_example() {
        let graph = create_part1_example_graph();
        let state = State::new();
        let released = state
            .move_to("DD", &graph)
            .1
            .open_valve(&graph)
            .1
            .move_to("CC", &graph)
            .1
            .move_to("BB", &graph)
            .1
            .open_valve(&graph)
            .1
            .move_to("AA", &graph)
            .1
            .move_to("II", &graph)
            .1
            .move_to("JJ", &graph)
            .1
            .open_valve(&graph)
            .1
            .move_to("II", &graph)
            .1
            .move_to("AA", &graph)
            .1
            .move_to("DD", &graph)
            .1
            .move_to("EE", &graph)
            .1
            .move_to("FF", &graph)
            .1
            .move_to("GG", &graph)
            .1
            .move_to("HH", &graph)
            .1
            .open_valve(&graph)
            .1
            .move_to("GG", &graph)
            .1
            .move_to("FF", &graph)
            .1
            .move_to("EE", &graph)
            .1
            .open_valve(&graph)
            .1
            .move_to("DD", &graph)
            .1
            .move_to("CC", &graph)
            .1
            .open_valve(&graph)
            .1
            .do_nothing()
            .1
            .do_nothing()
            .1
            .do_nothing()
            .1
            .do_nothing()
            .1
            .do_nothing()
            .1
            .do_nothing()
            .1
            .total_released;
        assert_eq!(1651, released);
    }

    #[test]
    fn shortest_paths() {
        let graph = create_part1_example_graph();
        assert_eq!(1, graph.shortest_paths("AA")["DD"].0.len());
        assert_eq!(7, graph.shortest_paths("HH")["JJ"].0.len());
        assert_eq!(2, graph.shortest_paths("II")["BB"].0.len());
        assert_eq!(0, graph.shortest_paths("JJ")["JJ"].0.len());
        assert_eq!(2, graph.shortest_paths("CC")["EE"].0.len());
    }
}
