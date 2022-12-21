use crate::day::*;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Day21 {}

type Output = i64;

impl Day for Day21 {
    fn tag(&self) -> &str {
        "21"
    }

    fn part1(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part1_impl(&mut *input()));
    }

    fn part2(&self, input: &dyn Fn() -> Box<dyn io::Read>) {
        println!("{:?}", self.part2_impl(&mut *input()));
    }
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Plus,
    Minus,
    Times,
    Divide,
}

impl FromStr for Operation {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Self::Plus,
            "-" => Self::Minus,
            "*" => Self::Times,
            "/" => Self::Divide,
            _ => Err(AocError)?,
        })
    }
}

#[derive(Debug)]
struct Expression {
    operation: Operation,
    left: String,
    right: String,
}

impl FromStr for Expression {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        let expression = Self {
            left: tokens.next().ok_or(AocError)?.to_owned(),
            operation: tokens.next().ok_or(AocError)?.parse()?,
            right: tokens.next().ok_or(AocError)?.to_owned(),
        };
        if tokens.next().is_some() {
            Err(AocError.into())
        } else {
            Ok(expression)
        }
    }
}

impl Expression {
    fn evaluate(&self, choir: &Choir) -> BoxResult<Output> {
        let left = choir.yell(&self.left)?;
        let right = choir.yell(&self.right)?;
        Ok(match self.operation {
            Operation::Plus => left + right,
            Operation::Minus => left - right,
            Operation::Times => left * right,
            Operation::Divide => left / right,
        })
    }
}

#[derive(Debug)]
enum Job {
    Number(Output),
    Expression(Expression),
}

impl FromStr for Job {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.chars().next().ok_or(AocError)?.is_ascii_digit() {
            Self::Number(s.parse()?)
        } else {
            Self::Expression(s.parse()?)
        })
    }
}

#[derive(Debug)]
struct Monkey {
    name: String,
    job: Job,
}

impl FromStr for Monkey {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, job) = s.split_once(':').ok_or(AocError)?;
        Ok(Self {
            name: name.to_owned(),
            job: job.trim().parse::<Job>()?,
        })
    }
}

impl Monkey {
    fn yell(&self, choir: &Choir) -> BoxResult<Output> {
        match &self.job {
            Job::Number(n) => Ok(*n),
            Job::Expression(expression) => expression.evaluate(choir),
        }
    }

    fn find_humn(&self, choir: &Choir) -> BoxResult<Option<Vec<bool>>> {
        if self.name == "humn" {
            return Ok(Some(vec![]));
        }
        Ok(match &self.job {
            Job::Number(_) => None,
            Job::Expression(expression) => {
                let left = choir.monkeys.get(&expression.left).ok_or(AocError)?;
                let right = choir.monkeys.get(&expression.right).ok_or(AocError)?;
                if let Some(mut path) = left.find_humn(choir)? {
                    path.push(true);
                    Some(path)
                } else {
                    if let Some(mut path) = right.find_humn(&choir)? {
                        path.push(false);
                        Some(path)
                    } else {
                        None
                    }
                }
            }
        })
    }

    fn deduce_humn_yell(&self, choir: &Choir, mut path: Vec<bool>, n: Output) -> BoxResult<Output> {
        Ok(if let Job::Expression(expression) = &self.job {
            let left = choir.monkeys.get(&expression.left).ok_or(AocError)?;
            let right = choir.monkeys.get(&expression.right).ok_or(AocError)?;
            let humn_is_left = path.pop().ok_or(AocError)?;
            let (humn_branch, other) = if humn_is_left {
                (left, right)
            } else {
                (right, left)
            };
            humn_branch.deduce_humn_yell(
                choir,
                path,
                match if self.name == "root" {
                    Operation::Minus
                } else {
                    expression.operation
                } {
                    Operation::Plus => n - other.yell(choir)?,
                    Operation::Minus => {
                        if humn_is_left {
                            n + other.yell(choir)?
                        } else {
                            other.yell(choir)? - n
                        }
                    }
                    Operation::Times => n / other.yell(choir)?,
                    Operation::Divide => {
                        if humn_is_left {
                            n * other.yell(choir)?
                        } else {
                            other.yell(choir)? / n
                        }
                    }
                },
            )?
        } else {
            if self.name == "humn" {
                n
            } else {
                Err(AocError)?
            }
        })
    }
}

#[derive(Debug)]
struct Choir {
    monkeys: HashMap<String, Monkey>,
}

impl Choir {
    fn from(input: &mut dyn io::Read) -> BoxResult<Choir> {
        Ok(io::BufReader::new(input)
            .lines()
            .map(|l| l.map_err(|e| e.into()))
            .map(|l: BoxResult<_>| {
                l.and_then(|l| {
                    let monkey = l.as_str().parse::<Monkey>()?;
                    Ok((monkey.name.to_owned(), monkey))
                })
            })
            .collect::<Result<HashMap<_, _>, _>>()
            .map(|monkeys| Choir { monkeys })?)
    }

    fn yell(&self, name: &str) -> BoxResult<Output> {
        self.monkeys.get(name).ok_or(AocError)?.yell(self)
    }

    fn patch_humn(&mut self, n: Output) -> BoxResult<()> {
        self.monkeys.get_mut("humn").ok_or(AocError)?.job = Job::Number(n);
        Ok(())
    }

    fn is_balanced(&self) -> BoxResult<bool> {
        let root = self.monkeys.get("root").ok_or(AocError)?;
        if let Job::Expression(expression) = &root.job {
            Ok(self
                .monkeys
                .get(&expression.left)
                .ok_or(AocError)?
                .yell(self)?
                == self
                    .monkeys
                    .get(&expression.right)
                    .ok_or(AocError)?
                    .yell(self)?)
        } else {
            Err(AocError)?
        }
    }
}

impl Day21 {
    fn part1_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let choir = Choir::from(input)?;
        Ok(choir.yell("root")?)
    }

    fn part2_impl(&self, input: &mut dyn io::Read) -> BoxResult<Output> {
        let mut choir = Choir::from(input)?;
        Self::deduce(choir)
    }

    fn deduce(choir: Choir) -> BoxResult<Output> {
        let root = choir.monkeys.get("root").ok_or(AocError)?;
        let path = root.find_humn(&choir)?.ok_or(AocError)?;
        root.deduce_humn_yell(&choir, path, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test1(s: &str, f: Output) {
        assert_eq!(Day21 {}.part1_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part1() {
        test1(
            "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32",
            152,
        );
    }

    fn test2(s: &str, f: Output) {
        assert_eq!(Day21 {}.part2_impl(&mut s.as_bytes()).ok(), Some(f));
    }

    #[test]
    fn part2() {
        test2(
            "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32",
            301,
        );
    }
}
