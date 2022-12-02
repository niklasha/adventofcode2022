use crate::day::*;
use std::iter;

pub struct Day01 {}

type Output = usize;

impl Day for Day01 {
    fn tag(&self) -> &str {
        "01"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day01 {
    fn process(input: &mut dyn io::Read, window: usize) -> BoxResult<Output> {
        Ok(io::BufReader::new(input)
            .lines()
            .group_by(|r| r.as_ref().map_or(false, |s| s.is_empty()))
            .into_iter()
            .filter(|&(is_blank, _)| !is_blank)
            .map(|(_, elf)| {
                elf.map(|calories| {
                    calories
                        .map_err(|e| e.into())
                        .and_then(|s| s.parse::<Output>().map_err(|e| e.into()))
                })
                .sum()
            })
            .try_fold(
                Vec::new(),
                |max_calories, calories: BoxResult<_>| -> BoxResult<Vec<Output>> {
                    Ok(max_calories
                        .into_iter()
                        .chain(iter::once(calories?))
                        .sorted()
                        .rev()
                        .take(window)
                        .collect())
                },
            )?
            .into_iter()
            .sum())
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process(input, 1)
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process(input, 3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day01 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
            24000,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day01 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
            45000,
        );
    }
}
