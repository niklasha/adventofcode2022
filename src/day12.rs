use crate::day::*;
use std::collections::HashSet;
use std::io::Read;
use std::iter;

pub struct Day12 {}

type Output = usize;

impl Day for Day12 {
    fn tag(&self) -> &str {
        "12"
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
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Map {
    area: Vec<Vec<u8>>,
    x_size: usize,
    y_size: usize,
    start: Coord,
    end: Coord,
}

impl Map {
    fn is_valid(&self, c: Coord) -> bool {
        c.x >= 0 && c.x < self.x_size as i64 && c.y >= 0 && c.y < self.y_size as i64
    }

    // Report the height of position.
    fn height(&self, c: Coord) -> Result<u8, AocError> {
        Ok(
            match *self
                .area
                .get(c.y as usize)
                .ok_or(AocError)?
                .get(c.x as usize)
                .ok_or(AocError)?
            {
                b'S' => b'a',
                b'E' => b'z',
                h => h,
            },
        )
    }

    // Take a step in the given direction, erroring out if we fall of the map.
    fn step(&self, c: Coord, dir: Dir) -> Result<Coord, AocError> {
        let next = c.step(dir);
        if self.is_valid(next) {
            Ok(next)
        } else {
            Err(AocError)
        }
    }

    // Find all moves that remains on the map, dosen't take us to where we've
    // already been, and isn't more than one unit higher than the current
    // position.
    fn candidates(&self, c: Coord, visited: &HashSet<Coord>) -> HashSet<Coord> {
        [Dir::Up, Dir::Down, Dir::Left, Dir::Right]
            .into_iter()
            .flat_map(|dir| self.step(c, dir).ok())
            .filter(|pos| !visited.contains(pos))
            .filter(|pos| self.height(*pos).unwrap() <= self.height(c).unwrap() + 1) // XXX unwrap
            .collect()
    }

    fn find_starts(&self) -> BoxResult<HashSet<Coord>> {
        (0..self.y_size as i64).fold(Ok(HashSet::new()), |starts: BoxResult<_>, y| {
            (0..self.x_size as i64).fold(starts, |starts, x| {
                let c = Coord::new(x, y);
                let mut starts = starts?;
                if self.height(c)? == b'a' {
                    starts.insert(c);
                }
                Ok(starts)
            })
        })
    }
}

impl Day12 {
    fn step_count(map: &Map, start: Coord, limit: Option<Output>) -> BoxResult<Output> {
        match (0..)
            .take_while(|i| {
                if let Some(limit) = limit {
                    *i < limit
                } else {
                    true
                }
            })
            .try_fold(
                Ok((
                    iter::once(start).collect::<HashSet<_>>(),
                    iter::once(start).collect::<HashSet<_>>(),
                )),
                |ctx, _i| {
                    // println!("{}", _i);
                    let (pos, mut visited) = ctx?;
                    let next = pos
                        .into_iter()
                        .flat_map(|pos| map.candidates(pos, &visited))
                        .collect::<HashSet<_>>();
                    for pos in next.iter() {
                        visited.insert(*pos);
                    }
                    if next.contains(&map.end) {
                        Err(Ok(_i + 1))
                    } else {
                        Ok(Ok((next, visited)))
                    }
                },
            ) {
            Err(rv) => rv,
            Ok(_) => Ok(limit.unwrap()), // XXX unwrap
        }
    }

    fn parse(input: &mut dyn Read) -> BoxResult<Map> {
        let area = Utils::byte_matrix(input)?;
        let y_size = area.len();
        let x_size = area.get(0).ok_or(AocError)?.len();
        let (start, end) = (0..y_size as i64).fold(Ok((None, None)), |ctx: BoxResult<_>, y| {
            (0..x_size as i64).fold(ctx, |ctx, x| {
                if let Ok((start, end)) = ctx {
                    Ok(
                        match area
                            .get(y as usize)
                            .ok_or(AocError)?
                            .get(x as usize)
                            .ok_or(AocError)?
                        {
                            b'S' => (Some(Coord::new(x, y)), end),
                            b'E' => (start, Some(Coord::new(x, y))),
                            _ => (start, end),
                        },
                    )
                } else {
                    ctx
                }
            })
        })?;
        let map = Map {
            area,
            x_size,
            y_size,
            start: start.ok_or(AocError)?,
            end: end.ok_or(AocError)?,
        };
        Ok(map)
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let map = Self::parse(input)?;
        Self::step_count(&map, map.start, None)
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let map = Self::parse(input)?;
        // XXX This limit should rather be maintained on the fly, by use of
        // XXX folding.
        let limit = Self::step_count(&map, map.start, None)?;
        let starts = map.find_starts()?;
        starts
            .into_iter()
            .map(|start| Self::step_count(&map, start, Some(limit)))
            .fold(Ok(None), |min, steps| {
                if let Ok(min) = min {
                    steps.map(|steps| {
                        Some(min.map_or(steps, |min| if steps < min { steps } else { min }))
                    })
                } else {
                    min
                }
            })
            .map(|min| min.unwrap()) // XXX unwrap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day12 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
            31,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day12 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
            29,
        );
    }
}
