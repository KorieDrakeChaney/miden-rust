use math::StarkField;

use crate::{program::error::MidenProgramError, MidenProgram, Operand};

use super::utils::{max, U32_MAX};

impl MidenProgram {
    pub fn is_valid_operand(&mut self, operand: &Operand) -> Option<MidenProgramError> {
        match operand {
            Operand::AdvPush(n) => {
                if !(*n >= 1 && *n <= 16) {
                    return Some(MidenProgramError::InvalidParameter(
                        Operand::AdvPush(*n).to_string(),
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
            Operand::Dup(n) => {
                if !(*n <= 15) {
                    return Some(MidenProgramError::InvalidParameter(
                        Operand::Dup(*n).to_string(),
                        *n,
                        0,
                        15,
                    ));
                }
            }
            Operand::Swap(n) => {
                if !(*n > 0 && *n <= 15) {
                    return Some(MidenProgramError::InvalidParameter(
                        Operand::Swap(*n).to_string(),
                        *n,
                        1,
                        15,
                    ));
                }
            }
            Operand::SwapW(n) => {
                if !(*n > 0 && *n <= 3) {
                    return Some(MidenProgramError::InvalidParameter(
                        Operand::Swap(*n).to_string(),
                        *n,
                        1,
                        3,
                    ));
                }
            }
            Operand::MovDn(n) => {
                if !(*n >= 2 && *n <= 15) {
                    return Some(MidenProgramError::InvalidParameter(
                        Operand::MovDn(*n).to_string(),
                        *n,
                        2,
                        3,
                    ));
                }
            }
            Operand::MovDnW(n) => {
                if !(*n >= 2 && *n <= 3) {
                    return Some(MidenProgramError::InvalidParameter(
                        Operand::MovDnW(*n).to_string(),
                        *n,
                        2,
                        15,
                    ));
                }
            }
            Operand::MovUp(n) => {
                if !(*n >= 2 && *n <= 15) {
                    return Some(MidenProgramError::InvalidParameter(
                        Operand::MovUp(*n).to_string(),
                        *n,
                        1,
                        15,
                    ));
                }
            }
            Operand::MovUpW(n) => {
                if !(*n >= 2 && *n <= 3) {
                    return Some(MidenProgramError::InvalidParameter(
                        Operand::MovUpW(*n).to_string(),
                        *n,
                        1,
                        3,
                    ));
                }
            }

            // Boolean
            Operand::Xor | Operand::And | Operand::Or => {
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

            Operand::Not | Operand::U32CheckedPopcnt => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    if a_int != 1 && a_int != 0 {
                        return Some(MidenProgramError::NotBinaryValue(a.as_int()));
                    }
                }
            }

            Operand::U32CheckedAdd => {
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

            Operand::U32CheckedAddImm(b) => {
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

            Operand::U32CheckedSub => {
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

            Operand::U32CheckedSubImm(b) => {
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

            Operand::U32CheckedMul => {
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

            Operand::U32CheckedMulImm(b) => {
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

            Operand::U32CheckedDiv => {
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

            Operand::U32CheckedDivImm(b) => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    let b_int = *b as u64;

                    if b_int == 0 {
                        return Some(MidenProgramError::InvalidParameter(
                            Operand::U32CheckedDivImm(*b).to_string(),
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

            Operand::U32CheckedMod => {
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

            Operand::U32CheckedModImm(b) => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    let b_int = *b as u64;

                    if b_int == 0 {
                        return Some(MidenProgramError::InvalidParameter(
                            Operand::U32CheckedModImm(*b).to_string(),
                            *b as usize,
                            1,
                            U32_MAX as usize,
                        ));
                    } else if a_int >= U32_MAX || b_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(max(a_int, b_int)));
                    }
                }
            }

            Operand::U32CheckedDivMod => {
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

            Operand::U32CheckedDivModImm(b) => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    let b_int = *b as u64;

                    if b_int == 0 {
                        return Some(MidenProgramError::InvalidParameter(
                            Operand::U32CheckedDivModImm(*b).to_string(),
                            *b as usize,
                            1,
                            U32_MAX as usize,
                        ));
                    } else if a_int >= U32_MAX || b_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(max(a_int, b_int)));
                    }
                }
            }

            Operand::U32CheckedAnd | Operand::U32CheckedOr | Operand::U32CheckedXor => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();

                    if a_int >= U32_MAX || b_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(max(a_int, b_int)));
                    }
                }
            }

            Operand::U32CheckedNot => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();

                    if a_int >= U32_MAX {}
                }
            }

            Operand::U32CheckedShl => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();

                    if b_int > 31 {
                        return Some(MidenProgramError::TopValueInvalid(
                            Operand::U32CheckedShl.to_string(),
                            b_int as usize,
                            0,
                            30,
                        ));
                    } else if a_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(max(a_int, b_int)));
                    }
                }
            }

            Operand::U32CheckedShlImm(b) => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    let b_int = *b as u64;

                    if b_int > 31 {
                        return Some(MidenProgramError::InvalidParameter(
                            Operand::U32CheckedShlImm(*b).to_string(),
                            b_int as usize,
                            0,
                            30,
                        ));
                    } else if a_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(max(a_int, b_int)));
                    }
                }
            }
            Operand::U32CheckedShr => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();

                    if b_int > 31 {
                        return Some(MidenProgramError::TopValueInvalid(
                            Operand::U32CheckedShr.to_string(),
                            b_int as usize,
                            0,
                            30,
                        ));
                    } else if a_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(max(a_int, b_int)));
                    }
                }
            }
            Operand::U32CheckedShrImm(b) => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    let b_int = *b as u64;

                    if b_int > 31 {
                        return Some(MidenProgramError::InvalidParameter(
                            Operand::U32CheckedShrImm(*b).to_string(),
                            b_int as usize,
                            0,
                            30,
                        ));
                    } else if a_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(max(a_int, b_int)));
                    }
                }
            }
            Operand::U32CheckedRotr => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();

                    if b_int > 31 {
                        return Some(MidenProgramError::TopValueInvalid(
                            Operand::U32CheckedRotr.to_string(),
                            b_int as usize,
                            0,
                            30,
                        ));
                    } else if a_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(max(a_int, b_int)));
                    }
                }
            }
            Operand::U32CheckedRotrImm(b) => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    let b_int = *b as u64;

                    if b_int > 31 {
                        return Some(MidenProgramError::InvalidParameter(
                            Operand::U32CheckedRotrImm(*b).to_string(),
                            b_int as usize,
                            0,
                            30,
                        ));
                    } else if a_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(max(a_int, b_int)));
                    }
                }
            }
            Operand::U32CheckedRotl => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();

                    if b_int > 31 {
                        return Some(MidenProgramError::TopValueInvalid(
                            Operand::U32CheckedRotl.to_string(),
                            b_int as usize,
                            0,
                            30,
                        ));
                    } else if a_int >= U32_MAX {
                        return Some(MidenProgramError::NotU32Value(max(a_int, b_int)));
                    }
                }
            }
            Operand::U32CheckedRotlImm(b) => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    let b_int = *b as u64;

                    if b_int > 31 {
                        return Some(MidenProgramError::InvalidParameter(
                            Operand::U32CheckedRotlImm(*b).to_string(),
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
