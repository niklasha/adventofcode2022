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
    fn beats(&self, opponent: &Self) -> bool {
        opponent.successor() == *self
    }

    fn successor(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn value(&self) -> Output {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
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
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn opponent(&self, choice: &Choice) -> Choice {
        match self {
            Self::Loss => choice.successor().successor(),
            Self::Draw => *choice,
            Self::Win => choice.successor(),
        }
    }

    fn value(&self) -> Output {
        match self {
            Self::Loss => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Self::Loss),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => Err(()),
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
                let outcome = if opponent.beats(&me) {
                    Outcome::Loss
                } else if opponent == me {
                    Outcome::Draw
                } else {
                    Outcome::Win
                };
                me.value() + outcome.value()
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
                let outcome = Outcome::try_from(cs.next().unwrap()).unwrap();
                let opponent = Choice::try_from(o).unwrap();
                let me = outcome.opponent(&opponent);
                me.value() + outcome.value()
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
