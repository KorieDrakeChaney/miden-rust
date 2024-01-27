mod token;
mod tokenizer;

use math::fields::f64::BaseElement;
use token::Token;
pub use tokenizer::tokenize;

use crate::{MidenProgram, Operand};

pub fn parse(tokens: Vec<Token>) -> Result<MidenProgram, String> {
    let mut program: MidenProgram;

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
                                program.add_operand(Operand::IF);
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
                program.add_operand(Operand::Pow2);
            }

            Token::Neg => {
                program.add_operand(Operand::Neg);
            }

            Token::Inv => {
                program.add_operand(Operand::Inv);
            }

            Token::Exp => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            program.add_operand(Operand::ExpImm(*n));
                            i += 1;
                        }
                        _ => {
                            program.add_operand(Operand::Exp);
                        }
                    }
                } else {
                    program.add_operand(Operand::Exp);
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
                        program.add_operand(Operand::Push(BaseElement::from(n)));
                    }
                } else {
                    return Err(format!("Expected number after push_back, found EOF"));
                }
            }

            Token::Else => {
                program.add_operand(Operand::ELSE);
            }
            Token::End => {
                if scope > 0 {
                    program.add_operand(Operand::END);
                }
                scope -= 1;
            }
            Token::While => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::String(statement) => {
                            if statement == "true" {
                                program.add_operand(Operand::WHILE);
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
                            program.add_operand(Operand::REPEAT(*n as usize));
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
                            program.add_operand(Operand::AddImm(BaseElement::from(*n)));
                            i += 1;
                        }
                        _ => {
                            program.add_operand(Operand::Add);
                        }
                    }
                } else {
                    program.add_operand(Operand::Add);
                }
            }
            Token::Sub => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            program.add_operand(Operand::SubImm(BaseElement::from(*n)));
                            i += 1;
                        }
                        _ => {
                            program.add_operand(Operand::Sub);
                        }
                    }
                } else {
                    program.add_operand(Operand::Sub);
                }
            }
            Token::Mul => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            program.add_operand(Operand::MulImm(BaseElement::from(*n)));
                            i += 1;
                        }
                        _ => {
                            program.add_operand(Operand::Mul);
                        }
                    }
                } else {
                    program.add_operand(Operand::Mul);
                }
            }
            Token::Div => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            program.add_operand(Operand::DivImm(BaseElement::from(*n)));
                            i += 1;
                        }
                        _ => {
                            program.add_operand(Operand::Div);
                        }
                    }
                } else {
                    program.add_operand(Operand::Div);
                }
            }
            Token::AdvPush => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            program.add_operand(Operand::AdvPush(*n as usize));
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
                            program.add_operand(Operand::MemLoadImm(*n as u32));
                            i += 1;
                        }
                        _ => {
                            program.add_operand(Operand::MemLoad);
                        }
                    }
                } else {
                    program.add_operand(Operand::MemLoad);
                }
            }

            Token::MemStore => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            program.add_operand(Operand::MemStoreImm(*n as u32));
                            i += 1;
                        }
                        _ => {
                            program.add_operand(Operand::MemStore);
                        }
                    }
                } else {
                    program.add_operand(Operand::MemStore);
                }
            }

            Token::MemLoadW => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            program.add_operand(Operand::MemLoadWImm(*n as u32));
                            i += 1;
                        }
                        _ => {
                            program.add_operand(Operand::MemLoadW);
                        }
                    }
                } else {
                    program.add_operand(Operand::MemLoadW);
                }
            }

            Token::MemStoreW => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            program.add_operand(Operand::MemStoreWImm(*n as u32));
                            i += 1;
                        }
                        _ => {
                            program.add_operand(Operand::MemStoreW);
                        }
                    }
                } else {
                    program.add_operand(Operand::MemStoreW);
                }
            }

            Token::LocLoad => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            program.add_operand(Operand::LocLoad(*n as u16));
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
                            program.add_operand(Operand::LocLoadW(*n as u16));
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
                            program.add_operand(Operand::LocStore(*n as u16));
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
                            program.add_operand(Operand::LocStoreW(*n as u16));
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
                            program.add_operand(Operand::Exec(name.to_string()));
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
                            program.add_operand(Operand::Dup(*n as usize));
                            i += 1;
                        }
                        _ => {
                            program.add_operand(Operand::Dup(0));
                        }
                    }
                } else {
                    program.add_operand(Operand::Dup(0));
                }
            }

            Token::Swap => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            program.add_operand(Operand::Swap(*n as usize));
                            i += 1;
                        }
                        _ => {
                            program.add_operand(Operand::Swap(1));
                        }
                    }
                } else {
                    program.add_operand(Operand::Swap(1));
                }
            }

            Token::SwapW => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            program.add_operand(Operand::SwapW(*n as usize));
                            i += 1;
                        }
                        _ => {
                            program.add_operand(Operand::SwapW(1));
                        }
                    }
                } else {
                    program.add_operand(Operand::SwapW(1));
                }
            }

            Token::SwapDw => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            program.add_operand(Operand::SwapDw(*n as usize));
                            i += 1;
                        }
                        _ => {
                            program.add_operand(Operand::SwapDw(1));
                        }
                    }
                } else {
                    program.add_operand(Operand::SwapDw(1));
                }
            }

            Token::Proc => {
                return Err(format!("Unexpected proc: {:?}", token));
            }

            Token::Begin => {
                return Err(format!("Unexpected begin: {:?}", token));
            }

            Token::PadW => {
                program.add_operand(Operand::PadW);
            }

            Token::MovUp => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            program.add_operand(Operand::MovUp(*n as usize));
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
                            program.add_operand(Operand::MovUpW(*n as usize));
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
                            program.add_operand(Operand::MovDn(*n as usize));
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
                            program.add_operand(Operand::MovDnW(*n as usize));
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
                program.add_operand(Operand::Drop);
            }

            Token::Ext2Add => {
                program.add_operand(Operand::Ext2Add);
            }

            Token::Ext2Sub => {
                program.add_operand(Operand::Ext2Sub);
            }

            Token::Ext2Mul => {
                program.add_operand(Operand::Ext2Mul);
            }

            Token::Ext2Div => {
                program.add_operand(Operand::Ext2Div);
            }

            Token::Ext2Neg => {
                program.add_operand(Operand::Ext2Neg);
            }

            Token::Ext2Inv => {
                program.add_operand(Operand::Ext2Inv);
            }

            Token::And => {
                program.add_operand(Operand::And);
            }

            Token::Or => {
                program.add_operand(Operand::Or);
            }

            Token::Xor => {
                program.add_operand(Operand::Xor);
            }

            Token::Not => {
                program.add_operand(Operand::Not);
            }

            Token::IsOdd => {
                program.add_operand(Operand::IsOdd);
            }

            Token::Eq => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            program.add_operand(Operand::EqImm(BaseElement::from(*n)));
                            i += 1;
                        }
                        _ => {
                            program.add_operand(Operand::Eq);
                        }
                    }
                } else {
                    program.add_operand(Operand::Eq);
                }
            }

            Token::Neq => {
                if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        Token::Number(n) => {
                            program.add_operand(Operand::NeqImm(BaseElement::from(*n)));
                        }
                        _ => {
                            program.add_operand(Operand::Neq);
                        }
                    }
                } else {
                    program.add_operand(Operand::Eq);
                }
            }

            Token::Lt => {
                program.add_operand(Operand::Lt);
            }

            Token::Lte => {
                program.add_operand(Operand::Lte);
            }

            Token::Gt => {
                program.add_operand(Operand::Gt);
            }

            Token::Gte => {
                program.add_operand(Operand::Gte);
            }

            Token::EqW => {
                program.add_operand(Operand::EqW);
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

    Ok(program)
}
