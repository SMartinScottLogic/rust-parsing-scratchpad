use crate::{Instruction, Parser};

pub struct RegexParser {
    re: regex::Regex,
}

impl Parser for RegexParser {
    fn parse_str(&self, s: &str) -> Instruction {
        let c = self.re.captures(s).unwrap();
        let mut sources = Vec::new();
        if let Some(source) = c.name("source1") {
            sources.push(source.as_str());
        }
        if let Some(source) = c.name("source2") {
            sources.push(source.as_str());
        }
        let op = c.name("op").map(|s| s.as_str()).unwrap_or_default();
        let target = c.name("target").map(|s| s.as_str()).unwrap();

        Instruction::try_from((&sources, op, target)).unwrap()
    }
}

impl RegexParser {
    /// # Panics
    ///
    /// Will panic if internal regex is invalid
    #[must_use]
    pub fn new() -> Self {
        let re = regex::Regex::new(r"^(((?P<source1>([0-9]+|[a-z]+)) )?(?P<op>[A-Z]+) )?(?P<source2>([0-9]+|[a-z]+)) -> (?P<target>[a-z]+)$").unwrap();
        Self { re }
    }
}

impl Default for RegexParser {
    fn default() -> Self {
        Self::new()
    }
}
