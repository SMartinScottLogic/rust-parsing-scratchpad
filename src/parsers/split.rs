use crate::{Instruction, Parser};

pub struct SplitParser {}

impl SplitParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl Parser for SplitParser {
    type Err = std::convert::Infallible;

    fn parse_str(&self, s: &str) -> Result<Instruction, Self::Err> {
        let mut sources = Vec::new();
        let mut op = None;
        let mut target = None;
        let mut expect_target = false;
        for w in s.split_whitespace() {
            if !expect_target {
                if Self::is_op(w) {
                    if op.is_some() {
                        panic!("multiple operators in instruction");
                    }
                    op = Some(w);
                } else if Self::is_identifier(w) || Self::is_number(w) {
                    sources.push(w);
                } else if "->" == w {
                    expect_target = true;
                }
            } else if Self::is_identifier(w) {
                if target.is_some() {
                    panic!("multiple outputs in instruction");
                }
                target = Some(w);
            } else if "->" == w {
                panic!("multiple output designators");
            } else {
                unreachable!()
            }
        }
        Ok(Instruction::from((
            &sources,
            op.unwrap_or_default(),
            target.unwrap(),
        )))
    }
}

impl SplitParser {
    fn is_identifier(s: &str) -> bool {
        s.chars().all(|c| c.is_lowercase())
    }

    fn is_number(s: &str) -> bool {
        s.chars().all(|c| c.is_digit(10))
    }

    fn is_op(s: &str) -> bool {
        s.chars().all(|c| c.is_uppercase())
    }
}

impl Default for SplitParser {
    fn default() -> Self {
        Self::new()
    }
}
