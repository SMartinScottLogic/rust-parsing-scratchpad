use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::char,
    sequence::tuple,
    IResult,
};

use crate::{Instruction, Parser};

pub struct NomParser {}

impl Parser for NomParser {
    type Err = std::convert::Infallible;

    fn parse_str(&self, s: &str) -> Result<Instruction, Self::Err> {
        let (_, i) = Self::instruction(s).unwrap();
        Ok(i)
    }
}

impl Default for NomParser {
    fn default() -> Self {
        Self::new()
    }
}

impl NomParser {
    pub fn new() -> Self {
        Self {}
    }

    fn is_digit(c: char) -> bool {
        c.is_digit(10)
    }
    fn is_upper(c: char) -> bool {
        c.is_uppercase()
    }

    fn is_lower(c: char) -> bool {
        c.is_lowercase()
    }

    fn op(input: &str) -> IResult<&str, &str> {
        take_while1(Self::is_upper)(input)
    }

    fn identifier(input: &str) -> IResult<&str, &str> {
        take_while1(Self::is_lower)(input)
    }

    fn source(input: &str) -> IResult<&str, &str> {
        alt((take_while1(Self::is_digit), Self::identifier))(input)
    }

    fn instruction_s_o_s(input: &str) -> IResult<&str, Instruction> {
        let (residue, (source1, _, op, _, source2, _, target)) = tuple((
            Self::source,
            char(' '),
            Self::op,
            char(' '),
            Self::source,
            tag(" -> "),
            Self::identifier,
        ))(input)?;
        let instruction = Instruction::from((&vec![source1, source2], op, target));

        Ok((residue, instruction))
    }

    fn instruction_o_s(input: &str) -> IResult<&str, Instruction> {
        let (residue, (op, _, source, _, target)) = tuple((
            Self::op,
            char(' '),
            Self::source,
            tag(" -> "),
            Self::identifier,
        ))(input)?;
        let instruction = Instruction::from((&vec![source], op, target));

        Ok((residue, instruction))
    }

    fn instruction_s(input: &str) -> IResult<&str, Instruction> {
        let (residue, (source, _, target)) =
            tuple((Self::source, tag(" -> "), Self::identifier))(input)?;
        let instruction = Instruction::from((&vec![source], "", target));

        Ok((residue, instruction))
    }

    fn instruction(input: &str) -> IResult<&str, Instruction> {
        alt((
            Self::instruction_s_o_s,
            Self::instruction_o_s,
            Self::instruction_s,
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use nom::{error::Error, error::ErrorKind::TakeWhile1, IResult};
    use std::collections::HashMap;

    use crate::{map, Instruction, NomParser};

    #[test]
    fn op() {
        let test: HashMap<&str, IResult<&str, &str>> = map![
            "NOT b -> c" => Ok((" b -> c", "NOT")),
            "not b -> c" => Err(nom::Err::Error(Error { input: "not b -> c", code: TakeWhile1 }))
        ];
        for (input, expected) in test {
            let actual = NomParser::op(input);
            println!("{actual:?}");
            println!("{expected:?}");
            assert_eq!(NomParser::op(input), expected);
        }
    }

    #[test]
    fn source() {
        let test: HashMap<&str, IResult<&str, &str>> = map![
            "123 -> a" => Ok((" -> a", "123")),
            "ab -> a" => Ok((" -> a", "ab"))
        ];
        for (input, expected) in test {
            let actual = NomParser::source(input);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn target() {
        let test: HashMap<&str, IResult<&str, &str>> = map![
            "abcd" => Ok(("", "abcd")),
            "123" => Err(nom::Err::Error(Error { input: "123", code: TakeWhile1 })),
            "" => Err(nom::Err::Error(Error { input: "", code: TakeWhile1 }))
        ];
        for (input, expected) in test {
            let actual = NomParser::identifier(input);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn instruction() {
        let test: HashMap<&str, IResult<&str, Instruction>> = map![
            "NOT e -> f" => Ok(("", Instruction::NOT("e".to_string(), "f".to_string()))),
            "e -> f" => Ok(("", Instruction::SET("e".to_string(), "f".to_string()))),
            "a AND b -> c" => Ok(("", Instruction::AND("a".to_string(), "b".to_string(), "c".to_string())))
        ];
        for (input, expected) in test {
            let actual = NomParser::instruction(input);
            assert_eq!(actual, expected);
        }
    }
}
