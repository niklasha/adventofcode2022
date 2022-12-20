use crate::day::*;

pub struct Day20 {}

type Output = i64;

impl Day for Day20 {
    fn tag(&self) -> &str {
        "20"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

impl Day20 {
    fn process<F>(input: &mut dyn io::Read, f: i64, mix: F) -> BoxResult<Output>
    where
        F: Fn(&mut Vec<(usize, Output)>, usize, &Vec<(usize, Output)>) -> Result<(), AocError>,
    {
        let mut v = io::BufReader::new(input)
            .lines()
            .map(|l| l.map_err(|e| e.into()))
            .map(|l: BoxResult<_>| l.and_then(|l| Ok(l.as_str().parse::<i64>()?)))
            .enumerate()
            .map(|(i, n)| n.map(|n| (i, n * f)))
            .collect::<Result<Vec<_>, _>>()?;
        let l = v.len();
        let init = v.clone();
        mix(&mut v, l, &init)?;
        let o = v.iter().position(|&(_, x)| x == 0).ok_or(AocError)?;
        Ok(v[(o + 1000) % l].1 + v[(o + 2000) % l].1 + v[(o + 3000) % l].1)
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process(
            input,
            1,
            |v: &mut Vec<(usize, Output)>,
             l: usize,
             init: &Vec<(usize, Output)>|
             -> Result<(), AocError> { Self::mix(v, l, &init) },
        )
    }

    fn mix(
        v: &mut Vec<(usize, Output)>,
        l: usize,
        init: &Vec<(usize, Output)>,
    ) -> Result<(), AocError> {
        for (i, n) in init {
            let j = v.iter().position(|&(j, _)| *i == j).ok_or(AocError)?;
            v.remove(j);
            let k = (j as Output + n).rem_euclid((l - 1) as Output) as usize;
            v.insert(k, init[*i]);
        }
        Ok(())
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process(
            input,
            811589153,
            |v: &mut Vec<(usize, Output)>,
             l: usize,
             init: &Vec<(usize, Output)>|
             -> Result<(), AocError> {
                for _i in 0..10 {
                    Self::mix(v, l, &init)?
                }
                Ok(())
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day20 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "1
2
-3
3
-2
0
4",
            3,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day20 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "1
2
-3
3
-2
0
4",
            1623178306,
        );
    }
}
