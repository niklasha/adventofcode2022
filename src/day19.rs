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
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

#[derive(Debug)]
struct OreCost {
    ore: usize,
}

#[derive(Debug)]
struct ClayCost {
    ore: usize,
}

#[derive(Debug)]
struct ObsidianCost {
    ore: usize,
    clay: usize,
}

#[derive(Debug)]
struct GeodeCost {
    ore: usize,
    obsidian: usize,
}

#[derive(Debug)]
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
        let s = s.trim().replace(':', "");
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
            let blueprint = Blueprint {
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
            };
            //println!("{:?}", blueprint);
            Ok(blueprint)
        }
    }
}

impl Blueprint {
    fn process(&self, n: usize, do_quality: bool) -> BoxResult<usize> {
        let inventories = (0..n).fold(
            iter::once(Inventory {
                ore_robot: 1,
                ..Default::default()
            })
            .collect::<HashSet<_>>(),
            |inventories, _t| {
                // println!("{}, {:?}", _t, inventories.len());
                let inventories: HashSet<_> = inventories
                    .into_iter()
                    .flat_map(|inventory| inventory.spend(self).into_iter())
                    .collect();
                // println!(
                //     "{:?}",
                //     inventories.iter().map(|inventory| inventory.geode).max()
                // );
                inventories
            },
        );
        let rv = if do_quality { self.id } else { 1 }
            * inventories
                .iter()
                .map(|Inventory { geode, .. }| geode)
                .max()
                .ok_or(AocError)?;
        // println!("{}", rv);
        Ok(rv)
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
        //println!("spend {:?}", self);
        let mut inventories = HashSet::new();
        let mut temp = self;
        temp.collect();
        let geode_robots_creatable = min(
            self.obsidian / blueprint.geode.obsidian,
            self.ore / blueprint.geode.ore,
        );
        let obsidian_robots_creatable = min(
            self.clay / blueprint.obsidian.clay,
            self.ore / blueprint.obsidian.ore,
        );
        let clay_robots_creatable = self.ore / blueprint.clay.ore;
        let ore_robots_creatable = self.ore / blueprint.ore.ore;
        if geode_robots_creatable >= 1 {
            let mut new = temp;
            new.obsidian -= blueprint.geode.obsidian;
            new.ore -= blueprint.geode.ore;
            new.geode_robot += 1;
            inventories.insert(new);
        }
        // Heuristic: Only allow collecting obsidian, when we can build at most
        // one geode robot, but not if we already have reached the needed
        // capacity per turn.
        if obsidian_robots_creatable >= 1
            && geode_robots_creatable < 2
            && self.obsidian_robot < blueprint.geode.obsidian
        {
            let mut new = temp;
            new.clay -= blueprint.obsidian.clay;
            new.ore -= blueprint.obsidian.ore;
            new.obsidian_robot += 1;
            inventories.insert(new);
        }
        // Heuristic: Only allow collecting clay, when we can build at most one
        // obsidian robot, but not if we already have reached the needed
        // capacity per turn.
        if clay_robots_creatable >= 1
            && obsidian_robots_creatable < 2
            && self.clay_robot < blueprint.obsidian.clay
        {
            let mut new = temp;
            new.ore -= blueprint.clay.ore;
            new.clay_robot += 1;
            inventories.insert(new);
        }
        // Heuristic: Only allow collecting ore, if we have not already reached
        // the needed capacity to build something otherwise buildable per turn.
        if ore_robots_creatable >= 1
            && self.ore_robot
                < max(
                    max(blueprint.ore.ore, blueprint.clay.ore),
                    max(
                        if self.clay >= blueprint.obsidian.clay {
                            blueprint.obsidian.ore
                        } else {
                            0
                        },
                        if self.obsidian >= blueprint.geode.obsidian {
                            blueprint.geode.ore
                        } else {
                            0
                        },
                    ),
                )
        {
            let mut new = temp;
            new.ore -= blueprint.ore.ore;
            new.ore_robot += 1;
            inventories.insert(new);
        }
        // Heuristic: Only allow waiting when we must, or only have one other
        // option.
        if geode_robots_creatable
            + obsidian_robots_creatable
            + clay_robots_creatable
            + ore_robots_creatable
            < 2
        {
            inventories.insert(temp);
        }
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
