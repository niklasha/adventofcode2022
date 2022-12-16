use crate::day::*;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::iter;
use std::str::FromStr;

pub struct Day16 {}

type Output = usize;

impl Day for Day16 {
    fn tag(&self) -> &str {
        "16"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input(), 30));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input(), 30));
    }
}

#[derive(Clone, Debug)]
struct Valve {
    name: String,
    rate: usize,
    neighbours: Vec<String>,
}

impl FromStr for Valve {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        let mut tokens = s.split_whitespace();
        let name = tokens.nth(1).ok_or(AocError)?.to_owned();
        let rate = tokens
            .nth(2)
            .ok_or(AocError)?
            .split('=')
            .nth(1)
            .ok_or(AocError)?
            .strip_suffix(';')
            .ok_or(AocError)?
            .parse::<usize>()?;
        let neighbours = tokens
            .skip(4)
            .map(|s| s.trim_end_matches(',').to_owned())
            .collect::<Vec<_>>();
        Ok(Self {
            name,
            rate,
            neighbours,
        })
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct ValveState {
    locked: bool,
    opened: Option<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    current: String,
    valves: HashMap<String, ValveState>,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", self.current);
        self.valves
            .iter()
            .filter(|(_, v)| !v.locked && v.opened.is_some())
            .sorted_by_key(|(k, _)| *k)
            .for_each(|(k, _)| {
                write!(f, "{}", k);
            });
        Ok(())
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.current.hash(state);
        for (k, v) in self
            .valves
            .iter()
            .filter(|(_, v)| !v.locked && v.opened.is_some())
            .sorted_by_key(|(k, _)| *k)
        {
            k.hash(state);
            v.hash(state);
        }
    }
}

impl Day16 {
    fn process(input: &mut dyn io::Read, n: usize) -> BoxResult<Output> {
        let cave = Self::parse(input)?;
        let init = vec![(
            State {
                current: "AA".to_owned(),
                valves: cave
                    .values()
                    .map(|v| {
                        (
                            v.name.to_owned(),
                            ValveState {
                                locked: v.rate == 0,
                                opened: if v.name == "AA" { Some(1) } else { None },
                            },
                        )
                    })
                    .collect(),
            },
            HashSet::new(),
        )];
        //let vcnt = cave.values().filter(|v| (*v).rate > 0).count();
        let states = (1..=n).fold(Ok(init), |states: BoxResult<_>, t| {
            println!("== {} == {:?}", t, states.as_ref().map(|s| s.len()));
            let mut next = Vec::new();
            for (state, mut visited) in states? {
                // If all valves with flow are open, just stay and wait.
                if state
                    .valves
                    .values()
                    .filter(|v| !v.locked)
                    .all(|v| v.opened.is_some())
                {
                    next.push((state, visited));
                } else {
                    //println!("-- {}<<", state);
                    //visited.iter().for_each(|v| println!("{}", v));
                    visited.insert(state.to_owned());
                    let valve = cave.get(&state.current).ok_or(AocError)?;
                    let valve_state = state.valves.get(&state.current).ok_or(AocError)?;
                    if !valve_state.locked && valve_state.opened.is_none() {
                        let mut new = state.clone();
                        let mut valve = new.valves.get_mut(&new.current).ok_or(AocError)?;
                        valve.opened = Some(t);
                        next.push((new, visited.to_owned()));
                    }
                    for neighbour in &valve.neighbours {
                        let mut new = state.clone();
                        new.current = neighbour.to_owned();
                        if !visited.contains(&new) {
                            next.push((new, visited.to_owned()));
                        }
                    }
                }
            }
            Ok(next)
        })?;
        let foo = states
            .iter()
            .map(|(state, _)| {
                let sum = state
                    .valves
                    .iter()
                    .map(|(k, v)| {
                        {
                            Ok(cave.get(k).ok_or(AocError)?.rate
                                * if let Some(t) = v.opened { n - t } else { 0 })
                        }
                    })
                    .sum::<BoxResult<_>>();
                sum.map(|sum: Output| (state, sum))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let (state, sum) = foo
            .into_iter()
            .max_by_key(|(_, sum)| sum.to_owned())
            .ok_or(AocError)?;
        //        println!("{:?} {:?}", sum, state);
        Ok(sum)
    }

    fn parse(input: &mut dyn Read) -> BoxResult<HashMap<String, Valve>> {
        io::BufReader::new(input)
            .lines()
            .map(|l| l.map_err(|e| e.into()))
            .map(|l| l.and_then(|l| l.parse::<Valve>()))
            .map(|v| v.map(|v| (v.name.to_owned(), v)))
            .collect::<BoxResult<HashMap<_, _>>>()
    }

    // fn parse2(input: &mut dyn Read) -> Graph<String, Valve> {
    //     let mut cave = io::BufReader::new(input)
    //         .lines()
    //         .map(|l| l.map_err(|e| e.into()))
    //         .map(|l| l.and_then(|l| l.parse::<Valve>()))
    //         .map(|v| v.map(|v| (v.name.to_owned(), v)))
    //         .collect::<BoxResult<HashMap<_, _>>>()?;
    //     cave
    // }

    fn part1_impl(&self, input: &mut dyn io::Read, n: usize) -> BoxResult<Output> {
        Self::process(input, n)
    }

    fn part2_impl(&self, input: &mut dyn io::Read, n: usize) -> BoxResult<Output> {
        Self::process(input, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, n: usize, f: Output) {
        assert_eq!(Day16 {}.part1_impl(&mut s.as_bytes(), n).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II",
            30,
            1651,
        );
    }

    fn test2(s: &str, n: usize, f: Output) {
        assert_eq!(Day16 {}.part2_impl(&mut s.as_bytes(), n).ok(), Some(f));
    }

    //#[test]
    fn part2() {
        test2(
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II",
            30,
            45000,
        );
    }
}
