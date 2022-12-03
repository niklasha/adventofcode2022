use crate::day::*;

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
            .map(|rucksack| -> BoxResult<_> {
                let rucksack = rucksack?;
                let size = rucksack.len() / 2;
                let comp1 = &rucksack[..size];
                let comp2 = &rucksack[size..];
                let duplicate = comp1
                    .bytes()
                    .find(|item| comp2.bytes().contains(item))
                    .unwrap();
                Ok(if duplicate.is_ascii_lowercase() {
                    1 + (duplicate - b'a')
                } else {
                    27 + (duplicate - b'A')
                } as Output)
            })
            .sum()
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        io::BufReader::new(input)
            .lines()
            .map(Result::unwrap)
            .tuples()
            .map(|(sack1, sack2, sack3)| -> BoxResult<_> {
                let badge: u8 = (sack1
                    .bytes()
                    .filter(|item| sack2.bytes().contains(item))
                    .filter(|item| sack3.bytes().contains(item))
                    .next()
                    .ok_or(AocError.into()) as BoxResult<_>)?;
                Ok(if badge.is_ascii_lowercase() {
                    1 + (badge - b'a')
                } else {
                    27 + (badge - b'A')
                } as Output)
            })
            .sum()
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
