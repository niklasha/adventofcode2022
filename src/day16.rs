use crate::day::*;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::io::Read;
use std::iter;
use std::ops::Add;
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
        println!("{:?}", self.part2_impl(&mut *input(), 26));
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

struct Cave {
    name_to_valve: HashMap<String, u8>,
    openable: Vec<u8>,
    neighbours: Vec<u64>,
    rate: Vec<Output>,
}

const OPENABLE_COUNT: usize = 16;
const NOT_OPENABLE: u8 = (OPENABLE_COUNT - 1) as u8;

impl Cave {
    fn parse(input: &mut dyn Read) -> BoxResult<Self> {
        let valves: Vec<Valve> = io::BufReader::new(input)
            .lines()
            .map(|l| l.map_err(|e| e.into()))
            .map(|l| l.and_then(|l| l.parse::<Valve>()))
            .collect::<BoxResult<_>>()?;
        assert!(valves.len() <= u8::MAX as usize + 1);
        let name_to_valve: HashMap<String, u8> = valves
            .iter()
            .enumerate()
            .map(|(i, _)| (valves[i].name.to_owned(), i as u8))
            .collect();
        assert!(valves.iter().filter(|v| v.rate > 0).count() <= NOT_OPENABLE as usize);
        let mut openable = vec![NOT_OPENABLE; valves.len()];
        for (j, (i, _)) in valves
            .iter()
            .enumerate()
            .filter(|(_, v)| v.rate > 0)
            .enumerate()
        {
            openable[i] = j as u8;
        }
        let neighbours = valves
            .iter()
            .map(|v| {
                v.neighbours.iter().fold(Ok(0), |set, neighbour| {
                    set.and_then(
                        |set| Ok(set | 1 << *name_to_valve.get(neighbour).ok_or(AocError)?),
                    )
                })
            })
            .collect::<BoxResult<_>>()?;
        let rate = valves
            .iter()
            .filter(|v| v.rate > 0)
            .map(|v| v.rate)
            .collect::<Vec<_>>();
        // println!("{:?}", name_to_valve);
        // println!("{:?}", openable);
        // println!("{:?}", neighbours);
        // println!("{:?}", rate);
        Ok(Cave {
            name_to_valve,
            openable,
            neighbours,
            rate,
        })
    }

    fn traverse(
        &self,
        valve_no: u8,
        valves_state: u16,
        visited: &mut [u8],
        t: usize,
        eol: usize,
        flow: usize,
        flow_max: &mut usize,
    ) {
        if t > eol {
            if flow > *flow_max {
                *flow_max = flow;
                //                println!("NEW MAX {}", flow);
            }
            return;
        }
        let index = (valve_no as usize) << OPENABLE_COUNT | valves_state as usize;
        let offset = index / 8;
        let visit_mask = 1 << (index % 8) as u16;
        if (visited[offset] & visit_mask) == 0 {
            visited[offset] |= visit_mask;
            let delta = (0..NOT_OPENABLE)
                .filter(|valve_no| valves_state & (1 << valve_no) != 0)
                .map(|valve_no| self.rate[valve_no as usize])
                .sum::<usize>();
            // println!(
            //     "t {} valve {} states {} flow {} delta {}",
            //     t, valve_no, valves_state, flow, delta
            // );
            let flow = flow + delta;
            let openable_valve_no = self.openable[valve_no as usize] as u16;
            if openable_valve_no != NOT_OPENABLE as u16 {
                let mask = 1 << openable_valve_no;
                let bit = valves_state & mask;
                if bit == 0 {
                    self.traverse(
                        valve_no,
                        valves_state | mask,
                        visited,
                        t + 1,
                        eol,
                        flow,
                        flow_max,
                    );
                }
            }
            let mut neighbours = self.neighbours[valve_no as usize];
            let mut neighbour = 0u8;
            while neighbours != 0 {
                let ffs = neighbours.trailing_zeros() as u8;
                neighbour += ffs;
                self.traverse(neighbour, valves_state, visited, t + 1, eol, flow, flow_max);
                neighbours >>= ffs + 1;
                neighbour += 1;
            }
            visited[offset] &= !visit_mask;
        }
    }

    fn single_action(&self, p: Player, s: State) -> HashSet<StateDiff> {
        let cur = p as Valve2;
        let mut diffs = HashSet::new();
        // When all valves has been opened there is no need to move anymore
        if s.valves.count_ones() as usize == self.rate.len() {
            diffs.insert(StateDiff::Noop);
        } else {
            let openable_valve_no = self.openable[cur as usize] as Valve2;
            if openable_valve_no != NOT_OPENABLE && (s.valves & (1 << cur)) == 0 {
                diffs.insert(StateDiff::Open(cur));
            }
            let mut neighbours = self.neighbours[cur as usize];
            let mut neighbour = 0 as Valve2;
            while neighbours != 0 {
                let ffs = neighbours.trailing_zeros() as Valve2;
                neighbour += ffs;
                diffs.insert(StateDiff::Travel(p, neighbour));
                neighbours >>= ffs + 1;
                neighbour += 1;
            }
        }
        diffs
    }

    fn traverse2(
        &self,
        states: HashSet<(State, usize)>,
        seen: &mut HashSet<State>,
        t: usize,
        eol: usize,
        flow_max: &mut usize,
    ) {
        *flow_max = states.iter().map(|(_, flow)| *flow).max().unwrap_or(0);
        //        println!("traverse2 t {} max {}", t, flow_max);
        if t > eol {
            return;
        }
        let mut new_states = HashSet::new();
        for (state, flow) in states {
            let delta = (0..self.openable.len())
                .filter(|valve_no| (state.valves & (1 << valve_no)) != 0)
                .map(|valve_no| self.rate[self.openable[valve_no] as usize])
                .sum::<usize>();
            let flow = flow + delta;
            // XXX Ugly heuristic to speed up the process by pruning unlikely
            // XXX branches.
            if flow + self.rate.iter().sum::<usize>() * (eol - t + 1) < 3000 {
                continue;
            }
            for action0 in self.single_action(state.duo.player0(), state) {
                for action1 in self.single_action(state.duo.player1(), state) {
                    let new = state + action0 + action1;
                    new_states.insert((new, flow));
                    if !seen.contains(&new) {
                        seen.insert(new);
                        //println!("  {:?} {:?}", action0, action1);
                    }
                }
            }
        }
        self.traverse2(new_states, seen, t + 1, eol, flow_max);
    }
}

type Valve2 = u8;
type ValveSet = u64;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum StateDiff {
    Noop,
    Open(Valve2),
    Travel(Player, Valve2),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct State {
    duo: Duo,
    valves: ValveSet,
}

impl Add<StateDiff> for State {
    type Output = State;

    fn add(mut self, rhs: StateDiff) -> Self::Output {
        match rhs {
            StateDiff::Noop => {}
            StateDiff::Open(v) => {
                self.valves |= 1 << v as ValveSet;
            }
            StateDiff::Travel(src, dst) => self.duo.travel(src, dst),
        }
        self
    }
}

type Player = Valve2;
type PlayerSet = ValveSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Duo {
    players: PlayerSet,
}

impl Duo {
    fn new(a: Player, b: Player) -> Self {
        Self {
            players: (1 << a as PlayerSet) | (1 << b as PlayerSet),
        }
    }
    fn player0(&self) -> Player {
        self.players.trailing_zeros() as Player
    }
    fn player1(&self) -> Player {
        (u64::BITS - self.players.leading_zeros() - 1) as Player
    }
    fn travel(&mut self, src: Player, dst: Valve2) {
        // If both players is at the same valve, the mask will be a
        // power of two, and we should retain the source as occupied.
        if !self.players.is_power_of_two() {
            self.players &= !(1 << src as PlayerSet);
        }
        self.players |= 1 << dst as PlayerSet;
    }
}

impl Display for Duo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Duo[{} {}]", self.player0(), self.player1())
    }
}

impl Day16 {
    fn process(input: &mut dyn io::Read, n: usize) -> BoxResult<Output> {
        let cave = Cave::parse(input)?;
        let mut flow_max = 0;
        cave.traverse(
            *cave.name_to_valve.get("AA").ok_or(AocError)?,
            0,
            &mut [0; 64 * (u16::MAX as usize + 1) / 8],
            1,
            n,
            0,
            &mut flow_max,
        );
        Ok(flow_max)
    }

    fn process2(input: &mut dyn io::Read, n: usize) -> BoxResult<Output> {
        let cave = Cave::parse(input)?;
        let start = *cave.name_to_valve.get("AA").ok_or(AocError)?;
        // println!(
        //     "full_rate {} start {}",
        //     cave.rate.iter().sum::<Output>(),
        //     start
        // );
        let mut flow_max = 0;
        cave.traverse2(
            iter::once((
                State {
                    duo: Duo::new(start, start),
                    valves: 0,
                },
                0,
            ))
            .collect(),
            &mut HashSet::new(),
            1,
            n,
            &mut flow_max,
        );
        Ok(flow_max)
    }

    fn part1_impl(&self, input: &mut dyn io::Read, n: usize) -> BoxResult<Output> {
        Self::process(input, n)
    }

    fn part2_impl(&self, input: &mut dyn io::Read, n: usize) -> BoxResult<Output> {
        Self::process2(input, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, n: usize, np: usize, f: Output) {
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

    fn test2(s: &str, n: usize, np: usize, f: Output) {
        assert_eq!(Day16 {}.part2_impl(&mut s.as_bytes(), n, np).ok(), Some(f));
    }

    #[test]
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
            26,
            1707,
        );
    }
}
