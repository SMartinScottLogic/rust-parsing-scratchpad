use crate::{Instruction, Parser};

pub struct SplitParser {}

impl SplitParser {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Parser for SplitParser {
    fn parse_str(&self, s: &str) -> Instruction {
        log::debug!("parse_str({s})");
        let mut sources = Vec::new();
        let mut op = None;
        let mut target = None;
        let mut expect_target = false;
        for w in s.split_whitespace() {
            log::trace!("{w}");
            if !expect_target {
                if Self::is_op(w) {
                    assert!(op.is_none(), "multiple operators in instruction");
                    op = Some(w);
                } else if Self::is_identifier(w) || Self::is_number(w) {
                    sources.push(w);
                } else if "->" == w {
                    expect_target = true;
                }
            } else if Self::is_identifier(w) {
                assert!(target.is_none(), "multiple outputs in instruction");
                target = Some(w);
            } else if "->" == w {
                panic!("multiple output designators");
            } else {
                unreachable!()
            }
            log::debug!("{{sources: {sources:?}, op: {op:?}, target: {target:?}, expect_target: {expect_target}}}");
        }
        Instruction::from((&sources, op.unwrap_or_default(), target.unwrap()))
    }
}

impl SplitParser {
    fn is_identifier(s: &str) -> bool {
        s.chars().all(char::is_lowercase)
    }

    fn is_number(s: &str) -> bool {
        s.chars().all(|c| c.is_digit(10))
    }

    fn is_op(s: &str) -> bool {
        s.chars().all(char::is_uppercase)
    }
}

impl Default for SplitParser {
    fn default() -> Self {
        Self::new()
    }
}
