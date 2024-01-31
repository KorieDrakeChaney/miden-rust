mod errors;
mod field_ops;
mod io_ops;
mod sanitize;
mod stack_ops;
mod sys_ops;
mod token;
mod tokenizer;
mod u32_ops;

pub(crate) use sanitize::sanitize;

use std::collections::VecDeque;

use token::Token;
pub use tokenizer::tokenize;

use crate::{Instruction, Proc};

fn simple_instruction(op: &Token, instruction: Instruction) -> Result<Instruction, String> {
    match op.num_parts() {
        0 => unreachable!(),
        1 => Ok(instruction),
        _ => Err("Instruction takes no arguments".to_string()),
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<(VecDeque<Instruction>, Vec<Proc>), String> {
    use Instruction::*;

    let mut procedures: Vec<Proc> = Vec::new();

    let mut instructions: VecDeque<Instruction> = VecDeque::new();

    let mut scope = 0;
    let mut in_proc = false;

    let mut has_begin = false;

    for token in tokens.iter() {
        let parts = token.parts();
        match parts[0] {
            "assert" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(sys_ops::parse_assert(token)?);
                } else {
                    instructions.push_back(sys_ops::parse_assert(token)?);
                }
            }
            "assertz" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(sys_ops::parse_assertz(token)?);
                } else {
                    instructions.push_back(sys_ops::parse_assertz(token)?);
                }
            }
            "assert_eq" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(sys_ops::parse_assert_eq(token)?);
                } else {
                    instructions.push_back(sys_ops::parse_assert_eq(token)?);
                }
            }
            "assert_eqw" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(sys_ops::parse_assert_eqw(token)?);
                } else {
                    instructions.push_back(sys_ops::parse_assert_eqw(token)?);
                }
            }

            "proc" => match token.num_parts() {
                0 => unreachable!(),
                1 => {
                    return Err(format!("Expected name after proc",));
                }
                2..=3 => {
                    procedures.push(Proc::new(parts[1]));
                    in_proc = true;
                    scope += 1;
                }
                _ => {
                    return Err(format!("Too many arguments after proc"));
                }
            },

            "begin" => {
                if !has_begin {
                    has_begin = true;
                    scope += 1;
                } else {
                    return Err(format!("Unexpected begin"));
                }
            }
            "print" => match token.num_parts() {
                1 => {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(PRINT(parts[1].to_string()));
                    } else {
                        instructions.push_back(PRINT(parts[1].to_string()));
                    }
                }
                _ => {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(PRINT("test".to_string()));
                    } else {
                        instructions.push_back(PRINT("test".to_string()));
                    }
                }
            },
            "if" => match token.num_parts() {
                0 => unreachable!(),
                1 => {
                    return Err(format!("Expected `true` after if"));
                }
                2 => {
                    if parts[1] == "true" {
                        if in_proc {
                            let index = procedures.len() - 1;
                            procedures[index].add_instruction(IF);
                        } else {
                            instructions.push_back(IF);
                        }
                        scope += 1;
                    } else {
                        return Err(format!("Expected `true` after if, found {}", parts[1]));
                    }
                }
                _ => {
                    return Err(format!("Too many arguments after if"));
                }
            },

            "exp" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(field_ops::parse_exp(token)?);
                } else {
                    instructions.push_back(field_ops::parse_exp(token)?);
                }
            }

            "add" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(field_ops::parse_add(token)?);
                } else {
                    instructions.push_back(field_ops::parse_add(token)?);
                }
            }

            "sub" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(field_ops::parse_sub(token)?);
                } else {
                    instructions.push_back(field_ops::parse_sub(token)?);
                }
            }

            "mul" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(field_ops::parse_mul(token)?);
                } else {
                    instructions.push_back(field_ops::parse_mul(token)?);
                }
            }

            "div" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(field_ops::parse_div(token)?);
                } else {
                    instructions.push_back(field_ops::parse_div(token)?);
                }
            }

            "eq" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(field_ops::parse_eq(token)?);
                } else {
                    instructions.push_back(field_ops::parse_eq(token)?);
                }
            }

            "neq" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(field_ops::parse_neq(token)?);
                } else {
                    instructions.push_back(field_ops::parse_neq(token)?);
                }
            }

            "neg" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, Neg)?);
                } else {
                    instructions.push_back(simple_instruction(token, Neg)?);
                }
            }

            "inv" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, Inv)?);
                } else {
                    instructions.push_back(simple_instruction(token, Inv)?);
                }
            }

            "pow2" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, Pow2)?);
                } else {
                    instructions.push_back(simple_instruction(token, Pow2)?);
                }
            }

            "lt" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, Lt)?);
                } else {
                    instructions.push_back(simple_instruction(token, Lt)?);
                }
            }

            "lte" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, Lte)?);
                } else {
                    instructions.push_back(simple_instruction(token, Lte)?);
                }
            }

            "gt" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, Gt)?);
                } else {
                    instructions.push_back(simple_instruction(token, Gt)?);
                }
            }

            "gte" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, Gte)?);
                } else {
                    instructions.push_back(simple_instruction(token, Gte)?);
                }
            }

            "is_odd" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, IsOdd)?);
                } else {
                    instructions.push_back(simple_instruction(token, IsOdd)?);
                }
            }

            "eqw" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, EqW)?);
                } else {
                    instructions.push_back(simple_instruction(token, EqW)?);
                }
            }

            "ext2add" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, Ext2Add)?);
                } else {
                    instructions.push_back(simple_instruction(token, Ext2Add)?);
                }
            }

            "ext2sub" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, Ext2Sub)?);
                } else {
                    instructions.push_back(simple_instruction(token, Ext2Sub)?);
                }
            }

            "ext2mul" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, Ext2Mul)?);
                } else {
                    instructions.push_back(simple_instruction(token, Ext2Mul)?);
                }
            }

            "ext2div" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, Ext2Div)?);
                } else {
                    instructions.push_back(simple_instruction(token, Ext2Div)?);
                }
            }

            "ext2neg" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, Ext2Neg)?);
                } else {
                    instructions.push_back(simple_instruction(token, Ext2Neg)?);
                }
            }

            "ext2inv" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, Ext2Inv)?);
                } else {
                    instructions.push_back(simple_instruction(token, Ext2Inv)?);
                }
            }

            "push" => {
                for instruction in io_ops::parse_push(token)? {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(instruction);
                    } else {
                        instructions.push_back(instruction);
                    }
                }
            }

            "else" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(ELSE);
                } else {
                    instructions.push_back(ELSE);
                }
            }

            "end" => {
                scope -= 1;
                if scope != 0 {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(END);
                    } else {
                        instructions.push_back(END);
                    }
                } else {
                    in_proc = false;
                }
            }

            "while" => match parts.len() {
                0 => unreachable!(),
                1 => {
                    return Err(format!("Expected `true` after while"));
                }
                2 => {
                    if parts[1] == "true" {
                        if in_proc {
                            let index = procedures.len() - 1;
                            procedures[index].add_instruction(WHILE);
                        } else {
                            instructions.push_back(WHILE);
                        }
                        scope += 1;
                    } else {
                        return Err(format!("Expected `true` after while, found {}", parts[1]));
                    }
                }
                _ => {
                    return Err(format!("Too many arguments after while"));
                }
            },

            "repeat" => match parts.len() {
                0 => unreachable!(),
                1 => {
                    return Err(format!("Expected number after repeat"));
                }
                2 => match parts[1].parse::<usize>() {
                    Ok(n) => {
                        if in_proc {
                            let index = procedures.len() - 1;
                            procedures[index].add_instruction(REPEAT(n));
                        } else {
                            instructions.push_back(REPEAT(n));
                        }
                        scope += 1;
                    }
                    Err(_) => {
                        return Err(format!("Expected number after repeat, found {}", parts[1]));
                    }
                },
                _ => {
                    return Err(format!("Too many arguments after while"));
                }
            },

            "drop" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, Drop)?);
                } else {
                    instructions.push_back(simple_instruction(token, Drop)?);
                }
            }

            "dropw" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, DropW)?);
                } else {
                    instructions.push_back(simple_instruction(token, DropW)?);
                }
            }

            "padw" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, PadW)?);
                } else {
                    instructions.push_back(simple_instruction(token, PadW)?);
                }
            }

            "swap" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(stack_ops::parse_swap(token)?);
                } else {
                    instructions.push_back(stack_ops::parse_swap(token)?);
                }
            }

            "swapw" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(stack_ops::parse_swapw(token)?);
                } else {
                    instructions.push_back(stack_ops::parse_swapw(token)?);
                }
            }

            "swapdw" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, SwapDw)?);
                } else {
                    instructions.push_back(simple_instruction(token, SwapDw)?);
                }
            }

            "movdn" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(stack_ops::parse_movdn(token)?);
                } else {
                    instructions.push_back(stack_ops::parse_movdn(token)?);
                }
            }

            "movdnw" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(stack_ops::parse_movdnw(token)?);
                } else {
                    instructions.push_back(stack_ops::parse_movdnw(token)?);
                }
            }

            "movup" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(stack_ops::parse_movup(token)?);
                } else {
                    instructions.push_back(stack_ops::parse_movup(token)?);
                }
            }

            "movupw" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(stack_ops::parse_movupw(token)?);
                } else {
                    instructions.push_back(stack_ops::parse_movupw(token)?);
                }
            }

            "dup" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(stack_ops::parse_dup(token)?);
                } else {
                    instructions.push_back(stack_ops::parse_dup(token)?);
                }
            }

            "dupw" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(stack_ops::parse_dupw(token)?);
                } else {
                    instructions.push_back(stack_ops::parse_dupw(token)?);
                }
            }

            "adv_push" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(io_ops::parse_adv_push(token)?);
                } else {
                    instructions.push_back(io_ops::parse_adv_push(token)?);
                }
            }

            "adv_loadw" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, AdvLoadW)?);
                } else {
                    instructions.push_back(simple_instruction(token, AdvLoadW)?);
                }
            }

            "adv_pipe" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, AdvPipe)?);
                } else {
                    instructions.push_back(simple_instruction(token, AdvPipe)?);
                }
            }

            "mem_load" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(io_ops::parse_mem_load(token)?);
                } else {
                    instructions.push_back(io_ops::parse_mem_load(token)?);
                }
            }

            "mem_store" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(io_ops::parse_mem_store(token)?);
                } else {
                    instructions.push_back(io_ops::parse_mem_store(token)?);
                }
            }

            "mem_loadw" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(io_ops::parse_mem_loadw(token)?);
                } else {
                    instructions.push_back(io_ops::parse_mem_loadw(token)?);
                }
            }

            "mem_storew" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(io_ops::parse_mem_storew(token)?);
                } else {
                    instructions.push_back(io_ops::parse_mem_storew(token)?);
                }
            }

            "loc_load" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(io_ops::parse_loc_load(token)?);
                } else {
                    instructions.push_back(io_ops::parse_loc_load(token)?);
                }
            }

            "loc_loadw" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(io_ops::parse_loc_loadw(token)?);
                } else {
                    instructions.push_back(io_ops::parse_loc_loadw(token)?);
                }
            }

            "loc_store" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(io_ops::parse_loc_store(token)?);
                } else {
                    instructions.push_back(io_ops::parse_loc_store(token)?);
                }
            }

            "loc_storew" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(io_ops::parse_loc_storew(token)?);
                } else {
                    instructions.push_back(io_ops::parse_loc_storew(token)?);
                }
            }

            "exec" => match token.num_parts() {
                0 => unreachable!(),
                1 => {
                    return Err(format!("Expected name after exec"));
                }
                2 => {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::Exec(parts[1].to_string()));
                    } else {
                        instructions.push_back(Instruction::Exec(parts[1].to_string()));
                    }
                }
                _ => {
                    return Err(format!("Too many arguments after exec"));
                }
            },

            "cswap" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, CSwap)?);
                } else {
                    instructions.push_back(simple_instruction(token, CSwap)?);
                }
            }

            "cswapw" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, CSwapW)?);
                } else {
                    instructions.push_back(simple_instruction(token, CSwapW)?);
                }
            }

            "cdrop" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, CDrop)?);
                } else {
                    instructions.push_back(simple_instruction(token, CDrop)?);
                }
            }

            "cdropw" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, CDropW)?);
                } else {
                    instructions.push_back(simple_instruction(token, CDropW)?);
                }
            }

            "and" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, And)?);
                } else {
                    instructions.push_back(simple_instruction(token, And)?);
                }
            }

            "or" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, Or)?);
                } else {
                    instructions.push_back(simple_instruction(token, Or)?);
                }
            }

            "xor" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, Xor)?);
                } else {
                    instructions.push_back(simple_instruction(token, Xor)?);
                }
            }

            "not" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, Not)?);
                } else {
                    instructions.push_back(simple_instruction(token, Not)?);
                }
            }

            "u32checked_add" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(u32_ops::parse_u32checked_add(token)?);
                } else {
                    instructions.push_back(u32_ops::parse_u32checked_add(token)?)
                }
            }

            "u32wrapping_add" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(u32_ops::parse_u32wrapping_add(token)?);
                } else {
                    instructions.push_back(u32_ops::parse_u32wrapping_add(token)?)
                }
            }

            "u32overflowing_add" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(u32_ops::parse_u32overflowing_add(token)?);
                } else {
                    instructions.push_back(u32_ops::parse_u32overflowing_add(token)?)
                }
            }

            "u32checked_sub" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(u32_ops::parse_u32checked_sub(token)?);
                } else {
                    instructions.push_back(u32_ops::parse_u32checked_sub(token)?)
                }
            }

            "u32wrapping_sub" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(u32_ops::parse_u32wrapping_sub(token)?);
                } else {
                    instructions.push_back(u32_ops::parse_u32wrapping_sub(token)?)
                }
            }

            "u32overflowing_sub" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(u32_ops::parse_u32overflowing_sub(token)?);
                } else {
                    instructions.push_back(u32_ops::parse_u32overflowing_sub(token)?)
                }
            }

            "u32checked_mul" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(u32_ops::parse_u32checked_mul(token)?);
                } else {
                    instructions.push_back(u32_ops::parse_u32checked_mul(token)?)
                }
            }

            "u32wrapping_mul" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(u32_ops::parse_u32wrapping_mul(token)?);
                } else {
                    instructions.push_back(u32_ops::parse_u32wrapping_mul(token)?)
                }
            }

            "u32overflowing_mul" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(u32_ops::parse_u32overflowing_mul(token)?);
                } else {
                    instructions.push_back(u32_ops::parse_u32overflowing_mul(token)?)
                }
            }

            "u32checked_div" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(u32_ops::parse_u32checked_div(token)?);
                } else {
                    instructions.push_back(u32_ops::parse_u32checked_div(token)?)
                }
            }

            "u32unchecked_div" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(u32_ops::parse_u32unchecked_div(token)?);
                } else {
                    instructions.push_back(u32_ops::parse_u32unchecked_div(token)?)
                }
            }

            "u32checked_mod" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(u32_ops::parse_u32checked_mod(token)?);
                } else {
                    instructions.push_back(u32_ops::parse_u32checked_mod(token)?)
                }
            }

            "u32unchecked_mod" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(u32_ops::parse_u32unchecked_mod(token)?);
                } else {
                    instructions.push_back(u32_ops::parse_u32unchecked_mod(token)?)
                }
            }

            "u32checked_divmod" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(u32_ops::parse_u32checked_divmod(token)?);
                } else {
                    instructions.push_back(u32_ops::parse_u32checked_divmod(token)?)
                }
            }

            "u32unchecked_divmod" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(u32_ops::parse_u32unchecked_divmod(token)?);
                } else {
                    instructions.push_back(u32_ops::parse_u32unchecked_divmod(token)?)
                }
            }

            "u32overflowing_add3" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index]
                        .add_instruction(simple_instruction(token, U32OverflowingAdd3)?);
                } else {
                    instructions.push_back(simple_instruction(token, U32OverflowingAdd3)?);
                }
            }

            "u32wrapping_add3" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, U32WrappingAdd3)?);
                } else {
                    instructions.push_back(simple_instruction(token, U32WrappingAdd3)?);
                }
            }

            "u32overflowing_madd" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index]
                        .add_instruction(simple_instruction(token, U32OverflowingMadd)?);
                } else {
                    instructions.push_back(simple_instruction(token, U32OverflowingMadd)?);
                }
            }

            "u32wrapping_madd" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, U32WrappingMadd)?);
                } else {
                    instructions.push_back(simple_instruction(token, U32WrappingMadd)?);
                }
            }

            "u32checked_and" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, U32CheckedAnd)?);
                } else {
                    instructions.push_back(simple_instruction(token, U32CheckedAnd)?);
                }
            }

            "u32checked_or" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, U32CheckedOr)?);
                } else {
                    instructions.push_back(simple_instruction(token, U32CheckedOr)?);
                }
            }

            "u32checked_xor" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, U32CheckedXor)?);
                } else {
                    instructions.push_back(simple_instruction(token, U32CheckedXor)?);
                }
            }

            "u32checked_not" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, U32CheckedNot)?);
                } else {
                    instructions.push_back(simple_instruction(token, U32CheckedNot)?);
                }
            }

            "u32checked_shl" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(u32_ops::parse_u32checked_shl(token)?);
                } else {
                    instructions.push_back(u32_ops::parse_u32checked_shl(token)?);
                }
            }

            "u32unchecked_shl" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(u32_ops::parse_u32unchecked_shl(token)?);
                } else {
                    instructions.push_back(u32_ops::parse_u32unchecked_shl(token)?);
                }
            }

            "u32checked_shr" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(u32_ops::parse_u32checked_shr(token)?);
                } else {
                    instructions.push_back(u32_ops::parse_u32checked_shr(token)?);
                }
            }

            "u32unchecked_shr" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(u32_ops::parse_u32unchecked_shr(token)?);
                } else {
                    instructions.push_back(u32_ops::parse_u32unchecked_shr(token)?);
                }
            }

            "u32checked_rotl" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(u32_ops::parse_u32checked_rotl(token)?);
                } else {
                    instructions.push_back(u32_ops::parse_u32checked_rotl(token)?);
                }
            }

            "u32unchecked_rotl" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(u32_ops::parse_u32unchecked_rotl(token)?);
                } else {
                    instructions.push_back(u32_ops::parse_u32unchecked_rotl(token)?);
                }
            }

            "u32checked_rotr" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(u32_ops::parse_u32checked_rotr(token)?);
                } else {
                    instructions.push_back(u32_ops::parse_u32checked_rotr(token)?);
                }
            }

            "u32unchecked_rotr" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(u32_ops::parse_u32unchecked_rotr(token)?);
                } else {
                    instructions.push_back(u32_ops::parse_u32unchecked_rotr(token)?);
                }
            }

            "u32checked_eq" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(u32_ops::parse_u32checked_eq(token)?);
                } else {
                    instructions.push_back(u32_ops::parse_u32checked_eq(token)?);
                }
            }

            "u32checked_neq" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(u32_ops::parse_u32checked_neq(token)?);
                } else {
                    instructions.push_back(u32_ops::parse_u32checked_neq(token)?);
                }
            }

            "u32checked_popcnt" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, U32CheckedPopcnt)?);
                } else {
                    instructions.push_back(simple_instruction(token, U32CheckedPopcnt)?);
                }
            }

            "u32unchecked_popcnt" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index]
                        .add_instruction(simple_instruction(token, U32UncheckedPopcnt)?);
                } else {
                    instructions.push_back(simple_instruction(token, U32UncheckedPopcnt)?);
                }
            }

            "u32checked_lt" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, U32CheckedLt)?);
                } else {
                    instructions.push_back(simple_instruction(token, U32CheckedLt)?);
                }
            }

            "u32unchecked_lte" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, U32UncheckedLte)?);
                } else {
                    instructions.push_back(simple_instruction(token, U32UncheckedLte)?);
                }
            }

            "u32checked_lte" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, U32CheckedLte)?);
                } else {
                    instructions.push_back(simple_instruction(token, U32CheckedLte)?);
                }
            }

            "u32unchecked_lt" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, U32UncheckedLt)?);
                } else {
                    instructions.push_back(simple_instruction(token, U32UncheckedLt)?);
                }
            }

            "u32checked_gte" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, U32CheckedGte)?);
                } else {
                    instructions.push_back(simple_instruction(token, U32CheckedGte)?);
                }
            }

            "u32unchecked_gte" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, U32UncheckedGte)?);
                } else {
                    instructions.push_back(simple_instruction(token, U32UncheckedGte)?);
                }
            }

            "u32checked_gt" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, U32CheckedGt)?);
                } else {
                    instructions.push_back(simple_instruction(token, U32CheckedGt)?);
                }
            }

            "u32unchecked_gt" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, U32UncheckedGt)?);
                } else {
                    instructions.push_back(simple_instruction(token, U32UncheckedGt)?);
                }
            }

            "u32checked_min" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, U32CheckedMin)?);
                } else {
                    instructions.push_back(simple_instruction(token, U32CheckedMin)?);
                }
            }

            "u32unchecked_min" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, U32UncheckedMin)?);
                } else {
                    instructions.push_back(simple_instruction(token, U32UncheckedMin)?);
                }
            }

            "u32checked_max" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, U32CheckedMax)?);
                } else {
                    instructions.push_back(simple_instruction(token, U32CheckedMax)?);
                }
            }

            "u32unchecked_max" => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(simple_instruction(token, U32UncheckedMax)?);
                } else {
                    instructions.push_back(simple_instruction(token, U32UncheckedMax)?);
                }
            }

            _ => {
                return Err(format!("Unknown instruction {}", parts[0]));
            }
        }
    }

    Ok((instructions, procedures))
}
