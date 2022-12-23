use crate::day::*;
use itertools::Itertools;
use std::collections::HashSet;
use std::io::Read;

pub struct Day23 {}

type Output = usize;

impl Day for Day23 {
    fn tag(&self) -> &str {
        "23"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input(), 10));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord(i64, i64);

#[derive(Debug)]
struct Board {
    elves: HashSet<Coord>,
}

impl Board {
    fn elf_count(&self) -> usize {
        self.elves.len()
    }

    fn area(&self) -> Result<usize, AocError> {
        let min_x = self.elves.iter().min_by_key(|elf| elf.0).ok_or(AocError)?.0;
        let max_x = self.elves.iter().max_by_key(|elf| elf.0).ok_or(AocError)?.0;
        let min_y = self.elves.iter().min_by_key(|elf| elf.1).ok_or(AocError)?.1;
        let max_y = self.elves.iter().max_by_key(|elf| elf.1).ok_or(AocError)?.1;
        Ok(((max_x - min_x + 1) * (max_y - min_y + 1)) as usize)
    }

    fn propositions(&self, step: usize) -> Vec<(Coord, Coord)> {
        self.elves
            .iter()
            .flat_map(|elf| {
                let ((north_occupied, south_occupied, west_occupied, east_occupied)) = (
                    (-1..=1).any(|i| self.elves.contains(&Coord(elf.0 + i, elf.1 - 1))),
                    (-1..=1).any(|i| self.elves.contains(&Coord(elf.0 + i, elf.1 + 1))),
                    (-1..=1).any(|i| self.elves.contains(&Coord(elf.0 - 1, elf.1 + i))),
                    (-1..=1).any(|i| self.elves.contains(&Coord(elf.0 + 1, elf.1 + i))),
                );
                if !north_occupied && !south_occupied && !west_occupied && !east_occupied {
                    None
                } else {
                    (0..4)
                        .flat_map(|i| match (step + i) % 4 {
                            0 if !north_occupied => Some(Coord(elf.0, elf.1 - 1)),
                            1 if !south_occupied => Some(Coord(elf.0, elf.1 + 1)),
                            2 if !west_occupied => Some(Coord(elf.0 - 1, elf.1)),
                            3 if !east_occupied => Some(Coord(elf.0 + 1, elf.1)),
                            _ => None,
                        })
                        .next()
                }
                .map(|target| (*elf, target))
            })
            .collect()
    }

    fn execute(&mut self, order: &(Coord, Coord)) -> Result<(), AocError> {
        self.elves.remove(&order.0);
        self.elves.insert(order.1);
        Ok(())
    }
}
impl Day23 {
    fn parse(input: &mut dyn io::Read) -> BoxResult<Board> {
        let elves: HashSet<Coord> = HashSet::new();
        let elves = io::BufReader::new(input)
            .split(b'\n')
            .enumerate()
            .map(|(i, l)| l.map(|l| (i, l)).map_err(|e| e.into()))
            .fold(Ok(elves), |elves: BoxResult<_>, r: BoxResult<_>| {
                let elves = elves?;
                let (row, l) = r?;
                l.iter().enumerate().fold(Ok(elves), |elves, (column, b)| {
                    let mut elves = elves?;
                    match b {
                        b'#' => {
                            elves.insert(Coord(column as i64, row as i64));
                        }
                        b'.' => {}
                        _ => Err(AocError)?,
                    }
                    Ok(elves)
                })
            })?;
        Ok(Board { elves })
    }

    fn part1_impl(&self, input: &mut dyn io::Read, n: usize) -> BoxResult<Output> {
        let mut board = Self::parse(input)?;
        for i in 0..n {
            let propositions = board.propositions(i);
            let duplicates = propositions
                .iter()
                .map(|(_, target)| target)
                .duplicates()
                .collect::<HashSet<_>>();
            for proposition in propositions
                .iter()
                .filter(|&(_, target)| !duplicates.contains(target))
            {
                board.execute(proposition)?;
            }
        }
        Ok(board.area()? - board.elf_count())
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let mut board = Self::parse(input)?;
        for i in 0.. {
            let propositions = board.propositions(i);
            let duplicates = propositions
                .iter()
                .map(|(_, target)| target)
                .duplicates()
                .collect::<HashSet<_>>();
            let propositions = propositions
                .iter()
                .filter(|&(_, target)| !duplicates.contains(target))
                .collect::<HashSet<_>>();
            if propositions.is_empty() {
                return Ok(i + 1);
            }
            for proposition in propositions {
                board.execute(proposition)?;
            }
        }
        Err(AocError)?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day23 {}.part1_impl(&mut s.as_bytes(), 10).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..",
            110,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day23 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..",
            20,
        );
    }
}
