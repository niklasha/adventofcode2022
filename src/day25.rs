use crate::day::*;
use std::str::FromStr;

pub struct Day25 {}

type Output = String;

impl Day for Day25 {
    fn tag(&self) -> &str {
        "25"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Snafu {
    inner: i64,
}

impl FromStr for Snafu {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.bytes()
            .fold(Ok(0i64), |n, b| {
                n.and_then(|n| {
                    Ok(n * 5
                        + match b {
                            b'=' => -2,
                            b'-' => -1,
                            b'0' | b'1' | b'2' => (b - b'0') as i64,
                            _ => Err(AocError)?,
                        })
                })
            })
            .map(|inner| Self { inner })
    }
}

impl ToString for Snafu {
    fn to_string(&self) -> String {
        let mut s = String::new();
        let mut i = self.inner;
        while i != 0 {
            let r = (i + 2) % 5 - 2;
            s.insert(
                0,
                match r {
                    -2 => '=',
                    -1 => '-',
                    0 => '0',
                    1 => '1',
                    2 => '2',
                    _ => panic!("a real panic situation"),
                },
            );
            i = (i - r) / 5;
        }
        s
    }
}

impl From<i64> for Snafu {
    fn from(i: i64) -> Self {
        Self { inner: i }
    }
}

impl From<Snafu> for i64 {
    fn from(i: Snafu) -> Self {
        i.inner
    }
}

impl Day25 {
    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        io::BufReader::new(input)
            .lines()
            .map(|r| {
                r.map_err(|e| Box::new(e) as Box<dyn error::Error>)
                    .and_then(|s| {
                        s.parse::<Snafu>()
                            .map_err(|e| e.into())
                            .map(|n| i64::from(n))
                    })
            })
            .sum::<BoxResult<i64>>()
            .map(|n| Snafu::from(n).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day25 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122",
            "2=-1=0".to_string(),
        );
    }
}
