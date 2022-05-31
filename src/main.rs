use parsing::*;
use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter, Debug)]
pub enum ParserType {
    Combine,
    Nom,
    Pest,
    Regex,
    Split,
    Yacc,
}

struct ParserFactory {}

impl ParserFactory {
    fn build(parser_type: &ParserType) -> Box<dyn Parser> {
        match parser_type {
            ParserType::Combine => Box::new(CombineParser::new()),
            ParserType::Nom => Box::new(NomParser::new()),
            ParserType::Pest => Box::new(PestParser::new()),
            ParserType::Regex => Box::new(RegexParser::new()),
            ParserType::Split => Box::new(SplitParser::new()),
            ParserType::Yacc => Box::new(YaccParser::new()),
        }
    }
}

fn run(parser_type: &ParserType) -> std::io::Result<Vec<Instruction>> {
    let filename = "input.txt";
    let filename = "day7.input";
    let parser = ParserFactory::build(parser_type);
    load(filename, parser)
}

fn main() {
    env_logger::init();

    for parser_type in ParserType::iter() {
        match run(&parser_type) {
            Ok(input) => log::info!("{:?}: {}", parser_type, input.len()),
            Err(e) => log::error!("{:?}: {}", parser_type, e),
        }
    }
}

#[cfg(test)]
mod tests {
    use parsing::*;

    #[test]
    fn combine() {
        let parser = CombineParser::new();

        let tests = map![
            "bn RSHIFT 2 -> bo" => Instruction::RSHIFT("bn".to_string(), "2".to_string(), "bo".to_string()),
            "cj OR cp -> cq" => Instruction::OR("cj".to_string(), "cp".to_string(), "cq".to_string()),
            "lx -> a" => Instruction::SET("lx".to_string(), "a".to_string()),
            "NOT ax -> ay" => Instruction::NOT("ax".to_string(), "ay".to_string()),
            "lr AND lt -> lu" => Instruction::AND("lr".to_string(), "lt".to_string(), "lu".to_string()),
            "hb LSHIFT 1 -> hv" => Instruction::LSHIFT("hb".to_string(), "1".to_string(), "hv".to_string())
        ];
        for (input, expected) in tests {
            assert_eq!(expected, parser.parse_str(input));
        }
    }

    #[test]
    fn regex() {
        let parser = RegexParser::new();

        let tests = map![
            "bn RSHIFT 2 -> bo" => Instruction::RSHIFT("bn".to_string(), "2".to_string(), "bo".to_string()),
            "cj OR cp -> cq" => Instruction::OR("cj".to_string(), "cp".to_string(), "cq".to_string()),
            "lx -> a" => Instruction::SET("lx".to_string(), "a".to_string()),
            "NOT ax -> ay" => Instruction::NOT("ax".to_string(), "ay".to_string()),
            "lr AND lt -> lu" => Instruction::AND("lr".to_string(), "lt".to_string(), "lu".to_string()),
            "hb LSHIFT 1 -> hv" => Instruction::LSHIFT("hb".to_string(), "1".to_string(), "hv".to_string())
        ];
        for (input, expected) in tests {
            assert_eq!(expected, parser.parse_str(input));
        }
    }

    #[test]
    fn split() {
        let parser = SplitParser::new();

        let tests = map![
            "bn RSHIFT 2 -> bo" => Instruction::RSHIFT("bn".to_string(), "2".to_string(), "bo".to_string()),
            "cj OR cp -> cq" => Instruction::OR("cj".to_string(), "cp".to_string(), "cq".to_string()),
            "lx -> a" => Instruction::SET("lx".to_string(), "a".to_string()),
            "NOT ax -> ay" => Instruction::NOT("ax".to_string(), "ay".to_string()),
            "lr AND lt -> lu" => Instruction::AND("lr".to_string(), "lt".to_string(), "lu".to_string()),
            "hb LSHIFT 1 -> hv" => Instruction::LSHIFT("hb".to_string(), "1".to_string(), "hv".to_string())
        ];
        for (input, expected) in tests {
            assert_eq!(expected, parser.parse_str(input));
        }
    }

    #[test]
    fn nom() {
        let parser = NomParser::new();

        let tests = map![
            "bn RSHIFT 2 -> bo" => Instruction::RSHIFT("bn".to_string(), "2".to_string(), "bo".to_string()),
            "cj OR cp -> cq" => Instruction::OR("cj".to_string(), "cp".to_string(), "cq".to_string()),
            "lx -> a" => Instruction::SET("lx".to_string(), "a".to_string()),
            "NOT ax -> ay" => Instruction::NOT("ax".to_string(), "ay".to_string()),
            "lr AND lt -> lu" => Instruction::AND("lr".to_string(), "lt".to_string(), "lu".to_string()),
            "hb LSHIFT 1 -> hv" => Instruction::LSHIFT("hb".to_string(), "1".to_string(), "hv".to_string())
        ];
        for (input, expected) in tests {
            assert_eq!(expected, parser.parse_str(input));
        }
    }

    #[test]
    fn pest() {
        let parser = PestParser::new();

        let tests = map![
            "bn RSHIFT 2 -> bo" => Instruction::RSHIFT("bn".to_string(), "2".to_string(), "bo".to_string()),
            "cj OR cp -> cq" => Instruction::OR("cj".to_string(), "cp".to_string(), "cq".to_string()),
            //"lx -> a" => Instruction::SET("lx".to_string(), "a".to_string()),
            "NOT ax -> ay" => Instruction::NOT("ax".to_string(), "ay".to_string()),
            "lr AND lt -> lu" => Instruction::AND("lr".to_string(), "lt".to_string(), "lu".to_string()),
            "hb LSHIFT 1 -> hv" => Instruction::LSHIFT("hb".to_string(), "1".to_string(), "hv".to_string())
        ];
        for (input, expected) in tests {
            assert_eq!(expected, parser.parse_str(input));
        }
    }

    #[test]
    fn yacc() {
        let parser = YaccParser::new();

        let tests = map![
            "bn RSHIFT 2 -> bo" => Instruction::RSHIFT("bn".to_string(), "2".to_string(), "bo".to_string()),
            "cj OR cp -> cq" => Instruction::OR("cj".to_string(), "cp".to_string(), "cq".to_string()),
            //"lx -> a" => Instruction::SET("lx".to_string(), "a".to_string()),
            "NOT ax -> ay" => Instruction::NOT("ax".to_string(), "ay".to_string()),
            "lr AND lt -> lu" => Instruction::AND("lr".to_string(), "lt".to_string(), "lu".to_string()),
            "hb LSHIFT 1 -> hv" => Instruction::LSHIFT("hb".to_string(), "1".to_string(), "hv".to_string())
        ];
        for (input, expected) in tests {
            assert_eq!(expected, parser.parse_str(input));
        }
    }
}
