use crate::day::*;
use std::cmp::max;

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
    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        io::BufReader::new(input)
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
                None,
                |max_calories, calories: BoxResult<_>| -> BoxResult<Option<Output>> {
                    let calories = calories?;
                    Ok(Some(if let Some(m) = max_calories {
                        max(m, calories)
                    } else {
                        calories
                    }))
                },
            )?
            .ok_or_else(|| AocError.into())
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let lines = io::BufReader::new(input).lines();
        let v = lines.collect::<Result<Vec<_>, _>>()?;
        let v = v.split(|l| l.is_empty());
        let mut v = v
            .map(|v| {
                v.iter()
                    .map(|r| r.parse::<Output>())
                    .sum::<Result<_, _>>()
                    .unwrap()
            })
            .collect::<Vec<_>>();
        v.sort();
        Ok(v.iter().rev().take(3).sum())
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
