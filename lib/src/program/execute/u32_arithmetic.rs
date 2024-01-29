use math::{fields::f64::BaseElement, StarkField};

use crate::{MidenProgram, Operand};

use super::utils::U32_MAX;

pub fn execute_u32_arithmetic(program: &mut MidenProgram, operand: &Operand) {
    match operand {
        Operand::U32CheckedAdd => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();
                let c_int = a_int + b_int;

                program.stack.push_front(BaseElement::from(c_int));
            }
        }
        Operand::U32CheckedAddImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();
                let c_int = a_int + *b as u64;

                program.stack.push_front(BaseElement::from(c_int));
            }
        }

        Operand::U32OverflowingAdd => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();
                let c_int = a_int + b_int;
                let d = c_int / U32_MAX;

                program.stack.push_front(BaseElement::from(c_int % U32_MAX));
                program.stack.push_front(BaseElement::from(d));
            }
        }
        Operand::U32OverflowingAddImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();
                let c_int = (a_int + *b as u64) % U32_MAX;

                program.stack.push_front(BaseElement::from(c_int));
            }
        }

        Operand::U32WrappingAdd => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();
                let c_int = (a_int + b_int) % U32_MAX;

                program.stack.push_front(BaseElement::from(c_int));
            }
        }
        Operand::U32WrappingAddImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();
                let c_int = (a_int + *b as u64) % U32_MAX;

                program.stack.push_front(BaseElement::from(c_int));
            }
        }

        Operand::U32OverflowingAdd3 => {
            if let (Some(c), Some(b), Some(a)) = (
                program.stack.pop_front(),
                program.stack.pop_front(),
                program.stack.pop_front(),
            ) {
                let a_int = a.as_int();
                let b_int = b.as_int();
                let c_int = c.as_int();
                let sum = a_int + b_int + c_int;
                let d = sum % U32_MAX;
                let e = sum / U32_MAX;

                program.stack.push_front(BaseElement::from(d));
                program.stack.push_front(BaseElement::from(e));
            }
        }

        Operand::U32WrappingAdd3 => {
            if let (Some(c), Some(b), Some(a)) = (
                program.stack.pop_front(),
                program.stack.pop_front(),
                program.stack.pop_front(),
            ) {
                let a_int = a.as_int();
                let b_int = b.as_int();
                let c_int = c.as_int();
                let sum = a_int + b_int + c_int;
                let d = sum % U32_MAX;

                program.stack.push_front(BaseElement::from(d));
            }
        }

        Operand::U32CheckedSub => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();
                let c_int = a_int - b_int;

                program.stack.push_front(BaseElement::from(c_int));
            }
        }

        Operand::U32CheckedSubImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();
                let b_int = *b as u64;

                program.stack.push_front(BaseElement::from(a_int - b_int));
            }
        }

        Operand::U32OverflowingSub => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();
                let c_int = a_int - b_int;
                let d = a_int < b_int;

                program.stack.push_front(BaseElement::from(c_int % U32_MAX));
                program.stack.push_front(BaseElement::from(d as u64));
            }
        }

        Operand::U32OverflowingSubImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();
                let b_int = *b as u64;
                let c_int = a_int - b_int;
                let d = a_int < b_int;

                program.stack.push_front(BaseElement::from(c_int % U32_MAX));
                program.stack.push_front(BaseElement::from(d as u64));
            }
        }

        Operand::U32WrappingSub => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();
                let c_int = a_int - b_int;

                program.stack.push_front(BaseElement::from(c_int % U32_MAX));
            }
        }

        Operand::U32WrappingSubImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();
                let b_int = *b as u64;
                let c_int = a_int - b_int;

                program.stack.push_front(BaseElement::from(c_int % U32_MAX));
            }
        }

        Operand::U32CheckedMul => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();
                let c_int = a_int * b_int;

                program.stack.push_front(BaseElement::from(c_int));
            }
        }

        Operand::U32CheckedMulImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();
                let b_int = *b as u64;
                let c_int = a_int * b_int;

                program.stack.push_front(BaseElement::from(c_int));
            }
        }

        Operand::U32OverflowingMul => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();
                let c_int = a_int * b_int;
                let d = c_int / U32_MAX;

                program.stack.push_front(BaseElement::from(c_int % U32_MAX));
                program.stack.push_front(BaseElement::from(d));
            }
        }

        Operand::U32OverflowingMulImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();
                let b_int = *b as u64;
                let c_int = a_int * b_int;
                let d = c_int / U32_MAX;

                program.stack.push_front(BaseElement::from(c_int % U32_MAX));
                program.stack.push_front(BaseElement::from(d));
            }
        }

        Operand::U32WrappingMul => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();
                let c_int = a_int * b_int;

                program.stack.push_front(BaseElement::from(c_int % U32_MAX));
            }
        }

        Operand::U32WrappingMulImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();
                let b_int = *b as u64;
                let c_int = a_int * b_int;

                program.stack.push_front(BaseElement::from(c_int % U32_MAX));
            }
        }

        Operand::U32OverflowingMadd => {
            if let (Some(b), Some(a), Some(c)) = (
                program.stack.pop_front(),
                program.stack.pop_front(),
                program.stack.pop_front(),
            ) {
                let a_int = a.as_int();
                let b_int = b.as_int();
                let c_int = c.as_int();

                program
                    .stack
                    .push_front(BaseElement::from((a_int + b_int + c_int) % U32_MAX));
                program
                    .stack
                    .push_front(BaseElement::from((a_int + b_int + c_int) / U32_MAX));
            }
        }

        Operand::U32WrappingMadd => {
            if let (Some(b), Some(a), Some(c)) = (
                program.stack.pop_front(),
                program.stack.pop_front(),
                program.stack.pop_front(),
            ) {
                let a_int = a.as_int();
                let b_int = b.as_int();
                let c_int = c.as_int();

                program
                    .stack
                    .push_front(BaseElement::from((a_int + b_int + c_int) % U32_MAX));
            }
        }

        Operand::U32CheckedDiv => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                program.stack.push_front(BaseElement::from(a_int / b_int));
            }
        }

        Operand::U32CheckedDivImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();
                let b_int = *b as u64;

                program.stack.push_front(BaseElement::from(a_int / b_int));
            }
        }

        Operand::U32UncheckedDiv => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                program.stack.push_front(BaseElement::from(a_int / b_int));
            }
        }

        Operand::U32UncheckedDivImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();
                let b_int = *b as u64;

                program.stack.push_front(BaseElement::from(a_int / b_int));
            }
        }

        Operand::U32CheckedMod => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                program.stack.push_front(BaseElement::from(a_int % b_int));
            }
        }

        Operand::U32CheckedModImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();
                let b_int = *b as u64;

                program.stack.push_front(BaseElement::from(a_int % b_int));
            }
        }

        Operand::U32UncheckedMod => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                program.stack.push_front(BaseElement::from(a_int % b_int));
            }
        }

        Operand::U32UncheckedModImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();
                let b_int = *b as u64;

                program.stack.push_front(BaseElement::from(a_int % b_int));
            }
        }

        Operand::U32CheckedDivMod => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                program.stack.push_front(BaseElement::from(a_int / b_int));
                program.stack.push_front(BaseElement::from(a_int % b_int));
            }
        }

        Operand::U32CheckedDivModImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();
                let b_int = *b as u64;

                program.stack.push_front(BaseElement::from(a_int / b_int));
                program.stack.push_front(BaseElement::from(a_int % b_int));
            }
        }

        Operand::U32UncheckedDivMod => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();
                program.stack.push_front(BaseElement::from(a_int / b_int));
                program.stack.push_front(BaseElement::from(a_int % b_int));
            }
        }

        Operand::U32UncheckedDivModImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();
                let b_int = *b as u64;

                program.stack.push_front(BaseElement::from(a_int / b_int));
                program.stack.push_front(BaseElement::from(a_int % b_int));
            }
        }

        _ => {}
    }
}
