use crate::{program::error::MidenProgramError, Instruction, MidenProgram};
use miden::math::{Felt, FieldElement, StarkField};

use super::utils::{max, U32_MAX};

impl MidenProgram {
    pub fn is_valid_operand(&mut self, operand: &Instruction) -> Option<MidenProgramError> {
        match operand {
            Instruction::CDrop | Instruction::CSwap | Instruction::CDropW | Instruction::CSwapW => {
                if let Some(c) = self.stack.get(0) {
                    if *c != Felt::ZERO && *c != Felt::ONE {
                        return Some(MidenProgramError::NotBinaryValue(c.as_int()));
                    }
                }
            }

            Instruction::Ext2Div | Instruction::Div => {
                if let Some(a) = self.stack.get(0) {
                    if *a == Felt::ZERO {
                        return Some(MidenProgramError::DivideByZero);
                    }
                }
            }
            Instruction::Ext2Inv => {
                if let (Some(a1), Some(a0)) = (self.stack.get(0), self.stack.get(1)) {
                    if *a0 == Felt::ZERO || *a1 == Felt::ZERO {
                        return Some(MidenProgramError::ZeroInvertInvalid);
                    }
                }
            }
            Instruction::Inv => {
                if let Some(a) = self.stack.get(0) {
                    if *a == Felt::ZERO {
                        return Some(MidenProgramError::ZeroInvertInvalid);
                    }
                }
            }
            Instruction::AdvPush(n) => {
                if !(*n >= 1 && *n <= 16) {
                    return Some(MidenProgramError::InvalidParameter(
                        Instruction::AdvPush(*n).to_string(),
                        *n,
                        1,
                        16,
                    ));
                } else if *n > self.advice_stack.len() {
                    return Some(MidenProgramError::AdviceStackReadOutOfBounds(
                        *n,
                        self.advice_stack.len(),
                    ));
                }
            }
            // Manipulation
            Instruction::Dup(n) => {
                if !(*n <= 15) {
                    return Some(MidenProgramError::InvalidParameter(
                        Instruction::Dup(*n).to_string(),
                        *n,
                        0,
                        15,
                    ));
                }
            }
            Instruction::Swap(n) => {
                if !(*n > 0 && *n <= 15) {
                    return Some(MidenProgramError::InvalidParameter(
                        Instruction::Swap(*n).to_string(),
                        *n,
                        1,
                        15,
                    ));
                }
            }
            Instruction::SwapW(n) => {
                if !(*n > 0 && *n <= 3) {
                    return Some(MidenProgramError::InvalidParameter(
                        Instruction::Swap(*n).to_string(),
                        *n,
                        1,
                        3,
                    ));
                }
            }
            Instruction::MovDn(n) => {
                if !(*n >= 2 && *n <= 15) {
                    return Some(MidenProgramError::InvalidParameter(
                        Instruction::MovDn(*n).to_string(),
                        *n,
                        2,
                        3,
                    ));
                }
            }
            Instruction::MovDnW(n) => {
                if !(*n >= 2 && *n <= 3) {
                    return Some(MidenProgramError::InvalidParameter(
                        Instruction::MovDnW(*n).to_string(),
                        *n,
                        2,
                        15,
                    ));
                }
            }
            Instruction::MovUp(n) => {
                if !(*n >= 2 && *n <= 15) {
                    return Some(MidenProgramError::InvalidParameter(
                        Instruction::MovUp(*n).to_string(),
                        *n,
                        1,
                        15,
                    ));
                }
            }
            Instruction::MovUpW(n) => {
                if !(*n >= 2 && *n <= 3) {
                    return Some(MidenProgramError::InvalidParameter(
                        Instruction::MovUpW(*n).to_string(),
                        *n,
                        1,
                        3,
                    ));
                }
            }

            // Boolean
            Instruction::Xor | Instruction::And | Instruction::Or => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();

                    if a_int != 1 && a_int != 0 {
                        return Some(MidenProgramError::NotBinaryValue(a.as_int()));
                    } else if b_int != 1 && b_int != 0 {
                        return Some(MidenProgramError::NotBinaryValue(b.as_int()));
                    }
                }
            }

            Instruction::Not | Instruction::U32CheckedPopcnt => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    if a_int != 1 && a_int != 0 {
                        return Some(MidenProgramError::NotBinaryValue(a.as_int()));
                    }
                }
            }

            Instruction::U32CheckedAdd => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();
                    let c_int = a_int + b_int;

                    if a_int >= U32_MAX || b_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(max(a_int, b_int)));
                    } else if c_int >= U32_MAX {
                        return Some(MidenProgramError::U32Overflow(c_int));
                    }
                }
            }

            Instruction::U32CheckedAddImm(b) => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    let b_int = *b as u64;
                    let c_int = a_int + b_int;

                    if a_int >= U32_MAX || b_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(max(a_int, b_int)));
                    } else if c_int >= U32_MAX {
                        return Some(MidenProgramError::U32Overflow(c_int));
                    }
                }
            }

            Instruction::U32CheckedSub => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();

                    if a_int < b_int {
                        return Some(MidenProgramError::U32InvalidSubtraction(a_int, b_int));
                    } else if a_int >= U32_MAX || b_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(if a_int < b_int {
                            a_int
                        } else {
                            max(a_int, b_int)
                        }));
                    }
                }
            }

            Instruction::U32CheckedSubImm(b) => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    let b_int = *b as u64;

                    if a_int < b_int {
                        return Some(MidenProgramError::U32InvalidSubtraction(a_int, b_int));
                    } else if a_int >= U32_MAX || b_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(if a_int < b_int {
                            a_int
                        } else {
                            max(a_int, b_int)
                        }));
                    }
                }
            }

            Instruction::U32CheckedMul => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();
                    let c_int = a_int * b_int;

                    if a_int >= U32_MAX || b_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(max(a_int, b_int)));
                    } else if c_int >= U32_MAX {
                        return Some(MidenProgramError::U32Overflow(c_int));
                    }
                }
            }

            Instruction::U32CheckedMulImm(b) => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    let b_int = *b as u64;
                    let c_int = a_int * b_int;

                    if a_int >= U32_MAX || b_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(max(a_int, b_int)));
                    } else if c_int >= U32_MAX {
                        return Some(MidenProgramError::U32Overflow(c_int));
                    }
                }
            }

            Instruction::U32CheckedDiv => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();

                    if b_int == 0 {
                        return Some(MidenProgramError::DivideByZero);
                    } else if a_int >= U32_MAX || b_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(if b_int == 0 {
                            b_int
                        } else {
                            max(a_int, b_int)
                        }));
                    }
                }
            }

            Instruction::U32CheckedDivImm(b) => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    let b_int = *b as u64;

                    if b_int == 0 {
                        return Some(MidenProgramError::InvalidParameter(
                            Instruction::U32CheckedDivImm(*b).to_string(),
                            *b as usize,
                            1,
                            U32_MAX as usize,
                        ));
                    } else if a_int >= U32_MAX || b_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(if b_int == 0 {
                            b_int
                        } else {
                            max(a_int, b_int)
                        }));
                    }
                }
            }

            Instruction::U32CheckedMod => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();

                    if b_int == 0 {
                        return Some(MidenProgramError::ModulusByZero);
                    } else if a_int >= U32_MAX || b_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(max(a_int, b_int)));
                    }
                }
            }

            Instruction::U32CheckedModImm(b) => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    let b_int = *b as u64;

                    if b_int == 0 {
                        return Some(MidenProgramError::InvalidParameter(
                            Instruction::U32CheckedModImm(*b).to_string(),
                            *b as usize,
                            1,
                            U32_MAX as usize,
                        ));
                    } else if a_int >= U32_MAX || b_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(max(a_int, b_int)));
                    }
                }
            }

            Instruction::U32CheckedDivMod => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();

                    if b_int == 0 {
                        return Some(MidenProgramError::DivModByZero);
                    } else if a_int >= U32_MAX || b_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(max(a_int, b_int)));
                    }
                }
            }

            Instruction::U32CheckedDivModImm(b) => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    let b_int = *b as u64;

                    if b_int == 0 {
                        return Some(MidenProgramError::InvalidParameter(
                            Instruction::U32CheckedDivModImm(*b).to_string(),
                            *b as usize,
                            1,
                            U32_MAX as usize,
                        ));
                    } else if a_int >= U32_MAX || b_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(max(a_int, b_int)));
                    }
                }
            }

            Instruction::U32CheckedAnd | Instruction::U32CheckedOr | Instruction::U32CheckedXor => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();

                    if a_int >= U32_MAX || b_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(max(a_int, b_int)));
                    }
                }
            }

            Instruction::U32CheckedNot => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();

                    if a_int >= U32_MAX {}
                }
            }

            Instruction::U32CheckedShl => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();

                    if b_int > 31 {
                        return Some(MidenProgramError::TopValueInvalid(
                            Instruction::U32CheckedShl.to_string(),
                            b_int as usize,
                            0,
                            30,
                        ));
                    } else if a_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(max(a_int, b_int)));
                    }
                }
            }

            Instruction::U32CheckedShlImm(b) => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    let b_int = *b as u64;

                    if b_int > 31 {
                        return Some(MidenProgramError::InvalidParameter(
                            Instruction::U32CheckedShlImm(*b).to_string(),
                            b_int as usize,
                            0,
                            30,
                        ));
                    } else if a_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(max(a_int, b_int)));
                    }
                }
            }
            Instruction::U32CheckedShr => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();

                    if b_int > 31 {
                        return Some(MidenProgramError::TopValueInvalid(
                            Instruction::U32CheckedShr.to_string(),
                            b_int as usize,
                            0,
                            30,
                        ));
                    } else if a_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(max(a_int, b_int)));
                    }
                }
            }
            Instruction::U32CheckedShrImm(b) => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    let b_int = *b as u64;

                    if b_int > 31 {
                        return Some(MidenProgramError::InvalidParameter(
                            Instruction::U32CheckedShrImm(*b).to_string(),
                            b_int as usize,
                            0,
                            30,
                        ));
                    } else if a_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(max(a_int, b_int)));
                    }
                }
            }
            Instruction::U32CheckedRotr => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();

                    if b_int > 31 {
                        return Some(MidenProgramError::TopValueInvalid(
                            Instruction::U32CheckedRotr.to_string(),
                            b_int as usize,
                            0,
                            30,
                        ));
                    } else if a_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(max(a_int, b_int)));
                    }
                }
            }
            Instruction::U32CheckedRotrImm(b) => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    let b_int = *b as u64;

                    if b_int > 31 {
                        return Some(MidenProgramError::InvalidParameter(
                            Instruction::U32CheckedRotrImm(*b).to_string(),
                            b_int as usize,
                            0,
                            30,
                        ));
                    } else if a_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(max(a_int, b_int)));
                    }
                }
            }
            Instruction::U32CheckedRotl => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();

                    if b_int > 31 {
                        return Some(MidenProgramError::TopValueInvalid(
                            Instruction::U32CheckedRotl.to_string(),
                            b_int as usize,
                            0,
                            30,
                        ));
                    } else if a_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(max(a_int, b_int)));
                    }
                }
            }
            Instruction::U32CheckedRotlImm(b) => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    let b_int = *b as u64;

                    if b_int > 31 {
                        return Some(MidenProgramError::InvalidParameter(
                            Instruction::U32CheckedRotlImm(*b).to_string(),
                            b_int as usize,
                            0,
                            30,
                        ));
                    } else if a_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(max(a_int, b_int)));
                    }
                }
            }

            _ => {}
        }

        None
    }
}
