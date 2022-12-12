use crate::day::*;
use std::fmt::{Debug, Display};
use std::io::Read;
use std::ops::{Add, Div, Mul, Rem};
use std::str::FromStr;

pub struct Day11 {}

type Output = usize;

impl Day for Day11 {
    fn tag(&self) -> &str {
        "11"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

#[derive(Clone, Copy, Debug)]
struct Item<T> {
    level: T,
}

impl<T: FromStr> FromStr for Item<T> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { level: s.parse()? })
    }
}

#[derive(Debug)]
struct Monkey<T> {
    items: Vec<Item<T>>,
    operation: String,
    divisor: T,
    true_target: usize,
    false_target: usize,
    count: usize,
}

impl<T> Monkey<T>
where
    T: Copy
        + FromStr
        + Add<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + PartialEq<usize>,
{
    fn evaluate(&self, item: Item<T>) -> BoxResult<Item<T>>
    where
        <T as FromStr>::Err: error::Error + 'static,
    {
        let tokens = self
            .operation
            .as_str()
            .split_whitespace()
            .collect::<Vec<_>>();
        let lhs = match tokens.first() {
            Some(&"old") => item,
            Some(s) => s.parse()?,
            _ => Err(AocError)?,
        };
        let rhs = match tokens.get(2) {
            Some(&"old") => item,
            Some(s) => s.parse()?,
            _ => Err(AocError)?,
        };
        match tokens.get(1) {
            Some(&"+") => Ok(Item {
                level: lhs.level + rhs.level,
            }),
            Some(&"*") => Ok(Item {
                level: lhs.level * rhs.level,
            }),
            _ => Err(AocError.into()),
        }
    }

    fn inspect(&self, relief: T, modulus: T) -> BoxResult<Vec<(Item<T>, usize)>>
    where
        <T as FromStr>::Err: std::error::Error + 'static,
    {
        self.items
            .iter()
            .map(|item| {
                let level = (self.evaluate(*item)?.level / relief) % modulus;
                let target = if level % self.divisor == 0 {
                    self.true_target
                } else {
                    self.false_target
                };
                Ok((Item { level }, target))
            })
            .collect()
    }
}

impl<T: FromStr> FromStr for Monkey<T>
where
    <T as FromStr>::Err: error::Error + 'static,
{
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split('\n');
        let _no = lines
            .next()
            .ok_or(AocError)?
            .split_whitespace()
            .last()
            .ok_or(AocError)?
            .trim_end_matches(':')
            .parse::<usize>()?;
        let items = lines
            .next()
            .ok_or(AocError)?
            .strip_prefix("  Starting items: ")
            .ok_or(AocError)?
            .split(", ")
            .map(|s| s.parse::<Item<T>>().map_err(|e| e.into()))
            .collect::<BoxResult<Vec<_>>>()?;
        let operation = lines
            .next()
            .ok_or(AocError)?
            .strip_prefix("  Operation: new = ")
            .ok_or(AocError)?
            .to_owned();
        let divisor = lines
            .next()
            .ok_or(AocError)?
            .strip_prefix("  Test: divisible by ")
            .ok_or(AocError)?
            .parse::<T>()?;
        let true_target = lines
            .next()
            .ok_or(AocError)?
            .strip_prefix("    If true: throw to monkey ")
            .ok_or(AocError)?
            .parse::<usize>()?;
        let false_target = lines
            .next()
            .ok_or(AocError)?
            .strip_prefix("    If false: throw to monkey ")
            .ok_or(AocError)?
            .parse::<usize>()?;
        Ok(Self {
            items,
            operation,
            divisor,
            true_target,
            false_target,
            count: 0,
        })
    }
}

#[derive(Debug)]
struct Monkeys<T>(Vec<Monkey<T>>);

impl<T: FromStr> FromStr for Monkeys<T>
where
    <T as FromStr>::Err: error::Error + 'static,
{
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split("\n\n")
            .map(|s| s.parse::<Monkey<T>>())
            .collect::<Result<Vec<_>, _>>()
            .map(|v| Self(v))
    }
}

impl<T> Monkeys<T>
where
    T: Copy
        + Debug
        + FromStr
        + Add<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + PartialEq<usize>,
{
    fn get(&self, i: usize) -> Option<&Monkey<T>> {
        self.0.get(i)
    }

    fn get_mut(&mut self, i: usize) -> Option<&mut Monkey<T>> {
        self.0.get_mut(i)
    }

    fn do_turn(&mut self, relief: T, modulus: T) -> BoxResult<()>
    where
        <T as FromStr>::Err: error::Error + 'static,
    {
        for source in 0..self.0.len() {
            let moves = self.get(source).ok_or(AocError)?.inspect(relief, modulus)?;
            for (item, target) in moves {
                {
                    let target = self.get_mut(target).ok_or(AocError)?;
                    target.items.push(item);
                }
                self.get_mut(source).ok_or(AocError)?.count += 1;
            }
            self.get_mut(source).ok_or(AocError)?.items = vec![];
        }
        Ok(())
    }

    fn sort(&mut self) {
        self.0.sort_by_key(|monkey| usize::MAX - monkey.count)
    }
}

impl Day11 {
    fn process<T: FromStr + Debug>(
        input: &mut dyn io::Read,
        relief: T,
        round_count: usize,
    ) -> BoxResult<Output>
    where
        T: Copy
            + FromStr
            + Display
            + Add<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Rem<Output = T>
            + PartialEq<usize>,
        <T as FromStr>::Err: error::Error + 'static,
    {
        let mut monkeys = io::BufReader::new(input)
            .bytes()
            .map(|b| b.map(|b| b as char))
            .collect::<Result<String, _>>()?
            .parse::<Monkeys<T>>()?;
        let modulus = monkeys
            .0
            .iter()
            .map(|m| m.divisor)
            .fold("1".parse::<T>()?, |product, n| product * n);
        for _ in 0..round_count {
            monkeys.do_turn(relief, modulus)?;
        }
        monkeys.sort();
        Ok(monkeys
            .0
            .iter()
            .take(2)
            .map(|monkey| monkey.count)
            .product())
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process::<usize>(input, 3, 20)
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process::<usize>(input, 1, 10000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day11 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1",
            10605,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day11 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1",
            2713310158,
        );
    }
}
