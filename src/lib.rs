use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::str::FromStr;

mod parsers;

pub use crate::parsers::NomParser;
pub use crate::parsers::PestParser;
pub use crate::parsers::RegexParser;
pub use crate::parsers::SplitParser;

#[macro_export]
macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

pub trait Parser: Sized {
    type Err;

    fn parse_str(&self, s: &str) -> Result<Instruction, Self::Err>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    SET(String, String),
    NOT(String, String),
    AND(String, String, String),
    OR(String, String, String),
    RSHIFT(String, String, String),
    LSHIFT(String, String, String),
}

impl From<(&Vec<&str>, &str, &str)> for Instruction {
    fn from((sources, op, target): (&Vec<&str>, &str, &str)) -> Self {
        match Operation::from_str(op).unwrap() {
            Operation::SET => {
                Instruction::SET(sources.get(0).unwrap().to_string(), target.to_owned())
            }
            Operation::NOT => {
                Instruction::NOT(sources.get(0).unwrap().to_string(), target.to_owned())
            }
            Operation::OR => Instruction::OR(
                sources.get(0).unwrap().to_string(),
                sources.get(1).unwrap().to_string(),
                target.to_owned(),
            ),
            Operation::AND => Instruction::AND(
                sources.get(0).unwrap().to_string(),
                sources.get(1).unwrap().to_string(),
                target.to_owned(),
            ),
            Operation::RSHIFT => Instruction::RSHIFT(
                sources.get(0).unwrap().to_string(),
                sources.get(1).unwrap().to_string(),
                target.to_owned(),
            ),
            Operation::LSHIFT => Instruction::LSHIFT(
                sources.get(0).unwrap().to_string(),
                sources.get(1).unwrap().to_string(),
                target.to_owned(),
            ),
        }
    }
}

#[derive(Debug)]
pub enum Operation {
    SET,
    NOT,
    AND,
    OR,
    RSHIFT,
    LSHIFT,
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Self::AND),
            "OR" => Ok(Self::OR),
            "RSHIFT" => Ok(Self::RSHIFT),
            "LSHIFT" => Ok(Self::LSHIFT),
            "NOT" => Ok(Self::NOT),
            "" => Ok(Self::SET),
            _ => Err(Error::new(std::io::ErrorKind::Other, s)),
        }
    }
}

pub fn load<P: Parser>(filename: &str, parser: P) -> std::io::Result<Vec<Instruction>>
where
    <P as Parser>::Err: Debug,
{
    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    let mut solution = Vec::new();
    for s in reader.lines().flatten() {
        if let Ok(instruction) = parser.parse_str(&s) {
            solution.push(instruction);
        }
    }
    Ok(solution)
}
