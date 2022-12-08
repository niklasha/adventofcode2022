use crate::day::*;

pub struct Day08 {}

type Output = usize;

impl Day for Day08 {
    fn tag(&self) -> &str {
        "08"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

type Forest = Vec<Vec<u8>>;

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Day08 {
    fn tree(forest: &Forest, x: usize, y: usize) -> Result<u8, AocError> {
        forest
            .get(y)
            .ok_or(AocError)?
            .get(x)
            .ok_or(AocError)
            .map(|t| *t)
    }

    // fn step(p: (usize, usize), d: (i8, i8)) -> Option<(usize, usize)> {}

    fn is_visible_in_dir(
        forest: &Forest,
        xs: usize,
        ys: usize,
        x: usize,
        y: usize,
        dir: Dir,
    ) -> BoxResult<bool> {
        let h = Self::tree(forest, x, y)?;
        match dir {
            Dir::Up => {
                for yy in 0..y {
                    if Self::tree(forest, x, yy)? >= h {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            Dir::Down => {
                for yy in y + 1..ys {
                    if Self::tree(forest, x, yy)? >= h {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            Dir::Left => {
                for xx in 0..x {
                    if Self::tree(forest, xx, y)? >= h {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            Dir::Right => {
                for xx in x + 1..xs {
                    if Self::tree(forest, xx, y)? >= h {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
        }
    }

    fn is_visible(forest: &Forest, xs: usize, ys: usize, x: usize, y: usize) -> BoxResult<bool> {
        if x == 0 || y == 0 || x == xs - 1 || y == ys - 1 {
            return Ok(true);
        }
        if Self::is_visible_in_dir(forest, xs, ys, x, y, Dir::Up)? {
            return Ok(true);
        }
        if Self::is_visible_in_dir(forest, xs, ys, x, y, Dir::Down)? {
            return Ok(true);
        }
        if Self::is_visible_in_dir(forest, xs, ys, x, y, Dir::Left)? {
            return Ok(true);
        }
        if Self::is_visible_in_dir(forest, xs, ys, x, y, Dir::Right)? {
            return Ok(true);
        }
        Ok(false)
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let forest = Utils::byte_matrix(input)?;
        let ys = forest.len();
        let xs = forest.get(0).ok_or(AocError)?.len();
        (0..ys).fold(Ok(0), |c, y| {
            (0..xs).fold(c, |c, x| {
                if Self::is_visible(&forest, xs, ys, x, y)? {
                    c.map(|c| c + 1)
                } else {
                    c
                }
            })
        })
    }

    fn visible_trees(
        forest: &Forest,
        xs: usize,
        ys: usize,
        x: usize,
        y: usize,
        dir: Dir,
    ) -> BoxResult<usize> {
        let h = Self::tree(forest, x, y)?;
        match dir {
            Dir::Up => (0..y)
                .rev()
                .map(|yy| Self::tree(forest, x, yy))
                .fold(Ok((false, 0)), |state, hh| {
                    let (blocked, count) = state?;
                    Ok((
                        blocked || hh? >= h,
                        if !blocked { count + 1 } else { count },
                    ))
                })
                .map(|(_, c)| c),
            Dir::Down => ((y + 1)..ys)
                .map(|yy| Self::tree(forest, x, yy))
                .fold(Ok((false, 0)), |state, hh| {
                    let (blocked, count) = state?;
                    Ok((
                        blocked || hh? >= h,
                        if !blocked { count + 1 } else { count },
                    ))
                })
                .map(|(_, c)| c),
            Dir::Left => (0..x)
                .rev()
                .map(|xx| Self::tree(forest, xx, y))
                .fold(Ok((false, 0)), |state, hh| {
                    let (blocked, count) = state?;
                    Ok((
                        blocked || hh? >= h,
                        if !blocked { count + 1 } else { count },
                    ))
                })
                .map(|(_, c)| c),
            Dir::Right => ((x + 1)..xs)
                .map(|xx| Self::tree(forest, xx, y))
                .fold(Ok((false, 0)), |state, hh| {
                    let (blocked, count) = state?;
                    Ok((
                        blocked || hh? >= h,
                        if !blocked { count + 1 } else { count },
                    ))
                })
                .map(|(_, c)| c),
        }
    }

    fn scenic_score(forest: &Forest, xs: usize, ys: usize, x: usize, y: usize) -> BoxResult<usize> {
        Ok(Self::visible_trees(forest, xs, ys, x, y, Dir::Up)?
            * Self::visible_trees(forest, xs, ys, x, y, Dir::Down)?
            * Self::visible_trees(forest, xs, ys, x, y, Dir::Left)?
            * Self::visible_trees(forest, xs, ys, x, y, Dir::Right)?)
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let forest = Utils::byte_matrix(input)?;
        let ys = forest.len();
        let xs = forest.get(0).ok_or(AocError)?.len();
        (0..ys)
            .fold(Ok(vec![]), |v: BoxResult<_>, y| {
                (0..xs).fold(v, |v, x| {
                    let mut v = v?;
                    v.push(Self::scenic_score(&forest, xs, ys, x, y)?);
                    Ok(v)
                })
            })?
            .into_iter()
            .max()
            .ok_or_else(|| AocError.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day08 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "30373
25512
65332
33549
35390",
            21,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day08 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "30373
25512
65332
33549
35390",
            8,
        );
    }
}
