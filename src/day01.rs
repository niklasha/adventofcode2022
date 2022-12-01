use crate::day::*;

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
        let lines = io::BufReader::new(input).lines();
        let v = lines.collect::<Vec<_>>();
        let v = v.split(|l| (*l).as_ref().unwrap().is_empty());
        let v = v.map(|v| {
            v.iter()
                .map(|r| (*r).as_ref().unwrap())
                .map(|s| s.parse::<Output>().unwrap())
                .sum::<Output>()
        });
        Ok(v.max().unwrap())
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let lines = io::BufReader::new(input).lines();
        let v = lines.collect::<Vec<_>>();
        let v = v.split(|l| (*l).as_ref().unwrap().is_empty());
        let mut v = v
            .map(|v| {
                v.iter()
                    .map(|r| (*r).as_ref().unwrap())
                    .map(|s| s.parse::<Output>().unwrap())
                    .sum::<Output>()
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
