mod sanitize;
mod token;
mod tokenizer;

use sanitize::sanitize;

use std::collections::VecDeque;

use math::fields::f64::BaseElement;
use token::Token;
pub use tokenizer::tokenize;

use crate::{Operand, Proc};

pub fn parse(tokens: Vec<Token>) -> Result<(VecDeque<Operand>, Vec<Proc>), String> {
    let mut procedures: Vec<Proc> = Vec::new();

    let mut operands: VecDeque<Operand> = VecDeque::new();

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
                    procedures[index].add_operand(Operand::Assert);
                } else {
                    operands.push_back(Operand::Assert);
                }
            }
            Token::AssertZ => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::AssertZ);
                } else {
                    operands.push_back(Operand::AssertZ);
                }
            }
            Token::AssertEq => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::AssertEq);
                } else {
                    operands.push_back(Operand::AssertEq);
                }
            }
            Token::AssertEqW => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::AssertEqW);
                } else {
                    operands.push_back(Operand::AssertEqW);
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
                                procedures[index].add_operand(Operand::PRINT(name.to_string()));
                            } else {
                                operands.push_back(Operand::PRINT(name.to_string()));
                            }
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::PRINT("test".to_string()));
                        }
                    }
                } else {
                    operands.push_back(Operand::PRINT("test".to_string()));
                }
            }
            Token::If => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::String(statement) => {
                            if statement == "true" {
                                if in_proc {
                                    let index = procedures.len() - 1;
                                    procedures[index].add_operand(Operand::IF);
                                } else {
                                    operands.push_back(Operand::IF);
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
                    procedures[index].add_operand(Operand::Pow2);
                } else {
                    operands.push_back(Operand::Pow2);
                }
            }

            Token::Neg => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::Neg);
                } else {
                    operands.push_back(Operand::Neg);
                }
            }

            Token::Inv => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::Inv);
                } else {
                    operands.push_back(Operand::Inv);
                }
            }

            Token::Exp => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::ExpImm(*n));
                            } else {
                                operands.push_back(Operand::ExpImm(*n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::Exp);
                            } else {
                                operands.push_back(Operand::Exp);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::Exp);
                    } else {
                        operands.push_back(Operand::Exp);
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
                            procedures[index].add_operand(Operand::Push(BaseElement::from(n)));
                        } else {
                            operands.push_back(Operand::Push(BaseElement::from(n)));
                        }
                    }
                } else {
                    return Err(format!("Expected number after push_back, found EOF"));
                }
            }

            Token::Else => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::ELSE);
                } else {
                    operands.push_back(Operand::ELSE);
                }
            }
            Token::End => {
                scope -= 1;

                if scope != 0 {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::END);
                    } else {
                        operands.push_back(Operand::END);
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
                                    procedures[index].add_operand(Operand::WHILE);
                                } else {
                                    operands.push_back(Operand::WHILE);
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
                                procedures[index].add_operand(Operand::REPEAT(*n as usize));
                            } else {
                                operands.push_back(Operand::REPEAT(*n as usize));
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
                                    .add_operand(Operand::AddImm(BaseElement::from(*n)));
                            } else {
                                operands.push_back(Operand::AddImm(BaseElement::from(*n)));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::Add);
                            } else {
                                operands.push_back(Operand::Add);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::Add);
                    } else {
                        operands.push_back(Operand::Add);
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
                                    .add_operand(Operand::SubImm(BaseElement::from(*n)));
                            } else {
                                operands.push_back(Operand::SubImm(BaseElement::from(*n)));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::Sub);
                            } else {
                                operands.push_back(Operand::Sub);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::Sub);
                    } else {
                        operands.push_back(Operand::Sub);
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
                                    .add_operand(Operand::MulImm(BaseElement::from(*n)));
                            } else {
                                operands.push_back(Operand::MulImm(BaseElement::from(*n)));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::Mul);
                            } else {
                                operands.push_back(Operand::Mul);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::Mul);
                    } else {
                        operands.push_back(Operand::Mul);
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
                                    .add_operand(Operand::DivImm(BaseElement::from(*n)));
                            } else {
                                operands.push_back(Operand::DivImm(BaseElement::from(*n)));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::Div);
                            } else {
                                operands.push_back(Operand::Div);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::Div);
                    } else {
                        operands.push_back(Operand::Div);
                    }
                }
            }
            Token::AdvPush => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::AdvPush(*n as usize));
                            } else {
                                operands.push_back(Operand::AdvPush(*n as usize));
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
                    procedures[index].add_operand(Operand::AdvLoadW);
                } else {
                    operands.push_back(Operand::AdvLoadW);
                }
            }

            Token::AdvPipe => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::AdvPipe);
                } else {
                    operands.push_back(Operand::AdvPipe);
                }
            }

            Token::MemLoad => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::MemLoadImm(*n as u32));
                            } else {
                                operands.push_back(Operand::MemLoadImm(*n as u32));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::MemLoad);
                            } else {
                                operands.push_back(Operand::MemLoad);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::MemLoad);
                    } else {
                        operands.push_back(Operand::MemLoad);
                    }
                }
            }

            Token::MemStore => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::MemStoreImm(*n as u32));
                            } else {
                                operands.push_back(Operand::MemStoreImm(*n as u32));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::MemStore);
                            } else {
                                operands.push_back(Operand::MemStore);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::MemStore);
                    } else {
                        operands.push_back(Operand::MemStore);
                    }
                }
            }

            Token::MemLoadW => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::MemLoadWImm(*n as u32));
                            } else {
                                operands.push_back(Operand::MemLoadWImm(*n as u32));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::MemLoadW);
                            } else {
                                operands.push_back(Operand::MemLoadW);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::MemLoadW);
                    } else {
                        operands.push_back(Operand::MemLoadW);
                    }
                }
            }

            Token::MemStoreW => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::MemStoreWImm(*n as u32));
                            } else {
                                operands.push_back(Operand::MemStoreWImm(*n as u32));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::MemStoreW);
                            } else {
                                operands.push_back(Operand::MemStoreW);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::MemStoreW);
                    } else {
                        operands.push_back(Operand::MemStoreW);
                    }
                }
            }

            Token::LocLoad => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::LocLoad(*n as u16));
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
                                procedures[index].add_operand(Operand::LocLoadW(*n as u16));
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
                                procedures[index].add_operand(Operand::LocStore(*n as u16));
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
                                procedures[index].add_operand(Operand::LocStoreW(*n as u16));
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
                                procedures[index].add_operand(Operand::Exec(name.to_string()));
                            } else {
                                operands.push_back(Operand::Exec(name.to_string()));
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
                                procedures[index].add_operand(Operand::Dup(*n as usize));
                            } else {
                                operands.push_back(Operand::Dup(*n as usize));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::Dup(0));
                            } else {
                                operands.push_back(Operand::Dup(0));
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::Dup(0));
                    } else {
                        operands.push_back(Operand::Dup(0));
                    }
                    operands.push_back(Operand::Dup(0));
                }
            }

            Token::Swap => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::Swap(*n as usize));
                            } else {
                                operands.push_back(Operand::Swap(*n as usize));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::Swap(1));
                            } else {
                                operands.push_back(Operand::Swap(1));
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::Swap(1));
                    } else {
                        operands.push_back(Operand::Swap(1));
                    }
                }
            }

            Token::SwapW => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::SwapW(*n as usize));
                            } else {
                                operands.push_back(Operand::SwapW(*n as usize));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::SwapW(1));
                            } else {
                                operands.push_back(Operand::SwapW(1));
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::SwapW(1));
                    } else {
                        operands.push_back(Operand::SwapW(1));
                    }
                }
            }

            Token::SwapDw => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::SwapDw(*n as usize));
                            } else {
                                operands.push_back(Operand::SwapDw(*n as usize));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::SwapDw(1));
                            } else {
                                operands.push_back(Operand::SwapDw(1));
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::SwapDw(1));
                    } else {
                        operands.push_back(Operand::SwapDw(1));
                    }
                }
            }

            Token::PadW => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::PadW);
                } else {
                    operands.push_back(Operand::PadW);
                }
            }

            Token::MovUp => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::MovUp(*n as usize));
                            } else {
                                operands.push_back(Operand::MovUp(*n as usize));
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
                                procedures[index].add_operand(Operand::MovUpW(*n as usize));
                            } else {
                                operands.push_back(Operand::MovUpW(*n as usize));
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
                                procedures[index].add_operand(Operand::MovDn(*n as usize));
                            } else {
                                operands.push_back(Operand::MovDn(*n as usize));
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
                                procedures[index].add_operand(Operand::MovDnW(*n as usize));
                            } else {
                                operands.push_back(Operand::MovDnW(*n as usize));
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
                    procedures[index].add_operand(Operand::Drop);
                } else {
                    operands.push_back(Operand::Drop);
                }
            }

            Token::DropW => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::DropW);
                } else {
                    operands.push_back(Operand::DropW);
                }
            }

            Token::CSwap => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::CSwap);
                } else {
                    operands.push_back(Operand::CSwap);
                }
            }

            Token::CSwapW => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::CSwapW);
                } else {
                    operands.push_back(Operand::CSwapW);
                }
            }

            Token::CDrop => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::CDrop);
                } else {
                    operands.push_back(Operand::CDrop);
                }
            }

            Token::CDropW => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::CDropW);
                } else {
                    operands.push_back(Operand::CDropW);
                }
            }

            Token::Ext2Add => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::Ext2Add);
                } else {
                    operands.push_back(Operand::Ext2Add);
                }
            }

            Token::Ext2Sub => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::Ext2Sub);
                } else {
                    operands.push_back(Operand::Ext2Sub);
                }
            }

            Token::Ext2Mul => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::Ext2Mul);
                } else {
                    operands.push_back(Operand::Ext2Mul);
                }
            }

            Token::Ext2Div => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::Ext2Div);
                } else {
                    operands.push_back(Operand::Ext2Div);
                }
            }

            Token::Ext2Neg => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::Ext2Neg);
                } else {
                    operands.push_back(Operand::Ext2Neg);
                }
            }

            Token::Ext2Inv => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::Ext2Inv);
                } else {
                    operands.push_back(Operand::Ext2Inv);
                }
            }

            Token::And => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::And);
                } else {
                    operands.push_back(Operand::And);
                }
            }

            Token::Or => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::Or);
                } else {
                    operands.push_back(Operand::Or);
                }
            }

            Token::Xor => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::Xor);
                } else {
                    operands.push_back(Operand::Xor);
                }
            }

            Token::Not => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::Not);
                } else {
                    operands.push_back(Operand::Not);
                }
            }

            Token::IsOdd => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::IsOdd);
                } else {
                    operands.push_back(Operand::IsOdd);
                }
            }

            Token::Eq => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index]
                                    .add_operand(Operand::EqImm(BaseElement::from(*n)));
                            } else {
                                operands.push_back(Operand::EqImm(BaseElement::from(*n)));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::Eq);
                            } else {
                                operands.push_back(Operand::Eq);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::Eq);
                    } else {
                        operands.push_back(Operand::Eq);
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
                                    .add_operand(Operand::NeqImm(BaseElement::from(*n)));
                            } else {
                                operands.push_back(Operand::NeqImm(BaseElement::from(*n)));
                            }
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::Neq);
                            } else {
                                operands.push_back(Operand::Neq);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::Neq);
                    } else {
                        operands.push_back(Operand::Neq);
                    }
                }
            }

            Token::Lt => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::Lt);
                } else {
                    operands.push_back(Operand::Lt);
                }
            }

            Token::Lte => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::Lte);
                } else {
                    operands.push_back(Operand::Lte);
                }
            }

            Token::Gt => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::Gt);
                } else {
                    operands.push_back(Operand::Gt);
                }
            }

            Token::Gte => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::Gte);
                } else {
                    operands.push_back(Operand::Gte);
                }
            }

            Token::EqW => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::EqW);
                } else {
                    operands.push_back(Operand::EqW);
                }
            }

            Token::U32CheckedAdd => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                let n = *n as u32;
                                procedures[index].add_operand(Operand::U32CheckedAddImm(n));
                            } else {
                                let n = *n as u32;
                                operands.push_back(Operand::U32CheckedAddImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32CheckedAdd);
                            } else {
                                operands.push_back(Operand::U32CheckedAdd);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::U32CheckedAdd);
                    } else {
                        operands.push_back(Operand::U32CheckedAdd);
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
                                procedures[index].add_operand(Operand::U32OverflowingAddImm(n));
                            } else {
                                let n = *n as u32;
                                operands.push_back(Operand::U32OverflowingAddImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32OverflowingAdd);
                            } else {
                                operands.push_back(Operand::U32OverflowingAdd);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::U32OverflowingAdd);
                    } else {
                        operands.push_back(Operand::U32OverflowingAdd);
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
                                procedures[index].add_operand(Operand::U32WrappingAddImm(n));
                            } else {
                                let n = *n as u32;
                                operands.push_back(Operand::U32WrappingAddImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32WrappingAdd);
                            } else {
                                operands.push_back(Operand::U32WrappingAdd);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::U32WrappingAdd);
                    } else {
                        operands.push_back(Operand::U32WrappingAdd);
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
                                procedures[index].add_operand(Operand::U32CheckedSubImm(n));
                            } else {
                                let n = *n as u32;
                                operands.push_back(Operand::U32CheckedSubImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32CheckedSub);
                            } else {
                                operands.push_back(Operand::U32CheckedSub);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::U32CheckedSub);
                    } else {
                        operands.push_back(Operand::U32CheckedSub);
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
                                procedures[index].add_operand(Operand::U32OverflowingSubImm(n));
                            } else {
                                let n = *n as u32;
                                operands.push_back(Operand::U32OverflowingSubImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32OverflowingSub);
                            } else {
                                operands.push_back(Operand::U32OverflowingSub);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::U32OverflowingSub);
                    } else {
                        operands.push_back(Operand::U32OverflowingSub);
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
                                procedures[index].add_operand(Operand::U32WrappingSubImm(n));
                            } else {
                                let n = *n as u32;
                                operands.push_back(Operand::U32WrappingSubImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32WrappingSub);
                            } else {
                                operands.push_back(Operand::U32WrappingSub);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::U32WrappingSub);
                    } else {
                        operands.push_back(Operand::U32WrappingSub);
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
                                procedures[index].add_operand(Operand::U32CheckedMulImm(n));
                            } else {
                                let n = *n as u32;
                                operands.push_back(Operand::U32CheckedMulImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32CheckedMul);
                            } else {
                                operands.push_back(Operand::U32CheckedMul);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::U32CheckedMul);
                    } else {
                        operands.push_back(Operand::U32CheckedMul);
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
                                procedures[index].add_operand(Operand::U32OverflowingMulImm(n));
                            } else {
                                let n = *n as u32;
                                operands.push_back(Operand::U32OverflowingMulImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32OverflowingMul);
                            } else {
                                operands.push_back(Operand::U32OverflowingMul);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::U32OverflowingMul);
                    } else {
                        operands.push_back(Operand::U32OverflowingMul);
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
                                procedures[index].add_operand(Operand::U32WrappingMulImm(n));
                            } else {
                                let n = *n as u32;
                                operands.push_back(Operand::U32WrappingMulImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32WrappingMul);
                            } else {
                                operands.push_back(Operand::U32WrappingMul);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::U32WrappingMul);
                    } else {
                        operands.push_back(Operand::U32WrappingMul);
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
                                procedures[index].add_operand(Operand::U32CheckedDivImm(n));
                            } else {
                                let n = *n as u32;
                                operands.push_back(Operand::U32CheckedDivImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32CheckedDiv);
                            } else {
                                operands.push_back(Operand::U32CheckedDiv);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::U32CheckedDiv);
                    } else {
                        operands.push_back(Operand::U32CheckedDiv);
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
                                procedures[index].add_operand(Operand::U32UncheckedDivImm(n));
                            } else {
                                let n = *n as u32;
                                operands.push_back(Operand::U32UncheckedDivImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32UncheckedDiv);
                            } else {
                                operands.push_back(Operand::U32UncheckedDiv);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::U32UncheckedDiv);
                    } else {
                        operands.push_back(Operand::U32UncheckedDiv);
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
                                procedures[index].add_operand(Operand::U32CheckedModImm(n));
                            } else {
                                let n = *n as u32;
                                operands.push_back(Operand::U32CheckedModImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32CheckedMod);
                            } else {
                                operands.push_back(Operand::U32CheckedMod);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::U32CheckedMod);
                    } else {
                        operands.push_back(Operand::U32CheckedMod);
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
                                procedures[index].add_operand(Operand::U32UncheckedModImm(n));
                            } else {
                                let n = *n as u32;
                                operands.push_back(Operand::U32UncheckedModImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32UncheckedMod);
                            } else {
                                operands.push_back(Operand::U32UncheckedMod);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::U32UncheckedMod);
                    } else {
                        operands.push_back(Operand::U32UncheckedMod);
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
                                procedures[index].add_operand(Operand::U32CheckedDivModImm(n));
                            } else {
                                let n = *n as u32;
                                operands.push_back(Operand::U32CheckedDivModImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32CheckedDivMod);
                            } else {
                                operands.push_back(Operand::U32CheckedDivMod);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::U32CheckedDivMod);
                    } else {
                        operands.push_back(Operand::U32CheckedDivMod);
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
                                procedures[index].add_operand(Operand::U32UncheckedDivModImm(n));
                            } else {
                                let n = *n as u32;
                                operands.push_back(Operand::U32UncheckedDivModImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32UncheckedDivMod);
                            } else {
                                operands.push_back(Operand::U32UncheckedDivMod);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::U32UncheckedDivMod);
                    } else {
                        operands.push_back(Operand::U32UncheckedDivMod);
                    }
                }
            }

            Token::U32OverflowingAdd3 => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::U32OverflowingAdd3);
                } else {
                    operands.push_back(Operand::U32OverflowingAdd3);
                }
            }

            Token::U32WrappingAdd3 => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::U32WrappingAdd3);
                } else {
                    operands.push_back(Operand::U32WrappingAdd3);
                }
            }

            Token::U32OverflowingMadd => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::U32OverflowingMadd);
                } else {
                    operands.push_back(Operand::U32OverflowingMadd);
                }
            }

            Token::U32WrappingMadd => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::U32WrappingMadd);
                } else {
                    operands.push_back(Operand::U32WrappingMadd);
                }
            }

            Token::U32CheckedAnd => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::U32CheckedAnd);
                } else {
                    operands.push_back(Operand::U32CheckedAnd);
                }
            }

            Token::U32CheckedOr => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::U32CheckedOr);
                } else {
                    operands.push_back(Operand::U32CheckedOr);
                }
            }

            Token::U32CheckedXor => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::U32CheckedXor);
                } else {
                    operands.push_back(Operand::U32CheckedXor);
                }
            }

            Token::U32CheckedNot => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::U32CheckedNot);
                } else {
                    operands.push_back(Operand::U32CheckedNot);
                }
            }

            Token::U32CheckedShl => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let n = *n as u32;
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32CheckedShlImm(n));
                            } else {
                                let n = *n as u32;
                                operands.push_back(Operand::U32CheckedShlImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32CheckedShl);
                            } else {
                                operands.push_back(Operand::U32CheckedShl);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::U32CheckedShl);
                    } else {
                        operands.push_back(Operand::U32CheckedShl);
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
                                procedures[index].add_operand(Operand::U32UncheckedShlImm(n));
                            } else {
                                let n = *n as u32;
                                operands.push_back(Operand::U32UncheckedShlImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32UncheckedShl);
                            } else {
                                operands.push_back(Operand::U32UncheckedShl);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::U32UncheckedShl);
                    } else {
                        operands.push_back(Operand::U32UncheckedShl);
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
                                procedures[index].add_operand(Operand::U32CheckedShrImm(n));
                            } else {
                                let n = *n as u32;
                                operands.push_back(Operand::U32CheckedShrImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32CheckedShr);
                            } else {
                                operands.push_back(Operand::U32CheckedShr);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::U32CheckedShr);
                    } else {
                        operands.push_back(Operand::U32CheckedShr);
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
                                procedures[index].add_operand(Operand::U32UncheckedShrImm(n));
                            } else {
                                let n = *n as u32;
                                operands.push_back(Operand::U32UncheckedShrImm(n));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32UncheckedShr);
                            } else {
                                operands.push_back(Operand::U32UncheckedShr);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::U32UncheckedShr);
                    } else {
                        operands.push_back(Operand::U32UncheckedShr);
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
                                procedures[index].add_operand(Operand::U32CheckedRotlImm(n as u32));
                            } else {
                                let n = *n;
                                operands.push_back(Operand::U32CheckedRotlImm(n as u32));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32CheckedRotl);
                            } else {
                                operands.push_back(Operand::U32CheckedRotl);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::U32CheckedRotl);
                    } else {
                        operands.push_back(Operand::U32CheckedRotl);
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
                                    .add_operand(Operand::U32UncheckedRotlImm(n as u32));
                            } else {
                                let n = *n;
                                operands.push_back(Operand::U32UncheckedRotlImm(n as u32));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32UncheckedRotl);
                            } else {
                                operands.push_back(Operand::U32UncheckedRotl);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::U32UncheckedRotl);
                    } else {
                        operands.push_back(Operand::U32UncheckedRotl);
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
                                procedures[index].add_operand(Operand::U32CheckedRotrImm(n as u32));
                            } else {
                                let n = *n;
                                operands.push_back(Operand::U32CheckedRotrImm(n as u32));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32CheckedRotr);
                            } else {
                                operands.push_back(Operand::U32CheckedRotr);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::U32CheckedRotr);
                    } else {
                        operands.push_back(Operand::U32CheckedRotr);
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
                                    .add_operand(Operand::U32UncheckedRotrImm(n as u32));
                            } else {
                                let n = *n;
                                operands.push_back(Operand::U32UncheckedRotrImm(n as u32));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32UncheckedRotr);
                            } else {
                                operands.push_back(Operand::U32UncheckedRotr);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::U32UncheckedRotr);
                    } else {
                        operands.push_back(Operand::U32UncheckedRotr);
                    }
                }
            }

            Token::U32CheckedPopcnt => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::U32CheckedPopcnt);
                } else {
                    operands.push_back(Operand::U32CheckedPopcnt);
                }
            }

            Token::U32UncheckedPopcnt => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::U32UncheckedPopcnt);
                } else {
                    operands.push_back(Operand::U32UncheckedPopcnt);
                }
            }

            Token::U32CheckedEq => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32CheckedEqImm(*n as u32));
                            } else {
                                operands.push_back(Operand::U32CheckedEqImm(*n as u32));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32CheckedEq);
                            } else {
                                operands.push_back(Operand::U32CheckedEq);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::U32CheckedEq);
                    } else {
                        operands.push_back(Operand::U32CheckedEq);
                    }
                }
            }

            Token::U32CheckedNeq => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32CheckedNeqImm(*n as u32));
                            } else {
                                operands.push_back(Operand::U32CheckedNeqImm(*n as u32));
                            }
                            i += 1;
                        }
                        _ => {
                            if in_proc {
                                let index = procedures.len() - 1;
                                procedures[index].add_operand(Operand::U32CheckedNeq);
                            } else {
                                operands.push_back(Operand::U32CheckedNeq);
                            }
                        }
                    }
                } else {
                    if in_proc {
                        let index = procedures.len() - 1;
                        procedures[index].add_operand(Operand::U32CheckedNeq);
                    } else {
                        operands.push_back(Operand::U32CheckedNeq);
                    }
                }
            }

            Token::U32CheckedLt => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::U32CheckedLt);
                } else {
                    operands.push_back(Operand::U32CheckedLt);
                }
            }

            Token::U32UncheckedLte => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::U32UncheckedLte);
                } else {
                    operands.push_back(Operand::U32UncheckedLte);
                }
            }

            Token::U32CheckedLte => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::U32CheckedLte);
                } else {
                    operands.push_back(Operand::U32CheckedLte);
                }
            }

            Token::U32UncheckedLt => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::U32UncheckedLt);
                } else {
                    operands.push_back(Operand::U32UncheckedLt);
                }
            }

            Token::U32CheckedGt => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::U32CheckedGt);
                } else {
                    operands.push_back(Operand::U32CheckedGt);
                }
            }

            Token::U32UncheckedGte => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::U32UncheckedGte);
                } else {
                    operands.push_back(Operand::U32UncheckedGte);
                }
            }

            Token::U32CheckedGte => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::U32CheckedGte);
                } else {
                    operands.push_back(Operand::U32CheckedGte);
                }
            }

            Token::U32UncheckedGt => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::U32UncheckedGt);
                } else {
                    operands.push_back(Operand::U32UncheckedGt);
                }
            }

            Token::U32CheckedMin => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::U32CheckedMin);
                } else {
                    operands.push_back(Operand::U32CheckedMin);
                }
            }

            Token::U32UncheckedMin => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::U32UncheckedMin);
                } else {
                    operands.push_back(Operand::U32UncheckedMin);
                }
            }

            Token::U32CheckedMax => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::U32CheckedMax);
                } else {
                    operands.push_back(Operand::U32CheckedMax);
                }
            }

            Token::U32UncheckedMax => {
                if in_proc {
                    let index = procedures.len() - 1;
                    procedures[index].add_operand(Operand::U32UncheckedMax);
                } else {
                    operands.push_back(Operand::U32UncheckedMax);
                }
            }

            Token::Number(_) => {}

            Token::String(_) => {}
        }
        i += 1;
    }

    Ok((operands, procedures))
}
