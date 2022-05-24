use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::str::FromStr;

mod parsers;

pub use crate::parsers::NomParser;
pub use crate::parsers::PestParser;
pub use crate::parsers::RegexParser;
pub use crate::parsers::SplitParser;
pub use crate::parsers::YaccParser;

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

pub trait Parser {
    fn parse_str(&self, s: &str) -> Instruction;
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Err,
    SET(String, String),
    NOT(String, String),
    AND(String, String, String),
    OR(String, String, String),
    RSHIFT(String, String, String),
    LSHIFT(String, String, String),
}

impl From<(&Vec<&str>, Operation, &str)> for Instruction {
    fn from((sources, operation, target): (&Vec<&str>, Operation, &str)) -> Self {
        let instruction = match operation {
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
        };
        log::debug!("Instruction {instruction:?}");
        instruction
    }
}

impl From<(&Vec<&str>, &str, &str)> for Instruction {
    fn from((sources, op, target): (&Vec<&str>, &str, &str)) -> Self {
        let operation = Operation::from_str(op).unwrap();
        Self::from((sources, operation, target))
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

pub fn load(filename: &str, parser: Box<dyn Parser>) -> std::io::Result<Vec<Instruction>> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    let mut solution = Vec::new();
    for s in reader.lines().flatten() {
        let instruction = parser.parse_str(&s);
        solution.push(instruction);
    }
    Ok(solution)
}
