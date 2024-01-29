use math::{fields::f64::BaseElement, StarkField};

use crate::{MidenProgram, Operand};

use super::utils::U32_MAX;

pub fn execute_u32_bitwise(program: &mut MidenProgram, operand: &Operand) {
    match operand {
        Operand::U32CheckedAnd => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                program
                    .stack
                    .push_front(BaseElement::from(a_int as u32 & b_int as u32));
            }
        }

        Operand::U32CheckedOr => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                program
                    .stack
                    .push_front(BaseElement::from(a_int as u32 | b_int as u32));
            }
        }

        Operand::U32CheckedXor => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                program
                    .stack
                    .push_front(BaseElement::from(a_int as u32 ^ b_int as u32));
            }
        }

        Operand::U32CheckedNot => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();

                program.stack.push_front(BaseElement::from(!a_int as u32));
            }
        }

        Operand::U32CheckedShl => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                program.stack.push_front(BaseElement::from(
                    (a_int * 2_i32.pow(b_int as u32) as u64) % U32_MAX,
                ));
            }
        }

        Operand::U32CheckedShlImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();

                program
                    .stack
                    .push_front(BaseElement::from((a_int * 2_i32.pow(*b) as u64) % U32_MAX));
            }
        }

        Operand::U32UncheckedShl => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                program.stack.push_front(BaseElement::from(
                    (a_int * 2_i32.pow(b_int as u32) as u64) % U32_MAX,
                ));
            }
        }

        Operand::U32UncheckedShlImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();

                program
                    .stack
                    .push_front(BaseElement::from((a_int * 2_i32.pow(*b) as u64) % U32_MAX));
            }
        }

        Operand::U32CheckedShr => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                program.stack.push_front(BaseElement::from(
                    (a_int / 2_i32.pow(b_int as u32) as u64) % U32_MAX,
                ));
            }
        }

        Operand::U32CheckedShrImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();

                program
                    .stack
                    .push_front(BaseElement::from((a_int / 2_i32.pow(*b) as u64) % U32_MAX));
            }
        }

        Operand::U32UncheckedShr => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                program.stack.push_front(BaseElement::from(
                    (a_int / 2_i32.pow(b_int as u32) as u64) % U32_MAX,
                ));
            }
        }

        Operand::U32UncheckedShrImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();

                program
                    .stack
                    .push_front(BaseElement::from((a_int / 2_i32.pow(*b) as u64) % U32_MAX));
            }
        }

        Operand::U32CheckedRotr => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                program
                    .stack
                    .push_front(BaseElement::from((a_int as u32).rotate_right(b_int as u32)));
            }
        }

        Operand::U32CheckedRotrImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();
                let b_int = *b as u32;

                program
                    .stack
                    .push_front(BaseElement::from((a_int as u32).rotate_right(b_int)));
            }
        }

        Operand::U32UncheckedRotr => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int() as u32;
                let b_int = b.as_int() as u32;

                program
                    .stack
                    .push_front(BaseElement::from((a_int as u32).rotate_right(b_int as u32)));
            }
        }

        Operand::U32UncheckedRotrImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int() as u32;
                let b_int = *b as u32;

                program
                    .stack
                    .push_front(BaseElement::from((a_int as u32).rotate_right(b_int)));
            }
        }

        Operand::U32CheckedRotl => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                program
                    .stack
                    .push_front(BaseElement::from((a_int as u32).rotate_left(b_int as u32)));
            }
        }

        Operand::U32CheckedRotlImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();
                let b_int = *b as u32;

                program
                    .stack
                    .push_front(BaseElement::from((a_int as u32).rotate_left(b_int)));
            }
        }

        Operand::U32UncheckedRotl => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                program
                    .stack
                    .push_front(BaseElement::from((a_int as u32).rotate_left(b_int as u32)));
            }
        }

        Operand::U32UncheckedRotlImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();
                let b_int = *b as u32;

                program
                    .stack
                    .push_front(BaseElement::from((a_int as u32).rotate_left(b_int)));
            }
        }

        Operand::U32CheckedPopcnt => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();

                program
                    .stack
                    .push_front(BaseElement::from(((a_int as u32) as u32).count_ones()));
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
