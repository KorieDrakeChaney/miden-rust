use std::ops::Neg;

use math::{fields::f64::BaseElement, FieldElement};

use crate::{MidenProgram, Operand};

pub fn execute_arithmetic(program: &mut MidenProgram, operand: &Operand) {
    match operand {
        Operand::Add => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                program.stack.push_front(a + b);
            }
        }
        Operand::Sub => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                program.stack.push_front(a - b);
            }
        }
        Operand::Mul => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                program.stack.push_front(a * b);
            }
        }
        Operand::Div => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                program.stack.push_front(a / b);
            }
        }
        Operand::AddImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                program.stack.push_front(a + *b);
            }
        }
        Operand::SubImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                program.stack.push_front(a - *b);
            }
        }
        Operand::MulImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                program.stack.push_front(a * *b);
            }
        }
        Operand::DivImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                program.stack.push_front(a / *b);
            }
        }
        Operand::Neg => {
            if let Some(a) = program.stack.pop_front() {
                program.stack.push_front(a.neg());
            }
        }
        Operand::Inv => {
            if let Some(a) = program.stack.pop_front() {
                program.stack.push_front(a.inv());
            }
        }

        Operand::Pow2 => {
            if let Some(a) = program.stack.pop_front() {
                program
                    .stack
                    .push_front(BaseElement::from(2_u64).exp(a.into()));
            }
        }

        Operand::Exp => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                program.stack.push_front(a.exp(b.into()));
            }
        }

        Operand::ExpImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                program.stack.push_front(a.exp(*b));
            }
        }

        Operand::Increment => {
            if let Some(a) = program.stack.pop_front() {
                program.stack.push_front(a + BaseElement::ONE);
            }
        }

        Operand::Decrement => {
            if let Some(a) = program.stack.pop_front() {
                program.stack.push_front(a - BaseElement::ONE);
            }
        }

        _ => {}
    }
}
