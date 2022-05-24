use crate::{Instruction, Parser};

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

// Using `lrlex_mod!` brings the lexer for `calc.l` into scope. By default the
// module name will be `calc_l` (i.e. the file name, minus any extensions,
// with a suffix of `_l`).
lrlex_mod!("calc.l");
// Using `lrpar_mod!` brings the parser for `calc.y` into scope. By default the
// module name will be `calc_y` (i.e. the file name, minus any extensions,
// with a suffix of `_y`).
lrpar_mod!("calc.y");

pub struct YaccParser {
    lexerdef: lrlex::LRNonStreamingLexerDef<lrlex::DefaultLexeme, u32>,
}

impl YaccParser {
    pub fn new() -> Self {
        Self {
            lexerdef: calc_l::lexerdef(),
        }
    }
}
impl Default for YaccParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser for YaccParser {
    fn parse_str(&self, s: &str) -> Instruction {
        let lexer = self.lexerdef.lexer(s);
        // Pass the lexer to the parser and lex and parse the input.
        let (res, errs) = calc_y::parse(&lexer);
        for e in errs {
            println!("{}", e.pp(&lexer, &calc_y::token_epp));
        }
        match res {
            Some(r) => r.unwrap(),
            _ => {
                eprintln!("Unable to evaluate expression: {s}");
                Instruction::Err
            }
        }
    }
}
