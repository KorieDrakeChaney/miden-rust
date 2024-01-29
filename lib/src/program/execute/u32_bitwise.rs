use math::{fields::f64::BaseElement, StarkField};

use crate::{program::error::MidenProgramError, MidenProgram, Operand};

use super::utils::U32_MAX;

pub fn execute_u32_bitwise(program: &mut MidenProgram, operand: &Operand) {
    match operand {
        Operand::U32CheckedAnd => {
            if let (Some(b), Some(a)) = (program.stack.get(0), program.stack.get(1)) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                if a_int >= U32_MAX {
                    program.add_operand(Operand::Error(MidenProgramError::NotU32Value(a_int)));
                } else if b_int >= U32_MAX {
                    program.add_operand(Operand::Error(MidenProgramError::NotU32Value(b_int)));
                } else {
                    program.stack.pop_front();
                    program.stack.pop_front();

                    program
                        .stack
                        .push_front(BaseElement::from(a_int as u32 & b_int as u32));
                }
            }
        }

        Operand::U32CheckedOr => {
            if let (Some(b), Some(a)) = (program.stack.get(0), program.stack.get(1)) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                if a_int >= U32_MAX {
                    program.add_operand(Operand::Error(MidenProgramError::NotU32Value(a_int)));
                } else if b_int >= U32_MAX {
                    program.add_operand(Operand::Error(MidenProgramError::NotU32Value(b_int)));
                } else {
                    program.stack.pop_front();
                    program.stack.pop_front();

                    program
                        .stack
                        .push_front(BaseElement::from(a_int as u32 | b_int as u32));
                }
            }
        }

        Operand::U32CheckedXor => {
            if let (Some(b), Some(a)) = (program.stack.get(0), program.stack.get(1)) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                if a_int >= U32_MAX {
                    program.add_operand(Operand::Error(MidenProgramError::NotU32Value(a_int)));
                } else if b_int >= U32_MAX {
                    program.add_operand(Operand::Error(MidenProgramError::NotU32Value(b_int)));
                } else {
                    program.stack.pop_front();
                    program.stack.pop_front();

                    program
                        .stack
                        .push_front(BaseElement::from(a_int as u32 ^ b_int as u32));
                }
            }
        }

        Operand::U32CheckedNot => {
            if let Some(a) = program.stack.get(0) {
                let a_int = a.as_int();

                if a_int >= U32_MAX {
                    program.add_operand(Operand::Error(MidenProgramError::NotU32Value(a_int)));
                } else {
                    program.stack.pop_front();

                    program.stack.push_front(BaseElement::from(!a_int as u32));
                }
            }
        }

        Operand::U32CheckedShl => {
            if let (Some(b), Some(a)) = (program.stack.get(0), program.stack.get(1)) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                if b_int > 31 {
                    program.add_operand(Operand::Error(MidenProgramError::TopValueInvalid(
                        Operand::U32CheckedShl.to_string(),
                        b_int as usize,
                        0,
                        30,
                    )));
                } else if a_int >= U32_MAX {
                    program.add_operand(Operand::Error(MidenProgramError::NotU32Value(a_int)));
                } else if b_int >= U32_MAX {
                    program.add_operand(Operand::Error(MidenProgramError::NotU32Value(b_int)));
                } else {
                    program.stack.pop_front();
                    program.stack.pop_front();

                    program.stack.push_front(BaseElement::from(
                        (a_int * 2_i32.pow(b_int as u32) as u64) % U32_MAX,
                    ));
                }
            }
        }

        Operand::U32CheckedShlImm(b) => {
            if let Some(a) = program.stack.get(0) {
                let a_int = a.as_int();
                let b_int = *b as u64;

                if b_int > 31 {
                    program.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                        Operand::U32CheckedShlImm(*b).to_string(),
                        b_int as usize,
                        0,
                        30,
                    )));
                } else if a_int >= U32_MAX {
                    program.add_operand(Operand::Error(MidenProgramError::NotU32Value(a_int)));
                } else {
                    program.stack.pop_front();

                    program
                        .stack
                        .push_front(BaseElement::from((a_int * 2_i32.pow(*b) as u64) % U32_MAX));
                }
            }
        }

        Operand::U32UncheckedShl => {
            if let (Some(b), Some(a)) = (program.stack.get(0), program.stack.get(1)) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                if b_int > 31 {
                    program.add_operand(Operand::Error(MidenProgramError::TopValueInvalid(
                        Operand::U32UncheckedShl.to_string(),
                        b_int as usize,
                        0,
                        30,
                    )));
                } else {
                    program.stack.pop_front();
                    program.stack.pop_front();

                    program.stack.push_front(BaseElement::from(
                        (a_int * 2_i32.pow(b_int as u32) as u64) % U32_MAX,
                    ));
                }
            }
        }

        Operand::U32UncheckedShlImm(b) => {
            if let Some(a) = program.stack.get(0) {
                let a_int = a.as_int();
                let b_int = *b as u64;

                if b_int > 31 {
                    program.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                        Operand::U32UncheckedShlImm(*b).to_string(),
                        b_int as usize,
                        0,
                        30,
                    )));
                } else {
                    program.stack.pop_front();

                    program
                        .stack
                        .push_front(BaseElement::from((a_int * 2_i32.pow(*b) as u64) % U32_MAX));
                }
            }
        }

        Operand::U32CheckedShr => {
            if let (Some(b), Some(a)) = (program.stack.get(0), program.stack.get(1)) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                if b_int > 31 {
                    program.add_operand(Operand::Error(MidenProgramError::TopValueInvalid(
                        Operand::U32CheckedShr.to_string(),
                        b_int as usize,
                        0,
                        30,
                    )));
                } else if a_int >= U32_MAX {
                    program.add_operand(Operand::Error(MidenProgramError::NotU32Value(a_int)));
                } else {
                    program.stack.pop_front();
                    program.stack.pop_front();

                    program.stack.push_front(BaseElement::from(
                        (a_int / 2_i32.pow(b_int as u32) as u64) % U32_MAX,
                    ));
                }
            }
        }

        Operand::U32CheckedShrImm(b) => {
            if let Some(a) = program.stack.get(0) {
                let a_int = a.as_int();
                let b_int = *b as u64;

                if b_int > 31 {
                    program.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                        Operand::U32CheckedShrImm(*b).to_string(),
                        b_int as usize,
                        0,
                        30,
                    )));
                } else if a_int >= U32_MAX {
                    program.add_operand(Operand::Error(MidenProgramError::NotU32Value(a_int)));
                } else {
                    program.stack.pop_front();

                    program
                        .stack
                        .push_front(BaseElement::from((a_int / 2_i32.pow(*b) as u64) % U32_MAX));
                }
            }
        }

        Operand::U32UncheckedShr => {
            if let (Some(b), Some(a)) = (program.stack.get(0), program.stack.get(1)) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                if b_int > 31 {
                    program.add_operand(Operand::Error(MidenProgramError::TopValueInvalid(
                        Operand::U32UncheckedShr.to_string(),
                        b_int as usize,
                        0,
                        30,
                    )));
                } else {
                    program.stack.pop_front();
                    program.stack.pop_front();

                    program.stack.push_front(BaseElement::from(
                        (a_int / 2_i32.pow(b_int as u32) as u64) % U32_MAX,
                    ));
                }
            }
        }

        Operand::U32UncheckedShrImm(b) => {
            if let Some(a) = program.stack.get(0) {
                let a_int = a.as_int();
                let b_int = *b as u64;

                if b_int > 31 {
                    program.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                        Operand::U32UncheckedShrImm(*b).to_string(),
                        b_int as usize,
                        0,
                        30,
                    )));
                } else {
                    program.stack.pop_front();
                    program
                        .stack
                        .push_front(BaseElement::from((a_int / 2_i32.pow(*b) as u64) % U32_MAX));
                }
            }
        }

        Operand::U32CheckedRotr => {
            if let (Some(b), Some(a)) = (program.stack.get(0), program.stack.get(1)) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                if b_int > 31 {
                    program.add_operand(Operand::Error(MidenProgramError::TopValueInvalid(
                        Operand::U32CheckedRotr.to_string(),
                        b_int as usize,
                        0,
                        30,
                    )));
                } else if a_int >= U32_MAX {
                    program.add_operand(Operand::Error(MidenProgramError::NotU32Value(a_int)));
                } else {
                    program.stack.pop_front();
                    program.stack.pop_front();

                    program
                        .stack
                        .push_front(BaseElement::from((a_int as u32).rotate_right(b_int as u32)));
                }
            }
        }

        Operand::U32CheckedRotrImm(b) => {
            if let Some(a) = program.stack.get(0) {
                let a_int = a.as_int();
                let b_int = *b as u32;

                if b_int > 31 {
                    program.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                        Operand::U32CheckedRotrImm(*b).to_string(),
                        b_int as usize,
                        0,
                        30,
                    )));
                } else if a_int >= U32_MAX {
                    program.add_operand(Operand::Error(MidenProgramError::NotU32Value(a_int)));
                } else {
                    program.stack.pop_front();

                    program
                        .stack
                        .push_front(BaseElement::from((a_int as u32).rotate_right(b_int)));
                }
            }
        }

        Operand::U32UncheckedRotr => {
            if let (Some(b), Some(a)) = (program.stack.get(0), program.stack.get(1)) {
                let a_int = a.as_int() as u32;
                let b_int = b.as_int() as u32;

                if b_int > 31 {
                    program.add_operand(Operand::Error(MidenProgramError::TopValueInvalid(
                        Operand::U32UncheckedRotr.to_string(),
                        b_int as usize,
                        0,
                        30,
                    )));
                } else {
                    program.stack.pop_front();
                    program.stack.pop_front();

                    program
                        .stack
                        .push_front(BaseElement::from((a_int as u32).rotate_right(b_int as u32)));
                }
            }
        }

        Operand::U32UncheckedRotrImm(b) => {
            if let Some(a) = program.stack.get(0) {
                let a_int = a.as_int() as u32;
                let b_int = *b as u32;

                if b_int > 31 {
                    program.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                        Operand::U32UncheckedRotrImm(*b).to_string(),
                        b_int as usize,
                        0,
                        30,
                    )));
                } else {
                    program.stack.pop_front();
                    program
                        .stack
                        .push_front(BaseElement::from((a_int as u32).rotate_right(b_int)));
                }
            }
        }

        Operand::U32CheckedRotl => {
            if let (Some(b), Some(a)) = (program.stack.get(0), program.stack.get(1)) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                if b_int > 31 {
                    program.add_operand(Operand::Error(MidenProgramError::TopValueInvalid(
                        Operand::U32CheckedRotl.to_string(),
                        b_int as usize,
                        0,
                        30,
                    )));
                } else if a_int >= U32_MAX {
                    program.add_operand(Operand::Error(MidenProgramError::NotU32Value(a_int)));
                } else {
                    program.stack.pop_front();
                    program.stack.pop_front();

                    program
                        .stack
                        .push_front(BaseElement::from((a_int as u32).rotate_left(b_int as u32)));
                }
            }
        }

        Operand::U32CheckedRotlImm(b) => {
            if let Some(a) = program.stack.get(0) {
                let a_int = a.as_int();
                let b_int = *b as u32;

                if b_int > 31 {
                    program.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                        Operand::U32CheckedRotlImm(*b).to_string(),
                        b_int as usize,
                        0,
                        30,
                    )));
                } else if a_int >= U32_MAX {
                    program.add_operand(Operand::Error(MidenProgramError::NotU32Value(a_int)));
                } else {
                    program.stack.pop_front();

                    program
                        .stack
                        .push_front(BaseElement::from((a_int as u32).rotate_left(b_int)));
                }
            }
        }

        Operand::U32UncheckedRotl => {
            if let (Some(b), Some(a)) = (program.stack.get(0), program.stack.get(1)) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                if b_int > 31 {
                    program.add_operand(Operand::Error(MidenProgramError::TopValueInvalid(
                        Operand::U32UncheckedRotl.to_string(),
                        b_int as usize,
                        0,
                        30,
                    )));
                } else {
                    program.stack.pop_front();
                    program.stack.pop_front();

                    program
                        .stack
                        .push_front(BaseElement::from((a_int as u32).rotate_left(b_int as u32)));
                }
            }
        }

        Operand::U32UncheckedRotlImm(b) => {
            if let Some(a) = program.stack.get(0) {
                let a_int = a.as_int();
                let b_int = *b as u32;

                if b_int > 31 {
                    program.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                        Operand::U32UncheckedRotlImm(*b).to_string(),
                        b_int as usize,
                        0,
                        30,
                    )));
                } else {
                    program.stack.pop_front();
                    program.stack.pop_front();
                    program
                        .stack
                        .push_front(BaseElement::from((a_int as u32).rotate_left(b_int)));
                }
            }
        }

        Operand::U32CheckedPopcnt => {
            if let Some(a) = program.stack.get(0) {
                let a_int = a.as_int();
                if a_int >= U32_MAX {
                    program.add_operand(Operand::Error(MidenProgramError::NotU32Value(a_int)));
                } else {
                    program.stack.pop_front();
                    program
                        .stack
                        .push_front(BaseElement::from(((a_int as u32) as u32).count_ones()));
                }
            }
        }

        Operand::U32UncheckedPopcnt => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();
                program
                    .stack
                    .push_front(BaseElement::from(((a_int as u32) as u32).count_ones()));
            }
        }

        _ => {}
    }
}
