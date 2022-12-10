use crate::day::*;

pub struct Day10 {}

type Output1 = i64;
type Output2 = Vec<String>;

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
struct State<T> {
    x: i64,
    clk: usize,
    acc: T,
}

impl Day10 {
    fn process<T, F>(input: &mut dyn io::Read, f: F, init: T, at_cycle_start: bool) -> BoxResult<T>
    where
        T: Sized,
        F: Fn(usize, i64, T) -> Result<T, AocError>,
    {
        io::BufReader::new(input)
            .lines()
            .map(|r| {
                let r: BoxResult<_> = r.map_err(|e| e.into());
                r
            })
            .fold(
                Ok(State::<T> {
                    x: 1,
                    clk: 1,
                    acc: init,
                }),
                |state, insn| match insn {
                    Ok(insn) => {
                        if let Ok(State::<T> {
                            mut x,
                            mut clk,
                            mut acc,
                        }) = state
                        {
                            if at_cycle_start {
                                acc = f(clk, x, acc)?;
                            }
                            let mut tokens = insn.split_whitespace();
                            match tokens.next().ok_or(AocError)? {
                                "noop" => clk += 1,
                                "addx" => {
                                    clk += 1;
                                    acc = f(clk, x, acc)?;
                                    clk += 1;
                                    let op = tokens.next().ok_or(AocError)?.parse::<Output1>()?;
                                    x += op;
                                }
                                _ => Err(AocError)?,
                            }
                            if !at_cycle_start {
                                acc = f(clk, x, acc)?;
                            }
                            Ok(State::<T> { x, clk, acc })
                        } else {
                            state
                        }
                    }
                    Err(e) => Err::<State<T>, _>(e),
                },
            )
            .map(|state| state.acc)
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output1> {
        fn check(clk: usize, x: i64, acc: Output1) -> Result<Output1, AocError> {
            Ok(if clk % 40 == 20 {
                acc + clk as Output1 * x
            } else {
                acc
            })
        }
        Self::process(input, check, 0, false)
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output2> {
        fn draw(clk: usize, x: Output1, mut acc: Output2) -> Result<Output2, AocError> {
            let (i, off) = ((clk - 1) / 40, (clk - 1) % 40);
            let sprite = (x - 1)..=(x + 1);
            let s = acc.get_mut(i).ok_or(AocError)?;
            s.push(if sprite.contains(&(off as i64)) {
                '#' // XXX Better contrast with '█'
            } else {
                '.' // XXX Better contrast with ' '
            });
            Ok(acc)
        }
        Self::process(input, draw, vec![String::new(); 6], true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output1) {
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

    fn test2(s: &str, f: Output2) {
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
