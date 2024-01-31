use crate::Instruction;

use super::token::Token;

pub fn parse_assert(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::Assert),
        _ => Err("Too many arguments for assert".to_string()),
    }
}

pub fn parse_assertz(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::Assertz),
        _ => Err("Too many arguments for assertz".to_string()),
    }
}

pub fn parse_assert_eq(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::AssertEq),
        _ => Err("Too many arguments for assert_eq".to_string()),
    }
}

pub fn parse_assert_eqw(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::AssertEqW),
        _ => Err("Too many arguments for assert_eqw".to_string()),
    }
}
