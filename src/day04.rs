use crate::day::*;
use lazy_static::lazy_static;
use regex::Regex;

pub struct Day04 {}

type Output = usize;

impl Day for Day04 {
    fn tag(&self) -> &str {
        "04"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

struct Section {
    start: usize,
    end: usize,
}

impl Section {
    fn contains(a: &Section, b: &Section) -> bool {
        a.start <= b.start && a.end >= b.end || b.start <= a.start && b.end >= a.end
    }

    fn overlaps(a: &Section, b: &Section) -> bool {
        !(a.end < b.start || a.start > b.end)
    }
}

impl Day04 {
    fn parse(s: &str) -> BoxResult<(Section, Section)> {
        lazy_static! {
            static ref RE: Regex = Regex::new("(.+)-(.*),(.+)-(.*)").unwrap();
        }
        let cap = RE.captures(s).ok_or(AocError)?;
        Ok((
            Section {
                start: cap[1].parse::<usize>()?,
                end: cap[2].parse::<usize>()?,
            },
            Section {
                start: cap[3].parse::<usize>()?,
                end: cap[4].parse::<usize>()?,
            },
        ))
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Ok(io::BufReader::new(input)
            .lines()
            .map(|l| Ok(Self::parse(&l?)?))
            .filter(|pair: &BoxResult<_>| {
                pair.as_ref()
                    .map_or(false, |pair| Section::contains(&pair.0, &pair.1))
            })
            .count())
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Ok(io::BufReader::new(input)
            .lines()
            .map(|l| Ok(Self::parse(&l?)?))
            .filter(|pair: &BoxResult<_>| {
                pair.as_ref()
                    .map_or(false, |pair| Section::overlaps(&pair.0, &pair.1))
            })
            .count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day04 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
            2,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day04 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("", 70);
    }
}