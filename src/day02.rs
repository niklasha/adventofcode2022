use crate::day::*;

pub struct Day02 {}

type Output = usize;

impl Day for Day02 {
    fn tag(&self) -> &str {
        "02"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn beats(&self, opponent: &Choice) -> bool {
        match self {
            Choice::Rock => *opponent == Self::Scissors,
            Choice::Paper => *opponent == Self::Rock,
            Choice::Scissors => *opponent == Self::Paper,
        }
    }

    fn better(&self) -> Choice {
        match self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        }
    }

    fn worse(&self) -> Choice {
        match self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
    }

    fn value(&self) -> Output {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

impl TryFrom<char> for Choice {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => {
                println!("{}", value);
                Err(())
            }
        }
    }
}

impl Day02 {
    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let lines = io::BufReader::new(input).lines();
        Ok(lines
            .map(|r| {
                let s = r.unwrap();
                let mut cs = s.chars();
                let o = cs.next().unwrap();
                cs.next();
                let m = cs.next().unwrap();
                let opponent = Choice::try_from(o).unwrap();
                let me = Choice::try_from(m).unwrap();
                let score = if opponent.beats(&me) {
                    0
                } else if opponent == me {
                    3
                } else {
                    6
                };
                me.value() + score
            })
            .sum())
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let lines = io::BufReader::new(input).lines();
        Ok(lines
            .map(|r| {
                let s = r.unwrap();
                let mut cs = s.chars();
                let o = cs.next().unwrap();
                cs.next();
                let result = cs.next().unwrap();
                let opponent = Choice::try_from(o).unwrap();
                let me = match result {
                    'X' => Choice::worse(&opponent),
                    'Y' => opponent,
                    'Z' => Choice::better(&opponent),
                    _ => panic!("foo"),
                };
                let score = if opponent.beats(&me) {
                    0
                } else if opponent == me {
                    3
                } else {
                    6
                };
                me.value() + score
            })
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day02 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "A Y
B X
C Z
",
            15,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day02 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "A Y
B X
C Z
",
            12,
        );
    }
}
