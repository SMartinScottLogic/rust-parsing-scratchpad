use itertools::Itertools;
use pest::{iterators::Pair, Parser};

use crate::Instruction;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"] // relative to src
struct InnerParser;

pub struct PestParser {}

impl crate::Parser for PestParser {
    fn parse_str(&self, s: &str) -> Instruction {
        let r = InnerParser::parse(Rule::instruction, s)
            .unwrap()
            .next()
            .unwrap();
        let r = Self::parse_value(r);
        r.0.unwrap()
    }
}

impl PestParser {
    fn parse_value(pair: Pair<Rule>) -> (Option<Instruction>, Option<&str>) {
        match pair.as_rule() {
            Rule::instruction => pair.into_inner().map(Self::parse_value).next().unwrap(),
            Rule::instruction_sos => {
                let d = pair.into_inner().map(Self::parse_value).next_tuple().map(
                    |(source1, op, source2, target)| {
                        Instruction::from((
                            &vec![source1.1.unwrap(), source2.1.unwrap()],
                            op.1.unwrap(),
                            target.1.unwrap(),
                        ))
                    },
                );
                (d, None)
            }
            Rule::instruction_os => {
                let d = pair
                    .into_inner()
                    .map(Self::parse_value)
                    .collect_tuple()
                    .map(|(op, source, target)| {
                        Instruction::from((
                            &vec![source.1.unwrap()],
                            op.1.unwrap(),
                            target.1.unwrap(),
                        ))
                    });
                (d, None)
            }
            Rule::instruction_s => {
                let d = pair
                    .into_inner()
                    .map(Self::parse_value)
                    .collect_tuple()
                    .map(|(source, target)| {
                        Instruction::from((&vec![source.1.unwrap()], "", target.1.unwrap()))
                    });
                (d, None)
            }
            Rule::identifier => (None, Some(pair.as_str())),
            Rule::source => (None, Some(pair.as_str())),
            Rule::number => (None, Some(pair.as_str())),
            Rule::op => (None, Some(pair.as_str())),
        }
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PestParser {
    fn default() -> Self {
        Self::new()
    }
}
