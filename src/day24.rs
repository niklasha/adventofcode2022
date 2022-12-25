use crate::day::*;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::iter;
use std::ops::ControlFlow::{Break, Continue};

pub struct Day24 {}

type Output = usize;

impl Day for Day24 {
    fn tag(&self) -> &str {
        "24"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    row: usize,
    column: usize,
}

impl Coord {
    fn neighbours(&self, board: &Board) -> HashSet<Coord> {
        [
            if self.row > 0 {
                Coord {
                    row: self.row - 1,
                    column: self.column,
                }
            } else {
                // XXX
                Coord {
                    row: self.row + 1,
                    column: self.column,
                }
            },
            Coord {
                row: self.row + 1,
                column: self.column,
            },
            Coord {
                row: self.row,
                column: self.column - 1,
            },
            Coord {
                row: self.row,
                column: self.column + 1,
            },
        ]
        .into_iter()
        .filter(|&Coord { row, column }| {
            (row > 0 && row <= board.height && column > 0 && column <= board.width)
                || (row == 0 && column == 1)
                || (row == board.height + 1 && column == board.width)
        })
        .collect()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Blizzard {
    LeftHorizontal(Coord),
    RightHorizontal(Coord),
    UpVertical(Coord),
    DownVertical(Coord),
}

impl Blizzard {
    fn next(&self, board: &Board) -> Self {
        match *self {
            Blizzard::LeftHorizontal(Coord { row, column }) => Blizzard::LeftHorizontal(Coord {
                row,
                column: (column + board.width - 2) % board.width + 1,
            }),
            Blizzard::RightHorizontal(Coord { row, column }) => Blizzard::RightHorizontal(Coord {
                row,
                column: column % board.width + 1,
            }),
            Blizzard::UpVertical(Coord { row, column }) => Blizzard::UpVertical(Coord {
                row: (row + board.height - 2) % board.height + 1,
                column,
            }),
            Blizzard::DownVertical(Coord { row, column }) => Blizzard::DownVertical(Coord {
                row: row % board.height + 1,
                column,
            }),
        }
    }

    fn coord(&self) -> Coord {
        match self {
            Blizzard::LeftHorizontal(c) => *c,
            Blizzard::RightHorizontal(c) => *c,
            Blizzard::UpVertical(c) => *c,
            Blizzard::DownVertical(c) => *c,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Board {
    width: usize,
    height: usize,
    blizzards: Vec<Blizzard>,
    expedition: Coord,
}

impl Board {
    fn next(&self) -> Vec<Board> {
        let mut neighbours = self.expedition.neighbours(self);
        neighbours.insert(self.expedition);
        //println!("neighbours {:?}", neighbours);
        let blizzards = self
            .blizzards
            .iter()
            .map(|blizzard| blizzard.next(self))
            .collect::<Vec<_>>();
        //println!("blizzards {:?}", blizzards);
        let next = neighbours
            .into_iter()
            .filter(|expedition| {
                blizzards
                    .iter()
                    .find(|blizzard| blizzard.coord() == *expedition)
                    .is_none()
            })
            .map(|expedition| Board {
                blizzards: blizzards.to_owned(),
                expedition,
                ..*self
            })
            .collect::<Vec<_>>();
        // println!(
        //     "next {:?}",
        //     next.iter()
        //         .map(|board| board.expedition)
        //         .collect::<Vec<_>>()
        // );
        next
    }

    fn is_at_the_entrance(&self) -> bool {
        self.expedition == Coord { row: 0, column: 1 }
    }

    fn is_at_the_gates(&self) -> bool {
        self.expedition
            == Coord {
                row: self.height + 1,
                column: self.width,
            }
    }

    fn print(&self) {
        println!("#.{}", "#".repeat(self.width));
        for row in 1..=self.height {
            print!("#");
            for column in 1..=self.width {
                let coord = Coord { row, column };
                let blizzards = self
                    .blizzards
                    .iter()
                    .filter(|blizzard| blizzard.coord() == coord)
                    .collect::<Vec<_>>();
                print!(
                    "{}",
                    match blizzards.len() {
                        0 =>
                            if self.expedition == coord {
                                'E'
                            } else {
                                '.'
                            },
                        1 => match blizzards[0] {
                            Blizzard::LeftHorizontal(_) => '<',
                            Blizzard::RightHorizontal(_) => '>',
                            Blizzard::UpVertical(_) => '^',
                            Blizzard::DownVertical(_) => 'v',
                        },
                        n => (n as u8 + b'0') as char,
                    }
                );
            }
            println!("#");
        }
        println!("{}.#", "#".repeat(self.width));
    }
}

impl Day24 {
    fn parse(input: &mut dyn io::Read) -> BoxResult<Board> {
        let Ok((blizzards, Some(width), Some(height))) = io::BufReader::new(input)
            .split(b'\n')
            .enumerate()
            .map(|(row, l)| l.map(|l| (row, l)).map_err(|e| e.into()))
            .fold(Ok((Vec::new(), None, None)), |state: BoxResult<_>, r: BoxResult<_>| {
                let state = state?;
                let (row, l) = r?;
                l.iter().enumerate().fold(Ok(state), |state, (column, b)| {
                    let (mut blizzards, _, _) = state?;
                    match b {
                        b'<' => {
                            blizzards.push(Blizzard::LeftHorizontal(Coord { row, column }));
                        }
                        b'>' => {
                            blizzards.push(Blizzard::RightHorizontal(Coord { row, column }));
                        }
                        b'^' => {
                            blizzards.push(Blizzard::UpVertical(Coord { row, column }));
                        }
                        b'v' => {
                            blizzards.push(Blizzard::DownVertical(Coord { row, column }));
                        }
                        b'#' => {}
                        b'.' => {}
                        _ => Err(AocError)?,
                    }
                    Ok((blizzards, Some(l.len() - 2), if row > 0 { Some(row - 1) } else { None }))
                })
            }) else {
            Err(AocError)?
        };
        Ok(Board {
            width,
            height,
            blizzards,
            expedition: Coord { row: 0, column: 1 },
        })
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        match (0..).try_fold(
            (
                iter::once(Self::parse(input)?).collect::<HashSet<_>>(),
                HashSet::new(),
                0usize,
            ),
            |(boards, mut state, t), _| {
                //println!("t {}", t);
                //boards.iter().for_each(|b| b.print());
                //println!(
                //    "boards.expedition {:?}",
                //    boards.iter().map(|b| b.expedition).collect_vec()
                //);
                if boards.iter().any(|board| board.is_at_the_gates()) {
                    return Break(t);
                }
                let mut next_boards = HashSet::new();
                for board in boards {
                    state.insert(board.to_owned());
                    for next in board.next() {
                        next_boards.insert(next);
                    }
                }
                Continue((next_boards, state, t + 1))
            },
        ) {
            Break(t) => Ok(t),
            _ => Err(AocError.into()),
        }
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let (t, board) = match (0..).try_fold(
            (
                iter::once(Self::parse(input)?).collect::<HashSet<_>>(),
                HashSet::new(),
                0usize,
            ),
            |(boards, mut state, t), _| {
                //println!("t {}", t);
                //boards.iter().for_each(|b| b.print());
                //println!(
                //"boards.expedition {:?}",
                //boards.iter().map(|b| b.expedition).collect_vec()
                //);
                if let Some(board) = boards.iter().find(|board| board.is_at_the_gates()) {
                    return Break((t, board.to_owned()));
                }
                let mut next_boards = HashSet::new();
                for board in boards {
                    state.insert(board.to_owned());
                    for next in board.next() {
                        next_boards.insert(next);
                    }
                }
                Continue((next_boards, state, t + 1))
            },
        ) {
            Break(t) => Ok::<_, Box<dyn error::Error>>(t),
            _ => Err(AocError.into()),
        }?;
        // println!("t {}", t);
        // board.print();
        // let board = board
        //     .next()
        //     .into_iter()
        //     .find(|b| b.expedition == board.expedition)
        //     .ok_or(AocError)?;
        // board.print();
        // let board = board
        //     .next()
        //     .into_iter()
        //     .find(|b| b.expedition == board.expedition)
        //     .ok_or(AocError)?;
        // board.print();
        let (t, board) = match (t..).try_fold(
            (iter::once(board).collect::<HashSet<_>>(), HashSet::new(), t),
            |(boards, mut state, t), _| {
                //println!("t {}", t);
                //boards.iter().for_each(|b| b.print());
                //println!(
                //"boards.expedition {:?}",
                //boards.iter().map(|b| b.expedition).collect_vec()
                //);
                if let Some(board) = boards.iter().find(|board| board.is_at_the_entrance()) {
                    return Break((t, board.to_owned()));
                }
                let mut next_boards = HashSet::new();
                for board in boards {
                    state.insert(board.to_owned());
                    for next in board.next() {
                        next_boards.insert(next);
                    }
                }
                Continue((next_boards, state, t + 1))
            },
        ) {
            Break(t) => Ok::<_, Box<dyn error::Error>>(t),
            _ => Err(AocError.into()),
        }?;
        // println!("t {}", t);
        // let mut board = board;
        // board.expedition = Coord {
        //     row: board.height + 1,
        //     column: board.width,
        // };
        // board.print();
        // let board = board
        //     .next()
        //     .into_iter()
        //     .find(|b| b.expedition == board.expedition)
        //     .ok_or(AocError)?;
        // let board = board
        //     .next()
        //     .into_iter()
        //     .find(|b| b.expedition == board.expedition)
        //     .ok_or(AocError)?;
        match (t..).try_fold(
            (iter::once(board).collect::<HashSet<_>>(), HashSet::new(), t),
            |(boards, mut state, t), _| {
                //println!("t {}", t);
                //boards.iter().for_each(|b| b.print());
                //println!(
                //"boards.expedition {:?}",
                //boards.iter().map(|b| b.expedition).collect_vec()
                //);
                if let Some(board) = boards.iter().find(|board| board.is_at_the_gates()) {
                    return Break((t, board.to_owned()));
                }
                let mut next_boards = HashSet::new();
                for board in boards {
                    state.insert(board.to_owned());
                    for next in board.next() {
                        next_boards.insert(next);
                    }
                }
                Continue((next_boards, state, t + 1))
            },
        ) {
            Break((t, _)) => Ok(t),
            _ => Err(AocError.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day24 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#",
            18,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day24 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#",
            54,
        );
    }
}
