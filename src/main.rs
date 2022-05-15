use parsing::*;

fn main() {
    let parser = RegexParser::new();

    if let Ok(input) = load("input.txt", parser) {
        for i in &input {
            println!("{i:?}");
        }
    }
}

#[cfg(test)]
mod tests {
    use parsing::*;

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
            assert_eq!(expected, parser.parse_str(input).unwrap());
        }
    }

    #[test]
    fn bychar() {
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
            assert_eq!(expected, parser.parse_str(input).unwrap());
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
            assert_eq!(expected, parser.parse_str(input).unwrap());
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
            assert_eq!(expected, parser.parse_str(input).unwrap());
        }
    }
}
