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
    fn fight(&self, opponent: &Choice) -> Outcome {
        if self == opponent {
            Outcome::Draw
        } else if *self == opponent.successor() {
            Outcome::Win
        } else {
            Outcome::Loss
        }
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

impl TryFrom<&str> for Choice {
    type Error = AocError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(AocError),
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

impl TryFrom<&str> for Outcome {
    type Error = AocError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(AocError),
        }
    }
}

impl Day02 {
    fn process<F>(input: &mut dyn io::Read, f: F) -> BoxResult<Output>
    where
        F: Fn(&str, &Choice) -> BoxResult<(Choice, Outcome)>,
    {
        io::BufReader::new(input)
            .lines()
            .map(|r| {
                let s = r?;
                let mut tokens = s.split_whitespace();
                let opponent = Choice::try_from(tokens.next().ok_or(AocError)?)?;
                let token = tokens.next().ok_or(AocError)?;
                let (me, outcome) = f(token, &opponent)?;
                Ok(me.value() + outcome.value())
            })
            .sum()
    }

    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process(input, |token, opponent| {
            let me = Choice::try_from(token)?;
            Ok((me, me.fight(opponent)))
        })
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        Self::process(input, |token, opponent| {
            let outcome = Outcome::try_from(token)?;
            Ok((outcome.opponent(opponent), outcome))
        })
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
