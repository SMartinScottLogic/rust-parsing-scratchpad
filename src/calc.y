%start Instruction
%%
Instruction -> Result<Instruction, ()>:
      Source Op Source 'INTO' Wire { Ok(make_instruction_sos($1?, $2?, $3?, $5?)) }
    | Op Source 'INTO' Wire { Ok(make_instruction_os($1?, $2?, $4?)) }
    | Source 'INTO' Wire { Ok(make_instruction_s($1?, $3?)) }
    ;

Source -> Result<&'input str, ()>:
      Wire { $1 }
    | Int  { $1 }
    ;

Op -> Result<Operation, ()>:
      'OP' {
          let v = $1.map_err(|_| ())?;
          parse_op($lexer.span_str(v.span()))
      }
    ;
Wire -> Result<&'input str, ()>:
      'WIRE' {
          let v = $1.map_err(|_| ())?;
          Ok($lexer.span_str(v.span()))
      }
    ;
Int -> Result<&'input str, ()>:
      'INT' {
          let v = $1.map_err(|_| ())?;
          Ok($lexer.span_str(v.span()))
      }
    ;
%%
use std::str::FromStr;
use crate::{Instruction, Operation};

// Any functions here are in scope for all the grammar actions above.
fn make_instruction_sos(source1: &str, op: Operation, source2: &str, target: &str) -> Instruction {
    log::debug!("{source1} {op:?} {source2} -> {target}");
    Instruction::from((&vec![source1, source2], op, target))
}

fn make_instruction_os(op: Operation, source: &str, target: &str) -> Instruction {
    log::debug!("{op:?} {source} -> {target}");
    Instruction::from((&vec![source], op, target))
}

fn make_instruction_s(source: &str, target: &str) -> Instruction {
    log::debug!("SET {source} -> {target}");
    Instruction::from((&vec![source], Operation::SET, target))
}

fn parse_op(s: &str) -> Result<Operation, ()> {
    Operation::from_str(s).map_err(|_| ())
}