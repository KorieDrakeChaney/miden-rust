mod sanitize;
mod token;
mod tokenizer;

use sanitize::sanitize;

use std::collections::VecDeque;

use math::fields::f64::BaseElement;
use token::Token;
pub use tokenizer::tokenize;

use crate::{Instruction, Proc};

pub fn parse(tokens: Vec<Token>) -> Result<(VecDeque<Instruction>, Vec<Proc>), String> {
    let mut procedures: Vec<Proc> = Vec::new();

    let mut instructions: VecDeque<Instruction> = VecDeque::new();

    let mut i = 0;

    let mut scope = 0;
    let mut in_proc = false;

    let mut has_begin = false;

    while i < tokens.len() {
        let token = &tokens[i];
        match token {
            Token::Assert => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::Assert);
                } else {
                    instructions.push_back(Instruction::Assert);
                }
            }
            Token::AssertZ => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::AssertZ);
                } else {
                    instructions.push_back(Instruction::AssertZ);
                }
            }
            Token::AssertEq => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::AssertEq);
                } else {
                    instructions.push_back(Instruction::AssertEq);
                }
            }
            Token::AssertEqW => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::AssertEqW);
                } else {
                    instructions.push_back(Instruction::AssertEqW);
                }
            }
            Token::Proc => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::String(name) => {
                            procedures.push(Proc::new(name));
                            in_proc = true;
                            scope += 1;
                            i += 1;
                        }
                        _ => {
                            return Err(format!(
                                "Expected name after proc, found {:?}",
                                tokens[i + 1]
                            ));
                        }
                    }
                    if i + 1 < tokens.len() {
                        match &tokens[i + 1] {
                            Token::Number(_) => {
                                i += 1;
                            }
                            _ => {}
                        }
                    }
                } else {
                    return Err(format!("Expected name after proc, found EOF"));
                }
            }

            Token::Begin => {
                if !has_begin {
                    has_begin = true;
                    scope += 1;
                } else {
                    return Err(format!("Unexpected begin"));
                }
            }
            Token::Print => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::String(name) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::PRINT(name.to_string()));
                            } else {
                                instructions.push_back(Instruction::PRINT(name.to_string()));
                            }
                            i += 1;
                        }
                        _ => {
                            instructions.push_back(Instruction::PRINT("test".to_string()));
                        }
                    }
                } else {
                    instructions.push_back(Instruction::PRINT("test".to_string()));
                }
            }
            Token::If => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::String(statement) => {
                            if statement == "true" {
                                if in_proc {
                                    let index = procedures.len() - 1;
                                    procedures[index].add_instruction(Instruction::IF);
                                } else {
                                    instructions.push_back(Instruction::IF);
                                }
                                scope += 1;
                                i += 1;
                            } else {
                                return Err(format!(
                                    "Expected `true` after if, found {:?}",
                                    statement
                                ));
                            }
                        }
                        _ => {
                            return Err(format!(
                                "Expected `true` after if, found {:?}",
                                tokens[i + 1]
                            ));
                        }
                    }
                } else {
                    return Err(format!("Expected `true` after if, found EOF"));
                }
            }

            Token::Pow2 => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::Pow2);
                } else {
                    instructions.push_back(Instruction::Pow2);
                }
            }

            Token::Neg => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::Neg);
                } else {
                    instructions.push_back(Instruction::Neg);
                }
            }

            Token::Inv => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::Inv);
                } else {
                    instructions.push_back(Instruction::Inv);
                }
            }

            Token::Exp => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::ExpImm(*n));
                            } else {
                                instructions.push_back(Instruction::ExpImm(*n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::Exp);
                            } else {
                                instructions.push_back(Instruction::Exp);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::Exp);
                    } else {
                        instructions.push_back(Instruction::Exp);
                    }
                }
            }

            Token::Push => {
                if i + 1 < tokens.len() {
                    let mut nums = Vec::new();

                    while i + 1 < tokens.len() {
                        match &tokens[i + 1] {
                            Token::Number(n) => {
                                nums.push(*n);
                                i += 1;
                            }
                            _ => {
                                break;
                            }
                        }
                    }

                    if nums.len() == 0 {
                        return Err(format!(
                            "Expected number after push_back, found {:?}",
                            tokens[i + 1]
                        ));
                    }

                    for n in nums {
                        if in_proc {
                            let index = procedures.len() - 1;
                            procedures[index]
                                .add_instruction(Instruction::Push(BaseElement::from(n)));
                        } else {
                            instructions.push_back(Instruction::Push(BaseElement::from(n)));
                        }
                    }
                } else {
                    return Err(format!("Expected number after push_back, found EOF"));
                }
            }

            Token::Else => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::ELSE);
                } else {
                    instructions.push_back(Instruction::ELSE);
                }
            }
            Token::End => {
                scope -= 1;

                if scope != 0 {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::END);
                    } else {
                        instructions.push_back(Instruction::END);
                    }
                } else {
                    in_proc = false;
                }
            }
            Token::While => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::String(statement) => {
                            if statement == "true" {
                                if in_proc {
                                    let index = procedures.len() - 1;
                                    procedures[index].add_instruction(Instruction::WHILE);
                                } else {
                                    instructions.push_back(Instruction::WHILE);
                                }
                                scope += 1;
                                i += 1;
                            } else {
                                return Err(format!(
                                    "Expected `true` after while, found {:?}",
                                    statement
                                ));
                            }
                        }
                        _ => {
                            return Err(format!(
                                "Expected `true` after while, found {:?}",
                                tokens[i + 1]
                            ));
                        }
                    }
                } else {
                    return Err(format!("Expected `true` after while, found EOF"));
                }
            }
            Token::Repeat => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::REPEAT(*n as usize));
                            } else {
                                instructions.push_back(Instruction::REPEAT(*n as usize));
                            }
                            i += 1;
                        }
                        _ => {
                            return Err(format!(
                                "Expected number after repeat, found {:?}",
                                tokens[i + 1]
                            ));
                        }
                    }
                } else {
                    return Err(format!("Expected number after repeat, found EOF"));
                }
                scope += 1;
            }
            Token::Add => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::AddImm(BaseElement::from(*n)));
                            } else {
                                instructions.push_back(Instruction::AddImm(BaseElement::from(*n)));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::Add);
                            } else {
                                instructions.push_back(Instruction::Add);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::Add);
                    } else {
                        instructions.push_back(Instruction::Add);
                    }
                }
            }
            Token::Sub => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::SubImm(BaseElement::from(*n)));
                            } else {
                                instructions.push_back(Instruction::SubImm(BaseElement::from(*n)));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::Sub);
                            } else {
                                instructions.push_back(Instruction::Sub);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::Sub);
                    } else {
                        instructions.push_back(Instruction::Sub);
                    }
                }
            }
            Token::Mul => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::MulImm(BaseElement::from(*n)));
                            } else {
                                instructions.push_back(Instruction::MulImm(BaseElement::from(*n)));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::Mul);
                            } else {
                                instructions.push_back(Instruction::Mul);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::Mul);
                    } else {
                        instructions.push_back(Instruction::Mul);
                    }
                }
            }
            Token::Div => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::DivImm(BaseElement::from(*n)));
                            } else {
                                instructions.push_back(Instruction::DivImm(BaseElement::from(*n)));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::Div);
                            } else {
                                instructions.push_back(Instruction::Div);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::Div);
                    } else {
                        instructions.push_back(Instruction::Div);
                    }
                }
            }
            Token::AdvPush => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::AdvPush(*n as usize));
                            } else {
                                instructions.push_back(Instruction::AdvPush(*n as usize));
                            }
                            i += 1;
                        }
                        _ => {
                            return Err(format!(
                                "Expected number after advpush_back, found {:?}",
                                tokens[i + 1]
                            ));
                        }
                    }
                } else {
                    return Err(format!("Expected number after advpush_back, found EOF"));
                }
            }

            Token::AdvLoadW => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::AdvLoadW);
                } else {
                    instructions.push_back(Instruction::AdvLoadW);
                }
            }

            Token::AdvPipe => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::AdvPipe);
                } else {
                    instructions.push_back(Instruction::AdvPipe);
                }
            }

            Token::MemLoad => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::MemLoadImm(*n as u32));
                            } else {
                                instructions.push_back(Instruction::MemLoadImm(*n as u32));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::MemLoad);
                            } else {
                                instructions.push_back(Instruction::MemLoad);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::MemLoad);
                    } else {
                        instructions.push_back(Instruction::MemLoad);
                    }
                }
            }

            Token::MemStore => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::MemStoreImm(*n as u32));
                            } else {
                                instructions.push_back(Instruction::MemStoreImm(*n as u32));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::MemStore);
                            } else {
                                instructions.push_back(Instruction::MemStore);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::MemStore);
                    } else {
                        instructions.push_back(Instruction::MemStore);
                    }
                }
            }

            Token::MemLoadW => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::MemLoadWImm(*n as u32));
                            } else {
                                instructions.push_back(Instruction::MemLoadWImm(*n as u32));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::MemLoadW);
                            } else {
                                instructions.push_back(Instruction::MemLoadW);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::MemLoadW);
                    } else {
                        instructions.push_back(Instruction::MemLoadW);
                    }
                }
            }

            Token::MemStoreW => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::MemStoreWImm(*n as u32));
                            } else {
                                instructions.push_back(Instruction::MemStoreWImm(*n as u32));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::MemStoreW);
                            } else {
                                instructions.push_back(Instruction::MemStoreW);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::MemStoreW);
                    } else {
                        instructions.push_back(Instruction::MemStoreW);
                    }
                }
            }

            Token::LocLoad => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::LocLoad(*n as u16));
                            } else {
                                return Err(format!(
                                    "Unexpected loc_load with value: {:?}",
                                    tokens[i + 1]
                                ));
                            }
                            i += 1;
                        }
                        _ => {
                            return Err(format!(
                                "Expected number after loc_load, found {:?}",
                                tokens[i + 1]
                            ));
                        }
                    }
                } else {
                    return Err(format!("Expected number after loc_load, found EOF"));
                }
            }

            Token::LocLoadW => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::LocLoadW(*n as u16));
                            } else {
                                return Err(format!(
                                    "Unexpected loc_loadw with value: {:?}",
                                    tokens[i + 1]
                                ));
                            }
                            i += 1;
                        }
                        _ => {
                            return Err(format!(
                                "Expected number after loc_loadw, found {:?}",
                                tokens[i + 1]
                            ));
                        }
                    }
                } else {
                    return Err(format!("Expected number after loc_loadw, found EOF"));
                }
            }

            Token::LocStore => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::LocStore(*n as u16));
                            } else {
                                return Err(format!(
                                    "Unexpected loc_store with value: {:?}",
                                    tokens[i + 1]
                                ));
                            }
                            i += 1;
                        }
                        _ => {
                            return Err(format!(
                                "Expected number after loc_store, found {:?}",
                                tokens[i + 1]
                            ));
                        }
                    }
                } else {
                    return Err(format!("Expected number after loc_store, found EOF"));
                }
            }

            Token::LocStoreW => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::LocStoreW(*n as u16));
                            } else {
                                return Err(format!(
                                    "Unexpected loc_storew with value: {:?}",
                                    tokens[i + 1]
                                ));
                            }
                            i += 1;
                        }
                        _ => {
                            return Err(format!(
                                "Expected number after loc_storew, found {:?}",
                                tokens[i + 1]
                            ));
                        }
                    }
                } else {
                    return Err(format!("Expected number after loc_storew, found EOF"));
                }
            }

            Token::Exec => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::String(name) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::Exec(name.to_string()));
                            } else {
                                instructions.push_back(Instruction::Exec(name.to_string()));
                            }
                            i += 1;
                        }
                        _ => {
                            return Err(format!(
                                "Expected name after exec, found {:?}",
                                tokens[i + 1]
                            ));
                        }
                    }
                } else {
                    return Err(format!("Expected name after exec, found EOF"));
                }
            }

            Token::Dup => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::Dup(*n as usize));
                            } else {
                                instructions.push_back(Instruction::Dup(*n as usize));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::Dup(0));
                            } else {
                                instructions.push_back(Instruction::Dup(0));
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::Dup(0));
                    } else {
                        instructions.push_back(Instruction::Dup(0));
                    }
                    instructions.push_back(Instruction::Dup(0));
                }
            }

            Token::Swap => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::Swap(*n as usize));
                            } else {
                                instructions.push_back(Instruction::Swap(*n as usize));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::Swap(1));
                            } else {
                                instructions.push_back(Instruction::Swap(1));
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::Swap(1));
                    } else {
                        instructions.push_back(Instruction::Swap(1));
                    }
                }
            }

            Token::SwapW => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::SwapW(*n as usize));
                            } else {
                                instructions.push_back(Instruction::SwapW(*n as usize));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::SwapW(1));
                            } else {
                                instructions.push_back(Instruction::SwapW(1));
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::SwapW(1));
                    } else {
                        instructions.push_back(Instruction::SwapW(1));
                    }
                }
            }

            Token::SwapDw => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::SwapDw(*n as usize));
                            } else {
                                instructions.push_back(Instruction::SwapDw(*n as usize));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::SwapDw(1));
                            } else {
                                instructions.push_back(Instruction::SwapDw(1));
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::SwapDw(1));
                    } else {
                        instructions.push_back(Instruction::SwapDw(1));
                    }
                }
            }

            Token::PadW => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::PadW);
                } else {
                    instructions.push_back(Instruction::PadW);
                }
            }

            Token::MovUp => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::MovUp(*n as usize));
                            } else {
                                instructions.push_back(Instruction::MovUp(*n as usize));
                            }
                            i += 1;
                        }
                        _ => {
                            return Err(format!(
                                "Expected number after movup, found {:?}",
                                tokens[i + 1]
                            ));
                        }
                    }
                } else {
                    return Err(format!("Expected number after movup, found EOF"));
                }
            }

            Token::MovUpW => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::MovUpW(*n as usize));
                            } else {
                                instructions.push_back(Instruction::MovUpW(*n as usize));
                            }
                            i += 1;
                        }
                        _ => {
                            return Err(format!(
                                "Expected number after movupw, found {:?}",
                                tokens[i + 1]
                            ));
                        }
                    }
                } else {
                    return Err(format!("Expected number after movupw, found EOF"));
                }
            }

            Token::MovDn => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::MovDn(*n as usize));
                            } else {
                                instructions.push_back(Instruction::MovDn(*n as usize));
                            }
                            i += 1;
                        }
                        _ => {
                            return Err(format!(
                                "Expected number after movdn, found {:?}",
                                tokens[i + 1]
                            ));
                        }
                    }
                } else {
                    return Err(format!("Expected number after movdn, found EOF"));
                }
            }

            Token::MovDnW => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::MovDnW(*n as usize));
                            } else {
                                instructions.push_back(Instruction::MovDnW(*n as usize));
                            }
                            i += 1;
                        }
                        _ => {
                            return Err(format!(
                                "Expected number after movdnw, found {:?}",
                                tokens[i + 1]
                            ));
                        }
                    }
                } else {
                    return Err(format!("Expected number after movdnw, found EOF"));
                }
            }

            Token::Drop => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::Drop);
                } else {
                    instructions.push_back(Instruction::Drop);
                }
            }

            Token::DropW => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::DropW);
                } else {
                    instructions.push_back(Instruction::DropW);
                }
            }

            Token::CSwap => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::CSwap);
                } else {
                    instructions.push_back(Instruction::CSwap);
                }
            }

            Token::CSwapW => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::CSwapW);
                } else {
                    instructions.push_back(Instruction::CSwapW);
                }
            }

            Token::CDrop => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::CDrop);
                } else {
                    instructions.push_back(Instruction::CDrop);
                }
            }

            Token::CDropW => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::CDropW);
                } else {
                    instructions.push_back(Instruction::CDropW);
                }
            }

            Token::Ext2Add => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::Ext2Add);
                } else {
                    instructions.push_back(Instruction::Ext2Add);
                }
            }

            Token::Ext2Sub => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::Ext2Sub);
                } else {
                    instructions.push_back(Instruction::Ext2Sub);
                }
            }

            Token::Ext2Mul => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::Ext2Mul);
                } else {
                    instructions.push_back(Instruction::Ext2Mul);
                }
            }

            Token::Ext2Div => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::Ext2Div);
                } else {
                    instructions.push_back(Instruction::Ext2Div);
                }
            }

            Token::Ext2Neg => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::Ext2Neg);
                } else {
                    instructions.push_back(Instruction::Ext2Neg);
                }
            }

            Token::Ext2Inv => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::Ext2Inv);
                } else {
                    instructions.push_back(Instruction::Ext2Inv);
                }
            }

            Token::And => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::And);
                } else {
                    instructions.push_back(Instruction::And);
                }
            }

            Token::Or => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::Or);
                } else {
                    instructions.push_back(Instruction::Or);
                }
            }

            Token::Xor => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::Xor);
                } else {
                    instructions.push_back(Instruction::Xor);
                }
            }

            Token::Not => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::Not);
                } else {
                    instructions.push_back(Instruction::Not);
                }
            }

            Token::IsOdd => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::IsOdd);
                } else {
                    instructions.push_back(Instruction::IsOdd);
                }
            }

            Token::Eq => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::EqImm(BaseElement::from(*n)));
                            } else {
                                instructions.push_back(Instruction::EqImm(BaseElement::from(*n)));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::Eq);
                            } else {
                                instructions.push_back(Instruction::Eq);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::Eq);
                    } else {
                        instructions.push_back(Instruction::Eq);
                    }
                }
            }

            Token::Neq => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::NeqImm(BaseElement::from(*n)));
                            } else {
                                instructions.push_back(Instruction::NeqImm(BaseElement::from(*n)));
                            }
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::Neq);
                            } else {
                                instructions.push_back(Instruction::Neq);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::Neq);
                    } else {
                        instructions.push_back(Instruction::Neq);
                    }
                }
            }

            Token::Lt => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::Lt);
                } else {
                    instructions.push_back(Instruction::Lt);
                }
            }

            Token::Lte => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::Lte);
                } else {
                    instructions.push_back(Instruction::Lte);
                }
            }

            Token::Gt => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::Gt);
                } else {
                    instructions.push_back(Instruction::Gt);
                }
            }

            Token::Gte => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::Gte);
                } else {
                    instructions.push_back(Instruction::Gte);
                }
            }

            Token::EqW => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::EqW);
                } else {
                    instructions.push_back(Instruction::EqW);
                }
            }

            Token::U32CheckedAdd => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                let n = *n as u32;
                                procedures[index].add_instruction(Instruction::U32CheckedAddImm(n));
                            } else {
                                let n = *n as u32;
                                instructions.push_back(Instruction::U32CheckedAddImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32CheckedAdd);
                            } else {
                                instructions.push_back(Instruction::U32CheckedAdd);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::U32CheckedAdd);
                    } else {
                        instructions.push_back(Instruction::U32CheckedAdd);
                    }
                }
            }

            Token::U32OverflowingAdd => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                let n = *n as u32;
                                procedures[index]
                                    .add_instruction(Instruction::U32OverflowingAddImm(n));
                            } else {
                                let n = *n as u32;
                                instructions.push_back(Instruction::U32OverflowingAddImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32OverflowingAdd);
                            } else {
                                instructions.push_back(Instruction::U32OverflowingAdd);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::U32OverflowingAdd);
                    } else {
                        instructions.push_back(Instruction::U32OverflowingAdd);
                    }
                }
            }

            Token::U32WrappingAdd => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                let n = *n as u32;
                                procedures[index]
                                    .add_instruction(Instruction::U32WrappingAddImm(n));
                            } else {
                                let n = *n as u32;
                                instructions.push_back(Instruction::U32WrappingAddImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32WrappingAdd);
                            } else {
                                instructions.push_back(Instruction::U32WrappingAdd);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::U32WrappingAdd);
                    } else {
                        instructions.push_back(Instruction::U32WrappingAdd);
                    }
                }
            }

            Token::U32CheckedSub => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let n = *n as u32;
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32CheckedSubImm(n));
                            } else {
                                let n = *n as u32;
                                instructions.push_back(Instruction::U32CheckedSubImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32CheckedSub);
                            } else {
                                instructions.push_back(Instruction::U32CheckedSub);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::U32CheckedSub);
                    } else {
                        instructions.push_back(Instruction::U32CheckedSub);
                    }
                }
            }

            Token::U32OverflowingSub => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let n = *n as u32;
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::U32OverflowingSubImm(n));
                            } else {
                                let n = *n as u32;
                                instructions.push_back(Instruction::U32OverflowingSubImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32OverflowingSub);
                            } else {
                                instructions.push_back(Instruction::U32OverflowingSub);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::U32OverflowingSub);
                    } else {
                        instructions.push_back(Instruction::U32OverflowingSub);
                    }
                }
            }

            Token::U32WrappingSub => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let n = *n as u32;
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::U32WrappingSubImm(n));
                            } else {
                                let n = *n as u32;
                                instructions.push_back(Instruction::U32WrappingSubImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32WrappingSub);
                            } else {
                                instructions.push_back(Instruction::U32WrappingSub);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::U32WrappingSub);
                    } else {
                        instructions.push_back(Instruction::U32WrappingSub);
                    }
                }
            }

            Token::U32CheckedMul => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let n = *n as u32;
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32CheckedMulImm(n));
                            } else {
                                let n = *n as u32;
                                instructions.push_back(Instruction::U32CheckedMulImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32CheckedMul);
                            } else {
                                instructions.push_back(Instruction::U32CheckedMul);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::U32CheckedMul);
                    } else {
                        instructions.push_back(Instruction::U32CheckedMul);
                    }
                }
            }

            Token::U32OverflowingMul => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let n = *n as u32;
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::U32OverflowingMulImm(n));
                            } else {
                                let n = *n as u32;
                                instructions.push_back(Instruction::U32OverflowingMulImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32OverflowingMul);
                            } else {
                                instructions.push_back(Instruction::U32OverflowingMul);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::U32OverflowingMul);
                    } else {
                        instructions.push_back(Instruction::U32OverflowingMul);
                    }
                }
            }

            Token::U32WrappingMul => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let n = *n as u32;
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::U32WrappingMulImm(n));
                            } else {
                                let n = *n as u32;
                                instructions.push_back(Instruction::U32WrappingMulImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32WrappingMul);
                            } else {
                                instructions.push_back(Instruction::U32WrappingMul);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::U32WrappingMul);
                    } else {
                        instructions.push_back(Instruction::U32WrappingMul);
                    }
                }
            }

            Token::U32CheckedDiv => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let n = *n as u32;
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32CheckedDivImm(n));
                            } else {
                                let n = *n as u32;
                                instructions.push_back(Instruction::U32CheckedDivImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32CheckedDiv);
                            } else {
                                instructions.push_back(Instruction::U32CheckedDiv);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::U32CheckedDiv);
                    } else {
                        instructions.push_back(Instruction::U32CheckedDiv);
                    }
                }
            }

            Token::U32UncheckedDiv => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let n = *n as u32;
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::U32UncheckedDivImm(n));
                            } else {
                                let n = *n as u32;
                                instructions.push_back(Instruction::U32UncheckedDivImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32UncheckedDiv);
                            } else {
                                instructions.push_back(Instruction::U32UncheckedDiv);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::U32UncheckedDiv);
                    } else {
                        instructions.push_back(Instruction::U32UncheckedDiv);
                    }
                }
            }

            Token::U32CheckedMod => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let n = *n as u32;
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32CheckedModImm(n));
                            } else {
                                let n = *n as u32;
                                instructions.push_back(Instruction::U32CheckedModImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32CheckedMod);
                            } else {
                                instructions.push_back(Instruction::U32CheckedMod);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::U32CheckedMod);
                    } else {
                        instructions.push_back(Instruction::U32CheckedMod);
                    }
                }
            }

            Token::U32UncheckedMod => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let n = *n as u32;
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::U32UncheckedModImm(n));
                            } else {
                                let n = *n as u32;
                                instructions.push_back(Instruction::U32UncheckedModImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32UncheckedMod);
                            } else {
                                instructions.push_back(Instruction::U32UncheckedMod);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::U32UncheckedMod);
                    } else {
                        instructions.push_back(Instruction::U32UncheckedMod);
                    }
                }
            }

            Token::U32CheckedDivMod => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let n = *n as u32;
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::U32CheckedDivModImm(n));
                            } else {
                                let n = *n as u32;
                                instructions.push_back(Instruction::U32CheckedDivModImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32CheckedDivMod);
                            } else {
                                instructions.push_back(Instruction::U32CheckedDivMod);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::U32CheckedDivMod);
                    } else {
                        instructions.push_back(Instruction::U32CheckedDivMod);
                    }
                }
            }

            Token::U32UncheckedDivMod => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let n = *n as u32;
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::U32UncheckedDivModImm(n));
                            } else {
                                let n = *n as u32;
                                instructions.push_back(Instruction::U32UncheckedDivModImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32UncheckedDivMod);
                            } else {
                                instructions.push_back(Instruction::U32UncheckedDivMod);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::U32UncheckedDivMod);
                    } else {
                        instructions.push_back(Instruction::U32UncheckedDivMod);
                    }
                }
            }

            Token::U32OverflowingAdd3 => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::U32OverflowingAdd3);
                } else {
                    instructions.push_back(Instruction::U32OverflowingAdd3);
                }
            }

            Token::U32WrappingAdd3 => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::U32WrappingAdd3);
                } else {
                    instructions.push_back(Instruction::U32WrappingAdd3);
                }
            }

            Token::U32OverflowingMadd => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::U32OverflowingMadd);
                } else {
                    instructions.push_back(Instruction::U32OverflowingMadd);
                }
            }

            Token::U32WrappingMadd => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::U32WrappingMadd);
                } else {
                    instructions.push_back(Instruction::U32WrappingMadd);
                }
            }

            Token::U32CheckedAnd => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::U32CheckedAnd);
                } else {
                    instructions.push_back(Instruction::U32CheckedAnd);
                }
            }

            Token::U32CheckedOr => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::U32CheckedOr);
                } else {
                    instructions.push_back(Instruction::U32CheckedOr);
                }
            }

            Token::U32CheckedXor => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::U32CheckedXor);
                } else {
                    instructions.push_back(Instruction::U32CheckedXor);
                }
            }

            Token::U32CheckedNot => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::U32CheckedNot);
                } else {
                    instructions.push_back(Instruction::U32CheckedNot);
                }
            }

            Token::U32CheckedShl => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let n = *n as u32;
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32CheckedShlImm(n));
                            } else {
                                let n = *n as u32;
                                instructions.push_back(Instruction::U32CheckedShlImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32CheckedShl);
                            } else {
                                instructions.push_back(Instruction::U32CheckedShl);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::U32CheckedShl);
                    } else {
                        instructions.push_back(Instruction::U32CheckedShl);
                    }
                }
            }

            Token::U32UncheckedShl => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let n = *n as u32;
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::U32UncheckedShlImm(n));
                            } else {
                                let n = *n as u32;
                                instructions.push_back(Instruction::U32UncheckedShlImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32UncheckedShl);
                            } else {
                                instructions.push_back(Instruction::U32UncheckedShl);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::U32UncheckedShl);
                    } else {
                        instructions.push_back(Instruction::U32UncheckedShl);
                    }
                }
            }

            Token::U32CheckedShr => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let n = *n as u32;
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32CheckedShrImm(n));
                            } else {
                                let n = *n as u32;
                                instructions.push_back(Instruction::U32CheckedShrImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32CheckedShr);
                            } else {
                                instructions.push_back(Instruction::U32CheckedShr);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::U32CheckedShr);
                    } else {
                        instructions.push_back(Instruction::U32CheckedShr);
                    }
                }
            }

            Token::U32UncheckedShr => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let n = *n as u32;
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::U32UncheckedShrImm(n));
                            } else {
                                let n = *n as u32;
                                instructions.push_back(Instruction::U32UncheckedShrImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32UncheckedShr);
                            } else {
                                instructions.push_back(Instruction::U32UncheckedShr);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::U32UncheckedShr);
                    } else {
                        instructions.push_back(Instruction::U32UncheckedShr);
                    }
                }
            }

            Token::U32CheckedRotl => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let n = *n;
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::U32CheckedRotlImm(n as u32));
                            } else {
                                let n = *n;
                                instructions.push_back(Instruction::U32CheckedRotlImm(n as u32));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32CheckedRotl);
                            } else {
                                instructions.push_back(Instruction::U32CheckedRotl);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::U32CheckedRotl);
                    } else {
                        instructions.push_back(Instruction::U32CheckedRotl);
                    }
                }
            }

            Token::U32UncheckedRotl => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let n = *n;
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::U32UncheckedRotlImm(n as u32));
                            } else {
                                let n = *n;
                                instructions.push_back(Instruction::U32UncheckedRotlImm(n as u32));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32UncheckedRotl);
                            } else {
                                instructions.push_back(Instruction::U32UncheckedRotl);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::U32UncheckedRotl);
                    } else {
                        instructions.push_back(Instruction::U32UncheckedRotl);
                    }
                }
            }

            Token::U32CheckedRotr => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let n = *n;
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::U32CheckedRotrImm(n as u32));
                            } else {
                                let n = *n;
                                instructions.push_back(Instruction::U32CheckedRotrImm(n as u32));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32CheckedRotr);
                            } else {
                                instructions.push_back(Instruction::U32CheckedRotr);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::U32CheckedRotr);
                    } else {
                        instructions.push_back(Instruction::U32CheckedRotr);
                    }
                }
            }

            Token::U32UncheckedRotr => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let n = *n;
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::U32UncheckedRotrImm(n as u32));
                            } else {
                                let n = *n;
                                instructions.push_back(Instruction::U32UncheckedRotrImm(n as u32));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32UncheckedRotr);
                            } else {
                                instructions.push_back(Instruction::U32UncheckedRotr);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::U32UncheckedRotr);
                    } else {
                        instructions.push_back(Instruction::U32UncheckedRotr);
                    }
                }
            }

            Token::U32CheckedPopcnt => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::U32CheckedPopcnt);
                } else {
                    instructions.push_back(Instruction::U32CheckedPopcnt);
                }
            }

            Token::U32UncheckedPopcnt => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::U32UncheckedPopcnt);
                } else {
                    instructions.push_back(Instruction::U32UncheckedPopcnt);
                }
            }

            Token::U32CheckedEq => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::U32CheckedEqImm(*n as u32));
                            } else {
                                instructions.push_back(Instruction::U32CheckedEqImm(*n as u32));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32CheckedEq);
                            } else {
                                instructions.push_back(Instruction::U32CheckedEq);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::U32CheckedEq);
                    } else {
                        instructions.push_back(Instruction::U32CheckedEq);
                    }
                }
            }

            Token::U32CheckedNeq => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_instruction(Instruction::U32CheckedNeqImm(*n as u32));
                            } else {
                                instructions.push_back(Instruction::U32CheckedNeqImm(*n as u32));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_instruction(Instruction::U32CheckedNeq);
                            } else {
                                instructions.push_back(Instruction::U32CheckedNeq);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_instruction(Instruction::U32CheckedNeq);
                    } else {
                        instructions.push_back(Instruction::U32CheckedNeq);
                    }
                }
            }

            Token::U32CheckedLt => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::U32CheckedLt);
                } else {
                    instructions.push_back(Instruction::U32CheckedLt);
                }
            }

            Token::U32UncheckedLte => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::U32UncheckedLte);
                } else {
                    instructions.push_back(Instruction::U32UncheckedLte);
                }
            }

            Token::U32CheckedLte => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::U32CheckedLte);
                } else {
                    instructions.push_back(Instruction::U32CheckedLte);
                }
            }

            Token::U32UncheckedLt => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::U32UncheckedLt);
                } else {
                    instructions.push_back(Instruction::U32UncheckedLt);
                }
            }

            Token::U32CheckedGt => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::U32CheckedGt);
                } else {
                    instructions.push_back(Instruction::U32CheckedGt);
                }
            }

            Token::U32UncheckedGte => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::U32UncheckedGte);
                } else {
                    instructions.push_back(Instruction::U32UncheckedGte);
                }
            }

            Token::U32CheckedGte => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::U32CheckedGte);
                } else {
                    instructions.push_back(Instruction::U32CheckedGte);
                }
            }

            Token::U32UncheckedGt => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::U32UncheckedGt);
                } else {
                    instructions.push_back(Instruction::U32UncheckedGt);
                }
            }

            Token::U32CheckedMin => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::U32CheckedMin);
                } else {
                    instructions.push_back(Instruction::U32CheckedMin);
                }
            }

            Token::U32UncheckedMin => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::U32UncheckedMin);
                } else {
                    instructions.push_back(Instruction::U32UncheckedMin);
                }
            }

            Token::U32CheckedMax => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::U32CheckedMax);
                } else {
                    instructions.push_back(Instruction::U32CheckedMax);
                }
            }

            Token::U32UncheckedMax => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_instruction(Instruction::U32UncheckedMax);
                } else {
                    instructions.push_back(Instruction::U32UncheckedMax);
                }
            }

            Token::Number(_) => {}

            Token::String(_) => {}
        }
        i += 1;
    }

    Ok((instructions, procedures))
}
