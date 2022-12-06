use crate::day::*;
use std::io::Read;

pub struct Day06 {}

type Output = usize;

impl Day for Day06 {
    fn tag(&self) -> &str {
        "06"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day06 {
    fn scan(input: &mut dyn io::Read, size: usize) -> BoxResult<Output> {
        let input = io::BufReader::new(input)
            .bytes()
            .collect::<Result<Vec<_>, _>>()?;
        let (i, _) = input
            .as_slice()
            .windows(size)
            .enumerate()
            .find(|(_, w)| w.iter().duplicates().next().is_none())
            .ok_or(AocError)?;
        Ok(i + size)
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::scan(input, 4)
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::scan(input, 14)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day06 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7);
        test1("bvwbjplbgvbhsrlpgdmjqwftvncz", 5);
        test1("nppdvjthqldpwncqszvftbrmjlhg", 6);
        test1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10);
        test1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11);
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day06 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19);
        test2("bvwbjplbgvbhsrlpgdmjqwftvncz", 23);
        test2("nppdvjthqldpwncqszvftbrmjlhg", 23);
        test2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29);
        test2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26);
    }
}
