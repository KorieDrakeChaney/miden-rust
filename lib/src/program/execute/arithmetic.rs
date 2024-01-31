use std::ops::Neg;

use math::{fields::f64::BaseElement, FieldElement};

use crate::{Instruction, MidenProgram};

pub fn execute_arithmetic(program: &mut MidenProgram, operand: &Instruction) {
    match operand {
        Instruction::Add => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                program.stack.push_front(a + b);
            }
        }
        Instruction::Sub => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                program.stack.push_front(a - b);
            }
        }
        Instruction::Mul => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                program.stack.push_front(a * b);
            }
        }
        Instruction::Div => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                program.stack.push_front(a / b);
            }
        }
        Instruction::AddImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                program.stack.push_front(a + *b);
            }
        }
        Instruction::SubImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                program.stack.push_front(a - *b);
            }
        }
        Instruction::MulImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                program.stack.push_front(a * *b);
            }
        }
        Instruction::DivImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                program.stack.push_front(a / *b);
            }
        }
        Instruction::Neg => {
            if let Some(a) = program.stack.pop_front() {
                program.stack.push_front(a.neg());
            }
        }
        Instruction::Inv => {
            if let Some(a) = program.stack.pop_front() {
                program.stack.push_front(a.inv());
            }
        }

        Instruction::Pow2 => {
            if let Some(a) = program.stack.pop_front() {
                program
                    .stack
                    .push_front(BaseElement::from(2_u64).exp(a.into()));
            }
        }

        Instruction::Exp => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                program.stack.push_front(a.exp(b.into()));
            }
        }

        Instruction::ExpImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                program.stack.push_front(a.exp(*b));
            }
        }

        Instruction::Increment => {
            if let Some(a) = program.stack.pop_front() {
                program.stack.push_front(a + BaseElement::ONE);
            }
        }

        Instruction::Decrement => {
            if let Some(a) = program.stack.pop_front() {
                program.stack.push_front(a - BaseElement::ONE);
            }
        }

        _ => {}
    }
}
