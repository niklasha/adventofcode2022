use crate::day::*;
use std::collections::HashSet;
use std::io::Read;
use std::iter;
use std::ops::ControlFlow;
use std::str::FromStr;

pub struct Day18 {}

type Output = usize;

impl Day for Day18 {
    fn tag(&self) -> &str {
        "18"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Side {
    Left,
    Right,
    Front,
    Rear,
    Bottom,
    Top,
}

impl Side {
    fn opposite(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Front => Self::Rear,
            Self::Rear => Self::Front,
            Self::Bottom => Self::Top,
            Self::Top => Self::Bottom,
        }
    }

    fn to_xyz(self) -> (i64, i64, i64) {
        match self {
            Self::Left => (-1, 0, 0),
            Self::Right => (1, 0, 0),
            Self::Front => (0, -1, 0),
            Self::Rear => (0, 1, 0),
            Self::Bottom => (0, 0, -1),
            Self::Top => (0, 0, 1),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Cube {
    x: i64,
    y: i64,
    z: i64,
}

impl FromStr for Cube {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(',');
        let cube = Cube::new(
            tokens.next().ok_or(AocError)?.parse()?,
            tokens.next().ok_or(AocError)?.parse()?,
            tokens.next().ok_or(AocError)?.parse()?,
        );
        if tokens.next().is_some() {
            Err(AocError)?
        }
        Ok(cube)
    }
}

impl Cube {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn is_adjacent_to(&self, other: &Cube) -> bool {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs() == 1
    }

    fn neighbours(&self) -> HashSet<(Side, Cube)> {
        [
            Side::Left,
            Side::Right,
            Side::Rear,
            Side::Front,
            Side::Bottom,
            Side::Top,
        ]
        .into_iter()
        .map(|side| {
            let (dx, dy, dz) = side.to_xyz();
            (side, Cube::new(self.x + dx, self.y + dy, self.z + dz))
        })
        .collect()
    }

    fn is_inside_bbox(&self, bbox: &(i64, i64, i64, i64, i64, i64)) -> bool {
        let (x0, x1, y0, y1, z0, z1) = *bbox;
        self.x >= x0 && self.x <= x1 && self.y >= y0 && self.y <= y1 && self.z >= z0 && self.z <= z1
    }
}

struct Droplet {
    cubes: Vec<Cube>,
    area: usize,
}

impl FromStr for Droplet {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cubes = s
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|cube| cube.parse::<Cube>())
            .collect::<BoxResult<Vec<_>>>()?;
        let (area, _) =
            cubes
                .iter()
                .fold((0, HashSet::<&Cube>::new()), |(area, mut seen), cube| {
                    let area = area + 6
                        - cubes
                            .iter()
                            .filter(|other| cube.is_adjacent_to(other))
                            .count();
                    seen.insert(cube);
                    (area, seen)
                });
        Ok(Self { cubes, area })
    }
}

impl Droplet {
    fn bounding_box(&self) -> Result<(i64, i64, i64, i64, i64, i64), AocError> {
        let (xs, ys, zs) = self
            .cubes
            .iter()
            .map(|cube| (cube.x, cube.y, cube.z))
            .multiunzip::<(Vec<_>, Vec<_>, Vec<_>)>();
        Ok((
            *xs.iter().min().ok_or(AocError)? - 1,
            *xs.iter().max().ok_or(AocError)? + 1,
            *ys.iter().min().ok_or(AocError)? - 1,
            *ys.iter().max().ok_or(AocError)? + 1,
            *zs.iter().min().ok_or(AocError)? - 1,
            *zs.iter().max().ok_or(AocError)? + 1,
        ))
    }

    fn surface_area(&self) -> Result<usize, AocError> {
        struct State {
            current: HashSet<Cube>,
            seen: HashSet<Cube>,
            touched: HashSet<(Cube, Side)>,
        }
        let bbox @ (x, _, y, _, z, _) = self.bounding_box()?;
        let start: HashSet<_> = iter::once(Cube::new(x, y, z)).collect();
        if let ControlFlow::Break(State { touched, .. }) = iter::repeat(()).try_fold(
            State {
                current: start.clone(),
                seen: start,
                touched: HashSet::new(),
            },
            |State {
                 current,
                 mut seen,
                 mut touched,
             },
             _| {
                let (new_touched, next): (HashSet<_>, HashSet<_>) = current
                    .iter()
                    .flat_map(|cube| {
                        (*cube)
                            .neighbours()
                            .into_iter()
                            .filter(|(_, cube)| cube.is_inside_bbox(&bbox) && !seen.contains(cube))
                            .map(|(side, cube)| {
                                if self.contains(&cube) {
                                    (Some((cube.to_owned(), side.opposite())), None)
                                } else {
                                    (None, Some(cube))
                                }
                            })
                    })
                    .unzip();
                let next: HashSet<_> = next.iter().flatten().copied().collect();
                for cube in &next {
                    seen.insert(cube.to_owned());
                }
                for new in new_touched.into_iter().flatten() {
                    touched.insert(new);
                }
                let state = State {
                    current: next,
                    seen,
                    touched,
                };
                if current.is_empty() {
                    ControlFlow::Break(state)
                } else {
                    ControlFlow::Continue(state)
                }
            },
        ) {
            Ok(touched.len())
        } else {
            Err(AocError)?
        }
    }

    fn contains(&self, cube: &Cube) -> bool {
        self.cubes.contains(cube)
    }
}

impl Day18 {
    fn parse(input: &mut dyn Read) -> BoxResult<Droplet> {
        let bytes = io::BufReader::new(input)
            .bytes()
            .map(|r| {
                let r: BoxResult<_> = r.map_err(|e| e.into());
                r
            })
            .collect::<Result<Vec<_>, _>>()?;
        String::from_utf8(bytes)?.parse()
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::parse(input).map(|droplet| droplet.area)
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::parse(input).and_then(|droplet| droplet.surface_area().map_err(|e| e.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day18 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "1,1,1
2,1,1
",
            10,
        );
        test1(
            "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
",
            64,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day18 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
",
            58,
        );
    }
}
