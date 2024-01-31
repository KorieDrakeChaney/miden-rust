use crate::Instruction;

use super::token::Token;

pub fn parse_u32checked_add(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::U32CheckedAdd),
        2 => {
            let num = op.parts[1].parse::<u32>();
            match num {
                Ok(num) => Ok(Instruction::U32CheckedAddImm(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for u32checked_add".to_string()),
    }
}

pub fn parse_u32wrapping_add(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::U32WrappingAdd),
        2 => {
            let num = op.parts[1].parse::<u32>();
            match num {
                Ok(num) => Ok(Instruction::U32WrappingAddImm(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for u32wrapping_add".to_string()),
    }
}

pub fn parse_u32overflowing_add(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::U32OverflowingAdd),
        2 => {
            let num = op.parts[1].parse::<u32>();
            match num {
                Ok(num) => Ok(Instruction::U32OverflowingAddImm(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for u32overflowing_add".to_string()),
    }
}

pub fn parse_u32checked_sub(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::U32CheckedSub),
        2 => {
            let num = op.parts[1].parse::<u32>();
            match num {
                Ok(num) => Ok(Instruction::U32CheckedSubImm(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for u32checked_sub".to_string()),
    }
}

pub fn parse_u32wrapping_sub(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::U32WrappingSub),
        2 => {
            let num = op.parts[1].parse::<u32>();
            match num {
                Ok(num) => Ok(Instruction::U32WrappingSubImm(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for u32wrapping_sub".to_string()),
    }
}

pub fn parse_u32overflowing_sub(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::U32OverflowingSub),
        2 => {
            let num = op.parts[1].parse::<u32>();
            match num {
                Ok(num) => Ok(Instruction::U32OverflowingSubImm(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for u32overflowing_sub".to_string()),
    }
}

pub fn parse_u32checked_mul(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::U32CheckedMul),
        2 => {
            let num = op.parts[1].parse::<u32>();
            match num {
                Ok(num) => Ok(Instruction::U32CheckedMulImm(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for u32checked_mul".to_string()),
    }
}

pub fn parse_u32wrapping_mul(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::U32WrappingMul),
        2 => {
            let num = op.parts[1].parse::<u32>();
            match num {
                Ok(num) => Ok(Instruction::U32WrappingMulImm(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for u32wrapping_mul".to_string()),
    }
}

pub fn parse_u32overflowing_mul(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::U32OverflowingMul),
        2 => {
            let num = op.parts[1].parse::<u32>();
            match num {
                Ok(num) => Ok(Instruction::U32OverflowingMulImm(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for u32overflowing_mul".to_string()),
    }
}

pub fn parse_u32checked_div(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::U32CheckedDiv),
        2 => {
            let num = op.parts[1].parse::<u32>();
            match num {
                Ok(num) => Ok(Instruction::U32CheckedDivImm(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for u32checked_div".to_string()),
    }
}

pub fn parse_u32unchecked_div(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::U32UncheckedDiv),
        2 => {
            let num = op.parts[1].parse::<u32>();
            match num {
                Ok(num) => Ok(Instruction::U32UncheckedDivImm(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for u32unchecked_div".to_string()),
    }
}

pub fn parse_u32checked_mod(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::U32CheckedMod),
        2 => {
            let num = op.parts[1].parse::<u32>();
            match num {
                Ok(num) => Ok(Instruction::U32CheckedModImm(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for u32checked_mod".to_string()),
    }
}

pub fn parse_u32unchecked_mod(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::U32UncheckedMod),
        2 => {
            let num = op.parts[1].parse::<u32>();
            match num {
                Ok(num) => Ok(Instruction::U32UncheckedModImm(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for u32unchecked_mod".to_string()),
    }
}

pub fn parse_u32checked_divmod(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::U32CheckedDivMod),
        2 => {
            let num = op.parts[1].parse::<u32>();
            match num {
                Ok(num) => Ok(Instruction::U32CheckedDivModImm(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for u32checked_divmod".to_string()),
    }
}

pub fn parse_u32unchecked_divmod(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::U32UncheckedDivMod),
        2 => {
            let num = op.parts[1].parse::<u32>();
            match num {
                Ok(num) => Ok(Instruction::U32UncheckedDivModImm(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for u32unchecked_divmod".to_string()),
    }
}

pub fn parse_u32checked_shr(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::U32CheckedShr),
        2 => {
            let num = op.parts[1].parse::<u32>();
            match num {
                Ok(num) => Ok(Instruction::U32CheckedShrImm(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for u32checked_shr".to_string()),
    }
}

pub fn parse_u32unchecked_shr(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::U32UncheckedShr),
        2 => {
            let num = op.parts[1].parse::<u32>();
            match num {
                Ok(num) => Ok(Instruction::U32UncheckedShrImm(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for u32unchecked_shr".to_string()),
    }
}

pub fn parse_u32checked_shl(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::U32CheckedShl),
        2 => {
            let num = op.parts[1].parse::<u32>();
            match num {
                Ok(num) => Ok(Instruction::U32CheckedShlImm(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for u32checked_shl".to_string()),
    }
}

pub fn parse_u32unchecked_shl(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::U32UncheckedShl),
        2 => {
            let num = op.parts[1].parse::<u32>();
            match num {
                Ok(num) => Ok(Instruction::U32UncheckedShlImm(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for u32unchecked_shl".to_string()),
    }
}

pub fn parse_u32checked_rotr(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::U32CheckedRotr),
        2 => {
            let num = op.parts[1].parse::<u32>();
            match num {
                Ok(num) => Ok(Instruction::U32CheckedRotrImm(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for u32checked_rotr".to_string()),
    }
}

pub fn parse_u32unchecked_rotr(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::U32UncheckedRotr),
        2 => {
            let num = op.parts[1].parse::<u32>();
            match num {
                Ok(num) => Ok(Instruction::U32UncheckedRotrImm(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for u32unchecked_rotr".to_string()),
    }
}

pub fn parse_u32checked_rotl(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::U32CheckedRotl),
        2 => {
            let num = op.parts[1].parse::<u32>();
            match num {
                Ok(num) => Ok(Instruction::U32CheckedRotlImm(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for u32checked_rotl".to_string()),
    }
}

pub fn parse_u32unchecked_rotl(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::U32UncheckedRotl),
        2 => {
            let num = op.parts[1].parse::<u32>();
            match num {
                Ok(num) => Ok(Instruction::U32UncheckedRotlImm(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for u32unchecked_rotl".to_string()),
    }
}

pub fn parse_u32checked_eq(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::U32CheckedEq),
        2 => {
            let num = op.parts[1].parse::<u32>();
            match num {
                Ok(num) => Ok(Instruction::U32CheckedEqImm(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for u32checked_eq".to_string()),
    }
}

pub fn parse_u32checked_neq(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::U32CheckedNeq),
        2 => {
            let num = op.parts[1].parse::<u32>();
            match num {
                Ok(num) => Ok(Instruction::U32CheckedNeqImm(num)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for u32checked_neq".to_string()),
    }
}

