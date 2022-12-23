use crate::day::*;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::iter;
use std::str::FromStr;

pub struct Day19 {}

type Output = usize;

impl Day for Day19 {
    fn tag(&self) -> &str {
        "19"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        //println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

struct OreCost {
    ore: usize,
}

struct ClayCost {
    ore: usize,
}

struct ObsidianCost {
    ore: usize,
    clay: usize,
}

struct GeodeCost {
    ore: usize,
    obsidian: usize,
}

struct Blueprint {
    id: usize,
    ore: OreCost,
    clay: ClayCost,
    obsidian: ObsidianCost,
    geode: GeodeCost,
}

impl FromStr for Blueprint {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().replace(":", "");
        let mut tokens = s.split_whitespace();
        let id = tokens.nth(1).ok_or(AocError)?.parse::<usize>()?;
        let ore_ore = tokens.nth(4).ok_or(AocError)?.parse::<usize>()?;
        let clay_ore = tokens.nth(5).ok_or(AocError)?.parse::<usize>()?;
        let obsidian_ore = tokens.nth(5).ok_or(AocError)?.parse::<usize>()?;
        let obsidian_clay = tokens.nth(2).ok_or(AocError)?.parse::<usize>()?;
        let geode_ore = tokens.nth(5).ok_or(AocError)?.parse::<usize>()?;
        let geode_obsidian = tokens.nth(2).ok_or(AocError)?.parse::<usize>()?;
        if tokens.count() != 1 {
            Err(AocError)?
        } else {
            Ok(Blueprint {
                id,
                ore: OreCost { ore: ore_ore },
                clay: ClayCost { ore: clay_ore },
                obsidian: ObsidianCost {
                    ore: obsidian_ore,
                    clay: obsidian_clay,
                },
                geode: GeodeCost {
                    ore: geode_ore,
                    obsidian: geode_obsidian,
                },
            })
        }
    }
}

impl Blueprint {
    fn process(&self, n: usize, do_quality: bool) -> BoxResult<usize> {
        let (inventories) = (0..n).fold(
            (iter::once(Inventory {
                ore_robot: 1,
                ..Default::default()
            })
            .collect::<HashSet<_>>()),
            |state, _t| {
                println!("{}, {:?}", _t, state.len());
                let (inventories) = state;
                let inventories = inventories
                    .into_iter()
                    .flat_map(|inventory| inventory.spend(self).into_iter())
                    .collect();
                (inventories)
            },
        );
        Ok(if do_quality { self.id } else { 1 }
            * inventories
                .iter()
                .map(|Inventory { geode, .. }| geode)
                .max()
                .ok_or(AocError)?)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Inventory {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
    ore_robot: usize,
    clay_robot: usize,
    obsidian_robot: usize,
    geode_robot: usize,
}

impl Inventory {
    fn spend(self, blueprint: &Blueprint) -> HashSet<Inventory> {
        let mut inventories = HashSet::new();
        let mut new = self;
        let (mut ng, mut nob, mut nc, mut no) = (0, 0, 0, 0);
        ng = min(
            new.obsidian / blueprint.geode.obsidian,
            new.ore / blueprint.geode.ore,
        );
        new.obsidian -= ng * blueprint.geode.obsidian;
        new.ore -= ng * blueprint.geode.ore;
        nob = min(
            new.clay / blueprint.obsidian.clay,
            new.ore / blueprint.obsidian.ore,
        );
        new.clay -= nob * blueprint.obsidian.clay;
        new.ore -= nob * blueprint.obsidian.ore;
        nc = new.ore / blueprint.clay.ore;
        if nc > 0 {
            let mut new = self;
            new.ore -= nc * blueprint.clay.ore;
            no = new.ore / blueprint.ore.ore;
            if no > 0 {
                let mut new = self;
                new.ore -= no * blueprint.ore.ore;
                new.collect();
                new.geode_robot += ng;
                new.obsidian_robot += nob;
                new.clay_robot += nc;
                new.ore_robot += no;
                inventories.insert(new);
            }
            new.collect();
            new.geode_robot += ng;
            new.obsidian_robot += nob;
            new.clay_robot += nc;
            inventories.insert(new);
        }
        no = new.ore / blueprint.ore.ore;
        if no > 0 {
            let mut new = self;
            new.ore -= no * blueprint.ore.ore;
            new.collect();
            new.geode_robot += ng;
            new.obsidian_robot += nob;
            new.clay_robot += nc;
            new.ore_robot += no;
            inventories.insert(new);
        }
        // new.collect();
        // new.geode_robot += ng;
        // new.obsidian_robot += nob;
        // inventories.insert(new);
        inventories
    }

    fn collect(&mut self) {
        self.ore += self.ore_robot;
        self.clay += self.clay_robot;
        self.obsidian += self.obsidian_robot;
        self.geode += self.geode_robot;
    }
}

impl Day19 {
    fn process(input: &mut dyn io::Read, n: usize, do_quality: bool) -> BoxResult<Output> {
        io::BufReader::new(input)
            .lines()
            .map(|l| l.map_err(|e| e.into()))
            .map(|l: BoxResult<_>| {
                l.and_then(|l| l.as_str().parse::<Blueprint>()?.process(n, do_quality))
            })
            .sum()
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        io::BufReader::new(input)
            .lines()
            .map(|l| l.map_err(|e| e.into()))
            .map(|l: BoxResult<_>| {
                l.and_then(|l| l.as_str().parse::<Blueprint>()?.process(24, true))
            })
            .sum()
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        io::BufReader::new(input)
            .lines()
            .take(3)
            .map(|l| l.map_err(|e| e.into()))
            .map(|l: BoxResult<_>| {
                l.and_then(|l| l.as_str().parse::<Blueprint>()?.process(32, false))
            })
            .product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day19 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.",
            33,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day19 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.",
              56 * 62);
    }
}
