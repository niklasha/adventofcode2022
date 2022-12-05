use crate::day::*;
use byte_set::ByteSet;

pub struct Day03 {}

type Output = usize;

impl Day for Day03 {
    fn tag(&self) -> &str {
        "03"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day03 {
    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        io::BufReader::new(input)
            .lines()
            .map(|rucksack| {
                let rucksack = rucksack?;
                let (comp1, comp2) = rucksack.split_at(rucksack.len() / 2); // XXX panics
                let (comp1, comp2): (ByteSet, ByteSet) = (comp1.into(), comp2.into());
                let duplicate = comp1.intersection(comp2).first().ok_or(AocError)?;
                Ok(Self::priority(duplicate)?)
            })
            .sum()
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        io::BufReader::new(input)
            .lines()
            .tuples()
            .map(|(sack1, sack2, sack3)| {
                let (sack1, sack2, sack3): (ByteSet, ByteSet, ByteSet) = (
                    sack1?.as_str().into(),
                    sack2?.as_str().into(),
                    sack3?.as_str().into(),
                );
                let badge = (sack1
                    .intersection(sack2)
                    .intersection(sack3)
                    .first()
                    .ok_or_else(|| AocError.into()) as BoxResult<_>)?;
                Ok(Self::priority(badge)?)
            })
            .sum()
    }

    fn priority(item: u8) -> Result<Output, AocError> {
        match item {
            b'a'..=b'z' => Ok((1 + (item - b'a')) as Output),
            b'A'..=b'Z' => Ok((27 + (item - b'A')) as Output),
            _ => Err(AocError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day03 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
            157,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day03 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
            70,
        );
    }
}
