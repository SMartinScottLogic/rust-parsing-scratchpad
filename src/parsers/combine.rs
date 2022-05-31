extern crate combine;
use combine::parser::char::{digit, lower, spaces, string, upper};
use combine::stream::easy;
use combine::{attempt, choice, many1, EasyParser, Parser};

use crate::Instruction;

pub struct CombineParser {}

impl CombineParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::Parser for CombineParser {
    fn parse_str(&self, source: &str) -> Instruction {
        let wire = || many1(lower());
        let value = || many1(digit());
        let op = || many1(upper());

        let sos = (
            choice((wire(), value())),
            spaces().with(op()),
            spaces().with(choice((wire(), value()))),
            spaces().with(string("->")),
            spaces().with(wire()),
        )
            .map(
                |(s1, o, s2, _, t): (String, String, String, &str, String)| {
                    Instruction::from((&vec![s1.as_str(), s2.as_str()], o.as_str(), t.as_str()))
                },
            );
        let os = (
            op(),
            spaces().with(choice((wire(), value()))),
            spaces().with(string("->")),
            spaces().with(wire()),
        )
            .map(|(o, s, _, t)| Instruction::from((&vec![s.as_str()], o.as_str(), t.as_str())));
        let s = (
            choice((wire(), value())),
            spaces().with(string("->")),
            spaces().with(wire()),
        )
            .map(|(s, _, t)| Instruction::from((&vec![s.as_str()], "", t.as_str())));
        let r: Result<(Instruction, &str), easy::ParseError<&str>> =
            choice((attempt(s), attempt(os), attempt(sos))).easy_parse(source);
        println!("{source} {r:?}");
        r.unwrap().0
    }
}

impl Default for CombineParser {
    fn default() -> Self {
        Self::new()
    }
}
