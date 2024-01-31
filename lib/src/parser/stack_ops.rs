use crate::Instruction;

use super::token::Token;

pub fn parse_dup(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::Dup(0)),
        2 => {
            let num = op.parts[1].parse::<usize>();
            match num {
                Ok(num) => Ok(Instruction::Dup(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for dup".to_string()),
    }
}

pub fn parse_dupw(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::DupW(0)),
        2 => {
            let num = op.parts[1].parse::<usize>();
            match num {
                Ok(num) => Ok(Instruction::DupW(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for dupw".to_string()),
    }
}

pub fn parse_swap(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::Swap(1)),
        2 => {
            let num = op.parts[1].parse::<usize>();
            match num {
                Ok(num) => Ok(Instruction::Swap(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for swap".to_string()),
    }
}

pub fn parse_swapw(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::SwapW(1)),
        2 => {
            let num = op.parts[1].parse::<usize>();
            match num {
                Ok(num) => Ok(Instruction::SwapW(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for swapw ".to_string()),
    }
}

pub fn parse_movup(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Err("Missing param, movup.<a?>".to_string()),
        2 => {
            let num = op.parts[1].parse::<usize>();
            match num {
                Ok(num) => Ok(Instruction::MovUp(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for movup".to_string()),
    }
}

pub fn parse_movupw(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Err("Missing param, movupw.<a?>".to_string()),
        2 => {
            let num = op.parts[1].parse::<usize>();
            match num {
                Ok(num) => Ok(Instruction::MovUpW(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for movupw".to_string()),
    }
}

pub fn parse_movdn(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Err("Missing param, movdn.<a?>".to_string()),
        2 => {
            let num = op.parts[1].parse::<usize>();
            match num {
                Ok(num) => Ok(Instruction::MovDn(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for movdn".to_string()),
    }
}

pub fn parse_movdnw(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Err("Missing param, movdnw.<a?>".to_string()),
        2 => {
            let num = op.parts[1].parse::<usize>();
            match num {
                Ok(num) => Ok(Instruction::MovDnW(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for movdnw".to_string()),
    }
}
