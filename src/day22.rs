use crate::day::*;
use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

pub struct Day22 {}

type Output = usize;

impl Day for Day22 {
    fn tag(&self) -> &str {
        "22"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

#[derive(Debug)]
struct Board {
    map: Vec<Vec<Option<u8>>>,
    horizontal: Vec<(usize, usize)>,
    vertical: Vec<(usize, usize)>,
}

impl Board {
    fn starting_position(&self) -> Result<(usize, usize, usize), AocError> {
        Ok((0, self.horizontal.get(0).ok_or(AocError)?.0, 0))
    }

    fn walk(
        &self,
        pos: (usize, usize, usize),
        distance: usize,
    ) -> Result<(usize, usize, usize), AocError> {
        let (row, column, facing) = pos;
        let row_limits = self.vertical.get(column).ok_or(AocError)?;
        let row_width = row_limits.1 - row_limits.0;
        let column_limits = self.horizontal.get(row).ok_or(AocError)?;
        let column_width = column_limits.1 - column_limits.0;
        println!(
            "{} {} {} {:?} {:?} {}",
            row, column, facing, row_limits, column_limits, distance
        );
        Ok(match facing {
            0 => {
                let mut c = column - column_limits.0;
                for i in 0..distance {
                    let nc = (c + 1) % column_width;
                    if self
                        .map
                        .get(row)
                        .ok_or(AocError)?
                        .get(nc + column_limits.0)
                        .ok_or(AocError)?
                        .ok_or(AocError)?
                        == b'#'
                    {
                        break;
                    }
                    c = nc;
                }
                (row, c + column_limits.0, facing)
            }
            1 => {
                let mut r = row - row_limits.0;
                for i in 0..distance {
                    let nr = (r + 1) % row_width;
                    if self
                        .map
                        .get(nr + row_limits.0)
                        .ok_or(AocError)?
                        .get(column)
                        .ok_or(AocError)?
                        .ok_or(AocError)?
                        == b'#'
                    {
                        break;
                    }
                    r = nr;
                }
                (r + row_limits.0, column, facing)
            }
            2 => {
                let mut c = column - column_limits.0;
                for i in 0..distance {
                    let nc = (c + column_width - 1) % column_width;
                    if self
                        .map
                        .get(row)
                        .ok_or(AocError)?
                        .get(nc + column_limits.0)
                        .ok_or(AocError)?
                        .ok_or(AocError)?
                        == b'#'
                    {
                        break;
                    }
                    c = nc;
                }
                (row, c + column_limits.0, facing)
            }
            3 => {
                let mut r = row - row_limits.0;
                for i in 0..distance {
                    let nr = (r + row_width - 1) % row_width;
                    if self
                        .map
                        .get(nr + row_limits.0)
                        .ok_or(AocError)?
                        .get(column)
                        .ok_or(AocError)?
                        .ok_or(AocError)?
                        == b'#'
                    {
                        break;
                    }
                    r = nr;
                }
                (r + row_limits.0, column, facing)
            }
            _ => Err(AocError)?,
        })
    }

    fn travel(
        &self,
        pos: (usize, usize, usize),
        moves: &Vec<Move>,
    ) -> Result<(usize, usize, usize), AocError> {
        moves.iter().fold(Ok(pos), |pos, m| {
            let pos = pos?;
            Ok(match m {
                Move::Walk(distance) => self.walk(pos, *distance)?,
                Move::TurnLeft => (pos.0, pos.1, (pos.2 + 3).rem_euclid(4)),
                Move::TurnRight => (pos.0, pos.1, (pos.2 + 1).rem_euclid(4)),
            })
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Move {
    Walk(usize),
    TurnLeft,
    TurnRight,
}

impl Day22 {
    fn parse(input: &mut dyn io::Read) -> BoxResult<(Board, Vec<Move>)> {
        let mut bytes = io::BufReader::new(input)
            .split(b'\n')
            .map(|l| l.map_err(|e| e.into()))
            .collect::<BoxResult<Vec<_>>>()?;
        let (moves, map) = bytes.split_last_mut().ok_or(AocError)?;
        let (blank_line, map) = map.split_last_mut().ok_or(AocError)?;
        if !blank_line.is_empty() {
            Err(AocError)?
        }
        let len = map.get(0).ok_or(AocError)?.len();
        let map = map
            .iter_mut()
            .map(|l| {
                l.resize(len, b' ');
                l.iter()
                    .map(|b| {
                        Ok(match b {
                            b' ' => None,
                            b'#' | b'.' => Some(*b),
                            _ => Err(AocError)?,
                        })
                    })
                    .collect::<BoxResult<Vec<_>>>()
            })
            .collect::<Result<Vec<_>, _>>()?;
        let horizontal = map
            .iter()
            .map(|l| {
                let left = l.iter().position(|t| t.is_some()).ok_or(AocError)?;
                let right = l.iter().rposition(|t| t.is_some()).ok_or(AocError)?;
                Ok((left, right + 1))
            })
            .collect::<BoxResult<Vec<_>>>()?;
        let vertical = (0..map.first().ok_or(AocError)?.len())
            .map(|i| {
                let column = map
                    .iter()
                    .map(|l| l.get(i).ok_or(AocError))
                    .collect::<Result<Vec<_>, _>>()?;
                let top = column.iter().position(|t| t.is_some()).ok_or(AocError)?;
                let bottom = column.iter().rposition(|t| t.is_some()).ok_or(AocError)?;
                Ok((top, bottom + 1))
            })
            .collect::<BoxResult<Vec<_>>>()?;
        let moves = moves.iter().fold(Ok(Vec::new()), |r: BoxResult<_>, b| {
            let mut moves = r?;
            match *b {
                b'0'..=b'9' => {
                    let n = (b - b'0') as usize;
                    let last = moves.pop();
                    let distance = if let Some(Move::Walk(distance)) = last {
                        distance * 10 + n
                    } else {
                        last.map(|last| moves.push(last));
                        n
                    };
                    moves.push(Move::Walk(distance))
                }
                b'L' => moves.push(Move::TurnLeft),
                b'R' => moves.push(Move::TurnRight),
                _ => Err(AocError)?,
            }
            Ok(moves)
        })?;
        Ok((
            Board {
                map,
                horizontal,
                vertical,
            },
            moves,
        ))
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let (board, moves) = Self::parse(input)?;
        println!("{:?}", moves);
        let pos = board.travel(board.starting_position()?, &moves)?;
        Ok((pos.0 + 1) * 1000 + (pos.1 + 1) * 4 + pos.2)
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        //let mut choir = Choir::from(input)?;
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day22 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5",
            6032,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day22 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("", 5031);
    }
}
