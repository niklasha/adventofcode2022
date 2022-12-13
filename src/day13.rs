use crate::day::*;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::iter;

pub struct Day13 {}

type Output = usize;

impl Day for Day13 {
    fn tag(&self) -> &str {
        "13"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input())); // XXX Magic offset 0, why? bug?
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

#[derive(Debug, Eq)]
enum Value {
    Int(u8),
    List(Vec<Value>),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(i) => i.fmt(f),
            Self::List(l) => {
                format!("[{}]", l.iter().map(|value| format!("{}", value)).join(",")).fmt(f)
            }
        }
    }
}

impl PartialEq<Self> for Value {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a.cmp(b),
            (Value::Int(a), Value::List(_)) => Value::List(vec![Value::Int(*a)]).cmp(other),
            (Value::List(_), Value::Int(b)) => self.cmp(&Value::List(vec![Value::Int(*b)])),
            (Value::List(a_list), Value::List(b_list)) => {
                match a_list
                    .iter()
                    .zip(b_list.iter())
                    .map(|(a, b)| a.cmp(b))
                    .find(|ord| *ord != Ordering::Equal)
                {
                    Some(ord) => ord,
                    None => a_list.len().cmp(&b_list.len()),
                }
            }
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Day13 {
    fn parse(s: &str) -> BoxResult<Value> {
        s.bytes()
            .fold(Ok((vec![], vec![], None)), |ctx, c| {
                let (mut v, mut stack, mut d) = ctx?;
                match c {
                    b'[' => {
                        stack.push(v);
                        v = vec![];
                    }
                    b'0'..=b'9' => {
                        d = Some(if let Some(d) = d {
                            d * 10 + (c - b'0')
                        } else {
                            c - b'0'
                        });
                    }
                    b',' => {
                        if let Some(i) = d {
                            v.push(Value::Int(i));
                            d = None;
                        }
                    }
                    b']' => {
                        if let Some(i) = d {
                            v.push(Value::Int(i));
                            d = None;
                        }
                        let list = Value::List(v);
                        v = stack.pop().ok_or(AocError)?;
                        v.push(list);
                    }
                    _ => Err(AocError)?,
                }
                Ok((v, stack, d))
            })
            .and_then(|(v, _, _)| v.into_iter().next().ok_or_else(|| AocError.into()))
    }

    fn process(input: &mut dyn io::Read) -> BoxResult<Output> {
        Ok(io::BufReader::new(input)
            .lines()
            .group_by(|r| r.as_ref().map_or(false, |s| s.is_empty()))
            .into_iter()
            .filter(|&(is_blank, _)| !is_blank)
            .map(|(_, mut pair)| {
                let a = Self::parse(&pair.next().ok_or(AocError)??)?;
                let b = Self::parse(&pair.next().ok_or(AocError)??)?;
                Ok(a.cmp(&b)) as BoxResult<_>
            })
            .enumerate()
            .filter(|(_, ord)| ord.as_ref().map_or(true, |ord| *ord == Ordering::Less))
            .map(|(i, _)| i + 1)
            .sum::<Output>())
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process(input)
    }

    fn process2(input: &mut dyn io::Read) -> BoxResult<Output> {
        let mut packets = io::BufReader::new(input)
            .lines()
            .map(|r| r.map_err(|e| e.into()))
            .filter(|l| l.as_ref().map_or(true, |s| !s.is_empty()))
            .chain(iter::once(Ok("[[2]]".to_string())))
            .chain(iter::once(Ok("[[6]]".to_string())))
            .map(|packet| packet.and_then(|p| Self::parse(&p)))
            .collect::<BoxResult<Vec<_>>>()?;
        packets.sort();
        Ok(packets
            .iter()
            .enumerate()
            .map(|(i, p)| {
                println!("{} {}", i, p);
                (i, p)
            })
            .filter(|(_, packet)| match packet {
                Value::List(v) => match &v.as_slice() {
                    &[Value::List(v)] => {
                        matches!(v.as_slice(), &[Value::Int(2)] | &[Value::Int(6)])
                    }
                    _ => false,
                },
                _ => false,
            })
            .map(|(i, _)| i + 1)
            .product())
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process2(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day13 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]",
            13,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day13 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]",
            140,
        );
    }
}
