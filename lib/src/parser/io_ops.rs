use miden::math::Felt;

use crate::Instruction;

use super::token::Token;

fn parse_hex(hex_str: &str) -> Result<u64, String> {
    match u64::from_str_radix(hex_str, 16) {
        Ok(num) => Ok(num),
        Err(_) => Err(format!("Invalid hex value: {}", hex_str)),
    }
}

fn parse_long_hex(hex_str: &str) -> Result<Vec<u64>, String> {
    if hex_str.len() != 64 {
        return Err(format!("Invalid hex value: {}", hex_str));
    }

    let values = (0..hex_str.len()).step_by(16).map(|i| {
        u64::from_str_radix(&hex_str[i..i + 16], 16)
            .map(|v| v.swap_bytes())
            .map_err(|_| "Invalid hex value".to_string())
    });

    values.collect()
}

pub fn parse_push(op: &Token) -> Result<Vec<Instruction>, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Err("Missing param, push.<a?>".to_string()),
        2 => match op.parts[1].parse::<u64>() {
            Ok(num) => Ok(vec![Instruction::Push(Felt::from(num))]),
            Err(_) => {
                if op.parts[1].starts_with("0x") {
                    let hex_str = &op.parts[1][2..];
                    if hex_str.len() < 16 && hex_str.len() % 2 == 0 && hex_str.len() != 0 {
                        return Ok(vec![Instruction::Push(Felt::from(parse_hex(hex_str)?))]);
                    } else if hex_str.len() == 64 {
                        return Ok(parse_long_hex(hex_str)?
                            .into_iter()
                            .map(|num| Instruction::Push(Felt::from(num)))
                            .collect());
                    } else {
                        Err(format!("Invalid hex value: {}", hex_str))
                    }
                } else {
                    Err(format!("parameter '{}' is invalid", op.parts[1]))
                }
            }
        },
        3..=17 => {
            let mut instructions = Vec::new();
            for i in 1..op.num_parts() {
                match op.parts[i].parse::<u64>() {
                    Ok(num) => instructions.push(Instruction::Push(Felt::from(num))),
                    Err(_) => {
                        if op.parts[i].starts_with("0x") {
                            let hex_str = &op.parts[i][2..];
                            if hex_str.len() < 16 && hex_str.len() % 2 == 0 && hex_str.len() != 0 {
                                instructions
                                    .push(Instruction::Push(Felt::from(parse_hex(hex_str)?)));
                            } else if hex_str.len() == 64 {
                                instructions.append(
                                    &mut parse_long_hex(hex_str)?
                                        .into_iter()
                                        .map(|num| Instruction::Push(Felt::from(num)))
                                        .collect(),
                                );
                            } else {
                                return Err(format!("Invalid hex value: {}", hex_str));
                            }
                        } else {
                            return Err(format!("parameter '{}' is invalid", op.parts[1]));
                        }
                    }
                }
            }
            Ok(instructions)
        }

        _ => Err("Too many parameters for push".to_string()),
    }
}

pub fn parse_mem_load(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::MemLoad),
        2 => {
            let index = op.parts[1].parse::<u32>();
            match index {
                Ok(index) => Ok(Instruction::MemLoadImm(index)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for mem_load".to_string()),
    }
}

pub fn parse_mem_store(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::MemStore),
        2 => {
            let index = op.parts[1].parse::<u32>();
            match index {
                Ok(index) => Ok(Instruction::MemStoreImm(index)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for mem_store".to_string()),
    }
}

pub fn parse_mem_loadw(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::MemLoadW),
        2 => {
            let index = op.parts[1].parse::<u32>();
            match index {
                Ok(index) => Ok(Instruction::MemLoadWImm(index)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for mem_loadw".to_string()),
    }
}

pub fn parse_mem_storew(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(Instruction::MemStoreW),
        2 => {
            let index = op.parts[1].parse::<u32>();
            match index {
                Ok(index) => Ok(Instruction::MemStoreWImm(index)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for mem_storew".to_string()),
    }
}

pub fn parse_loc_load(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Err("Missing param, loc_load.<a?>".to_string()),
        2 => {
            let index = op.parts[1].parse::<u16>();
            match index {
                Ok(index) => Ok(Instruction::LocLoad(index)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for loc_load".to_string()),
    }
}

pub fn parse_loc_store(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Err("Missing param, loc_store.<a?>".to_string()),
        2 => {
            let index = op.parts[1].parse::<u16>();
            match index {
                Ok(index) => Ok(Instruction::LocStore(index)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for loc_store".to_string()),
    }
}

pub fn parse_loc_loadw(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Err("Missing param, loc_loadw.<a?>".to_string()),
        2 => {
            let index = op.parts[1].parse::<u16>();
            match index {
                Ok(index) => Ok(Instruction::LocLoadW(index)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for loc_loadw".to_string()),
    }
}

pub fn parse_loc_storew(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Err("Missing param, loc_storew.<a?>".to_string()),
        2 => {
            let index = op.parts[1].parse::<u16>();
            match index {
                Ok(index) => Ok(Instruction::LocStoreW(index)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for loc_storew".to_string()),
    }
}

pub fn parse_adv_push(op: &Token) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Err("Missing param, adv_push.<a?>".to_string()),
        2 => {
            let index = op.parts[1].parse::<usize>();
            match index {
                Ok(index) => Ok(Instruction::AdvPush(index)),
                Err(_) => Err(format!("parameter '{}' is invalid", op.parts[1])),
            }
        }
        _ => Err("Too many arguments for adv_push".to_string()),
    }
}
