use math::{fields::f64::BaseElement, StarkField};

use crate::{Instruction, MidenProgram};

use super::utils::U32_MAX;

pub fn execute_u32_bitwise(program: &mut MidenProgram, operand: &Instruction) {
    match operand {
        Instruction::U32CheckedAnd => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                program
                    .stack
                    .push_front(BaseElement::from(a_int as u32 & b_int as u32));
            }
        }

        Instruction::U32CheckedOr => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                program
                    .stack
                    .push_front(BaseElement::from(a_int as u32 | b_int as u32));
            }
        }

        Instruction::U32CheckedXor => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                program
                    .stack
                    .push_front(BaseElement::from(a_int as u32 ^ b_int as u32));
            }
        }

        Instruction::U32CheckedNot => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();

                program.stack.push_front(BaseElement::from(!a_int as u32));
            }
        }

        Instruction::U32CheckedShl => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                program.stack.push_front(BaseElement::from(
                    (a_int * 2_i32.pow(b_int as u32) as u64) % U32_MAX,
                ));
            }
        }

        Instruction::U32CheckedShlImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();

                program
                    .stack
                    .push_front(BaseElement::from((a_int * 2_i32.pow(*b) as u64) % U32_MAX));
            }
        }

        Instruction::U32UncheckedShl => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                program.stack.push_front(BaseElement::from(
                    (a_int * 2_i32.pow(b_int as u32) as u64) % U32_MAX,
                ));
            }
        }

        Instruction::U32UncheckedShlImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();

                program
                    .stack
                    .push_front(BaseElement::from((a_int * 2_i32.pow(*b) as u64) % U32_MAX));
            }
        }

        Instruction::U32CheckedShr => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                program.stack.push_front(BaseElement::from(
                    (a_int / 2_i32.pow(b_int as u32) as u64) % U32_MAX,
                ));
            }
        }

        Instruction::U32CheckedShrImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();

                program
                    .stack
                    .push_front(BaseElement::from((a_int / 2_i32.pow(*b) as u64) % U32_MAX));
            }
        }

        Instruction::U32UncheckedShr => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                program.stack.push_front(BaseElement::from(
                    (a_int / 2_i32.pow(b_int as u32) as u64) % U32_MAX,
                ));
            }
        }

        Instruction::U32UncheckedShrImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();

                program
                    .stack
                    .push_front(BaseElement::from((a_int / 2_i32.pow(*b) as u64) % U32_MAX));
            }
        }

        Instruction::U32CheckedRotr => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                program
                    .stack
                    .push_front(BaseElement::from((a_int as u32).rotate_right(b_int as u32)));
            }
        }

        Instruction::U32CheckedRotrImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();
                let b_int = *b as u32;

                program
                    .stack
                    .push_front(BaseElement::from((a_int as u32).rotate_right(b_int)));
            }
        }

        Instruction::U32UncheckedRotr => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int() as u32;
                let b_int = b.as_int() as u32;

                program
                    .stack
                    .push_front(BaseElement::from((a_int as u32).rotate_right(b_int as u32)));
            }
        }

        Instruction::U32UncheckedRotrImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int() as u32;
                let b_int = *b as u32;

                program
                    .stack
                    .push_front(BaseElement::from((a_int as u32).rotate_right(b_int)));
            }
        }

        Instruction::U32CheckedRotl => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                program
                    .stack
                    .push_front(BaseElement::from((a_int as u32).rotate_left(b_int as u32)));
            }
        }

        Instruction::U32CheckedRotlImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();
                let b_int = *b as u32;

                program
                    .stack
                    .push_front(BaseElement::from((a_int as u32).rotate_left(b_int)));
            }
        }

        Instruction::U32UncheckedRotl => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                program
                    .stack
                    .push_front(BaseElement::from((a_int as u32).rotate_left(b_int as u32)));
            }
        }

        Instruction::U32UncheckedRotlImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();
                let b_int = *b as u32;

                program
                    .stack
                    .push_front(BaseElement::from((a_int as u32).rotate_left(b_int)));
            }
        }

        Instruction::U32CheckedPopcnt => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();

                program
                    .stack
                    .push_front(BaseElement::from(((a_int as u32) as u32).count_ones()));
            }
        }

        Instruction::U32UncheckedPopcnt => {
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
