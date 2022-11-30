use std::io;
use crate::day::*;

#[derive(Copy, Clone)]
pub enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

#[derive(Clone)]
pub struct Cpu {
    p: Vec<Instruction>,
    ip: usize,
    a: i64,
    debug: bool,
}

impl Cpu {
    pub fn from(input: &mut dyn io::Read) -> BoxResult<Self> {
        let cpu = Self {
            p: io::BufReader::new(input).lines().map(|r| r.map_err(|e| e.into()).and_then(|s| {
                let (opcode, arg) = s.split_ascii_whitespace().collect_tuple().ok_or(AocError)?;
                let arg = arg.parse()?;
                match opcode {
                    "acc" => Ok(Instruction::Acc(arg)),
                    "jmp" => Ok(Instruction::Jmp(arg)),
                    "nop" => Ok(Instruction::Nop(arg)),
                    _ => Err(AocError.into())
                }
            })).collect::<BoxResult<Vec<Instruction>>>()?,
            ip: 0,
            a: 0,
            debug: false,
        };
        Ok(cpu)
    }

    pub fn debug(mut self, b: bool) -> Self {
        self.debug = b;
        self
    }

    pub fn run(&mut self, init: i64) -> BoxResult<(bool, i64)> {
        self.a = init;
        let mut bp = self.p.iter().map(|_| false).collect::<Vec<_>>();
        while self.ip < self.p.len() && (!self.debug || !bp[self.ip]) {
            bp[self.ip] = true;
            let mut o = 1;
            match self.p[self.ip] {
                Instruction::Acc(x) => self.a += x,
                Instruction::Jmp(x) => o = x,
                Instruction::Nop(_) => (),
            }
            self.ip = (self.ip as i64 + o) as usize;
        }
        Ok((self.ip < self.p.len(), self.a))
    }

    pub fn instruction_index<'a, F>(&'a self, predicate: F) -> impl Iterator<Item=usize> + 'a
    where F: Fn(Instruction) -> bool + 'a {
        self.p.iter().enumerate().filter(move |(_, &i)| predicate(i)).map(|(p, _)| p)
    }

    pub fn patch<F>(mut self, i: usize, f: F) -> Self
        where F: Fn(Instruction) -> Instruction {
        self.p[i] = f(self.p[i]);
        self
    }
}