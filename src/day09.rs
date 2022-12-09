use crate::day::*;
use std::collections::HashSet;
use std::iter;
use std::str::FromStr;

pub struct Day09 {}

type Output = usize;

impl Day for Day09 {
    fn tag(&self) -> &str {
        "09"
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
    x: i64,
    y: i64,
}

impl Coord {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn step(self, dir: Dir) -> Self {
        match dir {
            Dir::Up => Self::new(self.x, self.y - 1),
            Dir::Down => Self::new(self.x, self.y + 1),
            Dir::Left => Self::new(self.x - 1, self.y),
            Dir::Right => Self::new(self.x + 1, self.y),
        }
    }

    fn move_towards(self, target: &Self) -> Self {
        let xd = target.x - self.x;
        let yd = target.y - self.y;
        if xd.abs() <= 1 && yd.abs() <= 1 {
            self
        } else {
            Self::new(self.x + xd.signum(), self.y + yd.signum())
        }
    }
}

#[derive(Debug, Default)]
struct Rope {
    parts: Vec<Coord>,
}

impl Rope {
    fn new(length: usize) -> Self {
        Self {
            parts: vec![Coord::default(); length],
        }
    }
}

#[derive(Debug, Default)]
struct State {
    visited: HashSet<Coord>,
    rope: Rope,
}

impl State {
    fn new(length: usize) -> Self {
        Self {
            rope: Rope::new(length),
            ..Default::default()
        }
    }

    fn step(mut self, dir: Dir) -> Result<Self, AocError> {
        let parts = &mut self.rope.parts;
        let head = parts.get_mut(0).ok_or(AocError)?;
        //        println!("head from {:?}", head);
        *head = head.step(dir);
        //        println!("head to {:?}", head);
        for i in 1..parts.len() {
            let target = parts.get(i - 1).ok_or(AocError)?.to_owned();
            let part = parts.get_mut(i).ok_or(AocError)?;
            //            println!("part {} from {:?}", i, part);
            *part = part.move_towards(&target);
            //            println!("part {} to {:?}", i, part);
        }
        self.visited.insert(*parts.last().ok_or(AocError)?);
        Ok(self)
    }
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Dir {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "U" => Dir::Up,
            "D" => Dir::Down,
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => Err(AocError)?,
        })
    }
}

impl Day09 {
    fn process(input: &mut dyn io::Read, n: usize) -> BoxResult<Output> {
        io::BufReader::new(input)
            .lines()
            .map(|r| {
                let r: BoxResult<_> = r.map_err(|e| e.into());
                r
            })
            .map(|l| {
                l.and_then(|l| {
                    let t = l
                        .split_whitespace()
                        .collect_tuple::<(_, _)>()
                        .ok_or_else(|| AocError.into())
                        .and_then(|(dir, count)| {
                            Ok((dir.parse::<Dir>()?, count.parse::<usize>()?))
                        });
                    t
                })
            })
            .fold(Ok(State::new(n)), |state, motion| {
                motion.and_then(|motion| {
                    //println!("{:?}", motion);
                    let (dir, count) = motion;
                    iter::repeat(dir).take(count).fold(state, |state, dir| {
                        state.and_then(|state| state.step(dir).map_err(|e| e.into()))
                    })
                })
            })
            .map(|state| state.visited.len())
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process(input, 2)
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process(input, 10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day09 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
            13,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day09 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
            1,
        );
        test2(
            "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
            36,
        );
    }
}
