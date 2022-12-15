use crate::day::*;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::io::Read;
use std::num::ParseIntError;
use std::ops::RangeInclusive;
use std::str::FromStr;

pub struct Day15 {}

type Output = usize;

impl Day for Day15 {
    fn tag(&self) -> &str {
        "15"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input(), 2000000));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input(), 0, 4000000));
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Coord) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
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

#[derive(Clone, Debug)]
struct Extents {
    vec: Vec<RangeInclusive<i64>>,
}

impl Extents {
    const fn new() -> Self {
        Self { vec: vec![] }
    }

    fn add_extent(self, new: RangeInclusive<i64>) -> Self {
        let (mut distinct, overlap): (Vec<_>, Vec<_>) = self
            .vec
            .into_iter()
            .partition(|old| old.end() < new.start() || old.start() > new.end());
        if let Some(first) = overlap.first() {
            let left = min(first.start(), new.start());
            let right = max(overlap.last().unwrap().end(), new.end()); // XXX unwrap, although safe.
            distinct.push(*left..=*right);
        } else {
            distinct.push(new);
        }
        distinct.sort_by_key(|r| *r.start());
        Extents { vec: distinct }
    }

    fn len(&self) -> usize {
        self.vec
            .iter()
            .map(|r| r.end() - r.start() + 1)
            .sum::<i64>() as usize
    }
}

impl Day15 {
    fn parse(s: &str) -> BoxResult<i64> {
        let r: BoxResult<_> = s.get(2..).ok_or_else(|| AocError.into());
        r?.trim_end_matches(|c: char| !c.is_ascii_digit())
            .parse()
            .map_err(|e: ParseIntError| e.into())
    }

    fn process(
        input: &mut dyn io::Read,
        y: i64,
        limits: Option<(i64, i64)>,
    ) -> BoxResult<(Output, Output, Option<usize>)> {
        let mut set = Extents::new();
        let mut bx = HashSet::new();
        for report in io::BufReader::new(input).lines().map(|r| {
            r.map(|report| {
                let mut tokens = report.split_whitespace();
                let t = &mut tokens;
                let sensor: BoxResult<_> = t
                    .skip(2)
                    .take(2)
                    .map(Self::parse)
                    .tuples()
                    .map(|(x, y)| Ok(Coord::new(x?, y?)))
                    .next()
                    .ok_or_else(|| AocError.into())
                    .and_then(|r| r);
                let beacon: BoxResult<_> = tokens
                    .skip(4)
                    .take(2)
                    .map(Self::parse)
                    .tuples()
                    .map(|(x, y)| Ok(Coord::new(x?, y?)))
                    .next()
                    .ok_or_else(|| AocError.into())
                    .and_then(|r| r);
                (sensor, beacon)
            })
        }) {
            let (sensor, beacon) = report?;
            let (sensor, beacon) = (sensor?, beacon?);
            if beacon.y == y {
                //                println!("beacon at {:?}", beacon);
                bx.insert(beacon.x);
            }
            let d = sensor.distance(&beacon);
            let dy = (sensor.y - y).abs();
            let w = d - dy;
            if w > 0 {
                let x1 = sensor.x - w;
                let x2 = sensor.x + w;
                let rx = if let Some((a, b)) = limits {
                    max(x1, a)..=min(x2, b)
                } else {
                    x1..=x2
                };
                set = set.add_extent(rx);
            }
        }
        let x = limits.and_then(|(a, b)| {
            if set.len() != (b - a + 1) as usize {
                Some(
                    (set.vec[0].end() + 1) as usize, // XXX panic potential
                )
            } else {
                None
            }
        });
        Ok((set.len(), bx.len(), x))
    }

    fn part1_impl(&self, input: &mut dyn io::Read, y: i64) -> BoxResult<Output> {
        Self::process(input, y, None).map(|(a, b, _)| a - b)
    }

    fn part2_impl(&self, input: &mut dyn io::Read, a: i64, b: i64) -> BoxResult<Output> {
        let input = io::BufReader::new(input)
            .bytes()
            .collect::<Result<Vec<u8>, _>>()?;
        for y in a..=b {
            let (_, _, x) = Self::process(&mut input.as_slice(), y, Some((a, b)))?;
            if let Some(x) = x {
                return Ok(x * 4000000 + y as usize);
            }
        }
        Err(AocError.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, y: i64, f: Output) {
        assert_eq!(Day15 {}.part1_impl(&mut s.as_bytes(), y).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
",
            10,
            26,
        );
    }

    fn test2(s: &str, a: i64, b: i64, f: Output) {
        assert_eq!(Day15 {}.part2_impl(&mut s.as_bytes(), a, b).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
",
            0,
            20,
            56000011,
        );
    }
}
