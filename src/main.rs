use parsing::*;

fn run(i: i32) -> std::io::Result<Vec<Instruction>> {
    let filename = "input.txt";
    match i {
        0 => load(filename, NomParser::new()),
        1 => load(filename, PestParser::new()),
        2 => load(filename, RegexParser::new()),
        3 => load(filename, SplitParser::new()),
        _ => unreachable!(),
    }
}

fn main() {
    env_logger::init();

    for i in 0..=3 {
        let input = run(i);
        if let Ok(input) = input {
            log::info!("{input:?}");
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
            assert_eq!(expected, parser.parse_str(input));
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
}
