use crate::day::*;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::str::FromStr;

pub struct Day14 {}

type Output = usize;

impl Day for Day14 {
    fn tag(&self) -> &str {
        "14"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl FromStr for Coord {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(',');
        let rv = Ok(Self {
            x: tokens.next().ok_or(AocError)?.parse()?,
            y: tokens.next().ok_or(AocError)?.parse()?,
        });
        if tokens.next().is_some() {
            Err(AocError)?;
        }
        rv
    }
}

enum Thing {
    Rock,
    Sand,
}

struct World {
    map: HashMap<Coord, Thing>,
    bottom: usize,
    floor: Option<usize>,
}

impl World {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            bottom: 0,
            floor: None,
        }
    }

    fn add_segments(&mut self, scan: &str, has_floor: bool) -> BoxResult<()> {
        for (from, to) in scan.split(" -> ").tuple_windows() {
            let (from, to) = (from.parse::<Coord>()?, to.parse::<Coord>()?);
            if from.x == to.x {
                for y in min(from.y, to.y)..=max(from.y, to.y) {
                    self.map.insert(Coord::new(from.x, y), Thing::Rock);
                    if y > self.bottom {
                        self.bottom = y;
                        if has_floor {
                            self.floor = Some(y + 2);
                        }
                    }
                }
            } else if from.y == to.y {
                for x in min(from.x, to.x)..=max(from.x, to.x) {
                    self.map.insert(Coord::new(x, from.y), Thing::Rock);
                }
                if from.y > self.bottom {
                    self.bottom = from.y;
                    if has_floor {
                        self.floor = Some(from.y + 2);
                    }
                }
            } else {
                Err(AocError)?
            }
        }
        Ok(())
    }

    fn pour(&mut self, start: Coord) -> bool {
        let mut sand = start;
        if self.map.contains_key(&sand) {
            return false;
        }
        while sand.y <= self.floor.unwrap_or(self.bottom) {
            let y = sand.y + 1;
            if let Some(floor) = self.floor {
                if floor == y {
                    self.map.insert(sand, Thing::Sand);
                    return true;
                }
            }
            let next = Coord::new(sand.x, y);
            if !self.map.contains_key(&next) {
                sand = next;
                continue;
            }
            if sand.x > 0 {
                let next = Coord::new(sand.x - 1, y);
                if !self.map.contains_key(&next) {
                    sand = next;
                    continue;
                }
            }
            let next = Coord::new(sand.x + 1, y);
            if !self.map.contains_key(&next) {
                sand = next;
                continue;
            }
            self.map.insert(sand, Thing::Sand);
            return true;
        }
        false
    }
}

impl Day14 {
    fn process(input: &mut dyn io::Read, start: Coord, has_floor: bool) -> BoxResult<Output> {
        let mut world = World::new();
        for segments in io::BufReader::new(input).lines() {
            world.add_segments(&segments?, has_floor)?;
        }
        match (0..).try_fold((), |_, i| -> Result<_, BoxResult<_>> {
            if world.pour(start) {
                Ok(())
            } else {
                Err(Ok(i))
            }
        }) {
            Err(rv) => rv,
            _ => Err(AocError.into()),
        }
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process(input, Coord::new(500, 0), false)
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process(input, Coord::new(500, 0), true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day14 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
",
            24,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day14 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
",
            93,
        );
    }
}
