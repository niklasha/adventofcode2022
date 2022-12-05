use crate::day::*;
use std::collections::HashMap;

pub struct Day05 {}

type Output = String;

impl Day for Day05 {
    fn tag(&self) -> &str {
        "05"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

const STACK_WIDTH: usize = 4;
const STACK_OFFSET: usize = 1;

impl Day05 {
    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let lines = &mut io::BufReader::new(input).lines();
        let stacks = lines
            .take_while(|r| r.as_ref().map_or(true, |l| l.contains('[')))
            .fold(
                Ok::<_, Box<dyn error::Error>>(HashMap::<usize, String>::new()),
                |stacks, l| {
                    Ok(l?
                        .chars()
                        .enumerate()
                        .filter(|(i, _)| i % STACK_WIDTH == STACK_OFFSET)
                        .map(|(i, c)| (i / STACK_WIDTH + 1, c))
                        .fold(stacks?, |mut stacks, (i, c)| {
                            let stack = stacks.get_mut(&i);
                            if c != ' ' {
                                match stack {
                                    Some(s) => s.push(c),
                                    None => {
                                        stacks.insert(i, c.to_string());
                                    }
                                };
                            }
                            stacks
                        }))
                },
            )?;
        let stacks = lines.skip(1).fold(
            Ok(stacks),
            |stacks: BoxResult<HashMap<usize, String>>, l| {
                l?.split_whitespace()
                    .tuples()
                    .fold(stacks, |stacks, (_, count, _, from, _, to)| {
                        let mut stacks = stacks?;
                        let (count, from, to) = (
                            count.parse::<usize>()?,
                            from.parse::<usize>()?,
                            to.parse::<usize>()?,
                        );
                        for _i in 0usize..count {
                            let source = stacks.get_mut(&from).ok_or(AocError)?;
                            let top = source.remove(0); // XXX panics
                            let target = stacks.get_mut(&to).ok_or(AocError)?;
                            target.insert(0, top);
                        }
                        Ok(stacks)
                    })
            },
        )?;
        let len = stacks.keys().max().ok_or(AocError)?;
        Ok((1..=*len)
            .map(|i| {
                Ok(stacks
                    .get(&i)
                    .unwrap_or(&String::from(" "))
                    .chars()
                    .next()
                    .ok_or(AocError)?)
            })
            .collect::<BoxResult<String>>()?)
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let lines = &mut io::BufReader::new(input).lines();
        let stacks = lines
            .take_while(|r| r.as_ref().map_or(true, |l| l.contains('[')))
            .fold(
                Ok::<_, Box<dyn error::Error>>(HashMap::<usize, String>::new()),
                |stacks, l| {
                    Ok(l?
                        .chars()
                        .enumerate()
                        .filter(|(i, _)| i % STACK_WIDTH == STACK_OFFSET)
                        .map(|(i, c)| (i / STACK_WIDTH + 1, c))
                        .fold(stacks?, |mut stacks, (i, c)| {
                            let stack = stacks.get_mut(&i);
                            if c != ' ' {
                                match stack {
                                    Some(s) => s.push(c),
                                    None => {
                                        stacks.insert(i, c.to_string());
                                    }
                                };
                            }
                            stacks
                        }))
                },
            )?;
        let stacks = lines.skip(1).fold(
            Ok(stacks),
            |stacks: BoxResult<HashMap<usize, String>>, l| {
                l?.split_whitespace()
                    .tuples()
                    .fold(stacks, |stacks, (_, count, _, from, _, to)| {
                        let mut stacks = stacks?;
                        let (count, from, to) = (
                            count.parse::<usize>()?,
                            from.parse::<usize>()?,
                            to.parse::<usize>()?,
                        );
                        for i in 0usize..count {
                            let source = stacks.get_mut(&from).ok_or(AocError)?;
                            let top = source.remove(count - 1 - i); // XXX panics
                            let target = stacks.get_mut(&to).ok_or(AocError)?;
                            target.insert(0, top);
                        }
                        Ok(stacks)
                    })
            },
        )?;
        let len = stacks.keys().max().ok_or(AocError)?;
        Ok((1..=*len)
            .map(|i| {
                Ok(stacks
                    .get(&i)
                    .unwrap_or(&String::from(" "))
                    .chars()
                    .next()
                    .ok_or(AocError)?)
            })
            .collect::<BoxResult<String>>()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day05 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
            String::from("CMZ"),
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day05 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
            String::from("MCD"),
        );
    }
}
