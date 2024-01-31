use miden::math::Felt;

use crate::Instruction;

use super::token::Token;

pub fn parse_add(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::Add),
        2 => {
            let num = op.parts[1].parse::<u64>();
            match num {
                Ok(num) => Ok(Instruction::AddImm(Felt::from(num))),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for add".to_string()),
    }
}

pub fn parse_sub(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::Sub),
        2 => {
            let num = op.parts[1].parse::<u64>();
            match num {
                Ok(num) => Ok(Instruction::SubImm(Felt::from(num))),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for sub".to_string()),
    }
}

pub fn parse_mul(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::Mul),
        2 => {
            let num = op.parts[1].parse::<u64>();
            match num {
                Ok(num) => Ok(Instruction::MulImm(Felt::from(num))),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for mul".to_string()),
    }
}

pub fn parse_div(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::Div),
        2 => {
            let num = op.parts[1].parse::<u64>();
            match num {
                Ok(num) => Ok(Instruction::DivImm(Felt::from(num))),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for div".to_string()),
    }
}

pub fn parse_eq(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::Eq),
        2 => {
            let num = op.parts[1].parse::<u64>();
            match num {
                Ok(num) => Ok(Instruction::EqImm(Felt::from(num))),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for eq".to_string()),
    }
}

pub fn parse_neq(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::Neq),
        2 => {
            let num = op.parts[1].parse::<u64>();
            match num {
                Ok(num) => Ok(Instruction::NeqImm(Felt::from(num))),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for neq".to_string()),
    }
}

pub fn parse_exp(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::Exp),
        2 => {
            let num = op.parts[1].parse::<u64>();
            match num {
                Ok(num) => Ok(Instruction::ExpImm(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for exp".to_string()),
    }
}
