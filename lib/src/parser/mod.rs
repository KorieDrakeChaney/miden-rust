mod token;
mod tokenizer;

use std::collections::VecDeque;

use math::fields::f64::BaseElement;
use token::Token;
pub use tokenizer::tokenize;

use crate::{MidenProgram, Operand};

pub fn parse(tokens: Vec<Token>) -> Result<MidenProgram, String> {
    let mut program: MidenProgram;
    let mut operands: VecDeque<Operand> = VecDeque::new();

    let mut i = 0;

    let mut scope = 0;

    match tokens[0] {
        Token::Proc => {
            let name = match &tokens[1] {
                Token::String(name) => {
                    i += 1;
                    name
                }
                _ => {
                    return Err(format!("Expected name after proc, found {:?}", tokens[1]));
                }
            };

            match &tokens[2] {
                Token::Number(_) => i += 1,
                _ => {}
            };

            program = MidenProgram::proc(name);
            i += 1;
        }
        Token::Begin => {
            program = MidenProgram::new();
            i += 1;
        }
        _ => program = MidenProgram::new(),
    }

    while i < tokens.len() {
        let token = &tokens[i];
        match token {
            Token::If => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::String(statement) => {
                            if statement == "true" {
                                operands.push_back(Operand::IF);
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
                operands.push_back(Operand::Pow2);
            }

            Token::Neg => {
                operands.push_back(Operand::Neg);
            }

            Token::Inv => {
                operands.push_back(Operand::Inv);
            }

            Token::Exp => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::ExpImm(*n));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::Exp);
                        }
                    }
                } else {
                    operands.push_back(Operand::Exp);
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
                        operands.push_back(Operand::Push(BaseElement::from(n)));
                    }
                } else {
                    return Err(format!("Expected number after push_back, found EOF"));
                }
            }

            Token::Else => {
                operands.push_back(Operand::ELSE);
            }
            Token::End => {
                if scope > 0 {
                    operands.push_back(Operand::END);
                }
                scope -= 1;
            }
            Token::While => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::String(statement) => {
                            if statement == "true" {
                                operands.push_back(Operand::WHILE);
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
                            operands.push_back(Operand::REPEAT(*n as usize));
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
                            operands.push_back(Operand::AddImm(BaseElement::from(*n)));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::Add);
                        }
                    }
                } else {
                    operands.push_back(Operand::Add);
                }
            }
            Token::Sub => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::SubImm(BaseElement::from(*n)));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::Sub);
                        }
                    }
                } else {
                    operands.push_back(Operand::Sub);
                }
            }
            Token::Mul => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::MulImm(BaseElement::from(*n)));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::Mul);
                        }
                    }
                } else {
                    operands.push_back(Operand::Mul);
                }
            }
            Token::Div => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::DivImm(BaseElement::from(*n)));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::Div);
                        }
                    }
                } else {
                    operands.push_back(Operand::Div);
                }
            }
            Token::AdvPush => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::AdvPush(*n as usize));
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

            Token::MemLoad => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::MemLoadImm(*n as u32));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::MemLoad);
                        }
                    }
                } else {
                    operands.push_back(Operand::MemLoad);
                }
            }

            Token::MemStore => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::MemStoreImm(*n as u32));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::MemStore);
                        }
                    }
                } else {
                    operands.push_back(Operand::MemStore);
                }
            }

            Token::MemLoadW => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::MemLoadWImm(*n as u32));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::MemLoadW);
                        }
                    }
                } else {
                    operands.push_back(Operand::MemLoadW);
                }
            }

            Token::MemStoreW => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::MemStoreWImm(*n as u32));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::MemStoreW);
                        }
                    }
                } else {
                    operands.push_back(Operand::MemStoreW);
                }
            }

            Token::LocLoad => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::LocLoad(*n as u16));
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
                            operands.push_back(Operand::LocLoadW(*n as u16));
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
                            operands.push_back(Operand::LocStore(*n as u16));
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
                            operands.push_back(Operand::LocStoreW(*n as u16));
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
                            operands.push_back(Operand::Exec(name.to_string()));
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
                            operands.push_back(Operand::Dup(*n as usize));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::Dup(0));
                        }
                    }
                } else {
                    operands.push_back(Operand::Dup(0));
                }
            }

            Token::Swap => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::Swap(*n as usize));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::Swap(1));
                        }
                    }
                } else {
                    operands.push_back(Operand::Swap(1));
                }
            }

            Token::SwapW => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::SwapW(*n as usize));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::SwapW(1));
                        }
                    }
                } else {
                    operands.push_back(Operand::SwapW(1));
                }
            }

            Token::SwapDw => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::SwapDw(*n as usize));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::SwapDw(1));
                        }
                    }
                } else {
                    operands.push_back(Operand::SwapDw(1));
                }
            }

            Token::Proc => {
                return Err(format!("Unexpected proc: {:?}", token));
            }

            Token::Begin => {
                return Err(format!("Unexpected begin: {:?}", token));
            }

            Token::PadW => {
                operands.push_back(Operand::PadW);
            }

            Token::MovUp => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::MovUp(*n as usize));
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
                            operands.push_back(Operand::MovUpW(*n as usize));
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
                            operands.push_back(Operand::MovDn(*n as usize));
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
                            operands.push_back(Operand::MovDnW(*n as usize));
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
                operands.push_back(Operand::Drop);
            }

            Token::Ext2Add => {
                operands.push_back(Operand::Ext2Add);
            }

            Token::Ext2Sub => {
                operands.push_back(Operand::Ext2Sub);
            }

            Token::Ext2Mul => {
                operands.push_back(Operand::Ext2Mul);
            }

            Token::Ext2Div => {
                operands.push_back(Operand::Ext2Div);
            }

            Token::Ext2Neg => {
                operands.push_back(Operand::Ext2Neg);
            }

            Token::Ext2Inv => {
                operands.push_back(Operand::Ext2Inv);
            }

            Token::And => {
                operands.push_back(Operand::And);
            }

            Token::Or => {
                operands.push_back(Operand::Or);
            }

            Token::Xor => {
                operands.push_back(Operand::Xor);
            }

            Token::Not => {
                operands.push_back(Operand::Not);
            }

            Token::IsOdd => {
                operands.push_back(Operand::IsOdd);
            }

            Token::Eq => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::EqImm(BaseElement::from(*n)));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::Eq);
                        }
                    }
                } else {
                    operands.push_back(Operand::Eq);
                }
            }

            Token::Neq => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::NeqImm(BaseElement::from(*n)));
                        }
                        _ => {
                            operands.push_back(Operand::Neq);
                        }
                    }
                } else {
                    operands.push_back(Operand::Eq);
                }
            }

            Token::Lt => {
                operands.push_back(Operand::Lt);
            }

            Token::Lte => {
                operands.push_back(Operand::Lte);
            }

            Token::Gt => {
                operands.push_back(Operand::Gt);
            }

            Token::Gte => {
                operands.push_back(Operand::Gte);
            }

            Token::EqW => {
                operands.push_back(Operand::EqW);
            }

            Token::U32CheckedAdd => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::U32CheckedAddImm(*n as u32));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::U32CheckedAdd);
                        }
                    }
                } else {
                    operands.push_back(Operand::U32CheckedAdd);
                }
            }

            Token::U32OverflowingAdd => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::U32OverflowingAddImm(*n as u32));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::U32OverflowingAdd);
                        }
                    }
                } else {
                    operands.push_back(Operand::U32OverflowingAdd);
                }
            }

            Token::U32WrappingAdd => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::U32WrappingAddImm(*n as u32));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::U32WrappingAdd);
                        }
                    }
                } else {
                    operands.push_back(Operand::U32WrappingAdd);
                }
            }

            Token::U32CheckedSub => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::U32CheckedSubImm(*n as u32));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::U32CheckedSub);
                        }
                    }
                } else {
                    operands.push_back(Operand::U32CheckedSub);
                }
            }

            Token::U32OverflowingSub => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::U32OverflowingSubImm(*n as u32));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::U32OverflowingSub);
                        }
                    }
                } else {
                    operands.push_back(Operand::U32OverflowingSub);
                }
            }

            Token::U32WrappingSub => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::U32WrappingSubImm(*n as u32));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::U32WrappingSub);
                        }
                    }
                } else {
                    operands.push_back(Operand::U32WrappingSub);
                }
            }

            Token::U32CheckedMul => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::U32CheckedMulImm(*n as u32));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::U32CheckedMul);
                        }
                    }
                } else {
                    operands.push_back(Operand::U32CheckedMul);
                }
            }

            Token::U32OverflowingMul => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::U32OverflowingMulImm(*n as u32));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::U32OverflowingMul);
                        }
                    }
                } else {
                    operands.push_back(Operand::U32OverflowingMul);
                }
            }

            Token::U32WrappingMul => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            let n = *n as u32;
                            operands.push_back(Operand::U32WrappingMulImm(n));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::U32WrappingMul);
                        }
                    }
                } else {
                    operands.push_back(Operand::U32WrappingMul);
                }
            }

            Token::U32CheckedDiv => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            let n = *n as u32;
                            operands.push_back(Operand::U32CheckedDivImm(n));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::U32CheckedDiv);
                        }
                    }
                } else {
                    operands.push_back(Operand::U32CheckedDiv);
                }
            }

            Token::U32UncheckedDiv => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            let n = *n as u32;
                            operands.push_back(Operand::U32UncheckedDivImm(n));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::U32UncheckedDiv);
                        }
                    }
                } else {
                    operands.push_back(Operand::U32UncheckedDiv);
                }
            }

            Token::U32CheckedMod => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            let n = *n as u32;
                            operands.push_back(Operand::U32CheckedModImm(n));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::U32CheckedMod);
                        }
                    }
                } else {
                    operands.push_back(Operand::U32CheckedMod);
                }
            }

            Token::U32UncheckedMod => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            let n = *n as u32;
                            operands.push_back(Operand::U32UncheckedModImm(n));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::U32UncheckedMod);
                        }
                    }
                } else {
                    operands.push_back(Operand::U32UncheckedMod);
                }
            }

            Token::U32CheckedDivMod => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            let n = *n as u32;
                            operands.push_back(Operand::U32CheckedDivModImm(n));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::U32CheckedDivMod);
                        }
                    }
                } else {
                    operands.push_back(Operand::U32CheckedDivMod);
                }
            }

            Token::U32UncheckedDivMod => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            let n = *n as u32;
                            operands.push_back(Operand::U32UncheckedDivModImm(n));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::U32UncheckedDivMod);
                        }
                    }
                } else {
                    operands.push_back(Operand::U32UncheckedDivMod);
                }
            }

            Token::U32OverflowingAdd3 => {
                operands.push_back(Operand::U32OverflowingAdd3);
            }

            Token::U32WrappingAdd3 => {
                operands.push_back(Operand::U32WrappingAdd3);
            }

            Token::U32OverflowingMadd => {
                operands.push_back(Operand::U32OverflowingMadd);
            }

            Token::U32WrappingMadd => {
                operands.push_back(Operand::U32WrappingMadd);
            }

            Token::U32CheckedAnd => {
                operands.push_back(Operand::U32CheckedAnd);
            }

            Token::U32CheckedOr => {
                operands.push_back(Operand::U32CheckedOr);
            }

            Token::U32CheckedXor => {
                operands.push_back(Operand::U32CheckedXor);
            }

            Token::U32CheckedNot => {
                operands.push_back(Operand::U32CheckedNot);
            }

            Token::U32CheckedShl => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::U32CheckedShlImm(*n as u32));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::U32CheckedShl);
                        }
                    }
                } else {
                    operands.push_back(Operand::U32CheckedShl);
                }
            }

            Token::U32UncheckedShl => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            let n = *n as u32;
                            operands.push_back(Operand::U32UncheckedShlImm(n));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::U32UncheckedShl);
                        }
                    }
                } else {
                    operands.push_back(Operand::U32UncheckedShl);
                }
            }

            Token::U32CheckedShr => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            let n = *n as u32;
                            operands.push_back(Operand::U32CheckedShrImm(n));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::U32CheckedShr);
                        }
                    }
                } else {
                    operands.push_back(Operand::U32CheckedShr);
                }
            }

            Token::U32UncheckedShr => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::U32UncheckedShrImm(*n as u32));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::U32UncheckedShr);
                        }
                    }
                } else {
                    operands.push_back(Operand::U32UncheckedShr);
                }
            }

            Token::U32CheckedRotl => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::U32CheckedRotlImm(*n as u32));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::U32CheckedRotl);
                        }
                    }
                } else {
                    operands.push_back(Operand::U32CheckedRotl);
                }
            }

            Token::U32UncheckedRotl => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::U32UncheckedRotlImm(*n as u32));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::U32UncheckedRotl);
                        }
                    }
                } else {
                    operands.push_back(Operand::U32UncheckedRotl);
                }
            }

            Token::U32CheckedRotr => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::U32CheckedRotrImm(*n as u32));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::U32CheckedRotr);
                        }
                    }
                } else {
                    operands.push_back(Operand::U32CheckedRotr);
                }
            }

            Token::U32UncheckedRotr => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::U32UncheckedRotrImm(*n as u32));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::U32UncheckedRotr);
                        }
                    }
                } else {
                    operands.push_back(Operand::U32UncheckedRotr);
                }
            }

            Token::U32CheckedPopcnt => {
                operands.push_back(Operand::U32CheckedPopcnt);
            }

            Token::U32UncheckedPopcnt => {
                operands.push_back(Operand::U32UncheckedPopcnt);
            }

            Token::U32CheckedEq => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::U32CheckedEqImm(*n as u32));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::U32CheckedEq);
                        }
                    }
                } else {
                    operands.push_back(Operand::U32CheckedEq);
                }
            }

            Token::U32CheckedNeq => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            operands.push_back(Operand::U32CheckedNeqImm(*n as u32));
                            i += 1;
                        }
                        _ => {
                            operands.push_back(Operand::U32CheckedNeq);
                        }
                    }
                } else {
                    operands.push_back(Operand::U32CheckedNeq);
                }
            }

            Token::U32CheckedLt => {
                operands.push_back(Operand::U32CheckedLt);
            }

            Token::U32UncheckedLte => {
                operands.push_back(Operand::U32UncheckedLte);
            }

            Token::U32CheckedLte => {
                operands.push_back(Operand::U32CheckedLte);
            }

            Token::U32UncheckedLt => {
                operands.push_back(Operand::U32UncheckedLt);
            }

            Token::U32CheckedGt => {
                operands.push_back(Operand::U32CheckedGt);
            }

            Token::U32UncheckedGte => {
                operands.push_back(Operand::U32UncheckedGte);
            }

            Token::U32CheckedGte => {
                operands.push_back(Operand::U32CheckedGte);
            }

            Token::U32UncheckedGt => {
                operands.push_back(Operand::U32UncheckedGt);
            }

            Token::U32CheckedMin => {
                operands.push_back(Operand::U32CheckedMin);
            }

            Token::U32UncheckedMin => {
                operands.push_back(Operand::U32UncheckedMin);
            }

            Token::U32CheckedMax => {
                operands.push_back(Operand::U32CheckedMax);
            }

            Token::U32UncheckedMax => {
                operands.push_back(Operand::U32UncheckedMax);
            }

            Token::Number(_) => {
                return Err(format!("Unexpected number: {:?}", token));
            }

            Token::String(_) => {
                return Err(format!("Unexpected string: {:?}", token));
            }
        }
        i += 1;
    }
    program.add_operands(operands);
    Ok(program)
}
