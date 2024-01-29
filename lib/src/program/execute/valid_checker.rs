use math::StarkField;

use crate::{program::error::MidenProgramError, MidenProgram, Operand};

use super::utils::{max, U32_MAX};

impl MidenProgram {
    pub fn is_valid(&mut self, operand: &Operand) -> bool {
        match operand {
            // Manipulation
            Operand::Dup(n) => {
                if *n <= 15 {
                    true
                } else {
                    self.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                        Operand::Dup(*n).to_string(),
                        *n,
                        0,
                        15,
                    )));
                    false
                }
            }
            Operand::Swap(n) => {
                if *n > 0 && *n <= 15 {
                    true
                } else {
                    self.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                        Operand::Swap(*n).to_string(),
                        *n,
                        1,
                        15,
                    )));
                    false
                }
            }
            Operand::SwapW(n) => {
                if *n > 0 && *n <= 3 {
                    true
                } else {
                    self.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                        Operand::Swap(*n).to_string(),
                        *n,
                        1,
                        3,
                    )));
                    false
                }
            }
            Operand::MovDn(n) => {
                if *n >= 2 && *n <= 15 {
                    true
                } else {
                    self.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                        Operand::MovDn(*n).to_string(),
                        *n,
                        2,
                        3,
                    )));
                    false
                }
            }
            Operand::MovDnW(n) => {
                if *n >= 2 && *n <= 3 {
                    true
                } else {
                    self.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                        Operand::MovDnW(*n).to_string(),
                        *n,
                        2,
                        15,
                    )));
                    false
                }
            }
            Operand::MovUp(n) => {
                if *n >= 2 && *n <= 15 {
                    true
                } else {
                    self.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                        Operand::MovUp(*n).to_string(),
                        *n,
                        1,
                        15,
                    )));
                    false
                }
            }
            Operand::MovUpW(n) => {
                if *n >= 2 && *n <= 3 {
                    true
                } else {
                    self.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                        Operand::MovUpW(*n).to_string(),
                        *n,
                        1,
                        3,
                    )));
                    false
                }
            }

            // Boolean
            Operand::Xor | Operand::And | Operand::Or => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();

                    if a_int != 1 && a_int != 0 {
                        self.add_operand(Operand::Error(MidenProgramError::NotBinaryValue(
                            a.as_int(),
                        )));

                        return false;
                    } else if b_int != 1 && b_int != 0 {
                        self.add_operand(Operand::Error(MidenProgramError::NotBinaryValue(
                            b.as_int(),
                        )));
                        return false;
                    }

                    true
                } else {
                    false
                }
            }

            Operand::Not | Operand::U32CheckedPopcnt => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    if a_int != 1 && a_int != 0 {
                        self.add_operand(Operand::Error(MidenProgramError::NotBinaryValue(
                            a.as_int(),
                        )));

                        return false;
                    }
                    true
                } else {
                    false
                }
            }

            Operand::U32CheckedAdd => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();
                    let c_int = a_int + b_int;

                    if a_int >= U32_MAX || b_int >= U32_MAX {
                        self.add_operand(Operand::Error(MidenProgramError::NotU32Value(max(
                            a_int, b_int,
                        ))));
                        return false;
                    } else if c_int >= U32_MAX {
                        self.add_operand(Operand::Error(MidenProgramError::U32Overflow(c_int)));
                        return false;
                    }
                    true
                } else {
                    false
                }
            }

            Operand::U32CheckedAddImm(b) => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    let b_int = *b as u64;
                    let c_int = a_int + b_int;

                    if a_int >= U32_MAX || b_int >= U32_MAX {
                        self.add_operand(Operand::Error(MidenProgramError::NotU32Value(max(
                            a_int, b_int,
                        ))));
                        return false;
                    } else if c_int >= U32_MAX {
                        self.add_operand(Operand::Error(MidenProgramError::U32Overflow(c_int)));
                        return false;
                    }
                    true
                } else {
                    false
                }
            }

            Operand::U32CheckedSub => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();

                    if a_int < b_int {
                        self.add_operand(Operand::Error(MidenProgramError::U32InvalidSubtraction(
                            a_int, b_int,
                        )))
                    } else if a_int >= U32_MAX || b_int >= U32_MAX {
                        self.add_operand(Operand::Error(MidenProgramError::NotU32Value(
                            if a_int < b_int {
                                a_int
                            } else {
                                max(a_int, b_int)
                            },
                        )));
                        return false;
                    }
                    true
                } else {
                    false
                }
            }

            Operand::U32CheckedSubImm(b) => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    let b_int = *b as u64;

                    if a_int < b_int {
                        self.add_operand(Operand::Error(MidenProgramError::U32InvalidSubtraction(
                            a_int, b_int,
                        )))
                    } else if a_int >= U32_MAX || b_int >= U32_MAX {
                        self.add_operand(Operand::Error(MidenProgramError::NotU32Value(
                            if a_int < b_int {
                                a_int
                            } else {
                                max(a_int, b_int)
                            },
                        )));
                        return false;
                    }
                    true
                } else {
                    false
                }
            }

            Operand::U32CheckedMul => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();
                    let c_int = a_int * b_int;

                    if a_int >= U32_MAX || b_int >= U32_MAX {
                        self.add_operand(Operand::Error(MidenProgramError::NotU32Value(max(
                            a_int, b_int,
                        ))));
                        return false;
                    } else if c_int >= U32_MAX {
                        self.add_operand(Operand::Error(MidenProgramError::U32Overflow(c_int)))
                    }
                    true
                } else {
                    false
                }
            }

            Operand::U32CheckedMulImm(b) => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    let b_int = *b as u64;
                    let c_int = a_int * b_int;

                    if a_int >= U32_MAX || b_int >= U32_MAX {
                        self.add_operand(Operand::Error(MidenProgramError::NotU32Value(max(
                            a_int, b_int,
                        ))));
                        return false;
                    } else if c_int >= U32_MAX {
                        self.add_operand(Operand::Error(MidenProgramError::U32Overflow(c_int)))
                    }
                    true
                } else {
                    false
                }
            }

            Operand::U32CheckedDiv => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();

                    if b_int == 0 {
                        self.add_operand(Operand::Error(MidenProgramError::DivideByZero));
                    } else if a_int >= U32_MAX || b_int >= U32_MAX {
                        self.add_operand(Operand::Error(MidenProgramError::NotU32Value(
                            if b_int == 0 { b_int } else { max(a_int, b_int) },
                        )));
                        return false;
                    }
                    true
                } else {
                    false
                }
            }

            Operand::U32CheckedDivImm(b) => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    let b_int = *b as u64;

                    if b_int == 0 {
                        self.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                            Operand::U32CheckedDivImm(*b).to_string(),
                            *b as usize,
                            1,
                            U32_MAX as usize,
                        )));
                    } else if a_int >= U32_MAX || b_int >= U32_MAX {
                        self.add_operand(Operand::Error(MidenProgramError::NotU32Value(
                            if b_int == 0 { b_int } else { max(a_int, b_int) },
                        )));
                        return false;
                    }
                    true
                } else {
                    false
                }
            }

            Operand::U32CheckedMod => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();

                    if b_int == 0 {
                        self.add_operand(Operand::Error(MidenProgramError::ModulusByZero));
                        return false;
                    } else if a_int >= U32_MAX || b_int >= U32_MAX {
                        self.add_operand(Operand::Error(MidenProgramError::NotU32Value(max(
                            a_int, b_int,
                        ))));
                        return false;
                    }
                    true
                } else {
                    false
                }
            }

            Operand::U32CheckedModImm(b) => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    let b_int = *b as u64;

                    if b_int == 0 {
                        self.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                            Operand::U32CheckedModImm(*b).to_string(),
                            *b as usize,
                            1,
                            U32_MAX as usize,
                        )));
                        return false;
                    } else if a_int >= U32_MAX || b_int >= U32_MAX {
                        self.add_operand(Operand::Error(MidenProgramError::NotU32Value(max(
                            a_int, b_int,
                        ))));
                        return false;
                    }
                    true
                } else {
                    false
                }
            }

            Operand::U32CheckedDivMod => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();

                    if b_int == 0 {
                        self.add_operand(Operand::Error(MidenProgramError::DivModByZero));
                        return false;
                    } else if a_int >= U32_MAX || b_int >= U32_MAX {
                        self.add_operand(Operand::Error(MidenProgramError::NotU32Value(max(
                            a_int, b_int,
                        ))));
                        return false;
                    }
                    true
                } else {
                    false
                }
            }

            Operand::U32CheckedDivModImm(b) => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    let b_int = *b as u64;

                    if b_int == 0 {
                        self.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                            Operand::U32CheckedDivModImm(*b).to_string(),
                            *b as usize,
                            1,
                            U32_MAX as usize,
                        )));
                        return false;
                    } else if a_int >= U32_MAX || b_int >= U32_MAX {
                        self.add_operand(Operand::Error(MidenProgramError::NotU32Value(max(
                            a_int, b_int,
                        ))));
                        return false;
                    }
                    true
                } else {
                    false
                }
            }

            Operand::U32CheckedAnd | Operand::U32CheckedOr | Operand::U32CheckedXor => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();

                    if a_int >= U32_MAX || b_int >= U32_MAX {
                        self.add_operand(Operand::Error(MidenProgramError::NotU32Value(max(
                            a_int, b_int,
                        ))));
                        return false;
                    }
                    true
                } else {
                    false
                }
            }

            Operand::U32CheckedNot => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();

                    if a_int >= U32_MAX {
                        return false;
                    }
                    true
                } else {
                    false
                }
            }

            Operand::U32CheckedShl => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();

                    if b_int > 31 {
                        self.add_operand(Operand::Error(MidenProgramError::TopValueInvalid(
                            Operand::U32CheckedShl.to_string(),
                            b_int as usize,
                            0,
                            30,
                        )));

                        return false;
                    } else if a_int >= U32_MAX {
                        self.add_operand(Operand::Error(MidenProgramError::NotU32Value(max(
                            a_int, b_int,
                        ))));
                        return false;
                    }
                    true
                } else {
                    false
                }
            }

            Operand::U32CheckedShlImm(b) => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    let b_int = *b as u64;

                    if b_int > 31 {
                        self.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                            Operand::U32CheckedShlImm(*b).to_string(),
                            b_int as usize,
                            0,
                            30,
                        )));

                        return false;
                    } else if a_int >= U32_MAX {
                        self.add_operand(Operand::Error(MidenProgramError::NotU32Value(max(
                            a_int, b_int,
                        ))));
                        return false;
                    }
                    true
                } else {
                    false
                }
            }
            Operand::U32CheckedShr => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();

                    if b_int > 31 {
                        self.add_operand(Operand::Error(MidenProgramError::TopValueInvalid(
                            Operand::U32CheckedShr.to_string(),
                            b_int as usize,
                            0,
                            30,
                        )));

                        return false;
                    } else if a_int >= U32_MAX {
                        self.add_operand(Operand::Error(MidenProgramError::NotU32Value(max(
                            a_int, b_int,
                        ))));
                        return false;
                    }
                    true
                } else {
                    false
                }
            }
            Operand::U32CheckedShrImm(b) => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    let b_int = *b as u64;

                    if b_int > 31 {
                        self.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                            Operand::U32CheckedShrImm(*b).to_string(),
                            b_int as usize,
                            0,
                            30,
                        )));

                        return false;
                    } else if a_int >= U32_MAX {
                        self.add_operand(Operand::Error(MidenProgramError::NotU32Value(max(
                            a_int, b_int,
                        ))));
                        return false;
                    }
                    true
                } else {
                    false
                }
            }
            Operand::U32CheckedRotr => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();

                    if b_int > 31 {
                        self.add_operand(Operand::Error(MidenProgramError::TopValueInvalid(
                            Operand::U32CheckedRotr.to_string(),
                            b_int as usize,
                            0,
                            30,
                        )));

                        return false;
                    } else if a_int >= U32_MAX {
                        self.add_operand(Operand::Error(MidenProgramError::NotU32Value(max(
                            a_int, b_int,
                        ))));
                        return false;
                    }
                    true
                } else {
                    false
                }
            }
            Operand::U32CheckedRotrImm(b) => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    let b_int = *b as u64;

                    if b_int > 31 {
                        self.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                            Operand::U32CheckedRotrImm(*b).to_string(),
                            b_int as usize,
                            0,
                            30,
                        )));

                        return false;
                    } else if a_int >= U32_MAX {
                        self.add_operand(Operand::Error(MidenProgramError::NotU32Value(max(
                            a_int, b_int,
                        ))));
                        return false;
                    }
                    true
                } else {
                    false
                }
            }
            Operand::U32CheckedRotl => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    let a_int = a.as_int();
                    let b_int = b.as_int();

                    if b_int > 31 {
                        self.add_operand(Operand::Error(MidenProgramError::TopValueInvalid(
                            Operand::U32CheckedRotl.to_string(),
                            b_int as usize,
                            0,
                            30,
                        )));

                        return false;
                    } else if a_int >= U32_MAX {
                        self.add_operand(Operand::Error(MidenProgramError::NotU32Value(max(
                            a_int, b_int,
                        ))));
                        return false;
                    }
                    true
                } else {
                    false
                }
            }
            Operand::U32CheckedRotlImm(b) => {
                if let Some(a) = self.stack.get(0) {
                    let a_int = a.as_int();
                    let b_int = *b as u64;

                    if b_int > 31 {
                        self.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                            Operand::U32CheckedRotlImm(*b).to_string(),
                            b_int as usize,
                            0,
                            30,
                        )));

                        return false;
                    } else if a_int >= U32_MAX {
                        self.add_operand(Operand::Error(MidenProgramError::NotU32Value(max(
                            a_int, b_int,
                        ))));
                        return false;
                    }
                    true
                } else {
                    false
                }
            }

            _ => true,
        }
    }
}
