use crate::day::*;
use std::ops::Range;
use std::str::FromStr;

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

struct Section(Range<usize>);

impl Section {
    fn contains(a: &Section, b: &Section) -> bool {
        let (a, b) = (&a.0, &b.0);
        a.start <= b.start && a.end >= b.end || b.start <= a.start && b.end >= a.end
    }

    fn overlaps(a: &Section, b: &Section) -> bool {
        let (a, b) = (&a.0, &b.0);
        !(a.end < b.start || a.start > b.end)
    }
}

impl FromStr for Section {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = s.split('-');
        let section = Section(
            i.next().ok_or(AocError)?.parse().or(Err(AocError))?
                ..i.next().ok_or(AocError)?.parse().or(Err(AocError))?,
        );
        if i.next().is_none() {
            Ok(section)
        } else {
            Err(AocError)
        }
    }
}

impl Day04 {
    fn parse(s: &str) -> BoxResult<(Section, Section)> {
        let mut i = s.split(',');
        let pair = (
            i.next().ok_or(AocError).and_then(Section::from_str)?,
            i.next().ok_or(AocError).and_then(Section::from_str)?,
        );
        if i.next().is_none() {
            Ok(pair)
        } else {
            Err(AocError)?
        }
    }

    fn process<F>(input: &mut dyn io::Read, f: F) -> BoxResult<Output>
    where
        F: Fn(&Section, &Section) -> bool,
    {
        Ok(io::BufReader::new(input)
            .lines()
            .map(|l| Self::parse(&l?))
            .filter(|pair| pair.as_ref().map_or(false, |pair| f(&pair.0, &pair.1)))
            .count())
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process(input, Section::contains)
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process(input, Section::overlaps)
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
        test2(
            "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
            4,
        );
    }
}
