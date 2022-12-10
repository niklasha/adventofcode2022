use crate::day::*;
use std::collections::HashSet;
use std::iter;
use std::str::FromStr;

pub struct Day10 {}

type Output = i64;

impl Day for Day10 {
    fn tag(&self) -> &str {
        "10"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:#?}", self.part2_impl(&mut *input()));
    }
}

#[derive(Debug)]
struct State1 {
    x: Output,
    clk: usize,
    acc: Output,
}

#[derive(Debug)]
struct State2 {
    x: Output,
    clk: usize,
    acc: Vec<String>,
}

impl Day10 {
    fn process1(input: &mut dyn io::Read) -> BoxResult<Output> {
        fn check(clk: usize, x: Output, acc: Output) -> Output {
            if clk % 40 == 20 {
                acc + clk as Output * x
            } else {
                acc
            }
        }
        io::BufReader::new(input)
            .lines()
            .map(|r| {
                let r: BoxResult<_> = r.map_err(|e| e.into());
                r
            })
            .fold(
                Ok(State1 {
                    x: 1,
                    clk: 1,
                    acc: 0,
                }),
                |state, insn| match insn {
                    Ok(insn) => {
                        if let Ok(State1 {
                            mut x,
                            mut clk,
                            mut acc,
                        }) = state
                        {
                            let mut tokens = insn.split_whitespace();
                            match tokens.next().ok_or(AocError)? {
                                "noop" => clk += 1,
                                "addx" => {
                                    clk += 1;
                                    acc = check(clk, x, acc);
                                    clk += 1;
                                    let op = tokens.next().ok_or(AocError)?.parse::<Output>()?;
                                    x += op;
                                }
                                _ => Err(AocError)?,
                            }
                            acc = check(clk, x, acc);
                            Ok(State1 { x, clk, acc })
                        } else {
                            state
                        }
                    }
                    Err(e) => Err::<State1, _>(e),
                },
            )
            .map(|state| state.acc)
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process1(input)
    }

    fn process2(input: &mut dyn io::Read) -> BoxResult<Vec<String>> {
        fn check(clk: usize, x: Output, mut acc: Vec<String>) -> Result<Vec<String>, AocError> {
            let (i, off) = ((clk - 1) / 40, (clk - 1) % 40);
            let sprite = (x - 1)..=(x + 1);
            let mut s = acc.get_mut(i).ok_or(AocError)?;
            s.push(if sprite.contains(&(off as i64)) {
                '#' // XXX Better contrast with '█'
            } else {
                '.' // XXX Better contrast with ' '
            });
            Ok(acc)
        }
        io::BufReader::new(input)
            .lines()
            .map(|r| {
                let r: BoxResult<_> = r.map_err(|e| e.into());
                r
            })
            .fold(
                Ok(State2 {
                    x: 1,
                    clk: 1,
                    acc: vec![String::new(); 6],
                }),
                |state, insn| match insn {
                    Ok(insn) => {
                        if let Ok(State2 {
                            mut x,
                            mut clk,
                            mut acc,
                        }) = state
                        {
                            acc = check(clk, x, acc)?;
                            let mut tokens = insn.split_whitespace();
                            match tokens.next().ok_or(AocError)? {
                                "noop" => clk += 1,
                                "addx" => {
                                    clk += 1;
                                    acc = check(clk, x, acc)?;
                                    clk += 1;
                                    let op = tokens.next().ok_or(AocError)?.parse::<Output>()?;
                                    x += op;
                                }
                                _ => Err(AocError)?,
                            }
                            Ok(State2 { x, clk, acc })
                        } else {
                            state
                        }
                    }
                    Err(e) => Err::<State2, _>(e),
                },
            )
            .map(|state| state.acc)
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Vec<String>> {
        Self::process2(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day10 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop",
            13140,
        );
    }

    fn test2(s: &str, f: Vec<String>) {
        assert_eq!(Day10 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop",
            vec![
                "##..##..##..##..##..##..##..##..##..##..".to_string(),
                "###...###...###...###...###...###...###.".to_string(),
                "####....####....####....####....####....".to_string(),
                "#####.....#####.....#####.....#####.....".to_string(),
                "######......######......######......####".to_string(),
                "#######.......#######.......#######.....".to_string(),
            ],
        );
    }
}
