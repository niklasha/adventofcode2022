use crate::day::*;
use std::cmp::max;
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
        println!("{:?}", self.part1_impl(&mut *input(), Board::flat_step));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input(), Board::cube_step));
    }
}

#[derive(Debug)]
struct Board {
    map: Vec<Vec<Option<u8>>>,
    horizontal: Vec<(usize, usize)>,
    vertical: Vec<(usize, usize)>,
    size: usize,
}

impl Board {
    fn from(map: Vec<Vec<Option<u8>>>) -> BoxResult<Self> {
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
        let size = max(map.len(), map.get(0).ok_or(AocError)?.len()) / 4;
        Ok(Self {
            map,
            vertical,
            horizontal,
            size,
        })
    }

    fn starting_position(&self) -> Result<(usize, usize, usize), AocError> {
        Ok((0, self.horizontal.get(0).ok_or(AocError)?.0, 0))
    }

    fn walk(
        &self,
        mut pos: (usize, usize, usize),
        distance: usize,
        step: fn(&Self, (usize, usize, usize)) -> Result<(usize, usize, usize), AocError>,
    ) -> Result<(usize, usize, usize), AocError> {
        let mut pos = pos;
        for _ in 0..distance {
            let new_pos = step(self, pos)?;
            if self
                .map
                .get(new_pos.0)
                .ok_or(AocError)?
                .get(new_pos.1)
                .ok_or(AocError)?
                .ok_or(AocError)?
                == b'#'
            {
                break;
            }
            pos = new_pos;
        }
        Ok(pos)
    }

    fn flat_step(&self, pos: (usize, usize, usize)) -> Result<(usize, usize, usize), AocError> {
        let (row, column, facing) = pos;
        let row_limits = self.vertical.get(column).ok_or(AocError)?;
        let row_width = row_limits.1 - row_limits.0;
        let column_limits = self.horizontal.get(row).ok_or(AocError)?;
        let column_width = column_limits.1 - column_limits.0;
        Ok(match facing {
            0 => (
                row,
                (column - column_limits.0 + 1) % column_width + column_limits.0,
                facing,
            ),
            1 => (
                (row - row_limits.0 + 1) % row_width + row_limits.0,
                column,
                facing,
            ),
            2 => (
                row,
                (column - column_limits.0 + column_width - 1) % column_width + column_limits.0,
                facing,
            ),
            3 => (
                (row - row_limits.0 + row_width - 1) % row_width + row_limits.0,
                column,
                facing,
            ),
            _ => Err(AocError)?,
        })
    }

    fn cube_step(&self, pos: (usize, usize, usize)) -> Result<(usize, usize, usize), AocError> {
        let (face, face_pos) = self.to_face(pos)?;
        Ok(match face {
            1 => match pos.2 {
                0 => {
                    if face_pos.1 + 1 == self.size {
                        self.from_face(2, (face_pos.0, 0, 0))?
                    } else {
                        (pos.0, pos.1 + 1, pos.2)
                    }
                }
                1 => {
                    if face_pos.0 + 1 == self.size {
                        self.from_face(3, (0, face_pos.1, 1))?
                    } else {
                        (pos.0 + 1, pos.1, pos.2)
                    }
                }
                2 => {
                    if face_pos.1 == 0 {
                        self.from_face(4, (self.size - 1 - face_pos.0, 0, 0))?
                    } else {
                        (pos.0, pos.1 - 1, pos.2)
                    }
                }
                3 => {
                    if face_pos.0 == 0 {
                        self.from_face(6, (face_pos.1, 0, 0))?
                    } else {
                        (pos.0 - 1, pos.1, pos.2)
                    }
                }
                _ => Err(AocError)?,
            },
            2 => match pos.2 {
                0 => {
                    if face_pos.1 + 1 == self.size {
                        self.from_face(5, (self.size - 1 - face_pos.0, self.size - 1, 2))?
                    } else {
                        (pos.0, pos.1 + 1, pos.2)
                    }
                }
                1 => {
                    if face_pos.0 + 1 == self.size {
                        self.from_face(3, (face_pos.1, self.size - 1, 2))?
                    } else {
                        (pos.0 + 1, pos.1, pos.2)
                    }
                }
                2 => {
                    if face_pos.1 == 0 {
                        self.from_face(1, (face_pos.0, self.size - 1, 2))?
                    } else {
                        (pos.0, pos.1 - 1, pos.2)
                    }
                }
                3 => {
                    if face_pos.0 == 0 {
                        self.from_face(6, (self.size - 1, face_pos.1, 3))?
                    } else {
                        (pos.0 - 1, pos.1, pos.2)
                    }
                }
                _ => Err(AocError)?,
            },
            3 => match pos.2 {
                0 => {
                    if face_pos.1 + 1 == self.size {
                        self.from_face(2, (self.size - 1, self.size - 1 - face_pos.0, 3))?
                    } else {
                        (pos.0, pos.1 + 1, pos.2)
                    }
                }
                1 => {
                    if face_pos.0 + 1 == self.size {
                        self.from_face(5, (0, face_pos.1, 1))?
                    } else {
                        (pos.0 + 1, pos.1, pos.2)
                    }
                }
                2 => {
                    if face_pos.1 == 0 {
                        self.from_face(4, (0, self.size - 1 - face_pos.0, 1))?
                    } else {
                        (pos.0, pos.1 - 1, pos.2)
                    }
                }
                3 => {
                    if face_pos.0 == 0 {
                        self.from_face(1, (self.size - 1, face_pos.1, 3))?
                    } else {
                        (pos.0 - 1, pos.1, pos.2)
                    }
                }
                _ => Err(AocError)?,
            },
            4 => match pos.2 {
                0 => {
                    if face_pos.1 + 1 == self.size {
                        self.from_face(5, (face_pos.0, 0, 0))?
                    } else {
                        (pos.0, pos.1 + 1, pos.2)
                    }
                }
                1 => {
                    if face_pos.0 + 1 == self.size {
                        self.from_face(6, (0, face_pos.1, 1))?
                    } else {
                        (pos.0 + 1, pos.1, pos.2)
                    }
                }
                2 => {
                    if face_pos.1 == 0 {
                        self.from_face(1, (0, face_pos.0, 1))?
                    } else {
                        (pos.0, pos.1 - 1, pos.2)
                    }
                }
                3 => {
                    if face_pos.0 == 0 {
                        self.from_face(3, (self.size - 1 - face_pos.1, 0, 0))?
                    } else {
                        (pos.0 - 1, pos.1, pos.2)
                    }
                }
                _ => Err(AocError)?,
            },
            5 => match pos.2 {
                0 => {
                    if face_pos.1 + 1 == self.size {
                        self.from_face(2, (self.size - 1 - face_pos.0, self.size - 1, 2))?
                    } else {
                        (pos.0, pos.1 + 1, pos.2)
                    }
                }
                1 => {
                    if face_pos.0 + 1 == self.size {
                        self.from_face(6, (face_pos.0, self.size - 1, 2))?
                    } else {
                        (pos.0 + 1, pos.1, pos.2)
                    }
                }
                2 => {
                    if face_pos.1 == 0 {
                        self.from_face(4, (face_pos.0, self.size - 1, 2))?
                    } else {
                        (pos.0, pos.1 - 1, pos.2)
                    }
                }
                3 => {
                    if face_pos.0 == 0 {
                        self.from_face(3, (self.size - 1, face_pos.1, 3))?
                    } else {
                        (pos.0 - 1, pos.1, pos.2)
                    }
                }
                _ => Err(AocError)?,
            },
            6 => match pos.2 {
                0 => {
                    if face_pos.1 + 1 == self.size {
                        self.from_face(5, (self.size - 1, face_pos.0, 3))?
                    } else {
                        (pos.0, pos.1 + 1, pos.2)
                    }
                }
                1 => {
                    if face_pos.0 + 1 == self.size {
                        self.from_face(2, (0, face_pos.1, 1))?
                    } else {
                        (pos.0 + 1, pos.1, pos.2)
                    }
                }
                2 => {
                    if face_pos.1 == 0 {
                        self.from_face(1, (0, face_pos.0, 1))?
                    } else {
                        (pos.0, pos.1 - 1, pos.2)
                    }
                }
                3 => {
                    if face_pos.0 == 0 {
                        self.from_face(4, (self.size - 1, face_pos.1, 3))?
                    } else {
                        (pos.0 - 1, pos.1, pos.2)
                    }
                }
                _ => Err(AocError)?,
            },
            _ => Err(AocError)?,
        })
    }

    fn cube_step_test(
        &self,
        pos: (usize, usize, usize),
    ) -> Result<(usize, usize, usize), AocError> {
        let (face, face_pos) = self.to_face_test(pos)?;
        Ok(match face {
            1 => match pos.2 {
                0 => {
                    if face_pos.1 + 1 == self.size {
                        self.from_face_test(6, (self.size - 1 - face_pos.0, self.size - 1, 2))?
                    } else {
                        (pos.0, pos.1 + 1, pos.2)
                    }
                }
                1 => {
                    if face_pos.0 + 1 == self.size {
                        self.from_face_test(4, (0, face_pos.1, 1))?
                    } else {
                        (pos.0 + 1, pos.1, pos.2)
                    }
                }
                2 => {
                    if face_pos.1 == 0 {
                        self.from_face_test(3, (0, face_pos.0, 1))?
                    } else {
                        (pos.0, pos.1 - 1, pos.2)
                    }
                }
                3 => {
                    if face_pos.0 == 0 {
                        self.from_face_test(2, (0, self.size - 1 - face_pos.1, 1))?
                    } else {
                        (pos.0 - 1, pos.1, pos.2)
                    }
                }
                _ => Err(AocError)?,
            },
            2 => match pos.2 {
                0 => {
                    if face_pos.1 + 1 == self.size {
                        self.from_face_test(3, (face_pos.0, 0, 0))?
                    } else {
                        (pos.0, pos.1 + 1, pos.2)
                    }
                }
                1 => {
                    if face_pos.0 + 1 == self.size {
                        self.from_face_test(5, (face_pos.0, self.size - 1 - face_pos.1, 3))?
                    } else {
                        (pos.0 + 1, pos.1, pos.2)
                    }
                }
                2 => {
                    if face_pos.1 == 0 {
                        self.from_face_test(6, (self.size - 1, self.size - 1 - face_pos.0, 3))?
                    } else {
                        (pos.0, pos.1 - 1, pos.2)
                    }
                }
                3 => {
                    if face_pos.0 == 0 {
                        self.from_face_test(1, (0, self.size - 1 - face_pos.1, 1))?
                    } else {
                        (pos.0 - 1, pos.1, pos.2)
                    }
                }
                _ => Err(AocError)?,
            },
            3 => match pos.2 {
                0 => {
                    if face_pos.1 + 1 == self.size {
                        self.from_face_test(4, (face_pos.0, 0, 0))?
                    } else {
                        (pos.0, pos.1 + 1, pos.2)
                    }
                }
                1 => {
                    if face_pos.0 + 1 == self.size {
                        self.from_face_test(5, (self.size - 1 - face_pos.1, 0, 0))?
                    } else {
                        (pos.0 + 1, pos.1, pos.2)
                    }
                }
                2 => {
                    if face_pos.1 == 0 {
                        self.from_face_test(2, (face_pos.0, self.size - 1, 2))?
                    } else {
                        (pos.0, pos.1 - 1, pos.2)
                    }
                }
                3 => {
                    if face_pos.0 == 0 {
                        self.from_face_test(1, (face_pos.1, 0, 0))?
                    } else {
                        (pos.0 - 1, pos.1, pos.2)
                    }
                }
                _ => Err(AocError)?,
            },
            4 => match pos.2 {
                0 => {
                    if face_pos.1 + 1 == self.size {
                        self.from_face_test(6, (0, self.size - 1 - face_pos.0, 1))?
                    } else {
                        (pos.0, pos.1 + 1, pos.2)
                    }
                }
                1 => {
                    if face_pos.0 + 1 == self.size {
                        self.from_face_test(5, (0, face_pos.1, 1))?
                    } else {
                        (pos.0 + 1, pos.1, pos.2)
                    }
                }
                2 => {
                    if face_pos.1 == 0 {
                        self.from_face_test(3, (face_pos.0, self.size - 1, 2))?
                    } else {
                        (pos.0, pos.1 - 1, pos.2)
                    }
                }
                3 => {
                    if face_pos.0 == 0 {
                        self.from_face_test(1, (self.size - 1, face_pos.1, 3))?
                    } else {
                        (pos.0 - 1, pos.1, pos.2)
                    }
                }
                _ => Err(AocError)?,
            },
            5 => match pos.2 {
                0 => {
                    if face_pos.1 + 1 == self.size {
                        self.from_face_test(6, (face_pos.0, 0, 0))?
                    } else {
                        (pos.0, pos.1 + 1, pos.2)
                    }
                }
                1 => {
                    if face_pos.0 + 1 == self.size {
                        self.from_face_test(2, (self.size - 1, self.size - 1 - face_pos.1, 3))?
                    } else {
                        (pos.0 + 1, pos.1, pos.2)
                    }
                }
                2 => {
                    if face_pos.1 == 0 {
                        self.from_face_test(3, (self.size - 1, self.size - 1 - face_pos.1, 3))?
                    } else {
                        (pos.0, pos.1 - 1, pos.2)
                    }
                }
                3 => {
                    if face_pos.0 == 0 {
                        self.from_face_test(4, (self.size - 1, face_pos.1, 3))?
                    } else {
                        (pos.0 - 1, pos.1, pos.2)
                    }
                }
                _ => Err(AocError)?,
            },
            6 => match pos.2 {
                0 => {
                    if face_pos.1 + 1 == self.size {
                        self.from_face_test(1, (self.size - 1 - face_pos.0, self.size - 1, 2))?
                    } else {
                        (pos.0, pos.1 + 1, pos.2)
                    }
                }
                1 => {
                    if face_pos.0 + 1 == self.size {
                        self.from_face_test(2, (self.size - 1 - face_pos.1, 0, 0))?
                    } else {
                        (pos.0 + 1, pos.1, pos.2)
                    }
                }
                2 => {
                    if face_pos.1 == 0 {
                        self.from_face_test(5, (face_pos.0, self.size - 1, 2))?
                    } else {
                        (pos.0, pos.1 - 1, pos.2)
                    }
                }
                3 => {
                    if face_pos.0 == 0 {
                        self.from_face_test(4, (self.size - 1 - face_pos.1, self.size - 1, 2))?
                    } else {
                        (pos.0 - 1, pos.1, pos.2)
                    }
                }
                _ => Err(AocError)?,
            },
            _ => Err(AocError)?,
        })
    }

    fn travel(
        &self,
        pos: (usize, usize, usize),
        moves: &Vec<Move>,
        step: fn(&Self, (usize, usize, usize)) -> Result<(usize, usize, usize), AocError>,
    ) -> Result<(usize, usize, usize), AocError> {
        moves.iter().fold(Ok(pos), |pos, m| {
            let pos = pos?;
            Ok(match m {
                Move::Walk(distance) => self.walk(pos, *distance, step)?,
                Move::TurnLeft => (pos.0, pos.1, (pos.2 + 3).rem_euclid(4)),
                Move::TurnRight => (pos.0, pos.1, (pos.2 + 1).rem_euclid(4)),
            })
        })
    }

    fn to_face_test(
        &self,
        pos: (usize, usize, usize),
    ) -> Result<(usize, (usize, usize, usize)), AocError> {
        let new_pos = (pos.0 % self.size, pos.1 % self.size, pos.2);
        Ok(match (pos.0 / self.size, pos.1 / self.size) {
            (0, 2) => (1, new_pos),
            (1, 0) => (2, new_pos),
            (1, 1) => (3, new_pos),
            (1, 2) => (4, new_pos),
            (2, 2) => (5, new_pos),
            (2, 3) => (6, new_pos),
            (_, _) => Err(AocError)?,
        })
    }

    fn from_face_test(
        &self,
        face: usize,
        pos: (usize, usize, usize),
    ) -> Result<((usize, usize, usize)), AocError> {
        Ok(match face {
            1 => (pos.0, self.size * 2 + pos.1, pos.2),
            2 => (self.size + pos.0, pos.1, pos.2),
            3 => (self.size + pos.0, self.size + pos.1, pos.2),
            4 => (self.size + pos.0, self.size * 2 + pos.1, pos.2),
            5 => (self.size * 2 + pos.0, self.size * 2 + pos.1, pos.2),
            6 => (self.size * 2 + pos.0, self.size * 3 + pos.1, pos.2),
            _ => Err(AocError)?,
        })
    }

    fn to_face(
        &self,
        pos: (usize, usize, usize),
    ) -> Result<(usize, (usize, usize, usize)), AocError> {
        let new_pos = (pos.0 % self.size, pos.1 % self.size, pos.2);
        Ok(match (pos.0 / self.size, pos.1 / self.size) {
            (0, 1) => (1, new_pos),
            (0, 2) => (2, new_pos),
            (1, 1) => (3, new_pos),
            (2, 0) => (4, new_pos),
            (2, 1) => (5, new_pos),
            (3, 0) => (6, new_pos),
            (_, _) => Err(AocError)?,
        })
    }

    fn from_face(
        &self,
        face: usize,
        pos: (usize, usize, usize),
    ) -> Result<((usize, usize, usize)), AocError> {
        Ok(match face {
            1 => (pos.0, self.size * 1 + pos.1, pos.2),
            2 => (pos.0, self.size * 2 + pos.1, pos.2),
            3 => (self.size + pos.0, self.size + pos.1, pos.2),
            4 => (self.size * 2 + pos.0, pos.1, pos.2),
            5 => (self.size * 2 + pos.0, self.size + pos.1, pos.2),
            6 => (self.size * 3 + pos.0, pos.1, pos.2),
            _ => Err(AocError)?,
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
        let len = map.iter().map(|row| row.len()).max().ok_or(AocError)?;
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
        let board = Board::from(map)?;
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
        Ok((board, moves))
    }

    fn part1_impl(
        &self,
        input: &mut dyn io::Read,
        step: fn(&Board, (usize, usize, usize)) -> Result<(usize, usize, usize), AocError>,
    ) -> BoxResult<Output> {
        let (board, moves) = Self::parse(input)?;
        let pos = board.travel(board.starting_position()?, &moves, step)?;
        Ok((pos.0 + 1) * 1000 + (pos.1 + 1) * 4 + pos.2)
    }

    fn part2_impl(
        &self,
        input: &mut dyn io::Read,
        step: fn(&Board, (usize, usize, usize)) -> Result<(usize, usize, usize), AocError>,
    ) -> BoxResult<Output> {
        let (board, moves) = Self::parse(input)?;
        let pos = board.travel(board.starting_position()?, &moves, step)?;
        Ok((pos.0 + 1) * 1000 + (pos.1 + 1) * 4 + pos.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(
            Day22 {}
                .part1_impl(&mut s.as_bytes(), Board::flat_step)
                .ok(),
            Some(f)
        );
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
        assert_eq!(
            Day22 {}
                .part2_impl(&mut s.as_bytes(), Board::cube_step_test)
                .ok(),
            Some(f)
        );
    }

    #[test]
    fn part2() {
        test2(
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
            5031,
        );
    }
}
