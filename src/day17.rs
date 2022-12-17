use crate::day::*;
use std::collections::HashMap;
use std::io::Read;
use std::ops::Index;

pub struct Day17 {}

type Output = usize;

impl Day for Day17 {
    fn tag(&self) -> &str {
        "17"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input(), 2022));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input(), 1000000000000));
    }
}

const WIDTH: usize = 7;

const ROCKS: &[&[[u8; WIDTH]]] = &[
    &[[b'.', b'.', b'#', b'#', b'#', b'#', b'.']],
    &[
        [b'.', b'.', b'.', b'#', b'.', b'.', b'.'],
        [b'.', b'.', b'#', b'#', b'#', b'.', b'.'],
        [b'.', b'.', b'.', b'#', b'.', b'.', b'.'],
    ],
    &[
        [b'.', b'.', b'.', b'.', b'#', b'.', b'.'],
        [b'.', b'.', b'.', b'.', b'#', b'.', b'.'],
        [b'.', b'.', b'#', b'#', b'#', b'.', b'.'],
    ],
    &[
        [b'.', b'.', b'#', b'.', b'.', b'.', b'.'],
        [b'.', b'.', b'#', b'.', b'.', b'.', b'.'],
        [b'.', b'.', b'#', b'.', b'.', b'.', b'.'],
        [b'.', b'.', b'#', b'.', b'.', b'.', b'.'],
    ],
    &[
        [b'.', b'.', b'#', b'#', b'.', b'.', b'.'],
        [b'.', b'.', b'#', b'#', b'.', b'.', b'.'],
    ],
];

impl Day17 {
    fn print(chamber: &Vec<[u8; WIDTH]>) {
        for y in (0..chamber.len()).rev() {
            println!(
                "{}",
                chamber[y]
                    .into_iter()
                    .map(|b| b as char)
                    .collect::<String>()
            );
        }
        println!("");
    }

    fn process(jets_in: &Vec<bool>, n: usize, compute_cycle: bool) -> BoxResult<Output> {
        let jets_len = jets_in.len();
        let mut jets = jets_in.iter().cycle();
        let mut rocks = ROCKS.into_iter().cycle();
        let mut chamber: Vec<[u8; WIDTH]> = Vec::new();
        let mut jet_count = 0;
        let mut seen = HashMap::<(usize, usize, Vec<[u8; 7]>), usize>::new();
        for nr in 0..n {
            let k = (
                nr % ROCKS.len(),
                jet_count % jets_len,
                chamber
                    .iter()
                    .rev()
                    .take(10)
                    .map(|x| x.to_owned())
                    .collect::<Vec<_>>(),
            );
            if compute_cycle {
                if let Some(&prefix_len) = seen.get(&k) {
                    let period = nr - prefix_len;
                    let cycles = (n - prefix_len) / period;
                    let suffix_len = (n - prefix_len) % period;
                    let h1 = Self::process(jets_in, prefix_len, false).unwrap();
                    let h2 = chamber.len();
                    let h3 =
                        Self::process(jets_in, prefix_len + period + suffix_len, false).unwrap();
                    return Ok(h1 + cycles * (h2 - h1) + h3 - h2);
                } else {
                    seen.insert(k, nr);
                }
            }
            let mut rock = rocks.next().ok_or(AocError)?.to_owned().to_owned();
            let height = rock.len();
            // Extend the chamber upwards to fit the rock.
            let mut ry = chamber.len() + 3;
            for _n in 0..(3 + height) {
                chamber.push([b'.', b'.', b'.', b'.', b'.', b'.', b'.']);
            }
            loop {
                // Apply jet
                let left = *jets.next().ok_or(AocError)?;
                if (0..height).all(|y| {
                    let rl = rock[height - y - 1];
                    let cl = chamber[ry + y];
                    if left {
                        let x = rl.iter().position(|b| *b == b'#').unwrap(); // XXX
                        x > 0 && cl[x - 1] != b'#'
                    } else {
                        let x = rl.iter().rposition(|b| *b == b'#').unwrap(); // XXX
                        x < WIDTH - 1 && cl[x + 1] != b'#'
                    }
                }) {
                    for l in rock.iter_mut() {
                        if left {
                            l.rotate_left(1);
                        } else {
                            l.rotate_right(1);
                        }
                    }
                }
                jet_count += 1;
                // If the rock hs reached the floor let it rest.
                if ry == 0 {
                    break;
                }
                // Also, if it hits another rock, let it rest.
                if (0..height).any(|y| {
                    let rl = rock[height - y - 1];
                    let cl = chamber[ry - 1 + y];
                    rl.iter()
                        .zip(cl.iter())
                        .any(|(a, b)| *a == b'#' && *b == b'#')
                }) {
                    break;
                }
                // Drop the rock one step.
                ry -= 1;
            }
            // Let rock rest in chamber.
            for y in 0..height {
                let rl = rock[height - y - 1];
                let mut cl = &mut chamber[ry + y];
                for (c, r) in cl.iter_mut().zip(rl.iter()) {
                    if *r == b'#' {
                        *c = b'#'
                    }
                }
            }
            // Remove empty top of chamber
            while chamber.ends_with(&[[b'.', b'.', b'.', b'.', b'.', b'.', b'.']]) {
                chamber.pop();
            }
            //Self::print(&chamber);
        }
        Ok(chamber.len())
    }

    fn parse(input: &mut dyn Read) -> BoxResult<Vec<bool>> {
        let jets = io::BufReader::new(input)
            .bytes()
            .map(|r| {
                let r: BoxResult<_> = r.map_err(|e| e.into());
                r
            })
            .filter(|b| b.as_ref().map_or(true, |b| *b != b'\n'))
            .map(|b| {
                b.and_then(|b| match b {
                    b'<' => Ok(true),
                    b'>' => Ok(false),
                    e => Err(AocError)?,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(jets)
    }

    fn part1_impl(&self, input: &mut dyn io::Read, n: usize) -> BoxResult<Output> {
        let jets = Self::parse(input)?;
        Self::process(&jets, n, true)
    }

    fn part2_impl(&self, input: &mut dyn io::Read, n: usize) -> BoxResult<Output> {
        let jets = Self::parse(input)?;
        Self::process(&jets, n, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, n: usize, f: Output) {
        assert_eq!(Day17 {}.part1_impl(&mut s.as_bytes(), n).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>", 2022, 3068);
    }

    fn test2(s: &str, n: usize, f: Output) {
        assert_eq!(Day17 {}.part2_impl(&mut s.as_bytes(), n).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>",
            1000000000000,
            1514285714288,
        );
    }
}
